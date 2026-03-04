# Aikido Security CLI & MCP Server

Rust workspace for interacting with the [Aikido Security](https://www.aikido.dev/) API.

## Crates

| Crate | Description |
|-------|-------------|
| `aikido` | Core client library — OAuth2, typed models, response caching |
| `aikido-cli` | CLI tool (`aikido`) for querying issues, repos, compliance, etc. |
| `aikido-mcp` | MCP server for use with Claude Code and other MCP clients |

## Install

```bash
# CLI
cargo install --path crates/aikido-cli

# MCP server
cargo install --path crates/aikido-mcp
```

## Configuration

Credentials are resolved in order: CLI flags → environment variables → config file → macOS Keychain.

```bash
# Environment variables
export AIKIDO_CLIENT_ID="your-client-id"
export AIKIDO_CLIENT_SECRET="your-client-secret"
export AIKIDO_REGION="eu"  # eu, us, or me

# Or macOS Keychain
security add-generic-password -s aikido-cli -a client_id -w "your-client-id"
security add-generic-password -s aikido-cli -a client_secret -w "your-client-secret"

# Or config file (~/.aikido/config.toml)
# [connection]
# region = "eu"
# client_id = "your-client-id"
# client_secret = "your-client-secret"
```

## CLI

### Global Flags

| Flag | Env Var | Description |
|------|---------|-------------|
| `--region` | `AIKIDO_REGION` | API region: `eu`, `us`, or `me` |
| `--client-id` | `AIKIDO_CLIENT_ID` | OAuth2 client ID |
| `--client-secret` | `AIKIDO_CLIENT_SECRET` | OAuth2 client secret |
| `--format` | | Output format: `pretty` (default), `json`, `toon` |

### Commands

#### Issues

| Command | Description |
|---------|-------------|
| `aikido issues` | List open issue groups |
| `aikido issues --all` | All findings including ignored/snoozed/closed |
| `aikido issue <GROUP_ID>` | Issue group detail with individual issues, affected files with line ranges, CVEs, CWE classes, and remediation guidance |
| `aikido issue-ignore <GROUP_ID> [--reason "..."]` | Dismiss an issue group |
| `aikido issue-unignore <GROUP_ID>` | Reopen a dismissed issue group |
| `aikido issue-counts` | Aggregated counts by severity and type |
| `aikido issue-export` | Export all issues as a JSON array |

#### Repositories & Infrastructure

| Command | Description |
|---------|-------------|
| `aikido repos` | List code repositories |
| `aikido repo <REPO_ID>` | Code repository detail |
| `aikido containers` | List container repositories |
| `aikido clouds` | List connected cloud accounts (AWS/GCP/Azure) |
| `aikido domains` | List monitored domains |
| `aikido firewall-apps` | List Zen/firewall apps |

#### Teams & Users

| Command | Description |
|---------|-------------|
| `aikido teams` | List teams |
| `aikido users` | List users |

#### Compliance & Reports

| Command | Description |
|---------|-------------|
| `aikido compliance-iso` | ISO 27001 compliance overview |
| `aikido compliance-soc2` | SOC2 compliance overview |
| `aikido compliance-nis2` | NIS2 compliance overview |
| `aikido activity-log` | Activity log |
| `aikido ci-scans` | CI scan results |

#### Other

| Command | Description |
|---------|-------------|
| `aikido workspace` | Workspace info (name, provider, org) |
| `aikido api get <ENDPOINT>` | Raw GET request to any API endpoint |
| `aikido api post <ENDPOINT> [--body JSON]` | Raw POST request |
| `aikido api put <ENDPOINT> [--body JSON]` | Raw PUT request |
| `aikido api delete <ENDPOINT>` | Raw DELETE request |

### Examples

```bash
# List critical issues
aikido issues --format json | jq '[.[] | select(.severity == "critical")]'

# Get detail for a specific issue group
aikido issue 20858361

# Dismiss a false positive
aikido issue-ignore 20858317 --reason "false positive"

# Raw API access for endpoints without a dedicated command
aikido api get "/repositories/code?page=0&per_page=5"
aikido api post "/domains" --body '{"domain":"https://example.com","kind":"front_end"}'
```

## MCP Server

The MCP server exposes the same functionality as the CLI as MCP tools, for use with Claude Code, Claude Desktop, or any MCP-compatible client.

```bash
aikido-mcp
```

Set `MCP_MAX_RESULTS` to control the default page size (default: 50).

### Tools

#### Issues

| Tool | Description |
|------|-------------|
| `aikido_issues_list` | List open issue groups with severity, type, status, and locations |
| `aikido_issues_group_get` | Issue group detail with all individual issues, affected files, packages, repos, and remediation |
| `aikido_issues_get` | Get a specific issue by ID |
| `aikido_issues_counts` | Issue counts aggregated by severity and type |
| `aikido_issues_export` | Export all issues as JSON |
| `aikido_issues_snooze` | Snooze an issue until a date |
| `aikido_issues_unsnooze` | Unsnooze an issue |
| `aikido_issues_ignore` | Ignore an issue with optional reason |
| `aikido_issues_unignore` | Unignore an issue |
| `aikido_issues_group_snooze` | Snooze an issue group until a date |
| `aikido_issues_group_unsnooze` | Unsnooze an issue group |
| `aikido_issues_group_ignore` | Ignore an issue group with optional reason |
| `aikido_issues_group_unignore` | Unignore an issue group |
| `aikido_issues_group_notes_add` | Add a note/comment to an issue group |
| `aikido_issues_group_tasks` | Get tasks linked to an issue group |

#### Repositories

| Tool | Description |
|------|-------------|
| `aikido_repos_list` | List code repositories with name, provider, branch, and scan status |
| `aikido_repos_get` | Code repository detail |
| `aikido_repos_scan` | Trigger a scan for a code repository |
| `aikido_repos_sast_rules` | List SAST rules |
| `aikido_repos_iac_rules` | List IaC rules |
| `aikido_repos_custom_rules` | List custom SAST rules |

#### Containers

| Tool | Description |
|------|-------------|
| `aikido_containers_list` | List container repositories |
| `aikido_containers_get` | Container repository detail |
| `aikido_containers_scan` | Trigger a scan for a container repository |

#### Infrastructure

| Tool | Description |
|------|-------------|
| `aikido_clouds_list` | List connected cloud environments |
| `aikido_domains_list` | List domains configured for surface monitoring |
| `aikido_domains_scan` | Start a scan for a domain |

#### Teams & Users

| Tool | Description |
|------|-------------|
| `aikido_teams_list` | List all teams |
| `aikido_teams_create` | Create a new team |
| `aikido_users_list` | List all users |
| `aikido_users_get` | User detail |

#### Firewall

| Tool | Description |
|------|-------------|
| `aikido_firewall_apps_list` | List Zen/firewall apps |
| `aikido_firewall_app_get` | Firewall app detail |

#### Compliance & Reports

| Tool | Description |
|------|-------------|
| `aikido_compliance_iso` | ISO 27001 compliance overview |
| `aikido_compliance_soc2` | SOC2 compliance overview |
| `aikido_compliance_nis2` | NIS2 compliance overview |
| `aikido_activity_log` | Activity log |
| `aikido_ci_scans` | CI scan results |

#### Raw API Passthrough

| Tool | Description |
|------|-------------|
| `aikido_api_get` | Raw GET request to any endpoint |
| `aikido_api_post` | Raw POST request |
| `aikido_api_put` | Raw PUT request |
| `aikido_api_delete` | Raw DELETE request |

## Agent Skill

Install the Aikido Security skill so your coding agent knows how to use the CLI:

```bash
npx skills add Spiris-Innovation-Tech-Dev/aikido-tools-rs
```

Works with Claude Code, Cursor, Codex, OpenCode, and [37 more agents](https://github.com/vercel-labs/skills#supported-agents).

## Architecture

The core `aikido` crate handles:

- **OAuth2 client_credentials** authentication with token caching
- **In-memory response cache** with TTL tiers (issues: 60s, metadata: 5min, rules: 10min, compliance: 5min)
- **Automatic cache invalidation** on write operations (POST/PUT/DELETE)
- **Retry with exponential backoff** for transient failures (429, 500, 502, 503, 504)
- **Typed models** for all API resources with `#[serde(flatten)] extra: Value` for forward compatibility
