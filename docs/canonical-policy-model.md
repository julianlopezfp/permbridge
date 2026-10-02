# Canonical policy model

**Status: experimental API.** The Rust types represent intent. A strict
version-1 YAML input maps to them; neither form enforces a policy.

`CanonicalPolicy` records a `PolicyScope`, a fallback `Decision`, and separate
filesystem, network, and execution policies. A new policy defaults every
decision to `Ask` and starts with empty domain and command rule lists. This
prevents an omitted field from implying `Allow`. The YAML loader uses the same
defaults; no runtime evaluation exists.

| Dimension | Represented intent | Current limitation |
| --- | --- | --- |
| Decision | `Allow`, `Ask`, `Deny` | Native approval states may differ |
| Filesystem | Read, write, and outside-workspace decisions | No path matching or access interception |
| Network | Default and domain-specific decisions | YAML selectors are nonempty opaque strings; matching unspecified |
| Execution | Default and command-specific decisions | YAML selectors are nonempty opaque strings; matching unspecified |
| Scope | Managed, global, project, session | Only global/project restriction rule stated; no merge engine |

`outside_workspace` is an additional boundary condition, not a substitute for
read or write. A future evaluator must combine it with the operation decision
without weakening either. For comparable global and project decisions,
`Deny > Ask > Allow`; `Decision::most_restrictive` expresses that ordering. The
precedence of managed and session policies and cross-scope conflict reporting
remain open. Malformed policy input fails loading.

An agent adapter may report an observed decision with `Declared`,
`ToolMediated`, or `OsSandbox` enforcement strength. The strength is separate
from the decision: a declared `Deny` is not evidence of the same boundary as an
OS sandbox restriction. See [adapter contract](adapter-contract.md) and
[security model](security-model.md).

`NotConfigured` now records that an adapter inspected its supported sources
but found no explicit setting. It does not imply the canonical policy's `Ask`
default or any native default. Both the [Codex](compatibility/codex.md) and
[Claude Code](compatibility/claude-code.md) file adapters use it.

Claude Code's tool-specific rules show a current limit of the broad canonical
capabilities: a rule for one tool does not describe every way to read, write,
execute, or access the network. The second adapter therefore reports
`Ambiguous` rather than manufacturing a `Known` broad decision. See
[ADR 0005](decisions/0005-claude-code-adapter-boundary.md).

The [in-memory comparator](comparison.md) uses the canonical decision order
only for defensibly mapped observations. It retains decision relationships
separately from final evidence-aware outcomes. The top-level fallback
`default_decision` does not generate additional capabilities; the five
baseline fields and explicit selector rules define what is compared.

The [version-1 format](policy-format.md) documents exact YAML fields,
defaults, and validation. The [examples](../policies/examples/README.md) are
valid inputs. Loading them does not configure or enforce any agent.

Canonical `Allow`, `Ask`, and `Deny` values describe desired agent behavior;
they are unrelated to the [license grant](licensing.md) for PermBridge itself.
