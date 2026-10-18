#[allow(
    non_trait_imports,
    reason = "integration fixtures keep published schema identity constructors explicit; owner: schema-migration-core"
)]
use schema_identity_core::{
    AdmissionLimits, IdentityMode, MemberLimit, NodeId, NodeLimit, NominalScope, Openness,
    ScalarKind, SchemaDescriptor, SchemaNode, SchemaNodeKind, admit_schema,
};
#[allow(
    non_trait_imports,
    reason = "integration fixtures exercise the public migration contract as an external consumer; owner: schema-migration-core"
)]
use schema_migration_core::*;

pub(crate) const TEXT_BYTES: u16 = 128;
const SCHEMA_NODES: u32 = 4;
const SCHEMA_MEMBERS: u32 = 4;
pub(crate) const MAX_ENTRIES: u16 = 16;
pub(crate) const MAX_OUTGOING: u16 = 8;
pub(crate) const MAX_STEPS: u16 = 8;
pub(crate) const MAX_CANDIDATES: u16 = 8;
pub(crate) const MAX_STATES: u32 = 64;
pub(crate) const MAX_PRECONDITIONS: u16 = 8;
pub(crate) const MAX_DIAGNOSTICS: u16 = 8;
pub(crate) const DIRECT_RISK: u64 = 5;
pub(crate) const DIRECT_EFFORT: u64 = 5;
pub(crate) const LOW_RISK: u64 = 1;
pub(crate) const LOW_EFFORT: u64 = 1;

pub(crate) type TestResult = Result<(), Box<dyn core::error::Error>>;

#[derive(Clone)]
pub(crate) struct Fixture {
    pub(crate) lineage: TypeLineageId,
    pub(crate) source: SchemaId,
    pub(crate) middle: SchemaId,
    pub(crate) target: SchemaId,
    pub(crate) preferred: ImplementationCohortId,
    pub(crate) fallback: ImplementationCohortId,
}

pub(crate) struct EntryInput<'a> {
    pub(crate) key: &'a str,
    pub(crate) source: SchemaId,
    pub(crate) target: SchemaId,
    pub(crate) direction: Direction,
    pub(crate) cohort: &'a ImplementationCohortId,
    pub(crate) risk: u64,
    pub(crate) effort: u64,
    pub(crate) preconditions: Vec<PreconditionRequirement>,
    pub(crate) reversibility: ReversibilityClass,
}

pub(crate) fn fixture() -> Result<Fixture, Box<dyn core::error::Error>> {
    let limit = TextLimit::new(TEXT_BYTES)?;
    let nominal = NominalScope::new("onix.test", "migration-lineage", schema_text_limit()?)?;
    let lineage = nominal.lineage_id()?;
    Ok(Fixture {
        lineage,
        source: schema(lineage, ScalarKind::Boolean)?,
        middle: schema(lineage, ScalarKind::Text)?,
        target: schema(lineage, ScalarKind::Bytes)?,
        preferred: ImplementationCohortId::new("reviewed-v2", limit)?,
        fallback: ImplementationCohortId::new("reviewed-v1", limit)?,
    })
}

pub(crate) fn schema_text_limit()
-> Result<schema_identity_core::TextLimit, schema_identity_core::SchemaIdentityError> {
    schema_identity_core::TextLimit::new(TEXT_BYTES)
}

fn schema(
    lineage: TypeLineageId,
    scalar: ScalarKind,
) -> Result<SchemaId, schema_identity_core::SchemaIdentityError> {
    let root = NodeId::new(1);
    let descriptor = SchemaDescriptor::new(
        IdentityMode::Nominal { lineage },
        Openness::Closed,
        root,
        vec![SchemaNode::new(root, SchemaNodeKind::Scalar(scalar))],
        None,
    );
    let limits = AdmissionLimits::new(
        NodeLimit::new(SCHEMA_NODES)?,
        MemberLimit::new(SCHEMA_MEMBERS)?,
    );
    Ok(admit_schema(descriptor, limits)?.schema_id())
}

pub(crate) fn entry(
    fixture: &Fixture,
    input: EntryInput<'_>,
) -> Result<MigrationEntry, Box<dyn core::error::Error>> {
    debug_assert!(!input.key.is_empty());
    debug_assert!(!input.cohort.as_str().is_empty());
    let limit = TextLimit::new(TEXT_BYTES)?;
    let reference = MigrationReference::new(
        input.source,
        input.target,
        MigrationArtifactId::new(format!("artifact-{}", input.key), schema_text_limit()?)?,
        PolicyProfileId::new("test-policy", schema_text_limit()?)?,
    );
    let metadata = MigrationMetadata::new(MigrationMetadataInput {
        direction: input.direction,
        implementation_cohort: input.cohort.clone(),
        profile: MigrationProfileId::new("strict", limit)?,
        preconditions: input.preconditions,
        reversibility: input.reversibility,
        cost: CostFacts::new(CostValues {
            risk: input.risk,
            effort: input.effort,
        }),
    })?;
    Ok(MigrationEntry::new(
        CatalogEntryKey::new(input.key, limit)?,
        MigrationBinding::new(fixture.lineage, reference),
        metadata,
    ))
}

pub(crate) fn direct_entry(
    fixture: &Fixture,
) -> Result<MigrationEntry, Box<dyn core::error::Error>> {
    entry(
        fixture,
        EntryInput {
            key: "direct",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: DIRECT_RISK,
            effort: DIRECT_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::SchemaRollback,
        },
    )
}

pub(crate) fn limits() -> Result<PlanningLimits, MigrationError> {
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

pub(crate) fn policy(fixture: &Fixture) -> Result<PlanningPolicy, MigrationError> {
    PlanningPolicy::new(policy_input(fixture))
}

pub(crate) fn policy_input(fixture: &Fixture) -> PlanningPolicyInput {
    PlanningPolicyInput {
        allows_forward: true,
        allows_reverse: false,
        implementation_cohorts: vec![fixture.preferred.clone(), fixture.fallback.clone()],
        required_preconditions: Vec::new(),
        maximum_steps: MAX_STEPS,
        cost_order: CostOrder::RiskThenEffortThenSteps,
        tie_policy: TiePolicy::DenyAmbiguous,
    }
}

pub(crate) fn request(
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

pub(crate) const fn selected(outcome: &PlanningOutcome) -> Option<&MigrationPlan> {
    match outcome {
        PlanningOutcome::Selected(plan) => Some(plan),
        PlanningOutcome::NoOp(_)
        | PlanningOutcome::NoPath { .. }
        | PlanningOutcome::Ambiguous { .. }
        | PlanningOutcome::BoundExhausted { .. }
        | PlanningOutcome::PreconditionDenied { .. }
        | PlanningOutcome::InvalidCatalog { .. } => None,
    }
}
