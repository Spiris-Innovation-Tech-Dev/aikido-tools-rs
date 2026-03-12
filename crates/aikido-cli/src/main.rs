use aikido::config::{Config, ConfigOverrides, Region};
use aikido::AikidoClient;
use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod models;
mod output;

use commands::Command;
use output::Formattable;

#[derive(Parser)]
#[command(name = "aikido")]
#[command(about = "CLI for Aikido Security API", long_about = None)]
struct Cli {
    /// Aikido region: eu, us, me
    #[arg(long, env = "AIKIDO_REGION")]
    region: Option<String>,

    /// Workspace alias from ~/.aikido/config.toml
    #[arg(long, env = "AIKIDO_WORKSPACE")]
    workspace: Option<String>,

    /// OAuth2 client ID
    #[arg(long, env = "AIKIDO_CLIENT_ID")]
    client_id: Option<String>,

    /// Output format: pretty, json, toon
    #[arg(long, value_enum, default_value = "pretty")]
    format: output::OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show workspace information
    Workspace(commands::workspace::WorkspaceArgs),

    /// List open issue groups
    Issues(commands::issues::IssueGroupsListArgs),

    /// Get issue group detail
    Issue(commands::issues::IssueGroupGetArgs),

    /// Ignore/dismiss an issue group
    IssueIgnore(commands::issues::IssueIgnoreArgs),

    /// Unignore an issue group
    IssueUnignore(commands::issues::IssueUnignoreArgs),

    /// Get issue counts
    IssueCounts(commands::issues::IssueCountsArgs),

    /// Export all issues
    IssueExport(commands::issues::IssueExportArgs),

    /// List code repositories
    Repos(commands::repositories::ReposListArgs),

    /// Get code repository detail
    Repo(commands::repositories::RepoGetArgs),

    /// List containers
    Containers(commands::containers::ContainersListArgs),

    /// List connected clouds
    Clouds(commands::clouds::CloudsListArgs),

    /// List domains
    Domains(commands::domains::DomainsListArgs),

    /// List teams
    Teams(commands::teams::TeamsListArgs),

    /// List users
    Users(commands::users::UsersListArgs),

    /// List firewall apps
    FirewallApps(commands::firewall::FirewallAppsListArgs),

    /// ISO 27001 compliance overview
    ComplianceIso(commands::reports::ComplianceIsoArgs),

    /// SOC2 compliance overview
    ComplianceSoc2(commands::reports::ComplianceSoc2Args),

    /// NIS2 compliance overview
    ComplianceNis2(commands::reports::ComplianceNis2Args),

    /// Activity log
    ActivityLog(commands::reports::ActivityLogArgs),

    /// CI scans
    CiScans(commands::reports::CiScansArgs),

    /// Raw API passthrough for full endpoint coverage
    Api {
        #[command(subcommand)]
        command: commands::api::ApiCommands,
    },
}

fn build_client(cli: &Cli) -> Result<AikidoClient> {
    let region_override = cli
        .region
        .as_ref()
        .map(|r| r.parse::<Region>())
        .transpose()?;

    let config = Config::load(ConfigOverrides {
        region: region_override,
        workspace: cli.workspace.clone(),
        client_id: cli.client_id.clone(),
        client_secret: None,
    })?;

    let region = config.region();
    let credentials = config.credentials()?;

    Ok(AikidoClient::new(region, credentials))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let format = cli.format;

    if let Commands::Workspace(args) = &cli.command {
        if !args.requires_client() {
            println!(
                "{}",
                args.execute_local(cli.workspace.as_deref())?
                    .format(format)?
            );
            return Ok(());
        }
    }

    let client = build_client(&cli)?;

    match cli.command {
        Commands::Workspace(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Issues(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Issue(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::IssueIgnore(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::IssueUnignore(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::IssueCounts(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::IssueExport(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::Repos(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Repo(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Containers(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::Clouds(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Domains(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Teams(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Users(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::FirewallApps(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::ComplianceIso(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::ComplianceSoc2(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::ComplianceNis2(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::ActivityLog(args) => {
            println!("{}", args.execute(&client).await?.format(format)?)
        }
        Commands::CiScans(args) => println!("{}", args.execute(&client).await?.format(format)?),
        Commands::Api { command } => {
            let out = match command {
                commands::api::ApiCommands::Ops(args) => {
                    args.execute(&client).await?.format(format)?
                }
                commands::api::ApiCommands::Exec(args) => {
                    args.execute(&client).await?.format(format)?
                }
                commands::api::ApiCommands::Get(args) => {
                    args.execute(&client).await?.format(format)?
                }
                commands::api::ApiCommands::Post(args) => {
                    args.execute(&client).await?.format(format)?
                }
                commands::api::ApiCommands::Put(args) => {
                    args.execute(&client).await?.format(format)?
                }
                commands::api::ApiCommands::Delete(args) => {
                    args.execute(&client).await?.format(format)?
                }
            };
            println!("{out}");
        }
    }

    Ok(())
}
