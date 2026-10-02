# Agent adapters

The experimental [Codex adapter](../crates/permbridge-codex/) implements the
`permbridge-core::AgentAdapter` contract outside the canonical model. Its
[compatibility guide](../docs/compatibility/codex.md) describes the selected
files, mappings, and evidence limits. It does not establish runtime security
or provide a CLI workflow.

Future adapters belong in separate crates and must document native behavior,
unsupported mappings, and validation evidence before broader support is
claimed. No Claude Code adapter exists.
