#[allow(
    non_trait_imports,
    reason = "bounded candidate projections use owned result vectors; owner: schema-migration-core"
)]
use alloc::vec::Vec;

// r[impl onix.schema_migration.functional_core]
// r[impl onix.schema_migration.plan_identity]
/// Admits one catalog and computes one bounded deterministic migration decision.
///
/// This pure function performs no registry, clock, file, process, migration,
/// storage, transaction, network, or publication effect.
#[must_use]
pub fn plan_migration(
    snapshot: &crate::CatalogSnapshot,
    request: &crate::PlanningRequest,
    policy: &crate::PlanningPolicy,
    limits: crate::PlanningLimits,
) -> crate::PlanningOutcome {
    debug_assert!(limits.maximum_entries() > 0);
    debug_assert!(policy.maximum_steps() > 0);
    let policy_id = match crate::framing::policy_id(policy) {
        Ok(identity) => identity,
        Err(error) => return invalid(&error, None),
    };
    let limits_id = match crate::framing::limits_id(limits) {
        Ok(identity) => identity,
        Err(error) => return invalid(&error, None),
    };
    let observations_id = match crate::framing::observations_id(request.observations()) {
        Ok(identity) => identity,
        Err(crate::MigrationError::DuplicateObservation { .. }) => {
            return crate::PlanningOutcome::PreconditionDenied {
                bindings: None,
                reason: crate::PreconditionDenial::ObservationSetInvalid,
            };
        }
        Err(error) => return invalid(&error, None),
    };
    let catalog = match crate::admit_catalog(snapshot, limits) {
        Ok(catalog) => catalog,
        Err(error) => {
            let bindings =
                crate::DecisionBindings::new(request, None, policy_id, limits_id, observations_id);
            return invalid(&error, Some(bindings));
        }
    };
    let bindings = crate::DecisionBindings::new(
        request,
        Some(catalog.id()),
        policy_id,
        limits_id,
        observations_id,
    );
    let shared = SharedPlanFacts {
        request,
        catalog: &catalog,
        policy_id,
        limits_id,
        observations_id,
        bindings,
    };
    if request.route().source() == request.route().target() {
        return build_noop(shared);
    }
    finish_selection(shared, policy, limits)
}

#[derive(Clone)]
struct SharedPlanFacts<'a> {
    request: &'a crate::PlanningRequest,
    catalog: &'a crate::AdmittedCatalog,
    policy_id: crate::PolicyId,
    limits_id: crate::LimitsId,
    observations_id: crate::ObservationSetId,
    bindings: crate::DecisionBindings,
}

#[allow(
    fragile_exhaustive_enum_match,
    missing_const_fn,
    reason = "the closed selection dispatch invokes bounded allocation helpers and consumes one sealed internal state machine; owner: schema-migration-core"
)]
fn finish_selection(
    shared: SharedPlanFacts<'_>,
    policy: &crate::PlanningPolicy,
    limits: crate::PlanningLimits,
) -> crate::PlanningOutcome {
    match crate::selection::search(shared.catalog, shared.request, policy, limits) {
        crate::selection::SearchDecision::Selected(candidate) => build_selected(shared, &candidate),
        crate::selection::SearchDecision::NoPath => crate::PlanningOutcome::NoPath {
            bindings: shared.bindings,
        },
        crate::selection::SearchDecision::Ambiguous(candidates) => {
            build_ambiguous(shared.catalog, shared.bindings, &candidates, limits)
        }
        crate::selection::SearchDecision::Bound(reason) => crate::PlanningOutcome::BoundExhausted {
            bindings: shared.bindings,
            reason,
        },
        crate::selection::SearchDecision::PreconditionDenied(reason) => {
            crate::PlanningOutcome::PreconditionDenied {
                bindings: Some(shared.bindings),
                reason,
            }
        }
        crate::selection::SearchDecision::Invalid(reason) => {
            crate::PlanningOutcome::InvalidCatalog {
                bindings: Some(shared.bindings),
                reason,
            }
        }
    }
}

fn build_noop(shared: SharedPlanFacts<'_>) -> crate::PlanningOutcome {
    let input = crate::result::BuildInput {
        decision: crate::PlanningDecisionClass::NoOp,
        request: shared.request,
        catalog_id: shared.catalog.id(),
        policy_id: shared.policy_id,
        limits_id: shared.limits_id,
        observations_id: shared.observations_id,
        entries: Vec::new(),
    };
    match crate::result::build(&input) {
        Ok(plan) => crate::PlanningOutcome::NoOp(plan),
        Err(error) => invalid(&error, Some(shared.bindings)),
    }
}

#[allow(
    missing_const_fn,
    reason = "selected-plan construction allocates bounded owned step and request projections; owner: schema-migration-core"
)]
fn build_selected(
    shared: SharedPlanFacts<'_>,
    candidate: &crate::selection::Candidate,
) -> crate::PlanningOutcome {
    let entries = candidate_entries(shared.catalog, candidate);
    let input = crate::result::BuildInput {
        decision: crate::PlanningDecisionClass::Selected,
        request: shared.request,
        catalog_id: shared.catalog.id(),
        policy_id: shared.policy_id,
        limits_id: shared.limits_id,
        observations_id: shared.observations_id,
        entries,
    };
    match crate::result::build(&input) {
        Ok(plan) => crate::PlanningOutcome::Selected(plan),
        Err(error) => invalid(&error, Some(shared.bindings)),
    }
}

fn build_ambiguous(
    catalog: &crate::AdmittedCatalog,
    bindings: crate::DecisionBindings,
    candidates: &[crate::selection::Candidate],
    limits: crate::PlanningLimits,
) -> crate::PlanningOutcome {
    let retained = candidates
        .len()
        .min(usize::from(limits.maximum_diagnostics()));
    let mut summaries = Vec::with_capacity(retained);
    for candidate in &candidates[..retained] {
        let entries = candidate_entries(catalog, candidate);
        let path_id = match crate::framing::path_id(&entries) {
            Ok(identity) => identity,
            Err(error) => return invalid(&error, Some(bindings)),
        };
        let entry_ids = entries.iter().map(|entry| entry.id()).collect();
        summaries.push(crate::PathSummary::new(path_id, entry_ids, candidate.cost));
    }
    crate::PlanningOutcome::Ambiguous {
        bindings,
        candidates: summaries,
    }
}

fn candidate_entries<'a>(
    catalog: &'a crate::AdmittedCatalog,
    candidate: &crate::selection::Candidate,
) -> Vec<&'a crate::AdmittedEntry> {
    candidate
        .indices
        .iter()
        .map(|index| &catalog.entries()[*index])
        .collect()
}

#[allow(
    assertion_density,
    reason = "this closed error-to-decision projection has no mutable state invariant beyond exhaustive mapping; owner: schema-migration-core"
)]
const fn invalid(
    error: &crate::MigrationError,
    bindings: Option<crate::DecisionBindings>,
) -> crate::PlanningOutcome {
    let reason = match error {
        crate::MigrationError::TooManyEntries => crate::InvalidCatalogReason::EntryLimit,
        crate::MigrationError::TooManyOutgoingEdges => {
            crate::InvalidCatalogReason::OutgoingEdgeLimit
        }
        crate::MigrationError::TooManyPreconditions => {
            crate::InvalidCatalogReason::PreconditionLimit
        }
        crate::MigrationError::DuplicateEntryKey { .. }
        | crate::MigrationError::DuplicatePrecondition { .. } => {
            crate::InvalidCatalogReason::DuplicateEntry
        }
        crate::MigrationError::SelfEdge => crate::InvalidCatalogReason::SelfEdge,
        crate::MigrationError::CostOverflow => crate::InvalidCatalogReason::CostOverflow,
        crate::MigrationError::IdentityFailure
        | crate::MigrationError::ZeroLimit { .. }
        | crate::MigrationError::EmptyText { .. }
        | crate::MigrationError::TextTooLong { .. }
        | crate::MigrationError::NoAllowedDirection
        | crate::MigrationError::NoImplementationCohort
        | crate::MigrationError::DuplicateCohort { .. }
        | crate::MigrationError::DuplicateObservation { .. } => {
            crate::InvalidCatalogReason::IdentityFailure
        }
    };
    crate::PlanningOutcome::InvalidCatalog { bindings, reason }
}
