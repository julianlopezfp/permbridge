# Architecture

PermBridge compares desired, agent-agnostic permissions with an AI coding
agent's observed effective posture. It separates desired policy, native-agent
inspection, and comparison so an adapter cannot silently claim that a native
setting provides a security guarantee it has not established.

```text
desired policy -> canonical model -> agent adapter -> effective posture
                                       |                  |
                                       +-- evidence limits-+
                                                          v
                                          comparator -> diagnostics -> report
```

## Component boundaries

| Component | Responsibility | Current state |
| --- | --- | --- |
| Canonical model | Represent desired decisions for filesystem, network, and execution capabilities | Implemented in memory; experimental API |
| Policy loader and validator | Parse versioned YAML and reject invalid policies | Planned |
| Adapter contract | Identify an agent and request observations relevant to a desired policy | Implemented trait; experimental API |
| Agent adapters and configuration importers | Read native settings and report supported, unsupported, or ambiguous mappings | Experimental Codex and Claude Code file adapters |
| Comparator | Relate each observed capability to the desired rule | Planned; outcome vocabulary exists only |
| Diagnostics and report layer | Explain evidence, gaps, and remediation | Planned |
| CLI | Present a unified developer workflow | Smoke program only |
| VS Code extension | Localized UI over Core results | Planned; placeholder only |

Here, **implemented** means code exists, **experimental** means its API or
semantics may change, **planned** means there is no behavior yet, and **not
supported** means users cannot rely on an integration today.

`permbridge-core` contains the in-memory model and contract. It has no UI,
provider SDK, or YAML dependency. The future CLI and extension must consume
Core results rather than implement their own policy semantics. Real adapters
belong outside the domain model; see [ADR 0002](decisions/0002-agent-adapters.md).
`permbridge-codex` separates file reading, TOML parsing, and conservative
normalization. Its paths and project trust are supplied by the caller. See
[Codex compatibility](compatibility/codex.md).

`permbridge-claude-code` similarly separates JSON file reading and
normalization, but reports broad capabilities as ambiguous because its native
rules are tool-specific. See [Claude Code compatibility](compatibility/claude-code.md).

## Data and failure boundaries

An adapter receives a `CanonicalPolicy` and returns an `EffectivePosture` or
an explicit adapter-specific error. Each returned capability is known with a
decision and enforcement strength, not configured, unsupported, or ambiguous.
A missing capability means it was not inspected. These states must remain
distinct when comparison and diagnostics are implemented. The adapter does
not return an `Equivalent` result; the future comparator owns that judgment.

Both adapters report selected file settings and their source alongside the
canonical posture. Codex `Known` observations use `Declared`, never an
unverified runtime enforcement strength. Claude Code reports no `Known`
decisions from static settings. Managed configuration and session overrides
remain outside these integrations.

The policy model stores domain and command selectors but does not validate or
match them. No default or scope precedence is applied to an agent today.
`Decision::most_restrictive` only captures the ordering needed for a future
global/project merge. Managed and session scope precedence remains open.

For security limits, see [security model](security-model.md). For why the
canonical model is preferred over pairwise translations, see
[ADR 0001](decisions/0001-canonical-policy-model.md).

The product's source license and the permissions represented by Core are
separate concerns. See [licensing](licensing.md) for the BSL grant and
authorship; no Core decision grants rights to use PermBridge in production.
