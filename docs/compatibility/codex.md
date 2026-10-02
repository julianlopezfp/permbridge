# Codex compatibility

**Status: implemented experimental file inspection; no runtime guarantee.**
The `permbridge-codex` crate implements `AgentAdapter` with identifier `codex`.
It reads selected local Codex TOML files and reports declared settings. It does
not launch Codex, modify configuration, inspect a running session, or compare
against desired policy. The CLI does not expose this adapter yet.

This mapping was reviewed against the official OpenAI Docs on 2026-10-02. It
targets the documented local config model shared by the Codex CLI and IDE
extension. Codex's desktop, native Windows, WSL, macOS, and Linux execution
paths have different sandbox implementations. No particular Codex binary
version or platform has been behavior-tested by this crate.

## Sources and precedence

The caller supplies a Codex home directory and optional workspace root through
`CodexConfigPaths`, plus an explicit project-trust flag. The adapter checks:

1. `<codex-home>/config.toml` for user settings.
2. `<workspace-root>/.codex/config.toml` for project settings, only when the
   caller marks the project trusted.

For each inspected key, an explicit project value takes precedence over a user
value. `CodexInspection` reports whether each file was read, missing, omitted,
or skipped for lack of trust. `SettingObservation` records the selected value
and source layer without returning file contents or absolute paths. Missing
files do not cause directories or files to be created.

OpenAI documents a broader precedence chain: CLI flags and `--config`, nested
trusted project files, selected profiles, user config, cloud-managed defaults,
system config, and built-in defaults. This adapter implements only the two
explicit file layers above. It does not establish trust itself, walk nested
project directories, or apply undocumented defaults. A caller must not treat
this limited file result as the active session configuration. See
[Config basics](https://learn.chatgpt.com/docs/config-file/config-basic).

## Selected settings and canonical mappings

| Setting | File observation | Canonical result |
| --- | --- | --- |
| `sandbox_mode = "read-only"` | Recognized local command sandbox declaration | Filesystem read inside workspace: `Known(Allow, Declared)`; workspace write and outside-workspace access: `Ambiguous` |
| `sandbox_mode = "workspace-write"` | Recognized local command sandbox declaration | Filesystem read and write inside workspace: `Known(Allow, Declared)`; outside-workspace access: `Ambiguous` |
| `sandbox_mode = "danger-full-access"` | Recognized absence of local sandbox restrictions | Filesystem read, workspace write, and outside-workspace access: `Known(Allow, Declared)` |
| `approval_policy = "on-request"` or `"never"` | Recognized approval declaration | Execution remains `Ambiguous`; approval policy is not a blanket decision for every command |
| `[sandbox_workspace_write].network_access` | Recognized Boolean for local command networking | Canonical network remains `Ambiguous`, because web search, apps, MCP, proxies, and other execution paths differ |
| No explicit relevant setting | `NotConfigured` | No `Allow`, `Ask`, `Deny`, or Codex default is inferred |
| `default_permissions`, `[permissions]`, or selected `profile` | `Unsupported` in the setting report | Canonical filesystem/network observations remain `Ambiguous` while a profile may apply |
| Domain or command selectors in the desired policy | No supported translation | `Unsupported` for each requested selector |

All `Known` observations use `EnforcementStrength::Declared`. This describes
the configuration text only. It never claims `ToolMediated` or `OsSandbox`:
the actual sandbox can vary by operating system and runtime overrides. A
read-only sandbox still permits commands and may allow approval-based
escalation, so the adapter does not turn its write restriction into canonical
`Deny` or `Ask`. An outside-workspace capability combines read and write,
which the limited sandbox mode cannot safely reduce to one decision. The
`danger-full-access` mapping records a broad declared allowance, not evidence
that every external service is available.

OpenAI distinguishes sandbox boundaries from approval prompts and documents
the three sandbox modes, command network toggle, and platform mechanisms in
[Agent approvals and security](https://learn.chatgpt.com/docs/agent-approvals-security).
The [Configuration Reference](https://learn.chatgpt.com/docs/config-file/config-reference)
defines the inspected keys and current approval values. New permission
profiles are a separate, evolving model that does not compose with legacy
sandbox settings; see [Permissions](https://learn.chatgpt.com/docs/permissions).

## Uncertainty and errors

`NotConfigured` means the inspected files lack an explicit relevant value;
it is distinct from a capability omitted from the report. A recognized but
unmapped setting is `Unsupported` in the setting report. An unfamiliar string
value is `Unknown` there and yields an ambiguous canonical observation.
Malformed TOML, invalid types for supported settings, unreadable files, and
the retired `approval_policy = "untrusted"` produce explicit `CodexError`
variants. Errors omit raw values and file paths.

The adapter does not read managed `requirements.toml`, system or cloud-managed
config, selected profile files, nested project config, command-line overrides,
session permission changes, hooks, rules, app or MCP permissions, or live
sandbox state. These can change what the agent actually does. Network domain
rules require an active proxy and cannot be inferred from a Boolean toggle.
Codex Cloud uses different controls. No canonical comparison, enforcement,
mediation, remediation, or security guarantee follows from this inspection.
