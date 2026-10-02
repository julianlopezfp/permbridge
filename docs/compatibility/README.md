# Agent compatibility

| Agent | Current support |
| --- | --- |
| [Codex](codex.md) | Experimental, read-only inspection of selected local configuration files |
| [Claude Code](claude-code.md) | Experimental, read-only inspection of selected local settings files; broad mappings remain ambiguous |

Neither adapter proves a running agent's effective security boundary. They
inspect different native models and do not claim feature parity.
The [Core comparator](../comparison.md) assesses each posture separately
against the same desired canonical policy; it does not rank the agents.
