// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Onix Research
// Earlier source grants remain effective; see NOTICE.

#![no_std]
#![forbid(unsafe_code)]
#![allow(
    unknown_lints,
    reason = "Octet Dylint names load only during the pinned cargo-octet gate; owner: schema-migration-core"
)]
#![allow(
    clippy::useless_attribute,
    reason = "Octet allow attributes become active only under the pinned Dylint driver; owner: schema-migration-core"
)]

//! Product-neutral bounded planning between exact schema identities.
//!
//! The functional core admits immutable migration catalogs, selects one
//! policy-directed path, emits typed effect requests, and checks freshness.
//! Consumers retain migration code, data access, authority, transactions,
//! storage, execution, rollback, publication, and release decisions.

// r[impl onix.schema_migration.functional_core.explicit]
extern crate alloc;

mod admission;
mod admit;
mod constraints;
mod evidence;
mod failure;
mod framing;
mod handoff;
mod identity;
mod model;
mod requests;
mod result;
mod selection;

pub use admission::{AdmittedCatalog, AdmittedEntry, admit_catalog};
pub use admit::plan_migration;
pub use constraints::{
    CostOrder, PlanningLimitValues, PlanningLimits, PlanningPolicy, PlanningPolicyInput, TiePolicy,
};
pub use evidence::{PlanReceipt, RECEIPT_NON_CLAIMS, ReceiptReason, project_receipt};
pub use failure::MigrationError;
pub use handoff::{
    FreshnessDecision, FreshnessInput, FreshnessObservation, StaleReason, check_freshness,
};
pub use identity::{
    CatalogEntryKey, CatalogId, DataSubjectId, EntryId, ImplementationCohortId,
    ImplementationSetId, LimitsId, MigrationProfileId, ObservationId, ObservationSetId, PathId,
    PlanId, PolicyId, PreconditionId, TextLimit,
};
pub use model::{
    CatalogSnapshot, CostFacts, CostValues, Direction, MigrationBinding, MigrationEntry,
    MigrationMetadata, MigrationMetadataInput, ObservationSet, PlanningRequest, PreconditionKind,
    PreconditionObservation, PreconditionRequirement, ReversibilityClass, SchemaRoute,
};
pub use requests::{EffectKind, EffectRequest};
pub use result::{
    BoundReason, DecisionBindings, InvalidCatalogReason, MigrationPlan, PathSummary, PlanStep,
    PlanningDecisionClass, PlanningOutcome, PreconditionDenial,
};
// r[impl onix.schema_migration.schema_identity_dependency]
pub use schema_identity_core::{
    MigrationArtifactId, MigrationReference, MigrationReferenceId, PolicyProfileId, SchemaId,
    TypeLineageId,
};
