use crate::output::Formattable;
use colored::*;
use serde::Serialize;

// ========== Workspace ==========

#[derive(Debug, Serialize)]
pub struct WorkspaceOutput {
    pub id: i64,
    pub name: String,
    pub provider: Option<String>,
    pub org_name: Option<String>,
}

impl Formattable for WorkspaceOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!("{}\n", "Workspace".bright_cyan().bold()));
        out.push_str(&format!("  {} {}\n", "ID:".bold(), self.id));
        out.push_str(&format!("  {} {}\n", "Name:".bold(), self.name));
        if let Some(p) = &self.provider {
            out.push_str(&format!("  {} {}\n", "Provider:".bold(), p));
        }
        if let Some(o) = &self.org_name {
            out.push_str(&format!("  {} {}\n", "Org:".bold(), o));
        }
        Ok(out)
    }
}

// ========== Issue Groups ==========

#[derive(Debug, Serialize)]
pub struct IssueGroupsOutput {
    pub groups: Vec<IssueGroupRow>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct IssueGroupRow {
    pub id: i64,
    pub title: String,
    pub severity: String,
    pub severity_score: i32,
    pub issue_type: String,
    pub status: String,
    pub locations: Vec<LocationRow>,
}

impl Formattable for IssueGroupsOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!(
            "{}\n",
            format!("Open issue groups ({})", self.total)
                .bright_green()
                .bold()
        ));
        out.push_str(&format!("{}\n", "═".repeat(80).bright_black()));

        for g in &self.groups {
            let severity_colored = match g.severity.as_str() {
                "critical" => g.severity.red().bold().to_string(),
                "high" => g.severity.yellow().bold().to_string(),
                "medium" => g.severity.cyan().to_string(),
                "low" => g.severity.green().to_string(),
                _ => g.severity.clone(),
            };

            out.push_str(&format!(
                "\n{} {} {}\n",
                severity_colored,
                g.title.bold(),
                format!("(ID: {})", g.id).dimmed()
            ));
            out.push_str(&format!(
                "  {} {} | {} {} | {} {}\n",
                "Type:".bright_black(),
                g.issue_type,
                "Score:".bright_black(),
                g.severity_score,
                "Status:".bright_black(),
                g.status,
            ));
            if !g.locations.is_empty() {
                let locs: Vec<String> = g.locations.iter().map(|l| l.name.clone()).collect();
                out.push_str(&format!(
                    "  {} {}\n",
                    "Locations:".bright_black(),
                    locs.join(", "),
                ));
            }
        }
        Ok(out)
    }
}

// ========== Issue Group Detail ==========

#[derive(Debug, Serialize)]
pub struct IssueGroupDetailOutput {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub severity: String,
    pub severity_score: i32,
    pub issue_type: String,
    pub status: String,
    pub time_to_fix_minutes: Option<i32>,
    pub locations: Vec<LocationRow>,
    pub how_to_fix: Option<String>,
    pub related_cve_ids: Vec<String>,
    pub issues: Vec<IssueRow>,
}

#[derive(Debug, Serialize)]
pub struct LocationRow {
    pub id: i64,
    pub name: String,
    pub location_type: String,
}

#[derive(Debug, Serialize)]
pub struct IssueRow {
    pub id: i64,
    pub severity: String,
    pub severity_score: i32,
    pub status: String,
    pub affected_package: Option<String>,
    pub affected_file: Option<String>,
    pub start_line: Option<i64>,
    pub end_line: Option<i64>,
    pub cve_id: Option<String>,
    pub code_repo_name: Option<String>,
    pub container_repo_name: Option<String>,
    pub cloud_name: Option<String>,
    pub domain_name: Option<String>,
    pub programming_language: Option<String>,
    pub installed_version: Option<String>,
    pub patched_versions: Vec<String>,
    pub cwe_classes: Vec<String>,
}

impl Formattable for IssueGroupDetailOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        let severity_colored = match self.severity.as_str() {
            "critical" => self.severity.red().bold().to_string(),
            "high" => self.severity.yellow().bold().to_string(),
            "medium" => self.severity.cyan().to_string(),
            "low" => self.severity.green().to_string(),
            _ => self.severity.clone(),
        };

        out.push_str(&format!(
            "{} {} {}\n",
            severity_colored,
            self.title.bold(),
            format!("(ID: {})", self.id).dimmed()
        ));
        out.push_str(&format!(
            "  {} {} | {} {} | {} {}\n",
            "Type:".bright_black(),
            self.issue_type,
            "Score:".bright_black(),
            self.severity_score,
            "Status:".bright_black(),
            self.status,
        ));
        if let Some(mins) = self.time_to_fix_minutes {
            out.push_str(&format!(
                "  {} {}min\n",
                "Est. fix time:".bright_black(),
                mins,
            ));
        }
        if let Some(desc) = &self.description {
            out.push_str(&format!("\n  {}\n", desc.dimmed()));
        }
        if !self.locations.is_empty() {
            out.push_str(&format!("\n  {}\n", "Locations:".bold()));
            for loc in &self.locations {
                out.push_str(&format!(
                    "    {} {} {}\n",
                    "●".bright_blue(),
                    loc.name,
                    format!("[{}, ID: {}]", loc.location_type, loc.id).dimmed(),
                ));
            }
        }
        if !self.related_cve_ids.is_empty() {
            out.push_str(&format!(
                "\n  {} {}\n",
                "CVEs:".bold(),
                self.related_cve_ids.join(", ")
            ));
        }
        if let Some(fix) = &self.how_to_fix {
            out.push_str(&format!("\n  {}\n  {}\n", "How to fix:".bold(), fix));
        }
        if !self.issues.is_empty() {
            out.push_str(&format!(
                "\n  {} ({})\n",
                "Issues:".bold(),
                self.issues.len()
            ));
            for i in &self.issues {
                let sev = match i.severity.as_str() {
                    "critical" => i.severity.red().bold().to_string(),
                    "high" => i.severity.yellow().bold().to_string(),
                    "medium" => i.severity.cyan().to_string(),
                    "low" => i.severity.green().to_string(),
                    _ => i.severity.clone(),
                };
                out.push_str(&format!(
                    "    {} #{} [{}]\n",
                    sev,
                    i.id.to_string().bold(),
                    i.status,
                ));
                if let Some(file) = &i.affected_file {
                    let location = match (i.start_line, i.end_line) {
                        (Some(start), Some(end)) if start == end => format!("{}:{}", file, start),
                        (Some(start), Some(end)) => format!("{}:{}-{}", file, start, end),
                        (Some(start), None) => format!("{}:{}", file, start),
                        _ => file.clone(),
                    };
                    out.push_str(&format!("      {} {}\n", "File:".bright_black(), location));
                }
                if let Some(pkg) = &i.affected_package {
                    let version_info = match &i.installed_version {
                        Some(v) => format!("{} ({})", pkg, v),
                        None => pkg.clone(),
                    };
                    out.push_str(&format!("      {} {}\n", "Package:".bright_black(), version_info));
                }
                if !i.patched_versions.is_empty() {
                    out.push_str(&format!(
                        "      {} {}\n",
                        "Fix:".bright_black(),
                        i.patched_versions.join(", "),
                    ));
                }
                if let Some(cve) = &i.cve_id {
                    out.push_str(&format!("      {} {}\n", "CVE:".bright_black(), cve));
                }
                if !i.cwe_classes.is_empty() {
                    out.push_str(&format!(
                        "      {} {}\n",
                        "CWE:".bright_black(),
                        i.cwe_classes.join(", "),
                    ));
                }
                if let Some(repo) = &i.code_repo_name {
                    out.push_str(&format!("      {} {}\n", "Repo:".bright_black(), repo));
                }
                if let Some(container) = &i.container_repo_name {
                    out.push_str(&format!(
                        "      {} {}\n",
                        "Container:".bright_black(),
                        container
                    ));
                }
                if let Some(cloud) = &i.cloud_name {
                    out.push_str(&format!("      {} {}\n", "Cloud:".bright_black(), cloud));
                }
                if let Some(domain) = &i.domain_name {
                    out.push_str(&format!("      {} {}\n", "Domain:".bright_black(), domain));
                }
                if let Some(lang) = &i.programming_language {
                    out.push_str(&format!("      {} {}\n", "Language:".bright_black(), lang));
                }
            }
        }
        Ok(out)
    }
}

// ========== Code Repos ==========

#[derive(Debug, Serialize)]
pub struct CodeReposOutput {
    pub repos: Vec<CodeRepoRow>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct CodeRepoRow {
    pub id: i64,
    pub name: String,
    pub provider: String,
    pub branch: Option<String>,
}

impl Formattable for CodeReposOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!(
            "{}\n",
            format!("Code repositories ({})", self.total)
                .bright_green()
                .bold()
        ));
        out.push_str(&format!("{}\n", "═".repeat(80).bright_black()));

        for r in &self.repos {
            out.push_str(&format!(
                "\n  {} {} {}\n",
                "●".bright_blue(),
                r.name.bold(),
                format!("(ID: {})", r.id).dimmed()
            ));
            out.push_str(&format!(
                "    {} {}",
                "Provider:".bright_black(),
                r.provider
            ));
            if let Some(branch) = &r.branch {
                out.push_str(&format!(" | {} {}", "Branch:".bright_black(), branch));
            }
            out.push('\n');
        }
        Ok(out)
    }
}

// ========== Containers ==========

#[derive(Debug, Serialize)]
pub struct ContainersOutput {
    pub containers: Vec<ContainerRow>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct ContainerRow {
    pub id: i64,
    pub name: String,
    pub provider: String,
    pub tag: Option<String>,
}

impl Formattable for ContainersOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!(
            "{}\n",
            format!("Container repositories ({})", self.total)
                .bright_green()
                .bold()
        ));
        out.push_str(&format!("{}\n", "═".repeat(80).bright_black()));

        for c in &self.containers {
            out.push_str(&format!(
                "\n  {} {} {}\n",
                "●".bright_blue(),
                c.name.bold(),
                format!("(ID: {})", c.id).dimmed()
            ));
            out.push_str(&format!(
                "    {} {}",
                "Provider:".bright_black(),
                c.provider
            ));
            if let Some(tag) = &c.tag {
                out.push_str(&format!(" | {} {}", "Tag:".bright_black(), tag));
            }
            out.push('\n');
        }
        Ok(out)
    }
}

// ========== Clouds ==========

#[derive(Debug, Serialize)]
pub struct CloudsOutput {
    pub clouds: Vec<CloudRow>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct CloudRow {
    pub id: i64,
    pub name: String,
    pub provider: String,
    pub environment: String,
}

impl Formattable for CloudsOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!(
            "{}\n",
            format!("Connected clouds ({})", self.total)
                .bright_green()
                .bold()
        ));
        out.push_str(&format!("{}\n", "═".repeat(80).bright_black()));

        for c in &self.clouds {
            out.push_str(&format!(
                "\n  {} {} {}\n",
                "●".bright_blue(),
                c.name.bold(),
                format!("(ID: {})", c.id).dimmed()
            ));
            out.push_str(&format!(
                "    {} {} | {} {}\n",
                "Provider:".bright_black(),
                c.provider,
                "Env:".bright_black(),
                c.environment,
            ));
        }
        Ok(out)
    }
}

// ========== Domains ==========

#[derive(Debug, Serialize)]
pub struct DomainsOutput {
    pub domains: Vec<DomainRow>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct DomainRow {
    pub id: i64,
    pub name: String,
}

impl Formattable for DomainsOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!(
            "{}\n",
            format!("Domains ({})", self.total).bright_green().bold()
        ));
        out.push_str(&format!("{}\n", "═".repeat(80).bright_black()));

        for d in &self.domains {
            out.push_str(&format!(
                "\n  {} {} {}\n",
                "●".bright_blue(),
                d.name.bold(),
                format!("(ID: {})", d.id).dimmed()
            ));
        }
        Ok(out)
    }
}

// ========== Teams ==========

#[derive(Debug, Serialize)]
pub struct TeamsOutput {
    pub teams: Vec<TeamRow>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct TeamRow {
    pub id: i64,
    pub name: String,
    pub responsibilities_count: usize,
}

impl Formattable for TeamsOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!(
            "{}\n",
            format!("Teams ({})", self.total).bright_green().bold()
        ));
        out.push_str(&format!("{}\n", "═".repeat(80).bright_black()));

        for t in &self.teams {
            out.push_str(&format!(
                "\n  {} {} {} — {} responsibilities\n",
                "●".bright_blue(),
                t.name.bold(),
                format!("(ID: {})", t.id).dimmed(),
                t.responsibilities_count,
            ));
        }
        Ok(out)
    }
}

// ========== Users ==========

#[derive(Debug, Serialize)]
pub struct UsersOutput {
    pub users: Vec<UserRow>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct UserRow {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub active: bool,
}

impl Formattable for UsersOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        let mut out = String::new();
        out.push_str(&format!(
            "{}\n",
            format!("Users ({})", self.total).bright_green().bold()
        ));
        out.push_str(&format!("{}\n", "═".repeat(80).bright_black()));

        for u in &self.users {
            let status = if u.active {
                "active".green()
            } else {
                "inactive".red()
            };
            out.push_str(&format!(
                "\n  {} {} {} [{}]\n",
                status,
                u.name.bold(),
                format!("(ID: {})", u.id).dimmed(),
                u.role.as_deref().unwrap_or("unknown"),
            ));
            if let Some(email) = &u.email {
                out.push_str(&format!("    {} {}\n", "Email:".bright_black(), email));
            }
        }
        Ok(out)
    }
}

// ========== Generic JSON ==========

#[derive(Debug, Serialize)]
pub struct JsonOutput {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

impl Formattable for JsonOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(&self.data)?)
    }

    fn format_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(&self.data)?)
    }

    fn format_toon(&self) -> anyhow::Result<String> {
        let options = toon_rs::Options::default();
        Ok(toon_rs::encode_to_string(&self.data, &options)?)
    }
}

// ========== Message ==========

#[derive(Debug, Serialize)]
pub struct MessageOutput {
    pub message: String,
}

impl Formattable for MessageOutput {
    fn format_pretty(&self) -> anyhow::Result<String> {
        Ok(format!("{}", self.message.bright_green()))
    }
}
