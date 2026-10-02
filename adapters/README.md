# Agent adapters

The experimental [Codex adapter](../crates/permbridge-codex/) and
[Claude Code adapter](../crates/permbridge-claude-code/) implement the
`permbridge-core::AgentAdapter` contract outside the canonical model. Their
[compatibility guides](../docs/compatibility/README.md) describe the selected
files, mappings, and evidence limits. Neither establishes runtime security or
provides a CLI workflow.

Future adapters belong in separate crates and must document native behavior,
unsupported mappings, and validation evidence before broader support is
claimed.
