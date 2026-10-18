## Phase 1: Contract and catalog boundary

- [x] [serial] r[onix.schema_migration.schema_identity_dependency] Pin the reviewed Schema Identity Core contract and add cross-version conformance fixtures.
- [x] [serial] r[onix.schema_migration.catalog_snapshot] Define canonical catalog, entry, implementation-cohort, precondition, cost, and profile identities.
- [x] [serial] r[onix.schema_migration.plan_identity] Define complete data-subject, source, target, catalog, policy, limit, path, and plan identities.
- [x] [serial] r[onix.schema_migration.source_revision] Create immutable Cargo and Nix source boundaries.
- [x] [serial] r[onix.schema_migration.quality_gate] Pin Octet, add the deny-all hook, and connect the error gate to `nix flake check`.

## Phase 2: Pure planning core

- [x] [parallel] r[onix.schema_migration.catalog_admission] Implement catalog shape, graph, identity, bound, and arithmetic admission.
- [x] [parallel] r[onix.schema_migration.path_selection] Implement policy-directed path selection with deterministic cost ordering.
- [x] [parallel] r[onix.schema_migration.ambiguity] Implement explicit ambiguity, no-path, no-op, and limit outcomes.
- [x] [parallel] r[onix.schema_migration.preconditions] Implement supplied precondition and implementation-cohort admission.
- [x] [serial] r[onix.schema_migration.effect_plan] Implement ordered execution, checkpoint, commit, rollback, compensation, and receipt requests.
- [x] [serial] r[onix.schema_migration.freshness] Implement exact pre-handoff freshness checks.
- [x] [serial] r[onix.schema_migration.receipts] Implement bounded plan and eligibility receipt projections.
- [x] [serial] r[onix.schema_migration.functional_core] Keep registries, clocks, files, processes, migration code, storage, transactions, and publication outside the core.

## Phase 3: Fixtures and pilots

- [x] [parallel] r[onix.schema_migration.conformance.positive] Add no-op, direct, multi-step, preferred-cohort, and reversible-path fixtures.
- [x] [parallel] r[onix.schema_migration.conformance.negative] Add malformed, cycle-bound, ambiguous, no-path, overflow, precondition, wrong-cohort, and stale-plan fixtures.
- [x] [parallel] r[onix.schema_migration.property_cases] Add seeded bounded graph and mutation tests for determinism and fail-closed behavior.
- [x] [serial] r[onix.schema_migration.consumer_pilots] Add independent Kamacite and Molten planning pilots without migration execution.
- [x] [serial] r[onix.schema_migration.source_revision] Run focused suites, Octet, Clippy, Cairn gates, and `nix flake check -L`.
