# Contributing to PermBridge

PermBridge is an early skeleton for comparing desired permissions with coding
agents' effective security posture. Keep changes focused and distinguish
planned capabilities from tested behavior. Do not claim support for an agent
without an implemented adapter and behavior tests.

## Language and documentation

Write source code, identifiers, comments, commit messages, issues, and
canonical technical documentation in English. Keep the main user-facing
README available in both [English](README.md) and [Spanish](README.es.md).
When changing user-facing instructions, update both versions. Keep internal
design documentation in English; future user guides can be paired under
`docs/user/en/` and `docs/user/es/`.

## Development setup

Install stable Rust with `rustfmt` and `clippy`. The VS Code placeholder does
not require Node.js. Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

Add focused tests for behavior you implement. Changes to policy merging must
show that a project policy cannot weaken a global restriction. Adapter changes
must document native capability limits and provide behavior evidence. Do not
commit secrets, private prompts, or machine-specific paths. Use a clear
conventional commit message, such as `docs: clarify comparison outcomes`.
