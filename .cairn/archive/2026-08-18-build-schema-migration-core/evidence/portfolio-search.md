# Portfolio search record

## Success contract

Goal: implement and publish a product-neutral, bounded schema migration planner that consumes the reviewed Schema Identity Core contract.

Completion evidence:

- deterministic catalog admission, path selection, plan identity, effect requests, freshness, and metadata-only receipts;
- positive, negative, seeded property, and mutation tests;
- independent Kamacite and Molten planning pilots pinned to one reviewed source revision;
- strict Clippy, Octet deny-all, `no_std` Wasm, Nickel, Nix, Cairn, and traceability gates;
- an immutable canonical Radicle source with no active Cairn change.

False completion includes a planner that executes migrations, selects by input order, uses floating dependencies, hides ambiguity, skips stale-plan checks, or passes warning-only quality output.

Audit risks include cyclic catalogs, candidate exhaustion, cost overflow, duplicate entries, identity omission, stale observations, missing preconditions, cohort substitution, payload leakage, and authority expansion.

Budget: three mechanism families, two exploration rounds, 24 targeted source reads, two consumer pilots, and one adversarial audit. Allowed outcomes are validated, blocked, exhausted, or user-decision-required.

## Approach registry

| Family | Mechanism | Claim | State | Next check |
|---|---|---|---|---|
| bounded-path-enumeration | Enumerate simple paths under explicit path and candidate limits, then rank canonical path facts. | The planner distinguishes no path, ambiguity, and exhausted search without ambient order. | validated | Equal-cost paths, cycles, candidate exhaustion, cost overflow, and input reordering pass deterministic tests. |
| dynamic-shortest-path | Compute one dynamic shortest path with predecessor reconstruction. | The planner can minimize one numeric cost efficiently. | falsified | A single-winner predecessor map collapses equal candidates and cannot preserve required ambiguity evidence. |
| consumer-owned-selection | Return all admitted edges and let each consumer select a path. | Consumers retain product migration policy. | falsified | This duplicates selection semantics and permits Kamacite and Molten to disagree on the shared fixture. |

Bounded path enumeration survived both rounds. It keeps policy explicit and preserves ambiguity as a terminal decision.

## Adversarial audit

The first audit found that non-selected receipts dropped source, catalog, policy, limit, and observation identities. That was a false-completion defect.

The result model now retains exact decision bindings for no-path, ambiguity, bound, precondition, and invalid-catalog outcomes. Metadata-only receipt tests verify these bindings.

Additional audit cases cover cyclic catalogs, candidate exhaustion, exact effect ordering, satisfied implementation preconditions, and catalog freshness drift.

All exploration passes are serial and therefore correlated. Deterministic fixtures, strict Octet, and consumer outputs remain authoritative.
