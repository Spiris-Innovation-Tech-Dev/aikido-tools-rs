# Aikido CLI Command Reference

## Output Formats

All commands accept `--format <pretty|json|toon>` (default: pretty).
Use `--format json` for machine-readable output or piping to `jq`.

## Commands

### Workspace

```bash
aikido workspace
aikido workspace --list
aikido workspace --use-workspace <ALIAS>
aikido workspace --clear-active
```

### Issues

```bash
# List open issue groups (Aikido-refined)
aikido issues

# List ALL findings (including ignored/snoozed/closed)
aikido issues --all

# Issue group detail with individual issues, file:line ranges, CWE, fix guidance
aikido issue <GROUP_ID>

# Ignore/unignore
aikido issue-ignore <GROUP_ID> --reason "false positive"
aikido issue-unignore <GROUP_ID>

# Aggregated counts
aikido issue-counts

# Full export as JSON array
aikido issue-export
```

### Repositories & Infrastructure

```bash
aikido repos                # List code repositories
aikido repo <REPO_ID>       # Repository detail
aikido containers           # List container repositories
aikido clouds               # Connected clouds (AWS/GCP/Azure)
aikido domains              # Monitored domains
aikido firewall-apps        # Zen/firewall apps
```

### Teams & Users

```bash
aikido teams
aikido users
```

### Compliance & Reports

```bash
aikido compliance-iso       # ISO 27001
aikido compliance-soc2      # SOC2
aikido compliance-nis2      # NIS2
aikido activity-log
aikido ci-scans
```

### Raw API Passthrough

For any endpoint not covered by built-in commands:

```bash
aikido api get <ENDPOINT>
aikido api post <ENDPOINT> --body '<JSON>'
aikido api put <ENDPOINT> --body '<JSON>'
aikido api delete <ENDPOINT>
```

Examples:

```bash
aikido api get "/repositories/code?page=0&per_page=5"
aikido api post "/domains" --body '{"domain":"https://example.com","kind":"front_end"}'
```

## Issue Detail Fields

`aikido issue <GROUP_ID>` returns:
- **Group**: title, description, severity, type, status, locations, CVEs, how_to_fix, time_to_fix
- **Individual issues**: affected_file with line range (e.g. `file.yml:25-75`), affected_package with installed_version, patched_versions, cve_id, cwe_classes, code_repo/container/cloud/domain name, programming_language

## Global Options

| Flag | Env var | Description |
|------|---------|-------------|
| `--region` | `AIKIDO_REGION` | eu, us, or me |
| `--workspace` | `AIKIDO_WORKSPACE` | workspace alias from config |
| `--client-id` | `AIKIDO_CLIENT_ID` | OAuth2 client ID |
| `--client-secret` | `AIKIDO_CLIENT_SECRET` | OAuth2 client secret |
| `--format` | | pretty, json, toon |
