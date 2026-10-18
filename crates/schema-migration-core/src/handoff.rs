#[allow(
    non_trait_imports,
    reason = "published exact schema identities remain explicit freshness facts; owner: schema-migration-core"
)]
use schema_identity_core::SchemaId;

/// Current exact identity observations supplied before execution handoff.
pub struct FreshnessInput {
    /// Current source data identity.
    pub source_data: crate::DataSubjectId,
    /// Current source schema identity.
    pub source_schema: SchemaId,
    /// Current target schema identity.
    pub target_schema: SchemaId,
    /// Current catalog snapshot identity.
    pub catalog_id: crate::CatalogId,
    /// Current planning policy identity.
    pub policy_id: crate::PolicyId,
    /// Current resource-limit identity.
    pub limits_id: crate::LimitsId,
    /// Current precondition observation-set identity.
    pub observations_id: crate::ObservationSetId,
    /// Current ordered implementation-cohort identity.
    pub implementations_id: crate::ImplementationSetId,
}

/// Complete current observations for exact pre-handoff freshness checking.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshnessObservation {
    source_data: crate::DataSubjectId,
    source_schema: SchemaId,
    target_schema: SchemaId,
    catalog_id: crate::CatalogId,
    policy_id: crate::PolicyId,
    limits_id: crate::LimitsId,
    observations_id: crate::ObservationSetId,
    implementations_id: crate::ImplementationSetId,
}

impl FreshnessObservation {
    /// Creates one complete current observation set.
    #[must_use]
    pub fn new(input: FreshnessInput) -> Self {
        Self {
            source_data: input.source_data,
            source_schema: input.source_schema,
            target_schema: input.target_schema,
            catalog_id: input.catalog_id,
            policy_id: input.policy_id,
            limits_id: input.limits_id,
            observations_id: input.observations_id,
            implementations_id: input.implementations_id,
        }
    }

    /// Projects matching current observations from one plan for conformance tests.
    #[must_use]
    pub fn matching(plan: &crate::MigrationPlan) -> Self {
        Self {
            source_data: plan.source_data().clone(),
            source_schema: plan.source_schema(),
            target_schema: plan.target_schema(),
            catalog_id: plan.catalog_id(),
            policy_id: plan.policy_id(),
            limits_id: plan.limits_id(),
            observations_id: plan.observations_id(),
            implementations_id: plan.implementations_id(),
        }
    }
}

/// Exact stale-plan reason classes.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaleReason {
    /// Source data identity changed.
    SourceData,
    /// Source schema identity changed.
    SourceSchema,
    /// Target schema identity changed.
    TargetSchema,
    /// Catalog snapshot identity changed.
    Catalog,
    /// Planning policy identity changed.
    Policy,
    /// Resource-limit identity changed.
    Limits,
    /// Precondition observation-set identity changed.
    Preconditions,
    /// Selected implementation cohort identity changed.
    Implementation,
}

/// Closed execution-handoff eligibility result.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FreshnessDecision {
    /// Every current identity equals the plan binding.
    Eligible {
        /// Exact eligible plan identity.
        plan_id: crate::PlanId,
    },
    /// One current identity differs from the plan.
    Stale {
        /// First exact stale binding in deterministic check order.
        reason: StaleReason,
    },
}

// r[impl onix.schema_migration.freshness]
/// Checks exact current facts before a consumer receives execution requests.
#[must_use]
pub fn check_freshness(
    plan: &crate::MigrationPlan,
    current: &FreshnessObservation,
) -> FreshnessDecision {
    debug_assert!(matches!(
        plan.decision(),
        crate::PlanningDecisionClass::NoOp | crate::PlanningDecisionClass::Selected
    ));
    debug_assert_eq!(
        plan.steps().is_empty(),
        plan.decision() == crate::PlanningDecisionClass::NoOp
    );
    if current.source_data != *plan.source_data() {
        return FreshnessDecision::Stale {
            reason: StaleReason::SourceData,
        };
    }
    if current.source_schema != plan.source_schema() {
        return FreshnessDecision::Stale {
            reason: StaleReason::SourceSchema,
        };
    }
    if current.target_schema != plan.target_schema() {
        return FreshnessDecision::Stale {
            reason: StaleReason::TargetSchema,
        };
    }
    if current.catalog_id != plan.catalog_id() {
        return FreshnessDecision::Stale {
            reason: StaleReason::Catalog,
        };
    }
    if current.policy_id != plan.policy_id() {
        return FreshnessDecision::Stale {
            reason: StaleReason::Policy,
        };
    }
    if current.limits_id != plan.limits_id() {
        return FreshnessDecision::Stale {
            reason: StaleReason::Limits,
        };
    }
    if current.observations_id != plan.observations_id() {
        return FreshnessDecision::Stale {
            reason: StaleReason::Preconditions,
        };
    }
    if current.implementations_id != plan.implementations_id() {
        return FreshnessDecision::Stale {
            reason: StaleReason::Implementation,
        };
    }
    FreshnessDecision::Eligible { plan_id: plan.id() }
}
