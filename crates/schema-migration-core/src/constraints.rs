#[allow(
    non_trait_imports,
    reason = "ordered cohorts and required classes are explicit policy facts; owner: schema-migration-core"
)]
use alloc::vec::Vec;

/// Raw values used to construct one checked planning limit profile.
#[derive(Clone, Copy)]
pub struct PlanningLimitValues {
    /// Maximum catalog entries.
    pub maximum_entries: u16,
    /// Maximum outgoing entries for one exact schema and lineage.
    pub maximum_outgoing_edges: u16,
    /// Maximum selected path steps.
    pub maximum_path_steps: u16,
    /// Maximum complete candidate paths retained by search.
    pub maximum_candidate_paths: u16,
    /// Maximum bounded search states.
    pub maximum_search_states: u32,
    /// Maximum preconditions on one entry.
    pub maximum_preconditions: u16,
    /// Maximum receipt diagnostics or candidate summaries.
    pub maximum_diagnostics: u16,
}

/// Explicit non-zero resource limits for admission, search, and receipts.
#[allow(
    clippy::struct_field_names,
    reason = "the maximum prefix distinguishes admitted bounds from observed counts; owner: schema-migration-core"
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanningLimits {
    maximum_entries: u16,
    maximum_outgoing_edges: u16,
    maximum_path_steps: u16,
    maximum_candidate_paths: u16,
    maximum_search_states: u32,
    maximum_preconditions: u16,
    maximum_diagnostics: u16,
}

impl PlanningLimits {
    /// Creates one checked resource-limit profile.
    ///
    /// # Errors
    ///
    /// Returns an error when any selected limit is zero.
    pub fn new(values: PlanningLimitValues) -> Result<Self, crate::MigrationError> {
        check_nonzero_u16(values.maximum_entries, "maximum entries")?;
        check_nonzero_u16(values.maximum_outgoing_edges, "maximum outgoing edges")?;
        check_nonzero_u16(values.maximum_path_steps, "maximum path steps")?;
        check_nonzero_u16(values.maximum_candidate_paths, "maximum candidate paths")?;
        if values.maximum_search_states == 0 {
            return Err(crate::MigrationError::ZeroLimit {
                field: "maximum search states",
            });
        }
        check_nonzero_u16(values.maximum_preconditions, "maximum preconditions")?;
        check_nonzero_u16(values.maximum_diagnostics, "maximum diagnostics")?;
        Ok(Self {
            maximum_entries: values.maximum_entries,
            maximum_outgoing_edges: values.maximum_outgoing_edges,
            maximum_path_steps: values.maximum_path_steps,
            maximum_candidate_paths: values.maximum_candidate_paths,
            maximum_search_states: values.maximum_search_states,
            maximum_preconditions: values.maximum_preconditions,
            maximum_diagnostics: values.maximum_diagnostics,
        })
    }

    /// Returns the catalog entry limit.
    #[must_use]
    pub const fn maximum_entries(self) -> u16 {
        self.maximum_entries
    }

    /// Returns the per-schema outgoing-edge limit.
    #[must_use]
    pub const fn maximum_outgoing_edges(self) -> u16 {
        self.maximum_outgoing_edges
    }

    /// Returns the selected path-step limit.
    #[must_use]
    pub const fn maximum_path_steps(self) -> u16 {
        self.maximum_path_steps
    }

    /// Returns the complete candidate-path limit.
    #[must_use]
    pub const fn maximum_candidate_paths(self) -> u16 {
        self.maximum_candidate_paths
    }

    /// Returns the search-state limit.
    #[must_use]
    pub const fn maximum_search_states(self) -> u32 {
        self.maximum_search_states
    }

    /// Returns the per-entry precondition limit.
    #[must_use]
    pub const fn maximum_preconditions(self) -> u16 {
        self.maximum_preconditions
    }

    /// Returns the receipt diagnostic limit.
    #[must_use]
    pub const fn maximum_diagnostics(self) -> u16 {
        self.maximum_diagnostics
    }
}

const fn check_nonzero_u16(value: u16, field: &'static str) -> Result<(), crate::MigrationError> {
    if value == 0 {
        return Err(crate::MigrationError::ZeroLimit { field });
    }
    Ok(())
}

/// Explicit ordering for additive path costs.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CostOrder {
    /// Compare risk, then effort, then step count.
    RiskThenEffortThenSteps,
    /// Compare effort, then risk, then step count.
    EffortThenRiskThenSteps,
    /// Compare step count, then risk, then effort.
    StepsThenRiskThenEffort,
}

/// Closed behavior for candidates equal under every selected policy fact.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TiePolicy {
    /// Return an explicit ambiguity denial.
    DenyAmbiguous,
}

/// Explicit values used to construct one planning policy.
pub struct PlanningPolicyInput {
    /// Whether forward entries are eligible.
    pub allows_forward: bool,
    /// Whether reverse entries are eligible.
    pub allows_reverse: bool,
    /// Reviewed cohorts in descending preference order.
    pub implementation_cohorts: Vec<crate::ImplementationCohortId>,
    /// Precondition classes every selected entry must declare.
    pub required_preconditions: Vec<crate::PreconditionKind>,
    /// Policy path-step limit.
    pub maximum_steps: u16,
    /// Selected additive cost order.
    pub cost_order: CostOrder,
    /// Selected equal-candidate behavior.
    pub tie_policy: TiePolicy,
}

/// Admitted explicit path-selection policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningPolicy {
    allows_forward: bool,
    allows_reverse: bool,
    implementation_cohorts: Vec<crate::ImplementationCohortId>,
    required_preconditions: Vec<crate::PreconditionKind>,
    maximum_steps: u16,
    cost_order: CostOrder,
    tie_policy: TiePolicy,
}

impl PlanningPolicy {
    /// Creates and canonicalizes one explicit policy.
    ///
    /// # Errors
    ///
    /// Returns an error for missing directions, missing cohorts, zero steps, or duplicate cohorts.
    pub fn new(mut input: PlanningPolicyInput) -> Result<Self, crate::MigrationError> {
        if !input.allows_forward && !input.allows_reverse {
            return Err(crate::MigrationError::NoAllowedDirection);
        }
        if input.implementation_cohorts.is_empty() {
            return Err(crate::MigrationError::NoImplementationCohort);
        }
        check_nonzero_u16(input.maximum_steps, "policy maximum steps")?;
        for left_index in 0..input.implementation_cohorts.len() {
            let cohort = &input.implementation_cohorts[left_index];
            let next_index = left_index
                .checked_add(1)
                .ok_or(crate::MigrationError::IdentityFailure)?;
            if input.implementation_cohorts[next_index..]
                .iter()
                .any(|candidate| candidate == cohort)
            {
                return Err(crate::MigrationError::DuplicateCohort {
                    cohort: cohort.as_str().into(),
                });
            }
        }
        input.required_preconditions.sort();
        input.required_preconditions.dedup();
        Ok(Self {
            allows_forward: input.allows_forward,
            allows_reverse: input.allows_reverse,
            implementation_cohorts: input.implementation_cohorts,
            required_preconditions: input.required_preconditions,
            maximum_steps: input.maximum_steps,
            cost_order: input.cost_order,
            tie_policy: input.tie_policy,
        })
    }

    /// Returns whether one direction is admitted.
    #[must_use]
    pub const fn allows_direction(&self, direction: crate::Direction) -> bool {
        match direction {
            crate::Direction::Forward => self.allows_forward,
            crate::Direction::Reverse => self.allows_reverse,
        }
    }

    /// Returns reviewed cohorts in preference order.
    #[must_use]
    pub fn implementation_cohorts(&self) -> &[crate::ImplementationCohortId] {
        &self.implementation_cohorts
    }

    /// Returns required precondition classes.
    #[must_use]
    pub fn required_preconditions(&self) -> &[crate::PreconditionKind] {
        &self.required_preconditions
    }

    /// Returns the policy path-step limit.
    #[must_use]
    pub const fn maximum_steps(&self) -> u16 {
        self.maximum_steps
    }

    /// Returns the selected additive cost order.
    #[must_use]
    pub const fn cost_order(&self) -> CostOrder {
        self.cost_order
    }

    /// Returns equal-candidate behavior.
    #[must_use]
    pub const fn tie_policy(&self) -> TiePolicy {
        self.tie_policy
    }

    pub(crate) fn cohort_rank(&self, cohort: &crate::ImplementationCohortId) -> Option<u16> {
        self.implementation_cohorts
            .iter()
            .position(|candidate| candidate == cohort)
            .and_then(|index| u16::try_from(index).ok())
    }
}
