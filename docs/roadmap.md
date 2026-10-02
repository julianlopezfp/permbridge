# Roadmap

This sequence records intended work, not release commitments.

| Stage | Deliverable | State |
| --- | --- | --- |
| 1 | Agent-agnostic in-memory policy types, observation vocabulary, and adapter contract | Implemented; experimental API |
| 2 | Versioned YAML schema, loader, validation, and scope-merging rules | Planned |
| 3 | Read-only Codex file inspection, then native behavior evidence | File inspection implemented experimentally; runtime behavior tests planned |
| 4 | Comparator, diagnostics, and useful CLI report | Planned |
| 5 | Additional tested adapters and localized VS Code UX | Planned |

The first Codex adapter reads selected local configuration, with explicit
uncertainty and no enforcement claim. No Claude Code adapter exists. Domain
and command selector semantics, managed/session precedence, and failure
behavior for incomplete native
configuration must be settled before implementing the loader or comparator.

PermBridge will not be a universal sandbox. Any future enforcement or
mediation claim must be scoped to a specific agent, capability, and verified
mechanism.

The licensing and authorship foundation is established before these product
milestones. Its per-version Change Date is independent of this roadmap; see
[licensing](licensing.md). External code contributions are not planned.
