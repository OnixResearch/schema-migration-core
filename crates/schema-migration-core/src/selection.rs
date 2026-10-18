#[allow(
    non_trait_imports,
    reason = "bounded path search owns explicit frontier and candidate vectors; owner: schema-migration-core"
)]
use alloc::{vec, vec::Vec};
#[allow(
    non_trait_imports,
    reason = "exact schema identities are explicit graph vertex facts; owner: schema-migration-core"
)]
use schema_identity_core::SchemaId;

#[derive(Clone, Debug)]
pub(crate) struct Candidate {
    pub indices: Vec<usize>,
    pub cost: crate::CostFacts,
    worst_cohort_rank: u16,
    cohort_rank_sum: u64,
}

pub(crate) enum SearchDecision {
    Selected(Candidate),
    NoPath,
    Ambiguous(Vec<Candidate>),
    Bound(crate::BoundReason),
    PreconditionDenied(crate::PreconditionDenial),
    Invalid(crate::InvalidCatalogReason),
}

#[derive(Clone)]
struct SearchState {
    current: SchemaId,
    indices: Vec<usize>,
    visited: Vec<SchemaId>,
    cost: crate::CostFacts,
    worst_cohort_rank: u16,
    cohort_rank_sum: u64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidateRank {
    worst_cohort_rank: u16,
    cohort_rank_sum: u64,
    first_cost: u64,
    second_cost: u64,
    third_cost: u64,
}

// r[impl onix.schema_migration.path_selection]
// r[impl onix.schema_migration.ambiguity]
pub(crate) fn search(
    catalog: &crate::AdmittedCatalog,
    request: &crate::PlanningRequest,
    policy: &crate::PlanningPolicy,
    limits: crate::PlanningLimits,
) -> SearchDecision {
    debug_assert!(catalog.entries().len() <= usize::from(limits.maximum_entries()));
    debug_assert!(policy.maximum_steps() > 0);
    let candidates = match enumerate(catalog, request.route(), policy, limits) {
        Ok(Enumeration::Complete(candidates)) => candidates,
        Ok(Enumeration::Bound(reason)) => return SearchDecision::Bound(reason),
        Err(crate::MigrationError::CostOverflow) => {
            return SearchDecision::Invalid(crate::InvalidCatalogReason::CostOverflow);
        }
        Err(_error) => {
            return SearchDecision::Invalid(crate::InvalidCatalogReason::IdentityFailure);
        }
    };
    if candidates.is_empty() {
        return SearchDecision::NoPath;
    }
    let mut eligible = Vec::with_capacity(candidates.len());
    let mut first_denial = None;
    for candidate in candidates {
        match preconditions(catalog, &candidate, request.observations(), policy) {
            Ok(()) => eligible.push(candidate),
            Err(reason) => {
                if first_denial.is_none() {
                    first_denial = Some(reason);
                }
            }
        }
    }
    if eligible.is_empty() {
        return SearchDecision::PreconditionDenied(
            first_denial.unwrap_or(crate::PreconditionDenial::ObservationMissing),
        );
    }
    select(eligible, policy)
}

enum Enumeration {
    Complete(Vec<Candidate>),
    Bound(crate::BoundReason),
}

#[allow(
    allocation_in_loop,
    unbounded_collection_growth,
    reason = "each frontier and path allocation is bounded by explicit entry, step, candidate, and state limits; owner: schema-migration-core"
)]
#[allow(
    function_length,
    reason = "the bounded iterative graph walk keeps frontier, candidate, cycle, overflow, and exhaustion checks in one auditable control flow; owner: schema-migration-core"
)]
fn enumerate(
    catalog: &crate::AdmittedCatalog,
    route: crate::SchemaRoute,
    policy: &crate::PlanningPolicy,
    limits: crate::PlanningLimits,
) -> Result<Enumeration, crate::MigrationError> {
    debug_assert!(limits.maximum_candidate_paths() > 0);
    debug_assert!(limits.maximum_search_states() > 0);
    let selected_steps = policy.maximum_steps().min(limits.maximum_path_steps());
    let maximum_steps = usize::from(selected_steps);
    let maximum_candidates = usize::from(limits.maximum_candidate_paths());
    let maximum_states = limits.maximum_search_states();
    let mut states_created = 1_u32;
    let mut frontier = Vec::with_capacity(maximum_steps);
    frontier.push(SearchState {
        current: route.source(),
        indices: Vec::with_capacity(maximum_steps),
        visited: vec![route.source()],
        cost: crate::CostFacts::new(crate::CostValues { risk: 0, effort: 0 }),
        worst_cohort_rank: 0,
        cohort_rank_sum: 0,
    });
    let mut candidates = Vec::with_capacity(maximum_candidates);
    let mut is_path_bound_reached = false;
    while let Some(state) = frontier.pop() {
        if state.current == route.target() && !state.indices.is_empty() {
            if candidates.len() >= maximum_candidates {
                return Ok(Enumeration::Bound(crate::BoundReason::CandidatePaths));
            }
            candidates.push(Candidate {
                indices: state.indices,
                cost: state.cost,
                worst_cohort_rank: state.worst_cohort_rank,
                cohort_rank_sum: state.cohort_rank_sum,
            });
            continue;
        }
        if state.indices.len() >= maximum_steps {
            is_path_bound_reached |= has_unvisited_edge(catalog, &state, route, policy);
            continue;
        }
        for (index, entry) in catalog.entries().iter().enumerate() {
            let Some(cohort_rank) = eligible_edge(entry, state.current, route, policy) else {
                continue;
            };
            let target = entry.binding().reference().target_schema();
            if state.visited.contains(&target) {
                continue;
            }
            if states_created >= maximum_states {
                return Ok(Enumeration::Bound(crate::BoundReason::SearchStates));
            }
            let cost = state.cost.checked_add(entry.metadata().cost())?;
            let mut indices = state.indices.clone();
            indices.push(index);
            let mut visited = state.visited.clone();
            visited.push(target);
            let cohort_rank_sum = state
                .cohort_rank_sum
                .checked_add(u64::from(cohort_rank))
                .ok_or(crate::MigrationError::CostOverflow)?;
            frontier.push(SearchState {
                current: target,
                indices,
                visited,
                cost,
                worst_cohort_rank: state.worst_cohort_rank.max(cohort_rank),
                cohort_rank_sum,
            });
            states_created = states_created
                .checked_add(1)
                .ok_or(crate::MigrationError::CostOverflow)?;
        }
    }
    if is_path_bound_reached {
        return Ok(Enumeration::Bound(crate::BoundReason::PathSteps));
    }
    Ok(Enumeration::Complete(candidates))
}

fn has_unvisited_edge(
    catalog: &crate::AdmittedCatalog,
    state: &SearchState,
    route: crate::SchemaRoute,
    policy: &crate::PlanningPolicy,
) -> bool {
    catalog.entries().iter().any(|entry| {
        eligible_edge(entry, state.current, route, policy).is_some()
            && !state
                .visited
                .contains(&entry.binding().reference().target_schema())
    })
}

#[allow(
    missing_const_fn,
    reason = "cohort lookup uses bounded iterator comparison on owned reviewed identities; owner: schema-migration-core"
)]
fn eligible_edge(
    entry: &crate::AdmittedEntry,
    current: SchemaId,
    route: crate::SchemaRoute,
    policy: &crate::PlanningPolicy,
) -> Option<u16> {
    let reference = entry.binding().reference();
    if entry.binding().lineage() != route.lineage()
        || reference.source_schema() != current
        || !policy.allows_direction(entry.metadata().direction())
    {
        return None;
    }
    policy.cohort_rank(entry.metadata().implementation_cohort())
}

// r[impl onix.schema_migration.preconditions]
fn preconditions(
    catalog: &crate::AdmittedCatalog,
    candidate: &Candidate,
    observations: &crate::ObservationSet,
    policy: &crate::PlanningPolicy,
) -> Result<(), crate::PreconditionDenial> {
    debug_assert!(!candidate.indices.is_empty());
    debug_assert!(
        candidate
            .indices
            .iter()
            .all(|index| *index < catalog.entries().len())
    );
    for index in &candidate.indices {
        let entry = &catalog.entries()[*index];
        for required_kind in policy.required_preconditions() {
            if !entry
                .metadata()
                .preconditions()
                .iter()
                .any(|requirement| requirement.kind() == *required_kind)
            {
                return Err(crate::PreconditionDenial::RequiredClassMissing);
            }
        }
        for requirement in entry.metadata().preconditions() {
            let Some(observation) = observations
                .observations()
                .iter()
                .find(|item| item.requirement() == requirement)
            else {
                return Err(crate::PreconditionDenial::ObservationMissing);
            };
            if !observation.is_satisfied() {
                return Err(crate::PreconditionDenial::ObservationDenied);
            }
            if requirement.kind() == crate::PreconditionKind::Implementation
                && observation.implementation_cohort()
                    != Some(entry.metadata().implementation_cohort())
            {
                return Err(crate::PreconditionDenial::ImplementationCohortMismatch);
            }
        }
    }
    Ok(())
}

fn select(candidates: Vec<Candidate>, policy: &crate::PlanningPolicy) -> SearchDecision {
    let mut best_rank = None;
    let mut best = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let Ok(rank) = rank(&candidate, policy.cost_order()) else {
            return SearchDecision::Invalid(crate::InvalidCatalogReason::CostOverflow);
        };
        match best_rank {
            None => {
                best_rank = Some(rank);
                best.push(candidate);
            }
            Some(current) if rank < current => {
                best_rank = Some(rank);
                best.clear();
                best.push(candidate);
            }
            Some(current) if rank == current => best.push(candidate),
            Some(_) => {}
        }
    }
    debug_assert!(!best.is_empty());
    match best.as_mut_slice() {
        [candidate] => SearchDecision::Selected(candidate.clone()),
        _ => match policy.tie_policy() {
            crate::TiePolicy::DenyAmbiguous => SearchDecision::Ambiguous(best),
        },
    }
}

fn rank(candidate: &Candidate, order: crate::CostOrder) -> Result<CandidateRank, ()> {
    let steps = u64::try_from(candidate.indices.len()).map_err(|_error| ())?;
    let risk = candidate.cost.risk();
    let effort = candidate.cost.effort();
    let (first_cost, second_cost, third_cost) = match order {
        crate::CostOrder::RiskThenEffortThenSteps => (risk, effort, steps),
        crate::CostOrder::EffortThenRiskThenSteps => (effort, risk, steps),
        crate::CostOrder::StepsThenRiskThenEffort => (steps, risk, effort),
    };
    Ok(CandidateRank {
        worst_cohort_rank: candidate.worst_cohort_rank,
        cohort_rank_sum: candidate.cohort_rank_sum,
        first_cost,
        second_cost,
        third_cost,
    })
}
