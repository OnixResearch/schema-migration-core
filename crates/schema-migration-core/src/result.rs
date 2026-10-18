#[allow(
    non_trait_imports,
    reason = "plans and bounded candidate summaries own explicit ordered facts; owner: schema-migration-core"
)]
use alloc::vec::Vec;
#[allow(
    non_trait_imports,
    reason = "published schema identity types remain explicit plan boundary facts; owner: schema-migration-core"
)]
use schema_identity_core::{MigrationArtifactId, PolicyProfileId, SchemaId, TypeLineageId};

/// Terminal migration planning result classes.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanningDecisionClass {
    /// Source and target schema identities are equal.
    NoOp,
    /// One unique eligible path was selected.
    Selected,
    /// No policy-admitted path reached the target.
    NoPath,
    /// Multiple paths remained equal under every selected rule.
    Ambiguous,
    /// Search stopped at one explicit bound.
    BoundExhausted,
    /// Paths existed, but supplied preconditions denied each path.
    PreconditionDenied,
    /// Catalog admission or checked arithmetic failed.
    InvalidCatalog,
}

/// Explicit bound that stopped complete path selection.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundReason {
    /// The path-step limit prevented complete exploration.
    PathSteps,
    /// The complete candidate-path limit was exceeded.
    CandidatePaths,
    /// The search-state limit was exceeded.
    SearchStates,
}

/// Closed precondition denial classes.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreconditionDenial {
    /// A policy-required precondition class was not declared by one step.
    RequiredClassMissing,
    /// A declared precondition had no supplied observation.
    ObservationMissing,
    /// A supplied observation was not satisfied.
    ObservationDenied,
    /// An implementation observation named another cohort.
    ImplementationCohortMismatch,
    /// The observation set was malformed or could not be identified.
    ObservationSetInvalid,
}

/// Closed invalid-catalog and checked-arithmetic classes.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvalidCatalogReason {
    /// The entry count exceeded the selected limit.
    EntryLimit,
    /// One schema exceeded the selected outgoing-edge limit.
    OutgoingEdgeLimit,
    /// One entry exceeded the selected precondition limit.
    PreconditionLimit,
    /// Stable entry keys were not unique.
    DuplicateEntry,
    /// An entry connected one exact schema to itself.
    SelfEdge,
    /// Cost addition overflowed.
    CostOverflow,
    /// Canonical identity framing failed.
    IdentityFailure,
}

/// One selected migration step with complete execution-plan bindings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanStep {
    index: u16,
    entry_id: crate::EntryId,
    source_schema: SchemaId,
    target_schema: SchemaId,
    artifact_id: MigrationArtifactId,
    reference_policy: PolicyProfileId,
    implementation_cohort: crate::ImplementationCohortId,
    profile: crate::MigrationProfileId,
    preconditions: Vec<crate::PreconditionRequirement>,
    reversibility: crate::ReversibilityClass,
    cost: crate::CostFacts,
}

impl PlanStep {
    pub(crate) fn from_entry(index: u16, entry: &crate::AdmittedEntry) -> Self {
        let reference = entry.binding().reference();
        Self {
            index,
            entry_id: entry.id(),
            source_schema: reference.source_schema(),
            target_schema: reference.target_schema(),
            artifact_id: reference.artifact_id().clone(),
            reference_policy: reference.policy_profile().clone(),
            implementation_cohort: entry.metadata().implementation_cohort().clone(),
            profile: entry.metadata().profile().clone(),
            preconditions: entry.metadata().preconditions().to_vec(),
            reversibility: entry.metadata().reversibility(),
            cost: entry.metadata().cost(),
        }
    }

    /// Returns the zero-based ordered step index.
    #[must_use]
    pub const fn index(&self) -> u16 {
        self.index
    }

    /// Returns the complete catalog entry identity.
    #[must_use]
    pub const fn entry_id(&self) -> crate::EntryId {
        self.entry_id
    }

    /// Returns the exact input schema.
    #[must_use]
    pub const fn source_schema(&self) -> SchemaId {
        self.source_schema
    }

    /// Returns the exact expected output schema.
    #[must_use]
    pub const fn target_schema(&self) -> SchemaId {
        self.target_schema
    }

    /// Returns the caller-owned migration artifact identity.
    #[must_use]
    pub const fn artifact_id(&self) -> &MigrationArtifactId {
        &self.artifact_id
    }

    /// Returns the Schema Identity Core policy profile bound to the reference.
    #[must_use]
    pub const fn reference_policy(&self) -> &PolicyProfileId {
        &self.reference_policy
    }

    /// Returns the reviewed implementation cohort.
    #[must_use]
    pub const fn implementation_cohort(&self) -> &crate::ImplementationCohortId {
        &self.implementation_cohort
    }

    /// Returns the migration behavior profile.
    #[must_use]
    pub const fn profile(&self) -> &crate::MigrationProfileId {
        &self.profile
    }

    /// Returns required facts.
    #[must_use]
    pub fn preconditions(&self) -> &[crate::PreconditionRequirement] {
        &self.preconditions
    }

    /// Returns the reversibility declaration.
    #[must_use]
    pub const fn reversibility(&self) -> crate::ReversibilityClass {
        self.reversibility
    }

    /// Returns additive step costs.
    #[must_use]
    pub const fn cost(&self) -> crate::CostFacts {
        self.cost
    }
}

/// Bounded metadata summary for one equal candidate path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathSummary {
    path_id: crate::PathId,
    entry_ids: Vec<crate::EntryId>,
    cost: crate::CostFacts,
}

impl PathSummary {
    pub(crate) fn new(
        path_id: crate::PathId,
        entry_ids: Vec<crate::EntryId>,
        cost: crate::CostFacts,
    ) -> Self {
        Self {
            path_id,
            entry_ids,
            cost,
        }
    }

    /// Returns the ordered path identity.
    #[must_use]
    pub const fn path_id(&self) -> crate::PathId {
        self.path_id
    }

    /// Returns bounded ordered entry identities.
    #[must_use]
    pub fn entry_ids(&self) -> &[crate::EntryId] {
        &self.entry_ids
    }

    /// Returns checked aggregate costs.
    #[must_use]
    pub const fn cost(&self) -> crate::CostFacts {
        self.cost
    }
}

/// Complete deterministic no-op or selected migration plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationPlan {
    decision: PlanningDecisionClass,
    id: crate::PlanId,
    source_data: crate::DataSubjectId,
    lineage: TypeLineageId,
    source_schema: SchemaId,
    target_schema: SchemaId,
    catalog_id: crate::CatalogId,
    policy_id: crate::PolicyId,
    limits_id: crate::LimitsId,
    observations_id: crate::ObservationSetId,
    path_id: crate::PathId,
    implementations_id: crate::ImplementationSetId,
    steps: Vec<PlanStep>,
    effects: Vec<crate::EffectRequest>,
}

impl MigrationPlan {
    /// Returns the terminal plan decision.
    #[must_use]
    pub const fn decision(&self) -> PlanningDecisionClass {
        self.decision
    }

    /// Returns the complete plan identity.
    #[must_use]
    pub const fn id(&self) -> crate::PlanId {
        self.id
    }

    /// Returns the source data identity.
    #[must_use]
    pub const fn source_data(&self) -> &crate::DataSubjectId {
        &self.source_data
    }

    /// Returns the durable schema lineage.
    #[must_use]
    pub const fn lineage(&self) -> TypeLineageId {
        self.lineage
    }

    /// Returns the exact source schema.
    #[must_use]
    pub const fn source_schema(&self) -> SchemaId {
        self.source_schema
    }

    /// Returns the exact target schema.
    #[must_use]
    pub const fn target_schema(&self) -> SchemaId {
        self.target_schema
    }

    /// Returns the catalog snapshot identity.
    #[must_use]
    pub const fn catalog_id(&self) -> crate::CatalogId {
        self.catalog_id
    }

    /// Returns the path-selection policy identity.
    #[must_use]
    pub const fn policy_id(&self) -> crate::PolicyId {
        self.policy_id
    }

    /// Returns the resource-limit identity.
    #[must_use]
    pub const fn limits_id(&self) -> crate::LimitsId {
        self.limits_id
    }

    /// Returns the precondition observation-set identity.
    #[must_use]
    pub const fn observations_id(&self) -> crate::ObservationSetId {
        self.observations_id
    }

    /// Returns the selected ordered path identity.
    #[must_use]
    pub const fn path_id(&self) -> crate::PathId {
        self.path_id
    }

    /// Returns the ordered selected implementation-cohort identity.
    #[must_use]
    pub const fn implementations_id(&self) -> crate::ImplementationSetId {
        self.implementations_id
    }

    /// Returns ordered migration steps.
    #[must_use]
    pub fn steps(&self) -> &[PlanStep] {
        &self.steps
    }

    /// Returns ordered typed effect requests.
    #[must_use]
    pub fn effects(&self) -> &[crate::EffectRequest] {
        &self.effects
    }
}

/// Exact identities retained by a terminal decision without a plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionBindings {
    source_data: crate::DataSubjectId,
    lineage: TypeLineageId,
    source_schema: SchemaId,
    target_schema: SchemaId,
    catalog_id: Option<crate::CatalogId>,
    policy_id: crate::PolicyId,
    limits_id: crate::LimitsId,
    observations_id: crate::ObservationSetId,
}

impl DecisionBindings {
    pub(crate) fn new(
        request: &crate::PlanningRequest,
        catalog_id: Option<crate::CatalogId>,
        policy_id: crate::PolicyId,
        limits_id: crate::LimitsId,
        observations_id: crate::ObservationSetId,
    ) -> Self {
        let route = request.route();
        Self {
            source_data: request.source_data().clone(),
            lineage: route.lineage(),
            source_schema: route.source(),
            target_schema: route.target(),
            catalog_id,
            policy_id,
            limits_id,
            observations_id,
        }
    }

    /// Returns the source data identity.
    #[must_use]
    pub const fn source_data(&self) -> &crate::DataSubjectId {
        &self.source_data
    }

    /// Returns the durable schema lineage.
    #[must_use]
    pub const fn lineage(&self) -> TypeLineageId {
        self.lineage
    }

    /// Returns the exact source schema.
    #[must_use]
    pub const fn source_schema(&self) -> SchemaId {
        self.source_schema
    }

    /// Returns the exact target schema.
    #[must_use]
    pub const fn target_schema(&self) -> SchemaId {
        self.target_schema
    }

    /// Returns the catalog identity when catalog admission succeeded.
    #[must_use]
    pub const fn catalog_id(&self) -> Option<crate::CatalogId> {
        self.catalog_id
    }

    /// Returns the planning policy identity.
    #[must_use]
    pub const fn policy_id(&self) -> crate::PolicyId {
        self.policy_id
    }

    /// Returns the resource-limit identity.
    #[must_use]
    pub const fn limits_id(&self) -> crate::LimitsId {
        self.limits_id
    }

    /// Returns the precondition observation-set identity.
    #[must_use]
    pub const fn observations_id(&self) -> crate::ObservationSetId {
        self.observations_id
    }
}

/// Complete terminal result from bounded migration planning.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanningOutcome {
    /// Equal exact schemas require no migration commands.
    NoOp(MigrationPlan),
    /// One unique eligible path and effect plan.
    Selected(MigrationPlan),
    /// No policy-admitted path reached the target.
    NoPath {
        /// Exact identities evaluated by the decision.
        bindings: DecisionBindings,
    },
    /// Equal best candidates remain unresolved.
    Ambiguous {
        /// Exact identities evaluated by the decision.
        bindings: DecisionBindings,
        /// Bounded equal candidate summaries.
        candidates: Vec<PathSummary>,
    },
    /// Search stopped at one explicit bound.
    BoundExhausted {
        /// Exact identities evaluated by the decision.
        bindings: DecisionBindings,
        /// Exact limiting class.
        reason: BoundReason,
    },
    /// Existing paths failed supplied preconditions.
    PreconditionDenied {
        /// Exact identities evaluated by the decision when available.
        bindings: Option<DecisionBindings>,
        /// Exact denial class.
        reason: PreconditionDenial,
    },
    /// Catalog admission or checked arithmetic failed.
    InvalidCatalog {
        /// Exact identities evaluated before failure when available.
        bindings: Option<DecisionBindings>,
        /// Exact invalidity class.
        reason: InvalidCatalogReason,
    },
}

impl PlanningOutcome {
    /// Returns the closed terminal decision class.
    #[must_use]
    pub const fn decision(&self) -> PlanningDecisionClass {
        match self {
            Self::NoOp(_) => PlanningDecisionClass::NoOp,
            Self::Selected(_) => PlanningDecisionClass::Selected,
            Self::NoPath { .. } => PlanningDecisionClass::NoPath,
            Self::Ambiguous { .. } => PlanningDecisionClass::Ambiguous,
            Self::BoundExhausted { .. } => PlanningDecisionClass::BoundExhausted,
            Self::PreconditionDenied { .. } => PlanningDecisionClass::PreconditionDenied,
            Self::InvalidCatalog { .. } => PlanningDecisionClass::InvalidCatalog,
        }
    }

    /// Returns a plan only for no-op and selected outcomes.
    #[must_use]
    pub const fn plan(&self) -> Option<&MigrationPlan> {
        match self {
            Self::NoOp(plan) | Self::Selected(plan) => Some(plan),
            Self::NoPath { .. }
            | Self::Ambiguous { .. }
            | Self::BoundExhausted { .. }
            | Self::PreconditionDenied { .. }
            | Self::InvalidCatalog { .. } => None,
        }
    }
}

pub(crate) struct BuildInput<'a> {
    pub decision: PlanningDecisionClass,
    pub request: &'a crate::PlanningRequest,
    pub catalog_id: crate::CatalogId,
    pub policy_id: crate::PolicyId,
    pub limits_id: crate::LimitsId,
    pub observations_id: crate::ObservationSetId,
    pub entries: Vec<&'a crate::AdmittedEntry>,
}

pub(crate) fn build(input: &BuildInput<'_>) -> Result<MigrationPlan, crate::MigrationError> {
    debug_assert!(u16::try_from(input.entries.len()).is_ok());
    debug_assert_eq!(
        input.entries.is_empty(),
        input.decision == PlanningDecisionClass::NoOp
    );
    let path_id = crate::framing::path_id(&input.entries)?;
    let implementations_id = crate::framing::implementations_id(&input.entries)?;
    let route = input.request.route();
    let id = crate::framing::plan_id(&crate::framing::PlanIdentityInput {
        source_data: input.request.source_data(),
        route,
        catalog_id: input.catalog_id,
        policy_id: input.policy_id,
        limits_id: input.limits_id,
        observations_id: input.observations_id,
        path_id,
        implementations_id,
    })?;
    let mut steps = Vec::with_capacity(input.entries.len());
    for (index, entry) in input.entries.iter().enumerate() {
        let index =
            u16::try_from(index).map_err(|_error| crate::MigrationError::IdentityFailure)?;
        steps.push(PlanStep::from_entry(index, entry));
    }
    let effects = crate::requests::build(&steps, input.limits_id);
    Ok(MigrationPlan {
        decision: input.decision,
        id,
        source_data: input.request.source_data().clone(),
        lineage: route.lineage(),
        source_schema: route.source(),
        target_schema: route.target(),
        catalog_id: input.catalog_id,
        policy_id: input.policy_id,
        limits_id: input.limits_id,
        observations_id: input.observations_id,
        path_id,
        implementations_id,
        steps,
        effects,
    })
}
