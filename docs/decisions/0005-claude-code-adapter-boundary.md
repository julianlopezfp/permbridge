# ADR 0005: Keep Claude Code tool rules outside broad canonical decisions

## Status

Accepted, 2026-10-02. The adapter and canonical API remain experimental.

## Context

Codex exposes a small set of sandbox modes that permit limited declared
filesystem observations. Claude Code combines tool-specific permission
rules, session modes, hooks, and a shell-only sandbox. The current canonical
capabilities describe broad filesystem, network, and execution behavior.
Equating one tool rule or sandbox switch with an entire capability would
overstate what the inspected files establish.

## Decision

Keep Core and `AgentAdapter` unchanged. The Claude Code adapter reports
selected setting declarations and source provenance in its own inspection
report. It maps relevant broad capabilities to `Ambiguous`, absent settings
to `NotConfigured`, and unsupported desired selectors to `Unsupported`.
No `Known` decision or enforcement strength is asserted from these files.

For scalar keys, apply the documented local > project > user precedence among
inspected files. Preserve all contributing sources for merged rule lists.
Require caller-supplied roots and project trust; do not read Claude Code's
private trust store or execute native tooling to discover runtime behavior.

## Consequences

The second adapter exercises the provider-agnostic boundary without adding
Claude-specific types to Core. Its evidence is useful for later diagnostics,
but the current broad model cannot express tool-specific permission behavior
precisely. The current comparator preserves uncertainty; operational
comparison or a narrower capability model requires explicit selector semantics
and behavior evidence before making stronger equivalence claims.
See the [compatibility guide](../compatibility/claude-code.md).
