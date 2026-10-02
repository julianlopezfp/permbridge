# Roadmap

This sequence records intended work, not release commitments.

| Stage | Deliverable | State |
| --- | --- | --- |
| 1 | Agent-agnostic in-memory policy types, observation vocabulary, and adapter contract | Implemented; experimental API |
| 2 | Versioned YAML schema, loader, validation, and scope-merging rules | Planned |
| 3 | Read-only Codex and Claude Code file inspection, then native behavior evidence | Both file adapters implemented experimentally; runtime behavior tests planned |
| 4 | In-memory comparator, then diagnostics and useful CLI report | Comparator implemented experimentally; diagnostics and CLI report planned |
| 5 | Broader tested agent support and localized VS Code UX | Planned |

The Codex and Claude Code adapters read selected local configuration, with
explicit uncertainty and no enforcement claim. Claude Code's tool-specific
rules reveal that broad canonical capabilities cannot yet express every native
decision. Domain and command selector semantics, managed/session precedence,
and incomplete native configuration need further design before operational
comparison or diagnostics. The current [comparator](comparison.md) preserves
those gaps instead of treating them as passing results.

PermBridge will not be a universal sandbox. Any future enforcement or
mediation claim must be scoped to a specific agent, capability, and verified
mechanism.

The licensing and authorship foundation is established before these product
milestones. Its per-version Change Date is independent of this roadmap; see
[licensing](licensing.md). External code contributions are not planned.
