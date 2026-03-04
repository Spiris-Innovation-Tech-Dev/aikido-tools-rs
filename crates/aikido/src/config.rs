use crate::error::{AikidoError, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
        "https://app.aikido.io/api/oauth"
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionConfig {
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

    fn apply_env_vars(&mut self) {
        if let Ok(region) = env::var("AIKIDO_REGION") {
            if let Ok(r) = region.parse() {
                self.connection.region = Some(r);
            }
        }
        if let Ok(id) = env::var("AIKIDO_CLIENT_ID") {
            self.connection.client_id = Some(id);
        }
        if let Ok(secret) = env::var("AIKIDO_CLIENT_SECRET") {
            self.connection.client_secret = Some(secret);
        }
    }

    fn apply_overrides(&mut self, overrides: ConfigOverrides) {
        if let Some(region) = overrides.region {
            self.connection.region = Some(region);
        }
        if let Some(id) = overrides.client_id {
            self.connection.client_id = Some(id);
        }
        if let Some(secret) = overrides.client_secret {
            self.connection.client_secret = Some(secret);
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.connection.client_id.is_none() {
            // Try keychain before failing
            if Self::read_keychain("client_id").is_err() {
                return Err(AikidoError::Auth(
                    "Client ID is required. Set AIKIDO_CLIENT_ID env var, config file, or store in keychain".into(),
                ));
            }
        }
        if self.connection.client_secret.is_none() {
            if Self::read_keychain("client_secret").is_err() {
                return Err(AikidoError::Auth(
                    "Client secret is required. Set AIKIDO_CLIENT_SECRET env var, config file, or store in keychain".into(),
                ));
            }
        }
        Ok(())
    }

    pub fn region(&self) -> Region {
        self.connection.region.unwrap_or_default()
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

        let client_id = self
            .connection
            .client_id
            .clone()
            .or_else(|| Self::read_keychain("client_id").ok())
            .ok_or_else(|| AikidoError::Auth("Client ID not found".into()))?;

        let client_secret = self
            .connection
            .client_secret
            .clone()
            .or_else(|| Self::read_keychain("client_secret").ok())
            .ok_or_else(|| AikidoError::Auth("Client secret not found".into()))?;

        Ok(Credentials::ClientCredentials {
            client_id,
            client_secret,
        })
    }

    fn read_keychain(account: &str) -> Result<String> {
        let output = std::process::Command::new("security")
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
        assert_eq!(Region::Eu.oauth_url(), "https://app.aikido.io/api/oauth");
        assert_eq!(Region::Us.oauth_url(), "https://app.aikido.io/api/oauth");
        assert_eq!(Region::Me.oauth_url(), "https://app.aikido.io/api/oauth");
    }
}
