# Roadmap

This sequence records intended work, not release commitments.

| Stage | Deliverable | State |
| --- | --- | --- |
| 1 | Agent-agnostic in-memory policy types, observation vocabulary, and adapter contract | Implemented; experimental API |
| 2 | Versioned YAML schema, loader, validation, and scope-merging rules | Planned |
| 3 | One real adapter with native-configuration evidence and behavior tests | Planned |
| 4 | Comparator, diagnostics, and useful CLI report | Planned |
| 5 | Additional tested adapters and localized VS Code UX | Planned |

No real agent integration is supported now. Codex and Claude Code adapters
are intentionally outside this stage. Domain and command selector semantics,
managed/session precedence, and failure behavior for incomplete native
configuration must be settled before implementing the loader or comparator.

PermBridge will not be a universal sandbox. Any future enforcement or
mediation claim must be scoped to a specific agent, capability, and verified
mechanism.
