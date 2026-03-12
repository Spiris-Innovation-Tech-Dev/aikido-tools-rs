---
name: aikido-security
description: "Query and manage Aikido Security vulnerabilities, issues, repositories, compliance, and infrastructure via the `aikido` CLI. Use when the user asks about security vulnerabilities, issue groups, SAST/SCA/IaC findings, compliance status (ISO 27001, SOC2, NIS2), code repositories, containers, clouds, domains, or wants to triage/ignore/unignore security issues. Trigger phrases: 'check security issues', 'list vulnerabilities', 'show aikido issues', 'ignore this finding', 'compliance status', 'what repos are scanned', 'show issue detail', 'security posture', 'SAST findings', 'open issues', 'triage issues'."
---

# Aikido Security CLI

The `aikido` CLI queries the Aikido Security API. Commands are flat (no `list`/`get` subcommands) and have no `--limit`/`--offset` pagination flags.

## All Commands

```
aikido issues                              # List open issue groups
aikido issues --all                        # ALL findings (incl. ignored/snoozed/closed)
aikido issue <GROUP_ID>                    # Issue group detail + individual issues
aikido issue-ignore <GROUP_ID> [--reason]  # Dismiss an issue group
aikido issue-unignore <GROUP_ID>           # Reopen a dismissed issue group
aikido issue-counts                        # Aggregated counts by severity/type
aikido issue-export                        # Export all issues as JSON array
aikido repos                               # List code repositories
aikido repo <REPO_ID>                      # Repository detail
aikido containers                          # List container repositories
aikido clouds                              # Connected clouds (AWS/GCP/Azure)
aikido domains                             # Monitored domains
aikido teams                               # List teams
aikido users                               # List users
aikido firewall-apps                       # Zen/firewall apps
aikido compliance-iso                      # ISO 27001 overview
aikido compliance-soc2                     # SOC2 overview
aikido compliance-nis2                     # NIS2 overview
aikido activity-log                        # Activity log
aikido ci-scans                            # CI scan results
aikido workspace                           # Workspace info for selected credentials
aikido workspace --list                    # List configured workspace aliases
aikido workspace --use-workspace <ALIAS>   # Set default workspace alias
aikido workspace --clear-active            # Clear default workspace alias
aikido api get <ENDPOINT>                  # Raw GET  (e.g. "/repos?page=0")
aikido api post <ENDPOINT> [--body JSON]   # Raw POST
aikido api put <ENDPOINT> [--body JSON]    # Raw PUT
aikido api delete <ENDPOINT>               # Raw DELETE
```

Global flags: `--format pretty|json|toon` (default: pretty), `--region eu|us|me`, `--workspace <alias>`

## Triage Workflow

1. `aikido issues` — list open groups sorted by severity
2. `aikido issue <GROUP_ID>` — detail with affected files + line ranges, CVEs, CWE, how-to-fix
3. `aikido issue-ignore <GROUP_ID> --reason "false positive"` — dismiss

## Output

Use `--format json` for machine-readable output. Pipe to `jq` for filtering:

```bash
aikido issues --format json | jq '.[] | select(.severity == "critical")'
```

For details on issue fields (start_line, end_line, patched_versions, cwe_classes, etc.), see [references/api_reference.md](references/api_reference.md).
