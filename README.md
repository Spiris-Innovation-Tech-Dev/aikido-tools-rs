# Aikido Security CLI & MCP Server

Rust workspace for interacting with the [Aikido Security](https://www.aikido.dev/) API.

## Crates

| Crate | Description |
|-------|-------------|
| `aikido` | Core client library — OAuth2, typed models, response caching |
| `aikido-cli` | CLI tool (`aikido`) for querying issues, repos, compliance, etc. |
| `aikido-mcp` | MCP server for use with Claude Code and other MCP clients |

## Install the CLI

```bash
cargo install --path crates/aikido-cli
```

## Configuration

Set credentials via environment variables, config file (`~/.aikido/config.toml`), or macOS Keychain:

```bash
# Environment variables
export AIKIDO_CLIENT_ID="your-client-id"
export AIKIDO_CLIENT_SECRET="your-client-secret"
export AIKIDO_REGION="eu"  # eu, us, or me

# Or macOS Keychain
security add-generic-password -s aikido-cli -a client_id -w "your-client-id"
security add-generic-password -s aikido-cli -a client_secret -w "your-client-secret"
```

## Usage

```bash
aikido issues                              # List open issue groups
aikido issue <GROUP_ID>                    # Detail with file:line ranges, CVEs, CWE
aikido issue-ignore <GROUP_ID> --reason .. # Dismiss a finding
aikido repos                               # List scanned repositories
aikido compliance-iso                      # ISO 27001 compliance overview
aikido api get "/endpoint?param=value"     # Raw API passthrough
```

Use `--format json` for machine-readable output or `--format toon` for structured text.

## Claude Code Skill

Install the Aikido Security skill in Claude Code so it knows how to use the CLI:

```
/install-skill <repo-url>
```

This teaches Claude Code the exact CLI syntax for querying vulnerabilities, triaging issues, checking compliance, and more.

## MCP Server

Run the MCP server for use with Claude Code or any MCP-compatible client:

```bash
cargo install --path crates/aikido-mcp
aikido-mcp
```
