// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Onix Research
// Earlier source grants remain effective; see NOTICE.

#![forbid(unsafe_code)]
#![allow(
    unknown_lints,
    reason = "Octet Dylint names load only during the pinned cargo-octet gate; owner: schema-migration-core"
)]
#![allow(
    clippy::useless_attribute,
    reason = "Octet allow attributes become active only under the pinned Dylint driver; owner: schema-migration-core"
)]

//! Portable positive and negative fixtures for Schema Migration Core.

#[allow(
    non_trait_imports,
    reason = "portable fixture construction keeps published schema identity facts explicit and adjacent; owner: schema-migration-core"
)]
use schema_identity_core::{
    AdmissionLimits, IdentityMode, MemberLimit, NodeId, NodeLimit, NominalScope, Openness,
    ScalarKind, SchemaDescriptor, SchemaNode, SchemaNodeKind, admit_schema,
};
#[allow(
    non_trait_imports,
    reason = "portable fixtures exercise the complete public migration contract without hiding selected types; owner: schema-migration-core"
)]
use schema_migration_core::{
    CatalogEntryKey, CatalogSnapshot, CostFacts, CostOrder, CostValues, DataSubjectId, Direction,
    EffectKind, FreshnessDecision, FreshnessInput, FreshnessObservation, ImplementationCohortId,
    MigrationArtifactId, MigrationBinding, MigrationEntry, MigrationError, MigrationMetadata,
    MigrationMetadataInput, MigrationProfileId, MigrationReference, ObservationId, ObservationSet,
    PlanningDecisionClass, PlanningLimitValues, PlanningLimits, PlanningOutcome, PlanningPolicy,
    PlanningPolicyInput, PlanningRequest, PolicyProfileId, PreconditionId, PreconditionKind,
    PreconditionObservation, PreconditionRequirement, ReversibilityClass, SchemaId, SchemaRoute,
    StaleReason, TextLimit, TiePolicy, TypeLineageId, check_freshness, plan_migration,
};
#[allow(
    non_trait_imports,
    reason = "Serde derive and formatting traits support the bounded conformance shell only; owner: schema-migration-core"
)]
use serde::Deserialize;
#[allow(
    non_trait_imports,
    reason = "standard formatting implements one bounded conformance error; owner: schema-migration-core"
)]
use std::fmt;

const VECTORS: &str = include_str!("vectors.json");
const TEXT_BYTES: u16 = 128;
const SCHEMA_NODES: u32 = 4;
const SCHEMA_MEMBERS: u32 = 4;
const MAX_ENTRIES: u16 = 16;
const MAX_OUTGOING: u16 = 8;
const MAX_STEPS: u16 = 8;
const MAX_CANDIDATES: u16 = 8;
const MAX_STATES: u32 = 64;
const MAX_PRECONDITIONS: u16 = 8;
const MAX_DIAGNOSTICS: u16 = 8;
const DIRECT_COST: u64 = 5;
const LOW_COST: u64 = 1;

/// Conformance fixture or execution failure.
#[derive(Debug)]
pub struct ConformanceError(String);

impl fmt::Display for ConformanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ConformanceError {}

impl From<schema_identity_core::SchemaIdentityError> for ConformanceError {
    fn from(error: schema_identity_core::SchemaIdentityError) -> Self {
        Self(error.to_string())
    }
}

impl From<MigrationError> for ConformanceError {
    fn from(error: MigrationError) -> Self {
        Self(error.to_string())
    }
}

impl From<serde_json::Error> for ConformanceError {
    fn from(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }
}

/// One portable conformance result without migration payloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseActual {
    decision: String,
    step_count: u16,
    freshness: Option<String>,
    has_conditional_recovery: bool,
    plan_id: Option<String>,
}

impl CaseActual {
    /// Returns the terminal decision label.
    #[must_use]
    pub fn decision(&self) -> &str {
        &self.decision
    }

    /// Returns the selected step count.
    #[must_use]
    pub const fn step_count(&self) -> u16 {
        self.step_count
    }

    /// Returns the optional freshness label.
    #[must_use]
    pub fn freshness(&self) -> Option<&str> {
        self.freshness.as_deref()
    }

    /// Returns whether the plan contains conditional recovery.
    #[must_use]
    pub const fn has_conditional_recovery(&self) -> bool {
        self.has_conditional_recovery
    }

    /// Returns the complete plan identity when one exists.
    #[must_use]
    pub fn plan_id(&self) -> Option<&str> {
        self.plan_id.as_deref()
    }
}

#[derive(Deserialize)]
struct FixtureFile {
    schema: String,
    cases: Vec<FixtureCase>,
}

#[derive(Deserialize)]
struct FixtureCase {
    id: String,
    scenario: String,
    expected: Expected,
}

#[derive(Deserialize)]
struct Expected {
    decision: String,
    steps: u16,
    freshness: Option<String>,
    conditional_recovery: bool,
}

#[derive(Clone)]
struct Fixture {
    lineage: TypeLineageId,
    source: SchemaId,
    middle: SchemaId,
    target: SchemaId,
    preferred: ImplementationCohortId,
    fallback: ImplementationCohortId,
}

struct EntryInput<'a> {
    key: &'a str,
    source: SchemaId,
    target: SchemaId,
    direction: Direction,
    cohort: &'a ImplementationCohortId,
    cost: u64,
    preconditions: Vec<PreconditionRequirement>,
    reversibility: ReversibilityClass,
}

/// Runs all embedded Nickel-generated positive and negative vectors.
///
/// # Errors
///
/// Returns the first parse, construction, or result mismatch.
// r[verify onix.schema_migration.conformance]
pub fn verify_embedded_vectors() -> Result<(), ConformanceError> {
    debug_assert!(!VECTORS.is_empty());
    let fixture_file: FixtureFile = serde_json::from_str(VECTORS)?;
    debug_assert!(!fixture_file.cases.is_empty());
    if fixture_file.schema != "schema-migration-conformance.v1" {
        return Err(ConformanceError("unexpected fixture schema".into()));
    }
    for case in fixture_file.cases {
        let actual = run_scenario(&case.scenario)?;
        if !matches_expected(&actual, &case.expected) {
            return Err(ConformanceError(format!(
                "fixture {} mismatch: expected {}/{}, got {}/{}",
                case.id,
                case.expected.decision,
                case.expected.steps,
                actual.decision,
                actual.step_count
            )));
        }
    }
    Ok(())
}

#[allow(
    missing_const_fn,
    reason = "owned String and Option equality are not const on the pinned Rust cohort; owner: schema-migration-core"
)]
fn matches_expected(actual: &CaseActual, expected: &Expected) -> bool {
    actual.decision == expected.decision
        && actual.step_count == expected.steps
        && actual.freshness == expected.freshness
        && actual.has_conditional_recovery == expected.conditional_recovery
}

/// Runs the shared effect-free consumer pilot fixture.
///
/// # Errors
///
/// Returns a construction or planning failure.
pub fn shared_fixture_report() -> Result<CaseActual, ConformanceError> {
    run_scenario("direct")
}

/// Runs one named fixture scenario.
///
/// # Errors
///
/// Returns an error for an unknown scenario or invalid fixture construction.
pub fn run_scenario(scenario: &str) -> Result<CaseActual, ConformanceError> {
    debug_assert!(!scenario.is_empty());
    let (outcome, freshness) = scenario_outcome(scenario)?;
    let plan = outcome.plan();
    let step_count = match plan {
        Some(plan) => u16::try_from(plan.steps().len())
            .map_err(|_error| ConformanceError("step count overflow".into()))?,
        None => 0,
    };
    debug_assert!(step_count <= MAX_STEPS);
    let has_conditional_recovery = plan.is_some_and(|plan| {
        plan.effects().iter().any(|effect| {
            effect.is_conditional()
                && matches!(
                    effect.kind(),
                    EffectKind::Rollback | EffectKind::Compensation
                )
        })
    });
    Ok(CaseActual {
        decision: decision_name(outcome.decision()).into(),
        step_count,
        freshness: freshness.map(freshness_name).map(String::from),
        has_conditional_recovery,
        plan_id: plan.map(|plan| plan.id().to_hex()),
    })
}

struct ScenarioContext {
    fixture: Fixture,
    limits: PlanningLimits,
    policy: PlanningPolicy,
    request: PlanningRequest,
}

fn scenario_outcome(
    scenario: &str,
) -> Result<(PlanningOutcome, Option<FreshnessDecision>), ConformanceError> {
    debug_assert!(!scenario.is_empty());
    let fixture = fixture()?;
    debug_assert_ne!(fixture.source, fixture.target);
    let context = ScenarioContext {
        limits: limits()?,
        policy: policy(&fixture, Vec::new())?,
        request: request(&fixture, "shared-consumer-fixture", Vec::new())?,
        fixture,
    };
    match scenario {
        "no-op" | "direct" | "multi-step" | "preferred-cohort" | "reversible" => {
            positive_scenario(scenario, &context)
        }
        "entry-limit" | "duplicate" | "ambiguous" | "no-path" | "overflow" => {
            negative_catalog_scenario(scenario, &context)
        }
        "missing-precondition" => guarded(&context.fixture, None),
        "wrong-cohort" => guarded(&context.fixture, Some(context.fixture.fallback.clone())),
        "stale" => stale(
            &context.fixture,
            &context.policy,
            context.limits,
            &context.request,
        ),
        _ => Err(ConformanceError(format!("unknown scenario: {scenario}"))),
    }
}

fn positive_scenario(
    scenario: &str,
    context: &ScenarioContext,
) -> Result<(PlanningOutcome, Option<FreshnessDecision>), ConformanceError> {
    debug_assert!(!scenario.is_empty());
    debug_assert!(context.limits.maximum_path_steps() > 0);
    if scenario == "no-op" {
        let request = PlanningRequest::new(
            DataSubjectId::new("shared-consumer-fixture", TextLimit::new(TEXT_BYTES)?)?,
            SchemaRoute::new(
                context.fixture.lineage,
                context.fixture.source,
                context.fixture.source,
            ),
            ObservationSet::new(Vec::new()),
        );
        return Ok((
            plan_migration(
                &CatalogSnapshot::new(Vec::new()),
                &request,
                &context.policy,
                context.limits,
            ),
            None,
        ));
    }
    let mut entries = match scenario {
        "direct" => vec![direct(&context.fixture, ReversibilityClass::Irreversible)?],
        "multi-step" | "preferred-cohort" => two_step(&context.fixture)?,
        "reversible" => vec![direct(
            &context.fixture,
            ReversibilityClass::SchemaRollback,
        )?],
        _ => return Err(ConformanceError("unknown positive scenario".into())),
    };
    if scenario == "preferred-cohort" {
        entries.push(direct(&context.fixture, ReversibilityClass::Irreversible)?);
    }
    Ok((
        plan_migration(
            &CatalogSnapshot::new(entries),
            &context.request,
            &context.policy,
            context.limits,
        ),
        None,
    ))
}

fn negative_catalog_scenario(
    scenario: &str,
    context: &ScenarioContext,
) -> Result<(PlanningOutcome, Option<FreshnessDecision>), ConformanceError> {
    debug_assert!(!scenario.is_empty());
    debug_assert!(context.limits.maximum_entries() > 0);
    let entries = match scenario {
        "duplicate" => {
            let duplicate = direct(&context.fixture, ReversibilityClass::Irreversible)?;
            vec![duplicate.clone(), duplicate]
        }
        "entry-limit" | "ambiguous" => vec![
            direct(&context.fixture, ReversibilityClass::Irreversible)?,
            entry(
                &context.fixture,
                EntryInput {
                    key: "direct-alternate",
                    source: context.fixture.source,
                    target: context.fixture.target,
                    direction: Direction::Forward,
                    cohort: &context.fixture.preferred,
                    cost: DIRECT_COST,
                    preconditions: Vec::new(),
                    reversibility: ReversibilityClass::Irreversible,
                },
            )?,
        ],
        "no-path" => vec![entry(
            &context.fixture,
            EntryInput {
                key: "reverse",
                source: context.fixture.source,
                target: context.fixture.target,
                direction: Direction::Reverse,
                cohort: &context.fixture.preferred,
                cost: LOW_COST,
                preconditions: Vec::new(),
                reversibility: ReversibilityClass::Irreversible,
            },
        )?],
        "overflow" => overflow_entries(&context.fixture)?,
        _ => return Err(ConformanceError("unknown negative scenario".into())),
    };
    let selected_limits = if scenario == "entry-limit" {
        // r[verify onix.schema_migration.catalog_admission.entries]
        PlanningLimits::new(PlanningLimitValues {
            maximum_entries: 1,
            maximum_outgoing_edges: MAX_OUTGOING,
            maximum_path_steps: MAX_STEPS,
            maximum_candidate_paths: MAX_CANDIDATES,
            maximum_search_states: MAX_STATES,
            maximum_preconditions: MAX_PRECONDITIONS,
            maximum_diagnostics: MAX_DIAGNOSTICS,
        })?
    } else {
        context.limits
    };
    Ok((
        plan_migration(
            &CatalogSnapshot::new(entries),
            &context.request,
            &context.policy,
            selected_limits,
        ),
        None,
    ))
}

fn overflow_entries(fixture: &Fixture) -> Result<Vec<MigrationEntry>, ConformanceError> {
    Ok(vec![
        entry(
            fixture,
            EntryInput {
                key: "overflow-source",
                source: fixture.source,
                target: fixture.middle,
                direction: Direction::Forward,
                cohort: &fixture.preferred,
                cost: u64::MAX,
                preconditions: Vec::new(),
                reversibility: ReversibilityClass::Irreversible,
            },
        )?,
        entry(
            fixture,
            EntryInput {
                key: "overflow-target",
                source: fixture.middle,
                target: fixture.target,
                direction: Direction::Forward,
                cohort: &fixture.preferred,
                cost: LOW_COST,
                preconditions: Vec::new(),
                reversibility: ReversibilityClass::Irreversible,
            },
        )?,
    ])
}

fn stale(
    fixture: &Fixture,
    policy: &PlanningPolicy,
    limits: PlanningLimits,
    request: &PlanningRequest,
) -> Result<(PlanningOutcome, Option<FreshnessDecision>), ConformanceError> {
    debug_assert_ne!(fixture.source, fixture.target);
    debug_assert!(limits.maximum_path_steps() > 0);
    let outcome = plan_migration(
        &CatalogSnapshot::new(vec![direct(fixture, ReversibilityClass::Irreversible)?]),
        request,
        policy,
        limits,
    );
    let plan = outcome
        .plan()
        .ok_or_else(|| ConformanceError("stale fixture did not produce a plan".into()))?;
    let current = FreshnessObservation::new(FreshnessInput {
        source_data: DataSubjectId::new("changed-data", TextLimit::new(TEXT_BYTES)?)?,
        source_schema: plan.source_schema(),
        target_schema: plan.target_schema(),
        catalog_id: plan.catalog_id(),
        policy_id: plan.policy_id(),
        limits_id: plan.limits_id(),
        observations_id: plan.observations_id(),
        implementations_id: plan.implementations_id(),
    });
    let decision = check_freshness(plan, &current);
    Ok((outcome, Some(decision)))
}

fn guarded(
    fixture: &Fixture,
    observed_cohort: Option<ImplementationCohortId>,
) -> Result<(PlanningOutcome, Option<FreshnessDecision>), ConformanceError> {
    debug_assert_ne!(fixture.preferred, fixture.fallback);
    debug_assert_ne!(fixture.source, fixture.target);
    let limit = TextLimit::new(TEXT_BYTES)?;
    let requirement = PreconditionRequirement::new(
        PreconditionId::new("implementation-ready", limit)?,
        PreconditionKind::Implementation,
    );
    let migration = entry(
        fixture,
        EntryInput {
            key: "guarded",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            cost: LOW_COST,
            preconditions: vec![requirement.clone()],
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let observations = match observed_cohort {
        Some(cohort) => vec![PreconditionObservation::new(
            requirement,
            ObservationId::new("observed-implementation", limit)?,
            true,
            Some(cohort),
        )],
        None => Vec::new(),
    };
    let request = request(fixture, "shared-consumer-fixture", observations)?;
    let policy = policy(fixture, vec![PreconditionKind::Implementation])?;
    Ok((
        plan_migration(
            &CatalogSnapshot::new(vec![migration]),
            &request,
            &policy,
            limits()?,
        ),
        None,
    ))
}

fn fixture() -> Result<Fixture, ConformanceError> {
    let limit = TextLimit::new(TEXT_BYTES)?;
    let scope = NominalScope::new(
        "onix.conformance",
        "migration-lineage",
        schema_text_limit()?,
    )?;
    let lineage = scope.lineage_id()?;
    Ok(Fixture {
        lineage,
        source: schema(lineage, ScalarKind::Boolean)?,
        middle: schema(lineage, ScalarKind::Text)?,
        target: schema(lineage, ScalarKind::Bytes)?,
        preferred: ImplementationCohortId::new("reviewed-v2", limit)?,
        fallback: ImplementationCohortId::new("reviewed-v1", limit)?,
    })
}

fn schema_text_limit()
-> Result<schema_identity_core::TextLimit, schema_identity_core::SchemaIdentityError> {
    schema_identity_core::TextLimit::new(TEXT_BYTES)
}

fn schema(
    lineage: TypeLineageId,
    kind: ScalarKind,
) -> Result<SchemaId, schema_identity_core::SchemaIdentityError> {
    let root = NodeId::new(1);
    let descriptor = SchemaDescriptor::new(
        IdentityMode::Nominal { lineage },
        Openness::Closed,
        root,
        vec![SchemaNode::new(root, SchemaNodeKind::Scalar(kind))],
        None,
    );
    let limits = AdmissionLimits::new(
        NodeLimit::new(SCHEMA_NODES)?,
        MemberLimit::new(SCHEMA_MEMBERS)?,
    );
    Ok(admit_schema(descriptor, limits)?.schema_id())
}

fn entry(fixture: &Fixture, input: EntryInput<'_>) -> Result<MigrationEntry, ConformanceError> {
    debug_assert!(!input.key.is_empty());
    debug_assert_ne!(input.source, input.target);
    let limit = TextLimit::new(TEXT_BYTES)?;
    let reference = MigrationReference::new(
        input.source,
        input.target,
        MigrationArtifactId::new(format!("artifact-{}", input.key), schema_text_limit()?)?,
        PolicyProfileId::new("conformance-policy", schema_text_limit()?)?,
    );
    let metadata = MigrationMetadata::new(MigrationMetadataInput {
        direction: input.direction,
        implementation_cohort: input.cohort.clone(),
        profile: MigrationProfileId::new("strict", limit)?,
        preconditions: input.preconditions,
        reversibility: input.reversibility,
        cost: CostFacts::new(CostValues {
            risk: input.cost,
            effort: input.cost,
        }),
    })?;
    Ok(MigrationEntry::new(
        CatalogEntryKey::new(input.key, limit)?,
        MigrationBinding::new(fixture.lineage, reference),
        metadata,
    ))
}

fn direct(
    fixture: &Fixture,
    reversibility: ReversibilityClass,
) -> Result<MigrationEntry, ConformanceError> {
    entry(
        fixture,
        EntryInput {
            key: "direct",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            cost: DIRECT_COST,
            preconditions: Vec::new(),
            reversibility,
        },
    )
}

fn two_step(fixture: &Fixture) -> Result<Vec<MigrationEntry>, ConformanceError> {
    Ok(vec![
        entry(
            fixture,
            EntryInput {
                key: "source-middle",
                source: fixture.source,
                target: fixture.middle,
                direction: Direction::Forward,
                cohort: &fixture.fallback,
                cost: LOW_COST,
                preconditions: Vec::new(),
                reversibility: ReversibilityClass::Irreversible,
            },
        )?,
        entry(
            fixture,
            EntryInput {
                key: "middle-target",
                source: fixture.middle,
                target: fixture.target,
                direction: Direction::Forward,
                cohort: &fixture.fallback,
                cost: LOW_COST,
                preconditions: Vec::new(),
                reversibility: ReversibilityClass::Irreversible,
            },
        )?,
    ])
}

fn limits() -> Result<PlanningLimits, MigrationError> {
    PlanningLimits::new(PlanningLimitValues {
        maximum_entries: MAX_ENTRIES,
        maximum_outgoing_edges: MAX_OUTGOING,
        maximum_path_steps: MAX_STEPS,
        maximum_candidate_paths: MAX_CANDIDATES,
        maximum_search_states: MAX_STATES,
        maximum_preconditions: MAX_PRECONDITIONS,
        maximum_diagnostics: MAX_DIAGNOSTICS,
    })
}

fn policy(
    fixture: &Fixture,
    required_preconditions: Vec<PreconditionKind>,
) -> Result<PlanningPolicy, MigrationError> {
    PlanningPolicy::new(PlanningPolicyInput {
        allows_forward: true,
        allows_reverse: false,
        implementation_cohorts: vec![fixture.preferred.clone(), fixture.fallback.clone()],
        required_preconditions,
        maximum_steps: MAX_STEPS,
        cost_order: CostOrder::RiskThenEffortThenSteps,
        tie_policy: TiePolicy::DenyAmbiguous,
    })
}

fn request(
    fixture: &Fixture,
    data: &str,
    observations: Vec<PreconditionObservation>,
) -> Result<PlanningRequest, MigrationError> {
    Ok(PlanningRequest::new(
        DataSubjectId::new(data, TextLimit::new(TEXT_BYTES)?)?,
        SchemaRoute::new(fixture.lineage, fixture.source, fixture.target),
        ObservationSet::new(observations),
    ))
}

const fn decision_name(decision: PlanningDecisionClass) -> &'static str {
    match decision {
        PlanningDecisionClass::NoOp => "no-op",
        PlanningDecisionClass::Selected => "selected",
        PlanningDecisionClass::NoPath => "no-path",
        PlanningDecisionClass::Ambiguous => "ambiguous",
        PlanningDecisionClass::BoundExhausted => "bound-exhausted",
        PlanningDecisionClass::PreconditionDenied => "precondition-denied",
        PlanningDecisionClass::InvalidCatalog => "invalid-catalog",
    }
}

const fn freshness_name(decision: FreshnessDecision) -> &'static str {
    match decision {
        FreshnessDecision::Eligible { .. } => "eligible",
        FreshnessDecision::Stale {
            reason: StaleReason::SourceData,
        } => "stale-source-data",
        FreshnessDecision::Stale {
            reason: StaleReason::SourceSchema,
        } => "stale-source-schema",
        FreshnessDecision::Stale {
            reason: StaleReason::TargetSchema,
        } => "stale-target-schema",
        FreshnessDecision::Stale {
            reason: StaleReason::Catalog,
        } => "stale-catalog",
        FreshnessDecision::Stale {
            reason: StaleReason::Policy,
        } => "stale-policy",
        FreshnessDecision::Stale {
            reason: StaleReason::Limits,
        } => "stale-limits",
        FreshnessDecision::Stale {
            reason: StaleReason::Preconditions,
        } => "stale-preconditions",
        FreshnessDecision::Stale {
            reason: StaleReason::Implementation,
        } => "stale-implementation",
    }
}
