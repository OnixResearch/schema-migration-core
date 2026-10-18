# Implementation evidence

## Published source boundary

- Canonical repository: `rad:z1C4YVMgDGyVdQa72uPNj3UDS5cY`
- Public adapter: `https://seed.radicle.garden/z1C4YVMgDGyVdQa72uPNj3UDS5cY.git`
- Reviewed source: `4fe90e130f2871cf69a6febcdc70785adca98aea`
- Superseded pre-audit source: `3f7b4315c8e1d07726446f1e53ba45bc091c5275`
- Schema Identity Core source: `2562c8aa38a034061f9af9f3e17280494a5b8de2`
- Content Identity Core source: `7f55597b5dc879b7601856e8d7fd0dbacaa2a498`

The adversarial audit found that the first source dropped exact context from non-selected receipts. The reviewed source closes that gap.

`profiles/source-boundary.ncl` binds Cargo and Nix consumers to the reviewed source. Negative fixtures reject branches, abbreviations, GitHub substitutions, sibling paths, wrong repository identities, and revision drift.

## Core implementation

The `no_std + alloc` functional core provides:

- canonical catalog, entry, policy, limit, observation, path, implementation, and plan identities;
- bounded catalog admission with duplicate, self-edge, outgoing-edge, precondition, and checked-arithmetic rejection;
- iterative simple-path enumeration with explicit step, candidate, and search-state bounds;
- policy-directed cohort and cost ordering without input-order tie breaking;
- explicit no-op, selected, no-path, ambiguous, exhausted, denied, and invalid outcomes;
- exact precondition and implementation-cohort admission;
- ordered load, validation, checkpoint, migration, commit, rollback, compensation, and receipt requests;
- exact pre-handoff freshness checks;
- bounded metadata-only receipts with exact decision bindings and non-claims.

The core performs no registry, clock, file, process, migration, storage, transaction, network, or publication effect.

## Conformance and adversarial evidence

The Nickel-generated cohort contains positive no-op, direct, multi-step, preferred-cohort, and reversible cases.

Negative cases cover entry bounds, duplicates, ambiguity, no path, overflow, missing preconditions, wrong cohorts, and stale plans.

Rust adversarial cases also cover cyclic catalogs, candidate exhaustion, exact effect ordering, satisfied implementation preconditions, catalog drift, receipt context, deterministic seeded replay, and semantic mutation.

## Independent consumer pilots

Kamacite:

- final commit: `239508c39a39e35d908db7381d1ad879a3dfbbe8`
- canonical branch: `origin/main`
- Cargo and Nix pin: `4fe90e130f2871cf69a6febcdc70785adca98aea`
- focused Nix source check passed
- three positive and negative pilot tests passed

Molten:

- final commit: `d5a1e1a6eab762400d17f8be704c00bb3ea0eec0`
- canonical branch: `origin/molten`
- Cargo and Nix pin: `4fe90e130f2871cf69a6febcdc70785adca98aea`
- Nix flake metadata and lock checks passed
- three positive and negative pilot tests passed

Both pilots produced plan identity `beaf5de24b47917d0d361f0bcb00a33fbaadb554a2ff81702b4bff7184930645`. Neither pilot executes migration or mutates product storage.

Molten's repository hooks could not find ambient `cargo` or the removed sibling Cairn path. The pinned dev-shell tests, strict Clippy, Nickel checks, Nix metadata check, and source-lock checks passed before guarded `--no-verify` publication.

## Validation

The following checks passed:

- 16 core positive, negative, property, and adversarial tests;
- 3 portable conformance tests;
- strict workspace Clippy with denied warnings;
- full pinned Octet deny-all with zero findings, warnings, and errors;
- `schema-migration-core` `wasm32-unknown-unknown` build;
- Nickel 1.17.0 positive export and invalid-fixture rejection;
- Cargo and Nix Schema Identity Core source alignment;
- architecture and duplicate-identity boundaries;
- source publication boundary and negative fixtures;
- Cairn validation and proposal, design, and task gates;
- full local-builder `nix flake check path:$PWD -L`;
- Cairn traceability coverage for all 58 of 58 accepted requirement and scenario identifiers.

## Non-claims

These checks do not prove migration implementation correctness, migrated data correctness, successful execution, atomicity, rollback success, restored external effects, authorization, remote trust, or release eligibility.
