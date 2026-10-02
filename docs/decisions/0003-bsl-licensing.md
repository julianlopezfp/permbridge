# ADR 0003: Use BSL 1.1 for future PermBridge versions

## Status

Accepted, 2026-10-02. Applies prospectively; published Git history is
preserved.

## Context

PermBridge is a public portfolio project and educational reference. Readers
should be able to inspect, copy, modify, redistribute, and run it for
non-production evaluation and learning, while production use of new versions
requires a separate commercial agreement. The initial published versions were
licensed under MIT. The author does not accept external code contributions.

## Decision

Use the official, unmodified Business Source License 1.1 terms with Julián
López Jiménez as Licensor, PermBridge as Licensed Work, and `None` as the
Additional Use Grant. The Change Date parameter is four calendar years after
the UTC committer timestamp of the Git commit for each BSL version, subject to
BSL's earlier fourth-anniversary cap measured from first public distribution.
The Change License is Apache License 2.0. The [licensing guide](../licensing.md)
gives the date lookup and release practice.

The [Apache Software Foundation's compatibility guidance](https://www.apache.org/licenses/GPL-compatibility)
confirms that Apache 2.0 code can be included in a GPLv3 program, satisfying
the [official BSL 1.1 covenant](https://mariadb.com/bsl11/) to choose a
GPLv2-or-later-compatible Change License. BSL 1.1 itself is
source-available and is not OSI Open Source before the Change Date. The Change
License takes effect for each version automatically on its applicable date.

## Alternatives considered

- MIT or Apache 2.0 immediately would allow production use without a separate
  agreement, contrary to the intended licensing of new versions. Previously
  published MIT versions keep their existing rights.
- A custom non-commercial license would add interpretation and maintenance
  risk. BSL 1.1 already has a recognized non-production grant and a required
  later Open Source license.
- GPL as the Change License would meet the BSL covenant but impose copyleft
  conditions after the Change Date. Apache 2.0 offers a permissive later grant
  while meeting the GPLv3 compatibility test.

## Consequences

New versions are not Open Source before their Change Date. Users needing
production use of a BSL-covered version before then must arrange separate
commercial terms with the Licensor. Each version has its own date; old dates
cannot be postponed by new releases. This decision does not revoke the MIT
grant on earlier published commits. The maintainer should record publication
dates for tagged releases and maintain clear license metadata and notices.
External Issues remain open for reports and suggestions, while external code
contributions and pull requests are declined. No CLA or DCO workflow is used.
