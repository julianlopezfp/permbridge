[English](README.md) | [Español](README.es.md)

# AgentGuard

AgentGuard is an early-stage project for a local security and policy layer for
AI coding agents working in local repositories. The intended decision for an
agent action is **ALLOW**, **ASK**, or **DENY**.

> **Current status (v0.1 skeleton):** AgentGuard does not yet load policies,
> evaluate actions, intercept tools, enforce decisions, request approval, or
> integrate with VS Code. Do not use this skeleton as a security boundary.

## What exists today

- A Rust workspace with `agentguard-core` and a local `agentguard-cli` scaffold.
- A shared `Decision` type whose default is `Ask` and whose ordering expresses
  the intended restriction precedence.
- Unit tests, CI configuration, draft YAML examples, and a TypeScript extension
  placeholder without an initialized package.

Running the CLI prints a scaffold notice. It does not inspect a repository or
make a policy decision.

## Direction

AgentGuard Core will remain independent of UI and programming language. A
global user policy and an optional project policy are planned. The project
policy may make a decision more restrictive but may never weaken a global
restriction. The intended precedence is **DENY > ASK > ALLOW**, and an action
that matches no rule should return **ASK**. These are design requirements, not
implemented enforcement behavior.

Future user-facing interfaces should support English and Spanish. Core
decision values are language-neutral; interfaces will localize their own text.

## Development

Install the stable Rust toolchain with the `rustfmt` and `clippy` components.
No Node.js or VS Code setup is needed for this skeleton.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo run -p agentguard-cli
```

The CLI command is a smoke test only. There is no installation, configuration,
or operational usage path yet. Files under `policies/examples/` are draft
design examples and are not consumed by the code.

## Repository map

| Path | Purpose |
| --- | --- |
| `crates/agentguard-core/` | UI-agnostic Rust decision types; future policy engine |
| `crates/agentguard-cli/` | Local development CLI scaffold |
| `extensions/vscode/` | Future TypeScript extension placeholder |
| `policies/examples/` | Draft, nonfunctional YAML examples |
| `docs/` | Canonical English technical design documentation |
| `tests/` | Reserved for cross-crate integration tests |
| `.github/workflows/` | Rust CI |

See [architecture](docs/architecture.md) and [policy concepts](docs/policy-model.md)
for the design and its current limits. See [contributing](CONTRIBUTING.md) and
[security reporting](SECURITY.md) before participating.

Licensed under the [MIT License](LICENSE).
