use crate::error::{AikidoError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[cfg(target_os = "macos")]
const SECURITY_COMMAND_PATH: &str = "/usr/bin/security";
#[cfg(not(target_os = "macos"))]
const SECURITY_COMMAND_PATH: &str = "security";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Region {
    #[default]
    Eu,
    Us,
    Me,
}

impl Region {
    pub fn api_base_url(&self) -> &str {
        match self {
            Region::Eu => "https://app.aikido.dev/api/public/v1",
            Region::Us => "https://app.us.aikido.dev/api/public/v1",
            Region::Me => "https://app.me.aikido.dev/api/public/v1",
        }
    }

    pub fn oauth_url(&self) -> &str {
        "https://app.aikido.dev/api/oauth"
    }
}

impl FromStr for Region {
    type Err = AikidoError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "eu" => Ok(Region::Eu),
            "us" => Ok(Region::Us),
            "me" => Ok(Region::Me),
            _ => Err(AikidoError::General(format!(
                "Invalid region '{}'. Valid options: eu, us, me",
                s
            ))),
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::Eu => write!(f, "eu"),
            Region::Us => write!(f, "us"),
            Region::Me => write!(f, "me"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Credentials {
    ClientCredentials {
        client_id: String,
        client_secret: String,
    },
    Token(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub connection: ConnectionConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub workspaces: BTreeMap<String, WorkspaceConfig>,
    #[serde(default)]
    pub active_workspace: Option<String>,
    #[serde(skip)]
    selected_workspace: Option<String>,
    #[serde(skip)]
    override_region: Option<Region>,
    #[serde(skip)]
    override_client_id: Option<String>,
    #[serde(skip)]
    override_client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionConfig {
    pub region: Option<Region>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceConfig {
    pub region: Option<Region>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultsConfig {
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    pub region: Option<Region>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub workspace: Option<String>,
}

impl Config {
    pub fn load(overrides: ConfigOverrides) -> Result<Self> {
        let mut config = Self::load_global().unwrap_or_default();
        config.apply_env_vars();
        config.apply_overrides(overrides);
        Ok(config)
    }

    pub fn load_and_validate(overrides: ConfigOverrides) -> Result<Self> {
        let config = Self::load(overrides)?;
        config.validate()?;
        Ok(config)
    }

    pub fn load_global_file() -> Result<Self> {
        Self::load_global()
    }

    fn load_global() -> Result<Self> {
        let config_path = Self::global_config_path()?;
        Self::load_from_file(&config_path)
    }

    fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents)
            .map_err(|e| AikidoError::General(format!("Config parse error: {e}")))
    }

    pub fn global_config_dir() -> Result<PathBuf> {
        let home_dir = directories::BaseDirs::new()
            .ok_or_else(|| AikidoError::General("Could not determine home directory".into()))?
            .home_dir()
            .to_path_buf();
        Ok(home_dir.join(".aikido"))
    }

    pub fn global_config_path() -> Result<PathBuf> {
        Ok(Self::global_config_dir()?.join("config.toml"))
    }

    fn global_config_path_hint() -> String {
        Self::global_config_path()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "~/.aikido/config.toml".to_string())
    }

    fn workspace_for_selection(&self) -> Result<Option<&WorkspaceConfig>> {
        let Some(name) = self.selected_workspace_name() else {
            return Ok(None);
        };

        self.workspaces.get(name).map(Some).ok_or_else(|| {
            AikidoError::General(format!(
                "Workspace '{}' is not configured in {}",
                name,
                Self::global_config_path_hint()
            ))
        })
    }

    pub fn selected_workspace_name(&self) -> Option<&str> {
        self.selected_workspace
            .as_deref()
            .or(self.active_workspace.as_deref())
    }

    pub fn active_workspace_name(&self) -> Option<&str> {
        self.active_workspace.as_deref()
    }

    pub fn set_active_workspace(&mut self, workspace: Option<String>) -> Result<()> {
        match workspace {
            Some(name) => {
                let trimmed = name.trim();
                if trimmed.is_empty() {
                    return Err(AikidoError::General(
                        "Workspace alias must not be empty".to_string(),
                    ));
                }
                if !self.workspaces.contains_key(trimmed) {
                    return Err(AikidoError::General(format!(
                        "Workspace '{}' is not configured in {}",
                        trimmed,
                        Self::global_config_path_hint()
                    )));
                }
                self.active_workspace = Some(trimmed.to_string());
            }
            None => {
                self.active_workspace = None;
            }
        }
        Ok(())
    }

    pub fn save_global(&self) -> Result<()> {
        let path = Self::global_config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut serializable = self.clone();
        serializable.selected_workspace = None;

        let data = toml::to_string_pretty(&serializable)
            .map_err(|e| AikidoError::General(format!("Config serialize error: {e}")))?;
        fs::write(path, data)?;
        Ok(())
    }

    fn apply_env_vars(&mut self) {
        if let Ok(workspace) = env::var("AIKIDO_WORKSPACE") {
            let trimmed = workspace.trim();
            if !trimmed.is_empty() {
                self.selected_workspace = Some(trimmed.to_string());
            }
        }
        if let Ok(region) = env::var("AIKIDO_REGION") {
            if let Ok(r) = region.parse() {
                self.override_region = Some(r);
            }
        }
        if let Ok(id) = env::var("AIKIDO_CLIENT_ID") {
            self.override_client_id = Some(id);
        }
        if let Ok(secret) = env::var("AIKIDO_CLIENT_SECRET") {
            self.override_client_secret = Some(secret);
        }
    }

    fn apply_overrides(&mut self, overrides: ConfigOverrides) {
        if let Some(workspace) = overrides.workspace {
            let trimmed = workspace.trim();
            if trimmed.is_empty() {
                self.selected_workspace = None;
            } else {
                self.selected_workspace = Some(trimmed.to_string());
            }
        }
        if let Some(region) = overrides.region {
            self.override_region = Some(region);
        }
        if let Some(id) = overrides.client_id {
            self.override_client_id = Some(id);
        }
        if let Some(secret) = overrides.client_secret {
            self.override_client_secret = Some(secret);
        }
    }

    pub fn validate(&self) -> Result<()> {
        let workspace = self.workspace_for_selection()?;

        let has_client_id = self.override_client_id.is_some()
            || workspace.and_then(|w| w.client_id.as_deref()).is_some();
        let has_client_id = has_client_id || self.connection.client_id.is_some();
        if !has_client_id {
            // Try keychain before failing
            if Self::read_keychain("client_id").is_err() {
                return Err(AikidoError::Auth(
                    "Client ID is required. Set AIKIDO_CLIENT_ID env var, config file, or store in keychain".into(),
                ));
            }
        }

        let has_client_secret = self.override_client_secret.is_some()
            || workspace.and_then(|w| w.client_secret.as_deref()).is_some();
        let has_client_secret = has_client_secret || self.connection.client_secret.is_some();
        if !has_client_secret {
            if Self::read_keychain("client_secret").is_err() {
                return Err(AikidoError::Auth(
                    "Client secret is required. Set AIKIDO_CLIENT_SECRET env var, config file, or store in keychain".into(),
                ));
            }
        }
        Ok(())
    }

    pub fn region(&self) -> Region {
        self.override_region
            .or_else(|| {
                self.selected_workspace_name()
                    .and_then(|name| self.workspaces.get(name))
                    .and_then(|workspace| workspace.region)
            })
            .or(self.connection.region)
            .unwrap_or_default()
    }

    pub fn credentials(&self) -> Result<Credentials> {
        // Check for pre-obtained token file
        if let Ok(token_file) = env::var("AIKIDO_TOKEN_FILE") {
            let token = fs::read_to_string(&token_file)
                .map_err(|e| AikidoError::Io(e))?
                .trim()
                .to_string();
            return Ok(Credentials::Token(token));
        }

        let workspace = self.workspace_for_selection()?;

        let client_id = self
            .override_client_id
            .clone()
            .or_else(|| workspace.and_then(|w| w.client_id.clone()))
            .or_else(|| self.connection.client_id.clone())
            .or_else(|| Self::read_keychain("client_id").ok())
            .ok_or_else(|| AikidoError::Auth("Client ID not found".into()))?;

        let client_secret = self
            .override_client_secret
            .clone()
            .or_else(|| workspace.and_then(|w| w.client_secret.clone()))
            .or_else(|| self.connection.client_secret.clone())
            .or_else(|| Self::read_keychain("client_secret").ok())
            .ok_or_else(|| AikidoError::Auth("Client secret not found".into()))?;

        Ok(Credentials::ClientCredentials {
            client_id,
            client_secret,
        })
    }

    fn read_keychain(account: &str) -> Result<String> {
        let output = std::process::Command::new(SECURITY_COMMAND_PATH)
            .args([
                "find-generic-password",
                "-s",
                "aikido-cli",
                "-a",
                account,
                "-w",
            ])
            .output()
            .map_err(|e| AikidoError::Keychain(format!("Failed to run security command: {e}")))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(AikidoError::Keychain(format!(
                "Keychain item 'aikido-cli/{account}' not found"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_from_str() {
        assert_eq!("eu".parse::<Region>().unwrap(), Region::Eu);
        assert_eq!("US".parse::<Region>().unwrap(), Region::Us);
        assert_eq!("me".parse::<Region>().unwrap(), Region::Me);
        assert!("invalid".parse::<Region>().is_err());
    }

    #[test]
    fn region_api_urls() {
        assert_eq!(
            Region::Eu.api_base_url(),
            "https://app.aikido.dev/api/public/v1"
        );
        assert_eq!(
            Region::Us.api_base_url(),
            "https://app.us.aikido.dev/api/public/v1"
        );
        assert_eq!(
            Region::Me.api_base_url(),
            "https://app.me.aikido.dev/api/public/v1"
        );
    }

    #[test]
    fn region_display() {
        assert_eq!(Region::Eu.to_string(), "eu");
        assert_eq!(Region::Us.to_string(), "us");
        assert_eq!(Region::Me.to_string(), "me");
    }

    #[test]
    fn oauth_host_is_normalized() {
        assert_eq!(Region::Eu.oauth_url(), "https://app.aikido.dev/api/oauth");
        assert_eq!(Region::Us.oauth_url(), "https://app.aikido.dev/api/oauth");
        assert_eq!(Region::Me.oauth_url(), "https://app.aikido.dev/api/oauth");
    }

    #[test]
    fn selected_workspace_prefers_explicit_override() {
        let config = Config {
            active_workspace: Some("prod".to_string()),
            selected_workspace: Some("staging".to_string()),
            ..Default::default()
        };

        assert_eq!(config.selected_workspace_name(), Some("staging"));
    }

    #[test]
    fn region_uses_selected_workspace_config() {
        let mut workspaces = BTreeMap::new();
        workspaces.insert(
            "staging".to_string(),
            WorkspaceConfig {
                region: Some(Region::Us),
                client_id: None,
                client_secret: None,
            },
        );
        let config = Config {
            workspaces,
            active_workspace: Some("staging".to_string()),
            ..Default::default()
        };

        assert_eq!(config.region(), Region::Us);
    }

    #[test]
    fn credentials_use_selected_workspace_config() {
        let mut workspaces = BTreeMap::new();
        workspaces.insert(
            "prod".to_string(),
            WorkspaceConfig {
                region: Some(Region::Eu),
                client_id: Some("workspace-id".to_string()),
                client_secret: Some("workspace-secret".to_string()),
            },
        );
        let config = Config {
            workspaces,
            active_workspace: Some("prod".to_string()),
            ..Default::default()
        };

        let creds = config.credentials().unwrap();
        match creds {
            Credentials::ClientCredentials {
                client_id,
                client_secret,
            } => {
                assert_eq!(client_id, "workspace-id");
                assert_eq!(client_secret, "workspace-secret");
            }
            Credentials::Token(_) => panic!("expected client credentials"),
        }
    }

    #[test]
    fn set_active_workspace_requires_configured_alias() {
        let mut workspaces = BTreeMap::new();
        workspaces.insert("prod".to_string(), WorkspaceConfig::default());
        let mut config = Config {
            workspaces,
            ..Default::default()
        };

        let error = config
            .set_active_workspace(Some("missing".to_string()))
            .unwrap_err();
        assert!(error
            .to_string()
            .contains("Workspace 'missing' is not configured"));
    }

    #[test]
    fn set_active_workspace_trims_alias() {
        let mut workspaces = BTreeMap::new();
        workspaces.insert("prod".to_string(), WorkspaceConfig::default());
        let mut config = Config {
            workspaces,
            ..Default::default()
        };

        config
            .set_active_workspace(Some("  prod  ".to_string()))
            .unwrap();
        assert_eq!(config.active_workspace_name(), Some("prod"));
    }
}
