# Security model

**Current protection: none.** PermBridge does not intercept agent actions,
enforce permissions, import live configuration, or compare effective posture.
The Codex and Claude Code adapters read selected local files, but cannot
verify a running session's sandbox, approvals, or managed constraints.

## Trust boundaries

The future comparison starts with a developer's desired policy and an
adapter's observations of a particular agent. An adapter may see only part of
the native configuration or may be unable to prove how a setting behaves at
runtime. `NotConfigured`, `Unsupported`, `Ambiguous`, and missing observations
must never be converted into a passing comparison. A `Known` observation
records the adapter's interpretation, but still needs evidence and comparison before any
equivalence claim.

`Decision` describes permission intent. `EnforcementStrength` separately
describes the mechanism an adapter observed for one capability. A declared
setting, a mediated tool call, and an OS sandbox boundary have different trust
properties. The enum is not a blanket security ranking; a sandbox claim must
be scoped to the specific operation and verified integration behavior.

The future loader should reject malformed or unrecognized policy input
explicitly. Until it exists, YAML examples are inert. The intended
global/project rule is to retain the more restrictive comparable decision;
managed and session policy precedence is not yet defined. No fallback should
silently grant permission because a mapping or observation is absent.

## Non-goals and claims

PermBridge is a compatibility and posture analysis layer, not a universal
sandbox or generic AI-agent firewall. Mediation may be possible only where a
specific agent supplies a reliable integration point. Both current adapters
are inspection-only. Codex `Declared` observations are not proof of OS
sandbox enforcement; Claude Code's tool-level settings yield no `Known` broad
capability decisions. No security guarantee or operational recommendation is
supported today. See [agent compatibility](compatibility/README.md).

The security limitations above are independent of permission to use the
project. BSL permits non-production evaluation but does not make this
experimental code suitable for security-critical use. See
[licensing](licensing.md) and the private reporting path in
[SECURITY.md](../SECURITY.md).
