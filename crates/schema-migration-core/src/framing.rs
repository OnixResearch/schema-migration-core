#[allow(
    non_trait_imports,
    reason = "temporary bounded reference ordering canonicalizes observation identities; owner: schema-migration-core"
)]
use alloc::vec::Vec;
#[allow(
    non_trait_imports,
    reason = "canonical identity framing uses the published content identity component; owner: schema-migration-core"
)]
use content_identity_core::{Domain, IdentityHasher};

const ENTRY_DOMAIN: &str = "schema-migration-entry.v1";
const CATALOG_DOMAIN: &str = "schema-migration-catalog.v1";
const LIMITS_DOMAIN: &str = "schema-migration-limits.v1";
const POLICY_DOMAIN: &str = "schema-migration-policy.v1";
const OBSERVATIONS_DOMAIN: &str = "schema-migration-observations.v1";
const PATH_DOMAIN: &str = "schema-migration-path.v1";
const IMPLEMENTATIONS_DOMAIN: &str = "schema-migration-implementations.v1";
const PLAN_DOMAIN: &str = "schema-migration-plan.v1";

#[derive(Clone, Copy)]
pub(crate) struct PlanIdentityInput<'a> {
    pub source_data: &'a crate::DataSubjectId,
    pub route: crate::SchemaRoute,
    pub catalog_id: crate::CatalogId,
    pub policy_id: crate::PolicyId,
    pub limits_id: crate::LimitsId,
    pub observations_id: crate::ObservationSetId,
    pub path_id: crate::PathId,
    pub implementations_id: crate::ImplementationSetId,
}

pub(crate) fn entry_id(
    entry: &crate::MigrationEntry,
) -> Result<crate::EntryId, crate::MigrationError> {
    let mut hasher = hasher(ENTRY_DOMAIN)?;
    let reference = entry.binding().reference();
    debug_assert!(!entry.key().as_str().is_empty());
    debug_assert_ne!(reference.source_schema(), reference.target_schema());
    let reference_id = reference
        .reference_id()
        .map_err(|_error| crate::MigrationError::IdentityFailure)?;
    hasher.update_tagged_str(b"key", entry.key().as_str())?;
    hasher.update_tagged_bytes(b"lineage", entry.binding().lineage().as_bytes())?;
    hasher.update_tagged_bytes(b"source", reference.source_schema().as_bytes())?;
    hasher.update_tagged_bytes(b"target", reference.target_schema().as_bytes())?;
    hasher.update_tagged_bytes(b"reference", reference_id.as_bytes())?;
    hasher.update_tagged_str(b"artifact", reference.artifact_id().as_str())?;
    hasher.update_tagged_str(b"reference-policy", reference.policy_profile().as_str())?;
    hasher.update_tagged_str(b"direction", direction_name(entry.metadata().direction()))?;
    hasher.update_tagged_str(b"cohort", entry.metadata().implementation_cohort().as_str())?;
    hasher.update_tagged_str(b"profile", entry.metadata().profile().as_str())?;
    hasher.update_len_le(entry.metadata().preconditions().len())?;
    for precondition in entry.metadata().preconditions() {
        update_precondition(&mut hasher, precondition)?;
    }
    hasher.update_tagged_str(
        b"reversibility",
        reversibility_name(entry.metadata().reversibility()),
    )?;
    hasher.update_tagged_u64_le(b"risk", entry.metadata().cost().risk());
    hasher.update_tagged_u64_le(b"effort", entry.metadata().cost().effort());
    Ok(crate::EntryId::from_bytes(*hasher.finish().as_bytes()))
}

pub(crate) fn catalog_id(
    entries: &[crate::AdmittedEntry],
) -> Result<crate::CatalogId, crate::MigrationError> {
    let mut hasher = hasher(CATALOG_DOMAIN)?;
    hasher.update_len_le(entries.len())?;
    for entry in entries {
        hasher.update_tagged_bytes(b"entry", entry.id().as_bytes())?;
    }
    Ok(crate::CatalogId::from_bytes(*hasher.finish().as_bytes()))
}

pub(crate) fn limits_id(
    limits: crate::PlanningLimits,
) -> Result<crate::LimitsId, crate::MigrationError> {
    let mut hasher = hasher(LIMITS_DOMAIN)?;
    hasher.update_tagged_u64_le(b"entries", u64::from(limits.maximum_entries()));
    hasher.update_tagged_u64_le(b"outgoing", u64::from(limits.maximum_outgoing_edges()));
    hasher.update_tagged_u64_le(b"steps", u64::from(limits.maximum_path_steps()));
    hasher.update_tagged_u64_le(b"candidates", u64::from(limits.maximum_candidate_paths()));
    hasher.update_tagged_u64_le(b"states", u64::from(limits.maximum_search_states()));
    hasher.update_tagged_u64_le(b"preconditions", u64::from(limits.maximum_preconditions()));
    hasher.update_tagged_u64_le(b"diagnostics", u64::from(limits.maximum_diagnostics()));
    Ok(crate::LimitsId::from_bytes(*hasher.finish().as_bytes()))
}

pub(crate) fn policy_id(
    policy: &crate::PlanningPolicy,
) -> Result<crate::PolicyId, crate::MigrationError> {
    debug_assert!(!policy.implementation_cohorts().is_empty());
    debug_assert!(policy.maximum_steps() > 0);
    let mut hasher = hasher(POLICY_DOMAIN)?;
    hasher.update_tagged_bool(
        b"forward",
        policy.allows_direction(crate::Direction::Forward),
    );
    hasher.update_tagged_bool(
        b"reverse",
        policy.allows_direction(crate::Direction::Reverse),
    );
    hasher.update_len_le(policy.implementation_cohorts().len())?;
    for cohort in policy.implementation_cohorts() {
        hasher.update_tagged_str(b"cohort", cohort.as_str())?;
    }
    hasher.update_len_le(policy.required_preconditions().len())?;
    for kind in policy.required_preconditions() {
        hasher.update_tagged_str(b"required", precondition_name(*kind))?;
    }
    hasher.update_tagged_u64_le(b"steps", u64::from(policy.maximum_steps()));
    hasher.update_tagged_str(b"cost-order", cost_order_name(policy.cost_order()))?;
    hasher.update_tagged_str(b"tie-policy", tie_policy_name(policy.tie_policy()))?;
    Ok(crate::PolicyId::from_bytes(*hasher.finish().as_bytes()))
}

pub(crate) fn observations_id(
    observations: &crate::ObservationSet,
) -> Result<crate::ObservationSetId, crate::MigrationError> {
    let mut ordered: Vec<&crate::PreconditionObservation> =
        observations.observations().iter().collect();
    debug_assert_eq!(ordered.len(), observations.observations().len());
    ordered.sort_by(|left, right| {
        left.requirement()
            .cmp(right.requirement())
            .then_with(|| left.observation_id().cmp(right.observation_id()))
            .then_with(|| {
                left.implementation_cohort()
                    .cmp(&right.implementation_cohort())
            })
            .then_with(|| left.is_satisfied().cmp(&right.is_satisfied()))
    });
    for adjacent in ordered.windows(2) {
        if adjacent[0].requirement() == adjacent[1].requirement() {
            return Err(crate::MigrationError::DuplicateObservation {
                precondition: adjacent[0].requirement().id().as_str().into(),
            });
        }
    }
    debug_assert!(
        ordered
            .windows(2)
            .all(|adjacent| adjacent[0].requirement() != adjacent[1].requirement())
    );
    let mut hasher = hasher(OBSERVATIONS_DOMAIN)?;
    hasher.update_len_le(ordered.len())?;
    for observation in ordered {
        update_precondition(&mut hasher, observation.requirement())?;
        hasher.update_tagged_str(b"observation", observation.observation_id().as_str())?;
        hasher.update_tagged_bool(b"satisfied", observation.is_satisfied());
        hasher.update_optional_str(
            b"implementation",
            observation
                .implementation_cohort()
                .map(crate::ImplementationCohortId::as_str),
        )?;
    }
    Ok(crate::ObservationSetId::from_bytes(
        *hasher.finish().as_bytes(),
    ))
}

pub(crate) fn path_id(
    entries: &[&crate::AdmittedEntry],
) -> Result<crate::PathId, crate::MigrationError> {
    let mut hasher = hasher(PATH_DOMAIN)?;
    hasher.update_len_le(entries.len())?;
    for entry in entries {
        hasher.update_tagged_bytes(b"entry", entry.id().as_bytes())?;
    }
    Ok(crate::PathId::from_bytes(*hasher.finish().as_bytes()))
}

pub(crate) fn implementations_id(
    entries: &[&crate::AdmittedEntry],
) -> Result<crate::ImplementationSetId, crate::MigrationError> {
    let mut hasher = hasher(IMPLEMENTATIONS_DOMAIN)?;
    hasher.update_len_le(entries.len())?;
    for entry in entries {
        hasher.update_tagged_str(b"cohort", entry.metadata().implementation_cohort().as_str())?;
    }
    Ok(crate::ImplementationSetId::from_bytes(
        *hasher.finish().as_bytes(),
    ))
}

pub(crate) fn plan_id(
    input: &PlanIdentityInput<'_>,
) -> Result<crate::PlanId, crate::MigrationError> {
    let mut hasher = hasher(PLAN_DOMAIN)?;
    hasher.update_tagged_str(b"source-data", input.source_data.as_str())?;
    hasher.update_tagged_bytes(b"lineage", input.route.lineage().as_bytes())?;
    hasher.update_tagged_bytes(b"source", input.route.source().as_bytes())?;
    hasher.update_tagged_bytes(b"target", input.route.target().as_bytes())?;
    hasher.update_tagged_bytes(b"catalog", input.catalog_id.as_bytes())?;
    hasher.update_tagged_bytes(b"policy", input.policy_id.as_bytes())?;
    hasher.update_tagged_bytes(b"limits", input.limits_id.as_bytes())?;
    hasher.update_tagged_bytes(b"observations", input.observations_id.as_bytes())?;
    hasher.update_tagged_bytes(b"path", input.path_id.as_bytes())?;
    hasher.update_tagged_bytes(b"implementations", input.implementations_id.as_bytes())?;
    Ok(crate::PlanId::from_bytes(*hasher.finish().as_bytes()))
}

#[allow(
    concrete_construction_in_core,
    reason = "the reviewed content-identity component is a deterministic value mechanism, not an external effect adapter; owner: schema-migration-core"
)]
fn hasher(domain: &'static str) -> Result<IdentityHasher, crate::MigrationError> {
    Ok(IdentityHasher::new(Domain::new(domain)?))
}

#[allow(
    mutating_input_in_pure,
    reason = "the mutable hasher is a local deterministic value accumulator with no external authority; owner: schema-migration-core"
)]
fn update_precondition(
    hasher: &mut IdentityHasher,
    precondition: &crate::PreconditionRequirement,
) -> Result<(), crate::MigrationError> {
    hasher.update_tagged_str(b"precondition", precondition.id().as_str())?;
    hasher.update_tagged_str(b"precondition-kind", precondition_name(precondition.kind()))?;
    Ok(())
}

pub(crate) const fn direction_name(direction: crate::Direction) -> &'static str {
    match direction {
        crate::Direction::Forward => "forward",
        crate::Direction::Reverse => "reverse",
    }
}

pub(crate) const fn precondition_name(kind: crate::PreconditionKind) -> &'static str {
    match kind {
        crate::PreconditionKind::Data => "data",
        crate::PreconditionKind::Environment => "environment",
        crate::PreconditionKind::Implementation => "implementation",
        crate::PreconditionKind::Feature => "feature",
        crate::PreconditionKind::AuthorityObservation => "authority-observation",
        crate::PreconditionKind::Resource => "resource",
    }
}

pub(crate) const fn reversibility_name(class: crate::ReversibilityClass) -> &'static str {
    match class {
        crate::ReversibilityClass::Irreversible => "irreversible",
        crate::ReversibilityClass::SchemaRollback => "schema-rollback",
        crate::ReversibilityClass::Compensatable => "compensatable",
    }
}

const fn cost_order_name(order: crate::CostOrder) -> &'static str {
    match order {
        crate::CostOrder::RiskThenEffortThenSteps => "risk-effort-steps",
        crate::CostOrder::EffortThenRiskThenSteps => "effort-risk-steps",
        crate::CostOrder::StepsThenRiskThenEffort => "steps-risk-effort",
    }
}

const fn tie_policy_name(policy: crate::TiePolicy) -> &'static str {
    match policy {
        crate::TiePolicy::DenyAmbiguous => "deny-ambiguous",
    }
}
