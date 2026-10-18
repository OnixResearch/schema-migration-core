#[allow(
    non_trait_imports,
    reason = "metadata-only receipts retain bounded candidate summaries; owner: schema-migration-core"
)]
use alloc::vec::Vec;
#[allow(
    non_trait_imports,
    reason = "published exact schema identities remain explicit evidence facts; owner: schema-migration-core"
)]
use schema_identity_core::SchemaId;

/// Required claim boundaries for every planner receipt.
pub const RECEIPT_NON_CLAIMS: [&str; 4] = [
    "a migration plan does not prove migration implementation correctness",
    "a migration plan does not prove successful execution or atomicity",
    "a rollback request does not prove rollback success or restored external effects",
    "a planner receipt does not grant migration, storage, transaction, or release authority",
];

/// Stable receipt reason projection.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptReason {
    /// Exact source and target schemas were equal.
    NoOp,
    /// One uniquely ranked eligible path was selected.
    UniquePath,
    /// No policy-admitted path reached the target.
    NoPath,
    /// Multiple best paths remained equal.
    Ambiguous,
    /// Search reached an explicit bound.
    Bound(crate::BoundReason),
    /// Supplied preconditions denied all paths.
    Preconditions(crate::PreconditionDenial),
    /// Catalog admission or checked arithmetic failed.
    InvalidCatalog(crate::InvalidCatalogReason),
}

/// Bounded metadata-only projection of one terminal planning result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanReceipt {
    decision: crate::PlanningDecisionClass,
    reason: ReceiptReason,
    plan_id: Option<crate::PlanId>,
    source_data: Option<crate::DataSubjectId>,
    lineage: Option<crate::TypeLineageId>,
    source_schema: Option<SchemaId>,
    target_schema: Option<SchemaId>,
    catalog_id: Option<crate::CatalogId>,
    policy_id: Option<crate::PolicyId>,
    limits_id: Option<crate::LimitsId>,
    observations_id: Option<crate::ObservationSetId>,
    path_id: Option<crate::PathId>,
    step_count: u16,
    candidates: Vec<crate::PathSummary>,
}

impl PlanReceipt {
    /// Returns the terminal decision class.
    #[must_use]
    pub const fn decision(&self) -> crate::PlanningDecisionClass {
        self.decision
    }

    /// Returns the stable reason projection.
    #[must_use]
    pub const fn reason(&self) -> ReceiptReason {
        self.reason
    }

    /// Returns the plan identity when planning produced a plan.
    #[must_use]
    pub const fn plan_id(&self) -> Option<crate::PlanId> {
        self.plan_id
    }

    /// Returns the source data identity when decision binding succeeded.
    #[must_use]
    pub const fn source_data(&self) -> Option<&crate::DataSubjectId> {
        self.source_data.as_ref()
    }

    /// Returns the durable schema lineage when decision binding succeeded.
    #[must_use]
    pub const fn lineage(&self) -> Option<crate::TypeLineageId> {
        self.lineage
    }

    /// Returns the exact source schema when decision binding succeeded.
    #[must_use]
    pub const fn source_schema(&self) -> Option<SchemaId> {
        self.source_schema
    }

    /// Returns the exact target schema when planning produced a plan.
    #[must_use]
    pub const fn target_schema(&self) -> Option<SchemaId> {
        self.target_schema
    }

    /// Returns the catalog snapshot identity when planning produced a plan.
    #[must_use]
    pub const fn catalog_id(&self) -> Option<crate::CatalogId> {
        self.catalog_id
    }

    /// Returns the planning policy identity when planning produced a plan.
    #[must_use]
    pub const fn policy_id(&self) -> Option<crate::PolicyId> {
        self.policy_id
    }

    /// Returns the resource-limit identity when planning produced a plan.
    #[must_use]
    pub const fn limits_id(&self) -> Option<crate::LimitsId> {
        self.limits_id
    }

    /// Returns the precondition observation-set identity when available.
    #[must_use]
    pub const fn observations_id(&self) -> Option<crate::ObservationSetId> {
        self.observations_id
    }

    /// Returns the selected path identity when planning produced a plan.
    #[must_use]
    pub const fn path_id(&self) -> Option<crate::PathId> {
        self.path_id
    }

    /// Returns the bounded selected step count.
    #[must_use]
    pub const fn step_count(&self) -> u16 {
        self.step_count
    }

    /// Returns bounded equal candidate summaries for ambiguity.
    #[must_use]
    pub fn candidates(&self) -> &[crate::PathSummary] {
        &self.candidates
    }

    /// Returns mandatory non-claims.
    #[must_use]
    pub const fn non_claims(&self) -> &'static [&'static str; 4] {
        &RECEIPT_NON_CLAIMS
    }
}

// r[impl onix.schema_migration.receipts]
/// Projects one terminal decision into bounded metadata without source data bytes.
#[must_use]
pub fn project_receipt(outcome: &crate::PlanningOutcome) -> PlanReceipt {
    match outcome {
        crate::PlanningOutcome::NoOp(plan) => from_plan(plan, ReceiptReason::NoOp),
        crate::PlanningOutcome::Selected(plan) => from_plan(plan, ReceiptReason::UniquePath),
        crate::PlanningOutcome::NoPath { bindings } => from_bindings(
            bindings,
            crate::PlanningDecisionClass::NoPath,
            ReceiptReason::NoPath,
            Vec::new(),
        ),
        crate::PlanningOutcome::Ambiguous {
            bindings,
            candidates,
        } => from_bindings(
            bindings,
            crate::PlanningDecisionClass::Ambiguous,
            ReceiptReason::Ambiguous,
            candidates.clone(),
        ),
        crate::PlanningOutcome::BoundExhausted { bindings, reason } => from_bindings(
            bindings,
            crate::PlanningDecisionClass::BoundExhausted,
            ReceiptReason::Bound(*reason),
            Vec::new(),
        ),
        crate::PlanningOutcome::PreconditionDenied { bindings, reason } => from_optional_bindings(
            bindings.as_ref(),
            crate::PlanningDecisionClass::PreconditionDenied,
            ReceiptReason::Preconditions(*reason),
        ),
        crate::PlanningOutcome::InvalidCatalog { bindings, reason } => from_optional_bindings(
            bindings.as_ref(),
            crate::PlanningDecisionClass::InvalidCatalog,
            ReceiptReason::InvalidCatalog(*reason),
        ),
    }
}

fn from_plan(plan: &crate::MigrationPlan, reason: ReceiptReason) -> PlanReceipt {
    debug_assert!(matches!(
        plan.decision(),
        crate::PlanningDecisionClass::NoOp | crate::PlanningDecisionClass::Selected
    ));
    debug_assert_eq!(
        plan.steps().is_empty(),
        plan.decision() == crate::PlanningDecisionClass::NoOp
    );
    let step_count = plan
        .steps()
        .last()
        .map_or(0, |step| step.index().saturating_add(1));
    PlanReceipt {
        decision: plan.decision(),
        reason,
        plan_id: Some(plan.id()),
        source_data: Some(plan.source_data().clone()),
        lineage: Some(plan.lineage()),
        source_schema: Some(plan.source_schema()),
        target_schema: Some(plan.target_schema()),
        catalog_id: Some(plan.catalog_id()),
        policy_id: Some(plan.policy_id()),
        limits_id: Some(plan.limits_id()),
        observations_id: Some(plan.observations_id()),
        path_id: Some(plan.path_id()),
        step_count,
        candidates: Vec::new(),
    }
}

fn from_bindings(
    bindings: &crate::DecisionBindings,
    decision: crate::PlanningDecisionClass,
    reason: ReceiptReason,
    candidates: Vec<crate::PathSummary>,
) -> PlanReceipt {
    PlanReceipt {
        decision,
        reason,
        plan_id: None,
        source_data: Some(bindings.source_data().clone()),
        lineage: Some(bindings.lineage()),
        source_schema: Some(bindings.source_schema()),
        target_schema: Some(bindings.target_schema()),
        catalog_id: bindings.catalog_id(),
        policy_id: Some(bindings.policy_id()),
        limits_id: Some(bindings.limits_id()),
        observations_id: Some(bindings.observations_id()),
        path_id: None,
        step_count: 0,
        candidates,
    }
}

fn from_optional_bindings(
    bindings: Option<&crate::DecisionBindings>,
    decision: crate::PlanningDecisionClass,
    reason: ReceiptReason,
) -> PlanReceipt {
    match bindings {
        Some(bindings) => from_bindings(bindings, decision, reason, Vec::new()),
        None => PlanReceipt {
            decision,
            reason,
            plan_id: None,
            source_data: None,
            lineage: None,
            source_schema: None,
            target_schema: None,
            catalog_id: None,
            policy_id: None,
            limits_id: None,
            observations_id: None,
            path_id: None,
            step_count: 0,
            candidates: Vec::new(),
        },
    }
}
