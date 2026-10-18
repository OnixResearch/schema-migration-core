#[allow(
    non_trait_imports,
    reason = "ordered effect requests own bounded precondition projections; owner: schema-migration-core"
)]
use alloc::vec::Vec;
#[allow(
    non_trait_imports,
    reason = "published schema and artifact identities remain explicit request facts; owner: schema-migration-core"
)]
use schema_identity_core::{MigrationArtifactId, SchemaId};

/// Closed migration effect-request classes.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectKind {
    /// Load the exact step input selected by the consumer.
    Load,
    /// Validate the exact input schema and preconditions.
    ValidateInput,
    /// Record a consumer-owned checkpoint before migration.
    Checkpoint,
    /// Invoke the selected migration artifact.
    Migrate,
    /// Validate the exact expected output schema.
    ValidateOutput,
    /// Request consumer-owned transaction commit.
    Commit,
    /// Conditionally request schema rollback.
    Rollback,
    /// Conditionally request external-effect compensation.
    Compensation,
    /// Record a metadata-only step receipt.
    Receipt,
}

/// One typed effect request emitted by the pure planner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectRequest {
    kind: EffectKind,
    step_index: u16,
    input_schema: SchemaId,
    expected_output_schema: SchemaId,
    artifact_id: MigrationArtifactId,
    implementation_cohort: crate::ImplementationCohortId,
    preconditions: Vec<crate::PreconditionRequirement>,
    limits_id: crate::LimitsId,
    is_conditional: bool,
}

impl EffectRequest {
    /// Returns the requested effect class.
    #[must_use]
    pub const fn kind(&self) -> EffectKind {
        self.kind
    }

    /// Returns the selected plan-step index.
    #[must_use]
    pub const fn step_index(&self) -> u16 {
        self.step_index
    }

    /// Returns the exact input schema.
    #[must_use]
    pub const fn input_schema(&self) -> SchemaId {
        self.input_schema
    }

    /// Returns the exact expected output schema.
    #[must_use]
    pub const fn expected_output_schema(&self) -> SchemaId {
        self.expected_output_schema
    }

    /// Returns the caller-owned migration artifact identity.
    #[must_use]
    pub const fn artifact_id(&self) -> &MigrationArtifactId {
        &self.artifact_id
    }

    /// Returns the reviewed implementation cohort.
    #[must_use]
    pub const fn implementation_cohort(&self) -> &crate::ImplementationCohortId {
        &self.implementation_cohort
    }

    /// Returns exact required preconditions.
    #[must_use]
    pub fn preconditions(&self) -> &[crate::PreconditionRequirement] {
        &self.preconditions
    }

    /// Returns the bound resource-limit identity.
    #[must_use]
    pub const fn limits_id(&self) -> crate::LimitsId {
        self.limits_id
    }

    /// Returns whether execution is conditional on a consumer-observed failure.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        self.is_conditional
    }
}

const MAXIMUM_REQUEST_COUNT_PER_STEP: usize = 8;

// r[impl onix.schema_migration.effect_plan]
pub(crate) fn build(steps: &[crate::PlanStep], limits_id: crate::LimitsId) -> Vec<EffectRequest> {
    debug_assert!(u16::try_from(steps.len()).is_ok());
    let mut requests =
        Vec::with_capacity(steps.len().saturating_mul(MAXIMUM_REQUEST_COUNT_PER_STEP));
    for step in steps {
        requests.push(request(EffectKind::Load, step, limits_id, false));
        requests.push(request(EffectKind::ValidateInput, step, limits_id, false));
        requests.push(request(EffectKind::Checkpoint, step, limits_id, false));
        requests.push(request(EffectKind::Migrate, step, limits_id, false));
        requests.push(request(EffectKind::ValidateOutput, step, limits_id, false));
        requests.push(request(EffectKind::Commit, step, limits_id, false));
        match step.reversibility() {
            crate::ReversibilityClass::Irreversible => {}
            crate::ReversibilityClass::SchemaRollback => {
                requests.push(request(EffectKind::Rollback, step, limits_id, true));
            }
            crate::ReversibilityClass::Compensatable => {
                requests.push(request(EffectKind::Compensation, step, limits_id, true));
            }
        }
        requests.push(request(EffectKind::Receipt, step, limits_id, false));
        debug_assert!(requests.len() <= requests.capacity());
    }
    debug_assert!(requests.len() <= requests.capacity());
    requests
}

fn request(
    kind: EffectKind,
    step: &crate::PlanStep,
    limits_id: crate::LimitsId,
    is_conditional: bool,
) -> EffectRequest {
    debug_assert_ne!(step.source_schema(), step.target_schema());
    EffectRequest {
        kind,
        step_index: step.index(),
        input_schema: step.source_schema(),
        expected_output_schema: step.target_schema(),
        artifact_id: step.artifact_id().clone(),
        implementation_cohort: step.implementation_cohort().clone(),
        preconditions: step.preconditions().to_vec(),
        limits_id,
        is_conditional,
    }
}
