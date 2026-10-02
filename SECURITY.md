# Security policy

## Current status

PermBridge has an experimental in-memory policy model and adapter contract.
It does not import real agent configurations, compare effective posture,
intercept actions, mediate approval, or enforce decisions. Do not depend on it
to protect a repository or system. Future diagnostics will be limited by each
adapter's evidence and the underlying agent's controls. See the
[security model](docs/security-model.md).

## Reporting a vulnerability

Please do not disclose a suspected vulnerability in a public issue. Use the
repository's GitHub **Report a vulnerability** option under its Security tab
when available. If private vulnerability reporting is unavailable, contact a
maintainer privately through an existing trusted channel before publishing
details. Include the affected version or commit, reproduction steps, expected
behavior, and security impact. Do not include credentials or private data.

No response-time or supported-version guarantee is established yet.
