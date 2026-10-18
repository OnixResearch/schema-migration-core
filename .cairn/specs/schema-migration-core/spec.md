# Schema Migration Core Specification

## Purpose

Defines the `schema-migration-core` capability.

## Requirements

### Requirement: Published schema identity dependency
r[onix.schema_migration.schema_identity_dependency] The planner MUST consume one immutable published Schema Identity Core contract for lineage, schema, compatibility, and migration-reference values.

#### Scenario: Matching contract passes
r[onix.schema_migration.schema_identity_dependency.valid]
- GIVEN planner and fixture inputs from the selected Schema Identity Core revision
- WHEN dependency validation runs
- THEN their exact type and identity cohort is admitted

#### Scenario: Duplicate identity implementation fails
r[onix.schema_migration.schema_identity_dependency.duplicate]
- GIVEN a local schema identity implementation beside the published dependency
- WHEN architecture validation runs
- THEN validation fails

#### Scenario: Revision mismatch fails
r[onix.schema_migration.schema_identity_dependency.revision]
- GIVEN Cargo and Nix declarations for different Schema Identity Core revisions
- WHEN source validation runs
- THEN validation fails

### Requirement: Canonical migration catalog snapshot
r[onix.schema_migration.catalog_snapshot] Every catalog snapshot MUST bind canonical entries, schema identities, migration artifacts, directions, implementation cohorts, profiles, preconditions, reversibility classes, costs, and limits.

#### Scenario: Equal catalogs have equal identities
r[onix.schema_migration.catalog_snapshot.equal]
- GIVEN two catalogs with equal normalized entries
- WHEN the core derives their identities
- THEN the identities are equal

#### Scenario: Entry order is canonical
r[onix.schema_migration.catalog_snapshot.order]
- GIVEN equal entries in different input orders
- WHEN catalog admission runs
- THEN the canonical catalog identities are equal

#### Scenario: Duplicate entry fails
r[onix.schema_migration.catalog_snapshot.duplicate]
- GIVEN two entries with the same entry identity and conflicting fields
- WHEN catalog admission runs
- THEN admission fails

### Requirement: Bounded catalog admission
r[onix.schema_migration.catalog_admission] Catalog admission MUST bound entries, outgoing edges, path steps, candidate paths, preconditions, diagnostics, and checked cost arithmetic.

#### Scenario: Bounded catalog passes
r[onix.schema_migration.catalog_admission.valid]
- GIVEN a well-formed catalog within all selected limits
- WHEN admission runs
- THEN the catalog is eligible for path selection

#### Scenario: Entry limit fails
r[onix.schema_migration.catalog_admission.entries]
- GIVEN a catalog above the selected entry limit
- WHEN admission runs
- THEN admission fails before path search

#### Scenario: Cost overflow fails
r[onix.schema_migration.catalog_admission.overflow]
- GIVEN path costs whose checked sum overflows
- WHEN path evaluation runs
- THEN the candidate is rejected with an arithmetic error

### Requirement: Complete migration plan identity
r[onix.schema_migration.plan_identity] Every plan MUST bind source data, source schema, target schema, lineage, catalog, policy, limits, preconditions, implementation cohorts, selected path, and plan identity.

#### Scenario: Equal plans have equal identities
r[onix.schema_migration.plan_identity.equal]
- GIVEN two plans with equal normalized bindings and steps
- WHEN the core derives their identities
- THEN the identities are equal

#### Scenario: Data identity change changes plan
r[onix.schema_migration.plan_identity.data]
- GIVEN two otherwise equal plans for different source data identities
- WHEN the core derives their identities
- THEN the identities are different

#### Scenario: Missing target fails
r[onix.schema_migration.plan_identity.missing]
- GIVEN a planning request without an exact target schema
- WHEN admission runs
- THEN admission fails

### Requirement: Policy-directed path selection
r[onix.schema_migration.path_selection] Path selection MUST use explicit direction, cohort, precondition, step, cost-order, and tie policy without ambient catalog order.

#### Scenario: Direct preferred path is selected
r[onix.schema_migration.path_selection.direct]
- GIVEN two valid paths and policy that ranks the admitted direct cohort first
- WHEN selection runs
- THEN the direct path is selected

#### Scenario: Disallowed direction fails
r[onix.schema_migration.path_selection.direction]
- GIVEN only reverse paths under a forward-only policy
- WHEN selection runs
- THEN no path is selected

#### Scenario: Input order does not change selection
r[onix.schema_migration.path_selection.order]
- GIVEN one catalog in two different entry orders
- WHEN selection runs with equal policy
- THEN it returns the same decision and selected path identity

### Requirement: Explicit terminal selection classes
r[onix.schema_migration.ambiguity] The planner MUST distinguish no-op, selected, no-path, ambiguous, bound-exhausted, precondition-denied, and invalid-catalog outcomes.

#### Scenario: Equal schemas are no-op
r[onix.schema_migration.ambiguity.noop]
- GIVEN equal source and target schema identities
- WHEN planning runs
- THEN the result is no-op without migration steps

#### Scenario: Equal candidates are ambiguous
r[onix.schema_migration.ambiguity.equal]
- GIVEN different valid paths equal under all selected policy and cost rules
- WHEN selection runs
- THEN the planner returns ambiguous without selecting by input order

#### Scenario: Search bound is explicit
r[onix.schema_migration.ambiguity.bound]
- GIVEN a catalog search that reaches the selected candidate or path bound
- WHEN planning stops
- THEN the result is bound-exhausted and not no-path

### Requirement: Explicit precondition admission
r[onix.schema_migration.preconditions] Every selected step MUST bind required data, environment, implementation, feature, authority-observation, and resource preconditions supplied by the consumer.

#### Scenario: Complete preconditions pass
r[onix.schema_migration.preconditions.valid]
- GIVEN current supplied observations that satisfy every selected step precondition
- WHEN precondition admission runs
- THEN the path remains eligible

#### Scenario: Missing precondition fails
r[onix.schema_migration.preconditions.missing]
- GIVEN one selected step without a required observation
- WHEN precondition admission runs
- THEN eligibility fails

#### Scenario: Wrong implementation cohort fails
r[onix.schema_migration.preconditions.cohort]
- GIVEN a migration implementation outside the selected reviewed cohort
- WHEN precondition admission runs
- THEN eligibility fails

### Requirement: Typed migration effect plan
r[onix.schema_migration.effect_plan] The core MUST emit ordered load, validate-input, migrate, validate-output, checkpoint, commit, rollback, compensation, and receipt requests without executing effects.

#### Scenario: Selected path produces ordered steps
r[onix.schema_migration.effect_plan.valid]
- GIVEN one selected eligible path
- WHEN effect planning runs
- THEN each command binds its step, input, expected output, preconditions, and limits

#### Scenario: No-op produces no migration command
r[onix.schema_migration.effect_plan.noop]
- GIVEN a no-op planning result
- WHEN effect planning runs
- THEN it emits no migration execution request

#### Scenario: Rollback remains conditional
r[onix.schema_migration.effect_plan.rollback]
- GIVEN a reversible-class step
- WHEN effect planning runs
- THEN rollback is a typed conditional request without a success claim

### Requirement: Exact plan freshness
r[onix.schema_migration.freshness] Execution handoff MUST recheck source data, source schema, target schema, catalog, policy, implementation, limits, and precondition identities against the plan.

#### Scenario: Exact observations pass
r[onix.schema_migration.freshness.valid]
- GIVEN current observations equal to every plan binding
- WHEN handoff admission runs
- THEN the plan is eligible for consumer execution

#### Scenario: Catalog drift fails
r[onix.schema_migration.freshness.catalog]
- GIVEN a current catalog identity different from the plan catalog
- WHEN handoff admission runs
- THEN eligibility fails

#### Scenario: Source data drift fails
r[onix.schema_migration.freshness.data]
- GIVEN a current source data identity different from the plan source data
- WHEN handoff admission runs
- THEN eligibility fails

### Requirement: Bounded migration receipts
r[onix.schema_migration.receipts] Plan and handoff receipts MUST bind exact identities, decisions, steps, limits, preconditions, diagnostics, and non-claims without data payloads.

#### Scenario: Plan receipt is metadata-only
r[onix.schema_migration.receipts.valid]
- GIVEN a terminal planning decision
- WHEN receipt projection runs
- THEN it contains identities, counts, path, reason classes, and bounds without source data bytes

#### Scenario: Ambiguity receipt lists bounded candidates
r[onix.schema_migration.receipts.ambiguous]
- GIVEN an ambiguous selection result
- WHEN receipt projection runs
- THEN it identifies the bounded equal candidate paths without selecting one

#### Scenario: Receipt does not prove execution
r[onix.schema_migration.receipts.claim]
- GIVEN a valid selected-plan receipt
- WHEN claim validation runs
- THEN it cannot claim migration success, atomicity, rollback success, or data correctness

### Requirement: Planner conformance suites
r[onix.schema_migration.conformance] The repository MUST publish positive and negative fixtures for catalog admission, selection, planning, freshness, and receipts.

#### Scenario: Positive suite passes
r[onix.schema_migration.conformance.positive]
- GIVEN no-op, direct, multi-step, preferred-cohort, and reversible fixtures
- WHEN the planner runs
- THEN it returns the expected deterministic plans and identities

#### Scenario: Negative suite passes
r[onix.schema_migration.conformance.negative]
- GIVEN malformed, ambiguous, no-path, overflow, precondition, cohort, and stale-plan fixtures
- WHEN the planner runs
- THEN it returns the expected fail-closed decisions

### Requirement: Seeded property cases
r[onix.schema_migration.property_cases] Generated graph and mutation cases MUST use explicit seeds, graph bounds, path bounds, and shrink bounds.

#### Scenario: Equal seed replays
r[onix.schema_migration.property_cases.seed]
- GIVEN one generator profile and seed
- WHEN generation runs twice
- THEN it produces the same ordered catalog and request identities

#### Scenario: Mutation changes or invalidates identity
r[onix.schema_migration.property_cases.mutation]
- GIVEN one admitted catalog and a semantic entry mutation
- WHEN identities and plans are recomputed
- THEN the catalog identity changes or admission fails

### Requirement: Independent consumer pilots
r[onix.schema_migration.consumer_pilots] Kamacite and Molten MUST pass independent planning pilots before stable publication.

#### Scenario: Both pilots agree on shared fixture
r[onix.schema_migration.consumer_pilots.valid]
- GIVEN one shared catalog and planning request
- WHEN both pilots use the same core revision
- THEN both produce the same plan identity and decision

#### Scenario: Pilot executes no migration
r[onix.schema_migration.consumer_pilots.no_effect]
- GIVEN the planning pilot suite
- WHEN it runs
- THEN it performs no product migration or storage mutation

### Requirement: Pure migration core
r[onix.schema_migration.functional_core] Catalog admission, path selection, effect planning, freshness, and receipt projection MUST be pure over explicit values.

#### Scenario: Core performs no migration effect
r[onix.schema_migration.functional_core.explicit]
- GIVEN complete catalog, request, policy, limit, precondition, and freshness values
- WHEN the core evaluates them
- THEN it performs no registry, clock, file, process, migration, storage, transaction, network, or publication effect

### Requirement: Immutable source alignment
r[onix.schema_migration.source_revision] Cargo and Nix consumers MUST select the same immutable reviewed migration-core revision.

#### Scenario: Source identities agree
r[onix.schema_migration.source_revision.agree]
- GIVEN Cargo and Nix source declarations
- WHEN source validation runs
- THEN both select the same full revision

#### Scenario: Local path fails release validation
r[onix.schema_migration.source_revision.local]
- GIVEN a release profile with an ambient sibling path
- WHEN source validation runs
- THEN validation fails

### Requirement: Strict Rust quality gate
r[onix.schema_migration.quality_gate] The Rust project MUST enforce the complete pinned Octet catalog as errors across its workspace, targets, and compatible features.

#### Scenario: Clean source passes
r[onix.schema_migration.quality_gate.valid]
- GIVEN source with passing positive and negative tests, Clippy, and no Octet findings
- WHEN local and Nix gates run
- THEN the quality gate passes

#### Scenario: Octet finding fails
r[onix.schema_migration.quality_gate.finding]
- GIVEN source with one Octet finding
- WHEN the deny-all hook or `nix flake check` runs
- THEN the quality gate fails

#### Scenario: Missing ambiguity tests fail
r[onix.schema_migration.quality_gate.ambiguity]
- GIVEN a planner suite without ambiguous-path and stale-plan cases
- WHEN quality validation runs
- THEN the quality gate fails
