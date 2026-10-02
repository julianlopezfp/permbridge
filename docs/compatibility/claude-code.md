# Claude Code compatibility

**Status: implemented experimental file inspection; no runtime guarantee.**
`permbridge-claude-code` implements `AgentAdapter` with identifier
`claude-code`. It reads selected JSON settings only. It does not launch Claude
Code, change settings, execute hooks, compare a desired policy, or verify a
running session. The CLI does not expose this adapter.

The mapping was reviewed against the official Anthropic documentation on
2026-10-02. It targets local Claude Code settings for terminal, IDE, and local
desktop sessions; cloud sessions load a different subset. No Claude Code
binary, account plan, or operating system was behavior-tested. Mode behavior
and settings locations vary by version and platform: the `manual` mode alias
requires v2.1.200, repository-root placement of local settings began in
v2.1.211 with documented exceptions, and the shell sandbox runs on macOS,
Linux, and WSL2 but not native Windows.

## Sources and precedence

The caller supplies a configuration directory, optional workspace root, and
project-trust flag through `ClaudeConfigPaths`. The adapter checks:

1. `<config-dir>/settings.json` (normally `~/.claude/settings.json`).
2. `<workspace-root>/.claude/settings.json` (shared project).
3. `<workspace-root>/.claude/settings.local.json` (project local).

It reads both project files only when the caller marks the workspace trusted.
This is a conservative inspection choice, not an implementation of Claude
Code's detailed trust rules: an untracked local file may apply without a trust
dialog, while a tracked one may not. The caller must supply the actual
workspace root; Claude Code can resolve local settings differently on Windows,
in worktrees, and when started below a repository root. The adapter does not
discover trust from `~/.claude.json`, read that file, or run Git.

For scalar settings, the inspected files use local > shared project > user.
For permission rule lists, the report preserves every contributing source
because Claude Code merges lists rather than replacing them. Source statuses
distinguish read, missing, omitted, and skipped files. Reports contain
normalized labels and source layers, never raw rules, paths, or file contents.
Missing files create nothing. When the workspace root is omitted or its files
are skipped for lack of trust, canonical capabilities remain `Ambiguous`
because source coverage is incomplete. Anthropic also documents managed settings,
command-line `--settings`, and session changes above these files; none are
inspected. The report must not be treated as active session configuration.
See [settings files and precedence](https://code.claude.com/docs/en/settings).

## Inspected settings and canonical mapping

| Setting | File observation | Canonical result |
| --- | --- | --- |
| `permissions.defaultMode` | Recognizes `default`/`manual`, `acceptEdits`, `plan`, `auto`, `dontAsk`, and `bypassPermissions`; unfamiliar strings are `Unknown` | Broad filesystem, network, and execution capabilities are `Ambiguous` |
| `permissions.allow`, `ask`, `deny` | Validates arrays of strings and reports source presence; rule text is not retained | Broad capabilities are `Ambiguous` |
| `permissions.additionalDirectories` | Validates a string array; does not resolve paths | Filesystem capabilities are `Ambiguous` |
| `sandbox.enabled`, `failIfUnavailable`, `allowUnsandboxedCommands` | Recognizes Boolean declarations | Relevant broad capabilities are `Ambiguous`; no OS boundary is claimed |
| `sandbox.filesystem`, `sandbox.network`, `hooks` | Reports presence as `Unsupported` detail | Relevant broad capabilities are `Ambiguous` |
| No relevant setting in all supplied, inspected files | `NotConfigured` | No native or canonical default is inferred |
| Requested domain or command selector | No supported translation | `Unsupported` for that selector |

There are currently no `Known` canonical decisions or enforcement-strength
claims from this adapter. This is intentional. Claude Code evaluates tool
rules with deny > ask > allow precedence, but a `Bash` rule does not describe
all execution, a `WebFetch` rule does not describe all network access, and a
file-tool rule does not cover shell commands. Permission modes change which
tool calls prompt or auto-approve; they are not blanket decisions for every
canonical capability. The sandbox protects shell commands, while built-in
file and web tools, hooks, and MCP servers run outside it. A configured
`sandbox.enabled` value is therefore neither a workspace-wide OS boundary nor
proof that the sandbox started. Anthropic documents the mode and rule behavior
in [permissions](https://code.claude.com/docs/en/permissions) and the boundary
and fallback behavior in [sandboxing](https://code.claude.com/docs/en/sandboxing).

## Uncertainty, privacy, and errors

`NotConfigured` means only that these inspected files contain no explicit
setting relevant to that capability. An unfamiliar permission-mode string is
reported as `Unknown` and maps to `Ambiguous`. The adapter validates JSON and
the types of the selected fields; malformed JSON, invalid selected types,
and unreadable files produce explicit `ClaudeError` variants without raw
values or absolute paths. It does not validate the syntax or behavior of
individual permission rules. Unexpected unrelated keys are ignored.

The adapter does not inspect managed or server settings, `~/.claude.json`,
CLI flags, environment variables, session approvals, resolved trust, nested
project files, skills, plugins, MCP configuration, hook programs, or live
sandbox state. These can alter actual behavior. Hooks can change tool-call
decisions, and managed settings can override local declarations. Anthropic's
[security](https://code.claude.com/docs/en/security) and
[hooks](https://code.claude.com/docs/en/hooks) documentation describes these
boundaries. The [Core comparator](../comparison.md) can consume this posture,
but its broad ambiguous observations yield no passing outcome. No
enforcement, remediation, or security guarantee follows from static inspection.
