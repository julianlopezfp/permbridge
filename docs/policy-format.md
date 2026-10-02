# Version-1 YAML policy format

**Status: implemented, experimental.** `permbridge-core` parses one UTF-8 YAML
document into a validated `CanonicalPolicy`. The file describes desired
permissions. Loading does not discover other files, merge scopes, inspect an
agent, configure an agent, or enforce a restriction.

```yaml
version: 1
scope: global
default_decision: ask
filesystem:
  read: allow
  write: ask
  outside_workspace: deny
network:
  default_decision: ask
  domains:
    - domain: example.test
      decision: deny
execution:
  default_decision: ask
  rules:
    - command: synthetic-build
      decision: ask
```

`version` and `scope` are required. Version must be the integer `1`. Supported
scopes are `managed`, `global`, `project`, and `session`. Scope records the
source of a policy; it does not currently imply merge order or authority.
Every decision must be exactly lowercase `allow`, `ask`, or `deny`, matching
canonical `Allow`, `Ask`, and `Deny`. Restriction order is
`deny > ask > allow`.

All other fields are optional. Omitted `default_decision`, every omitted
filesystem decision, `network.default_decision`, and
`execution.default_decision` become `ASK`, independently. Omitted `network`
domains and execution rules become empty lists. The top-level
`default_decision` is retained in the canonical model but does not override
these specific defaults or create extra comparator capabilities. An explicit
`allow` is required for any permissive decision.

`outside_workspace` is an additional filesystem boundary, not a replacement
for `read` or `write`. Domain and command selectors are nonempty exact opaque
strings. The loader preserves spelling, case, whitespace, and rule order. It
does not interpret wildcard characters, normalize domains, parse commands,
match requests, or establish rule precedence. The same selector cannot appear
twice within its rule list, even with an equal decision.

The loader rejects missing required fields, wrong YAML types, malformed YAML,
unsupported versions, unknown fields at every level, invalid decisions,
empty or whitespace-only selectors, and repeated selectors. Errors provide a
stable category and logical field path without echoing source values. An error
never yields a fallback policy. Reading from a path never modifies the file.

Use `permbridge_core::load_policy_yaml(&str)` for in-memory input or
`permbridge_core::load_policy_file(&Path)` for an explicit path. The latter
reads only the supplied file; no automatic global/project discovery, includes,
environment substitution, templates, profiles, or multi-file merge exists.
The CLI does not yet expose these functions as an end-user command. See the
tested [examples](../policies/examples/README.md).

Version 1 specifies the accepted input shape and current mapping to the
experimental canonical model. Unknown fields and future version numbers are
rejected instead of silently interpreted as version 1. This is not a promise
that later releases will retain the exact Rust API or never introduce a new
schema version. A future version will require an explicit parser and
documented migration; none is implemented now.

The parser uses the focused `serde_yaml_ng` library to decode YAML syntax;
Core validates the accepted structure and maps it explicitly to domain types.
Successful loading establishes only that PermBridge understood the desired
policy. Adapter observations and comparator outcomes remain separate, and
neither current adapter enforces this YAML.
