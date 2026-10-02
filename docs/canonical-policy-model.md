# Canonical policy model

**Status: experimental in-memory API.** The Rust types represent intent; they
do not define a stable YAML schema or enforce a policy.

`CanonicalPolicy` records a `PolicyScope`, a fallback `Decision`, and separate
filesystem, network, and execution policies. A new policy defaults every
decision to `Ask` and starts with empty domain and command rule lists. This
prevents an omitted in-memory field from implying `Allow`, but no parser or
runtime evaluation exists.

| Dimension | Represented intent | Current limitation |
| --- | --- | --- |
| Decision | `Allow`, `Ask`, `Deny` | Native approval states may differ |
| Filesystem | Read, write, and outside-workspace decisions | No path matching or access interception |
| Network | Default and domain-specific decisions | Domain syntax and matching unspecified |
| Execution | Default and command-specific decisions | Command syntax and matching unspecified |
| Scope | Managed, global, project, session | Only global/project restriction rule stated; no merge engine |

`outside_workspace` is an additional boundary condition, not a substitute for
read or write. A future evaluator must combine it with the operation decision
without weakening either. For comparable global and project decisions,
`Deny > Ask > Allow`; `Decision::most_restrictive` expresses that ordering. The
precedence of managed and session policies, conflict reporting, and malformed
policy behavior require decisions before a loader is built.

An agent adapter may report an observed decision with `Declared`,
`ToolMediated`, or `OsSandbox` enforcement strength. The strength is separate
from the decision: a declared `Deny` is not evidence of the same boundary as an
OS sandbox restriction. See [adapter contract](adapter-contract.md) and
[security model](security-model.md).

The illustrative [global](../policies/examples/global.yaml) and
[project](../policies/examples/project.yaml) YAML files show a possible
serialization. They are not parsed, validated, or executable. The `targets`
list in the global example does not indicate implemented agent support.

Canonical `Allow`, `Ask`, and `Deny` values describe desired agent behavior;
they are unrelated to the [license grant](licensing.md) for PermBridge itself.
