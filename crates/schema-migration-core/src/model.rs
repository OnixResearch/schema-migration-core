#[allow(
    non_trait_imports,
    reason = "bounded owned collections hold explicit catalog and observation facts; owner: schema-migration-core"
)]
use alloc::vec::Vec;
#[allow(
    non_trait_imports,
    reason = "published Schema Identity Core values remain explicit model boundary facts; owner: schema-migration-core"
)]
use schema_identity_core::{MigrationReference, SchemaId, TypeLineageId};

/// Direction class declared by one migration entry.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    /// Move from an older source to a newer target.
    Forward,
    /// Move from a newer source to an older target.
    Reverse,
}

/// Operational reversibility fact supplied by the catalog owner.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReversibilityClass {
    /// No rollback request is declared.
    Irreversible,
    /// A schema rollback request can be planned.
    SchemaRollback,
    /// A compensation request can be planned.
    Compensatable,
}

/// Closed precondition classes for migration planning.
#[doc = "`schema_migration_sealed`"]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PreconditionKind {
    /// Fact about the source data subject.
    Data,
    /// Fact about the execution environment.
    Environment,
    /// Fact about a reviewed migration implementation.
    Implementation,
    /// Fact about an enabled feature.
    Feature,
    /// Caller-supplied authority observation.
    AuthorityObservation,
    /// Fact about available bounded resources.
    Resource,
}

/// Named raw values for one additive migration cost.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CostValues {
    /// Caller-selected risk weight.
    pub risk: u64,
    /// Caller-selected effort weight.
    pub effort: u64,
}

/// Checked additive cost facts for one migration entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CostFacts {
    risk: u64,
    effort: u64,
}

impl CostFacts {
    /// Creates explicit risk and effort costs from named values.
    #[must_use]
    pub const fn new(values: CostValues) -> Self {
        Self {
            risk: values.risk,
            effort: values.effort,
        }
    }

    /// Returns the risk cost.
    #[must_use]
    pub const fn risk(self) -> u64 {
        self.risk
    }

    /// Returns the effort cost.
    #[must_use]
    pub const fn effort(self) -> u64 {
        self.effort
    }

    pub(crate) fn checked_add(self, other: Self) -> Result<Self, crate::MigrationError> {
        let risk = self
            .risk
            .checked_add(other.risk)
            .ok_or(crate::MigrationError::CostOverflow)?;
        let effort = self
            .effort
            .checked_add(other.effort)
            .ok_or(crate::MigrationError::CostOverflow)?;
        Ok(Self { risk, effort })
    }
}

/// One required fact declared by a migration entry.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PreconditionRequirement {
    id: crate::PreconditionId,
    kind: PreconditionKind,
}

impl PreconditionRequirement {
    /// Creates one exact precondition requirement.
    #[must_use]
    pub const fn new(id: crate::PreconditionId, kind: PreconditionKind) -> Self {
        Self { id, kind }
    }

    /// Returns the precondition identity.
    #[must_use]
    pub const fn id(&self) -> &crate::PreconditionId {
        &self.id
    }

    /// Returns the closed precondition class.
    #[must_use]
    pub const fn kind(&self) -> PreconditionKind {
        self.kind
    }
}

/// Current consumer observation for one migration precondition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreconditionObservation {
    requirement: PreconditionRequirement,
    observation_id: crate::ObservationId,
    is_satisfied: bool,
    implementation_cohort: Option<crate::ImplementationCohortId>,
}

impl PreconditionObservation {
    /// Creates one explicit current observation.
    #[must_use]
    pub const fn new(
        requirement: PreconditionRequirement,
        observation_id: crate::ObservationId,
        is_satisfied: bool,
        implementation_cohort: Option<crate::ImplementationCohortId>,
    ) -> Self {
        Self {
            requirement,
            observation_id,
            is_satisfied,
            implementation_cohort,
        }
    }

    /// Returns the observed requirement.
    #[must_use]
    pub const fn requirement(&self) -> &PreconditionRequirement {
        &self.requirement
    }

    /// Returns the observation identity.
    #[must_use]
    pub const fn observation_id(&self) -> &crate::ObservationId {
        &self.observation_id
    }

    /// Returns whether the caller observed the requirement as satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        self.is_satisfied
    }

    /// Returns the implementation cohort attached to an implementation fact.
    #[must_use]
    pub const fn implementation_cohort(&self) -> Option<&crate::ImplementationCohortId> {
        self.implementation_cohort.as_ref()
    }
}

/// Complete observation set supplied to one planning request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationSet {
    observations: Vec<PreconditionObservation>,
}

impl ObservationSet {
    /// Creates an observation set. Planning admission canonicalizes and validates it.
    #[must_use]
    pub const fn new(observations: Vec<PreconditionObservation>) -> Self {
        Self { observations }
    }

    /// Returns supplied observations.
    #[must_use]
    pub fn observations(&self) -> &[PreconditionObservation] {
        &self.observations
    }
}

/// Schema identity facts that bind one migration edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationBinding {
    lineage: TypeLineageId,
    reference: MigrationReference,
}

impl MigrationBinding {
    /// Creates one exact lineage and migration-reference binding.
    #[must_use]
    pub const fn new(lineage: TypeLineageId, reference: MigrationReference) -> Self {
        Self { lineage, reference }
    }

    /// Returns the durable schema lineage.
    #[must_use]
    pub const fn lineage(&self) -> TypeLineageId {
        self.lineage
    }

    /// Returns the exact published migration reference.
    #[must_use]
    pub const fn reference(&self) -> &MigrationReference {
        &self.reference
    }
}

/// Explicit metadata input for one migration entry.
pub struct MigrationMetadataInput {
    /// Direction class.
    pub direction: Direction,
    /// Reviewed implementation cohort.
    pub implementation_cohort: crate::ImplementationCohortId,
    /// Product-neutral migration behavior profile.
    pub profile: crate::MigrationProfileId,
    /// Required facts.
    pub preconditions: Vec<PreconditionRequirement>,
    /// Operational reversibility declaration.
    pub reversibility: ReversibilityClass,
    /// Additive path costs.
    pub cost: CostFacts,
}

/// Canonicalizable metadata for one migration edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationMetadata {
    direction: Direction,
    implementation_cohort: crate::ImplementationCohortId,
    profile: crate::MigrationProfileId,
    preconditions: Vec<PreconditionRequirement>,
    reversibility: ReversibilityClass,
    cost: CostFacts,
}

impl MigrationMetadata {
    /// Creates canonical migration metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when a precondition occurs more than once.
    pub fn new(input: MigrationMetadataInput) -> Result<Self, crate::MigrationError> {
        let mut preconditions = input.preconditions;
        preconditions.sort();
        for adjacent in preconditions.windows(2) {
            if adjacent[0] == adjacent[1] {
                return Err(crate::MigrationError::DuplicatePrecondition {
                    precondition: adjacent[0].id().as_str().into(),
                });
            }
        }
        Ok(Self {
            direction: input.direction,
            implementation_cohort: input.implementation_cohort,
            profile: input.profile,
            preconditions,
            reversibility: input.reversibility,
            cost: input.cost,
        })
    }

    /// Returns the direction class.
    #[must_use]
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    /// Returns the reviewed implementation cohort.
    #[must_use]
    pub const fn implementation_cohort(&self) -> &crate::ImplementationCohortId {
        &self.implementation_cohort
    }

    /// Returns the migration profile.
    #[must_use]
    pub const fn profile(&self) -> &crate::MigrationProfileId {
        &self.profile
    }

    /// Returns canonical required facts.
    #[must_use]
    pub fn preconditions(&self) -> &[PreconditionRequirement] {
        &self.preconditions
    }

    /// Returns the declared reversibility class.
    #[must_use]
    pub const fn reversibility(&self) -> ReversibilityClass {
        self.reversibility
    }

    /// Returns additive path costs.
    #[must_use]
    pub const fn cost(&self) -> CostFacts {
        self.cost
    }
}

/// One complete migration catalog entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationEntry {
    key: crate::CatalogEntryKey,
    binding: MigrationBinding,
    metadata: MigrationMetadata,
}

impl MigrationEntry {
    /// Creates one complete catalog entry.
    #[must_use]
    pub const fn new(
        key: crate::CatalogEntryKey,
        binding: MigrationBinding,
        metadata: MigrationMetadata,
    ) -> Self {
        Self {
            key,
            binding,
            metadata,
        }
    }

    /// Returns the stable caller-owned key.
    #[must_use]
    pub const fn key(&self) -> &crate::CatalogEntryKey {
        &self.key
    }

    /// Returns exact schema identity bindings.
    #[must_use]
    pub const fn binding(&self) -> &MigrationBinding {
        &self.binding
    }

    /// Returns canonical metadata.
    #[must_use]
    pub const fn metadata(&self) -> &MigrationMetadata {
        &self.metadata
    }
}

/// Supplied immutable migration catalog snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatalogSnapshot {
    entries: Vec<MigrationEntry>,
}

impl CatalogSnapshot {
    /// Creates a supplied snapshot. Admission validates and canonicalizes it.
    #[must_use]
    pub const fn new(entries: Vec<MigrationEntry>) -> Self {
        Self { entries }
    }

    /// Returns supplied entries.
    #[must_use]
    pub fn entries(&self) -> &[MigrationEntry] {
        &self.entries
    }
}

/// Exact source and target schema route for one request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaRoute {
    lineage: TypeLineageId,
    source: SchemaId,
    target: SchemaId,
}

impl SchemaRoute {
    // r[impl onix.schema_migration.plan_identity.missing]
    /// Creates one exact schema route. The typed constructor requires a target schema.
    #[must_use]
    pub const fn new(lineage: TypeLineageId, source: SchemaId, target: SchemaId) -> Self {
        Self {
            lineage,
            source,
            target,
        }
    }

    /// Returns the durable schema lineage.
    #[must_use]
    pub const fn lineage(self) -> TypeLineageId {
        self.lineage
    }

    /// Returns the exact source schema.
    #[must_use]
    pub const fn source(self) -> SchemaId {
        self.source
    }

    /// Returns the exact target schema.
    #[must_use]
    pub const fn target(self) -> SchemaId {
        self.target
    }
}

/// Complete pure planning request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningRequest {
    source_data: crate::DataSubjectId,
    route: SchemaRoute,
    observations: ObservationSet,
}

impl PlanningRequest {
    /// Creates one explicit request.
    #[must_use]
    pub const fn new(
        source_data: crate::DataSubjectId,
        route: SchemaRoute,
        observations: ObservationSet,
    ) -> Self {
        Self {
            source_data,
            route,
            observations,
        }
    }

    /// Returns the source data identity.
    #[must_use]
    pub const fn source_data(&self) -> &crate::DataSubjectId {
        &self.source_data
    }

    /// Returns the exact source and target route.
    #[must_use]
    pub const fn route(&self) -> SchemaRoute {
        self.route
    }

    /// Returns current precondition observations.
    #[must_use]
    pub const fn observations(&self) -> &ObservationSet {
        &self.observations
    }
}
