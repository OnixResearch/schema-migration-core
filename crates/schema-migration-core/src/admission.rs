#[allow(
    non_trait_imports,
    reason = "ordered sets and maps enforce canonical catalog bounds; owner: schema-migration-core"
)]
use alloc::collections::{BTreeMap, BTreeSet};
#[allow(
    non_trait_imports,
    reason = "the admitted catalog owns canonical entries; owner: schema-migration-core"
)]
use alloc::vec::Vec;

/// One canonical admitted catalog entry and its complete identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedEntry {
    id: crate::EntryId,
    entry: crate::MigrationEntry,
}

impl AdmittedEntry {
    /// Returns the complete entry identity.
    #[must_use]
    pub const fn id(&self) -> crate::EntryId {
        self.id
    }

    /// Returns the stable entry key.
    #[must_use]
    pub const fn key(&self) -> &crate::CatalogEntryKey {
        self.entry.key()
    }

    /// Returns the exact schema binding.
    #[must_use]
    pub const fn binding(&self) -> &crate::MigrationBinding {
        self.entry.binding()
    }

    /// Returns the canonical entry metadata.
    #[must_use]
    pub const fn metadata(&self) -> &crate::MigrationMetadata {
        self.entry.metadata()
    }
}

/// Canonical immutable migration catalog admitted under explicit limits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedCatalog {
    id: crate::CatalogId,
    entries: Vec<AdmittedEntry>,
}

impl AdmittedCatalog {
    /// Returns the canonical catalog identity.
    #[must_use]
    pub const fn id(&self) -> crate::CatalogId {
        self.id
    }

    /// Returns entries in canonical identity order.
    #[must_use]
    pub fn entries(&self) -> &[AdmittedEntry] {
        &self.entries
    }
}

// r[impl onix.schema_migration.catalog_admission]
// r[impl onix.schema_migration.catalog_snapshot]
/// Admits and canonicalizes one immutable migration catalog snapshot.
///
/// # Errors
///
/// Returns an error for duplicate keys, self-edges, bound violations, or identity failures.
pub fn admit_catalog(
    snapshot: &crate::CatalogSnapshot,
    limits: crate::PlanningLimits,
) -> Result<AdmittedCatalog, crate::MigrationError> {
    if snapshot.entries().len() > usize::from(limits.maximum_entries()) {
        return Err(crate::MigrationError::TooManyEntries);
    }
    let mut keys = BTreeSet::new();
    let mut outgoing = BTreeMap::<([u8; 32], [u8; 32]), u16>::new();
    let mut entries = Vec::with_capacity(snapshot.entries().len());
    for entry in snapshot.entries() {
        validate_entry(entry, limits, &mut keys, &mut outgoing)?;
        entries.push(AdmittedEntry {
            id: crate::framing::entry_id(entry)?,
            entry: entry.clone(),
        });
    }
    entries.sort_by(|left, right| {
        left.id
            .cmp(&right.id)
            .then_with(|| left.key().cmp(right.key()))
    });
    let id = crate::framing::catalog_id(&entries)?;
    debug_assert_eq!(entries.len(), snapshot.entries().len());
    Ok(AdmittedCatalog { id, entries })
}

#[allow(
    mutating_input_in_pure,
    reason = "caller-local canonical key and outgoing-count accumulators are explicit bounded admission state; owner: schema-migration-core"
)]
fn validate_entry(
    entry: &crate::MigrationEntry,
    limits: crate::PlanningLimits,
    keys: &mut BTreeSet<crate::CatalogEntryKey>,
    outgoing: &mut BTreeMap<([u8; 32], [u8; 32]), u16>,
) -> Result<(), crate::MigrationError> {
    debug_assert!(limits.maximum_outgoing_edges() > 0);
    debug_assert!(limits.maximum_preconditions() > 0);
    if !keys.insert(entry.key().clone()) {
        return Err(crate::MigrationError::DuplicateEntryKey {
            key: entry.key().as_str().into(),
        });
    }
    let reference = entry.binding().reference();
    if reference.source_schema() == reference.target_schema() {
        return Err(crate::MigrationError::SelfEdge);
    }
    if entry.metadata().preconditions().len() > usize::from(limits.maximum_preconditions()) {
        return Err(crate::MigrationError::TooManyPreconditions);
    }
    let outgoing_key = (
        *entry.binding().lineage().as_bytes(),
        *reference.source_schema().as_bytes(),
    );
    let count = outgoing.entry(outgoing_key).or_insert(0);
    *count = count
        .checked_add(1)
        .ok_or(crate::MigrationError::TooManyOutgoingEdges)?;
    if *count > limits.maximum_outgoing_edges() {
        return Err(crate::MigrationError::TooManyOutgoingEdges);
    }
    debug_assert!(keys.contains(entry.key()));
    debug_assert!(*count <= limits.maximum_outgoing_edges());
    Ok(())
}
