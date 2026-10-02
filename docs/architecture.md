# Architecture and scope

## Purpose

PermBridge is intended to compare a developer's desired permissions with the
effective security posture of AI coding agents. It addresses incompatible
permission models, configuration formats, approval behavior, and capabilities
across tools. It is not designed as a universal sandbox or generic agent
firewall.

The conceptual flow is:

```text
Desired policy -> canonical model -> agent adapter -> effective posture
               -> comparison -> diagnostics -> report
```

The Core remains UI- and language-agnostic. Agent integrations must describe
what they can observe and where their mapping is incomplete. A less restrictive
or unknown effective posture must never be presented as equivalent without
evidence.

## Planned responsibilities

| Component | Responsibility | Status |
| --- | --- | --- |
| Canonical Policy Model | Represent desired capabilities and decisions without agent-specific syntax | Decision enum only |
| Policy Loader & Validator | Parse a versioned YAML policy and reject invalid or unsafe input | Planned |
| Agent Adapter Interface | Define a common contract for agent-specific observation and capability limits | Planned |
| Configuration Importer | Read native agent configuration through an adapter | Planned |
| Policy Comparator | Compare desired permissions with observed effective posture | Planned |
| Diagnostics Engine | Describe gaps, ambiguity, unsupported rules, and remediation options | Planned |
| Audit/Report Layer | Present evidence and findings without overstating guarantees | Planned |
| CLI | Provide a unified developer experience | Startup smoke program only |

The future TypeScript VS Code extension will be a UI client. It must not own
the canonical policy semantics or silently invent an agent capability.

## Trust and security boundaries

An adapter's evidence is limited by the underlying agent's native controls
and observable configuration. A supported mapping still requires tests against
that agent's actual behavior. If a rule is unsupported or ambiguous, the
comparison should say so. Enforcement or mediation may be added only where a
specific integration can reliably implement it. PermBridge itself is not a
general-purpose action interceptor.

The planned policy sources are a global user policy and an optional project
policy. Project policy may increase restrictions, but must not weaken global
restrictions. Unknown desired actions default to ASK. Native agents may not
have exact ALLOW, ASK, and DENY equivalents; adapters will need to preserve
that distinction in their results. See [policy concepts](policy-model.md).

## Implemented scope

This migration commit retains the small executable skeleton: a Rust workspace,
a canonical `Decision` enum with ordering and tests, and a CLI that prints a
scaffold notice. It adds no policy parser, adapter, importer, comparator,
diagnostics, report, interception, approval flow, or VS Code implementation.
Draft YAML examples are not consumed by the code. The repository provides no
security protection in this state.

## Internationalization

Core values are language-neutral. Future user interfaces should localize
English and Spanish messages outside Core. The current CLI prints an English
development notice only and is not an operational interface.
