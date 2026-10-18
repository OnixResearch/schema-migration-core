//! Positive, negative, bounded-search, freshness, and identity tests.

#![allow(
    unknown_lints,
    reason = "Octet Dylint names load only during the pinned cargo-octet gate; owner: schema-migration-core"
)]
#![allow(
    clippy::useless_attribute,
    reason = "Octet allow attributes become active only under the pinned Dylint driver; owner: schema-migration-core"
)]

mod support;

#[allow(
    non_trait_imports,
    reason = "integration cases exercise the public migration contract as an external consumer; owner: schema-migration-core"
)]
use schema_migration_core::*;
use support::*;

#[test]
fn equal_catalogs_ignore_input_order() -> TestResult {
    // r[verify onix.schema_migration.catalog_admission.valid]
    // r[verify onix.schema_migration.catalog_snapshot.equal]
    // r[verify onix.schema_migration.catalog_snapshot.order]
    let fixture = fixture()?;
    let first = direct_entry(&fixture)?;
    let second = entry(
        &fixture,
        EntryInput {
            key: "source-middle",
            source: fixture.source,
            target: fixture.middle,
            direction: Direction::Forward,
            cohort: &fixture.fallback,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let left = admit_catalog(
        &CatalogSnapshot::new(vec![first.clone(), second.clone()]),
        limits()?,
    )?;
    let right = admit_catalog(&CatalogSnapshot::new(vec![second, first]), limits()?)?;
    assert_eq!(left.id(), right.id());
    assert_eq!(left.entries(), right.entries());
    Ok(())
}

#[test]
fn duplicate_entry_and_self_edge_fail_closed() -> TestResult {
    // r[verify onix.schema_migration.catalog_snapshot.duplicate]
    let fixture = fixture()?;
    let direct = direct_entry(&fixture)?;
    assert!(matches!(
        admit_catalog(
            &CatalogSnapshot::new(vec![direct.clone(), direct]),
            limits()?
        ),
        Err(MigrationError::DuplicateEntryKey { .. })
    ));
    let self_edge = entry(
        &fixture,
        EntryInput {
            key: "self",
            source: fixture.source,
            target: fixture.source,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    assert!(matches!(
        admit_catalog(&CatalogSnapshot::new(vec![self_edge]), limits()?),
        Err(MigrationError::SelfEdge)
    ));
    Ok(())
}

#[test]
fn preferred_cohort_precedes_lower_numeric_cost() -> TestResult {
    // r[verify onix.schema_migration.path_selection.direct]
    // r[verify onix.schema_migration.path_selection.order]
    let fixture = fixture()?;
    let first = entry(
        &fixture,
        EntryInput {
            key: "source-middle",
            source: fixture.source,
            target: fixture.middle,
            direction: Direction::Forward,
            cohort: &fixture.fallback,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let second = entry(
        &fixture,
        EntryInput {
            key: "middle-target",
            source: fixture.middle,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.fallback,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let catalog = CatalogSnapshot::new(vec![first, direct_entry(&fixture)?, second]);
    let outcome = plan_migration(
        &catalog,
        &request(&fixture, "data-a", Vec::new())?,
        &policy(&fixture)?,
        limits()?,
    );
    let plan = selected(&outcome).ok_or("expected selected plan")?;
    assert_eq!(plan.steps().len(), 1);
    assert_eq!(plan.steps()[0].artifact_id().as_str(), "artifact-direct");
    Ok(())
}

#[test]
fn equal_best_paths_are_ambiguous() -> TestResult {
    // r[verify onix.schema_migration.ambiguity.equal]
    let fixture = fixture()?;
    let left = direct_entry(&fixture)?;
    let right = entry(
        &fixture,
        EntryInput {
            key: "direct-alternate",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: DIRECT_RISK,
            effort: DIRECT_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::SchemaRollback,
        },
    )?;
    let outcome = plan_migration(
        &CatalogSnapshot::new(vec![left, right]),
        &request(&fixture, "data-a", Vec::new())?,
        &policy(&fixture)?,
        limits()?,
    );
    assert!(matches!(outcome, PlanningOutcome::Ambiguous { .. }));
    let receipt = project_receipt(&outcome);
    assert_eq!(receipt.candidates().len(), 2);
    assert_eq!(receipt.reason(), ReceiptReason::Ambiguous);
    Ok(())
}

#[test]
fn no_op_has_no_migration_effects() -> TestResult {
    // r[verify onix.schema_migration.ambiguity.noop]
    // r[verify onix.schema_migration.effect_plan.noop]
    let fixture = fixture()?;
    let no_op_request = PlanningRequest::new(
        DataSubjectId::new("data-a", TextLimit::new(TEXT_BYTES)?)?,
        SchemaRoute::new(fixture.lineage, fixture.source, fixture.source),
        ObservationSet::new(Vec::new()),
    );
    let outcome = plan_migration(
        &CatalogSnapshot::new(Vec::new()),
        &no_op_request,
        &policy(&fixture)?,
        limits()?,
    );
    let PlanningOutcome::NoOp(plan) = outcome else {
        return Err("expected no-op".into());
    };
    assert!(plan.steps().is_empty());
    assert!(plan.effects().is_empty());
    Ok(())
}

#[test]
#[allow(
    function_length,
    reason = "one integration test keeps direction denial and explicit path-bound exhaustion on the same graph fixture; owner: schema-migration-core"
)]
fn direction_and_path_bounds_are_explicit() -> TestResult {
    // r[verify onix.schema_migration.path_selection.direction]
    // r[verify onix.schema_migration.ambiguity.bound]
    let fixture = fixture()?;
    let reverse = entry(
        &fixture,
        EntryInput {
            key: "reverse",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Reverse,
            cohort: &fixture.preferred,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let request = request(&fixture, "data-a", Vec::new())?;
    assert_eq!(
        plan_migration(
            &CatalogSnapshot::new(vec![reverse]),
            &request,
            &policy(&fixture)?,
            limits()?
        )
        .decision(),
        PlanningDecisionClass::NoPath
    );
    let first = entry(
        &fixture,
        EntryInput {
            key: "source-middle",
            source: fixture.source,
            target: fixture.middle,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let second = entry(
        &fixture,
        EntryInput {
            key: "middle-target",
            source: fixture.middle,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let bounded_policy = PlanningPolicy::new(PlanningPolicyInput {
        maximum_steps: 1,
        ..policy_input(&fixture)
    })?;
    assert!(matches!(
        plan_migration(
            &CatalogSnapshot::new(vec![first, second]),
            &request,
            &bounded_policy,
            limits()?
        ),
        PlanningOutcome::BoundExhausted {
            reason: BoundReason::PathSteps,
            ..
        }
    ));
    Ok(())
}

#[test]
fn checked_cost_overflow_invalidates_selection() -> TestResult {
    // r[verify onix.schema_migration.catalog_admission.overflow]
    let fixture = fixture()?;
    debug_assert_ne!(fixture.source, fixture.target);
    debug_assert!(limits()?.maximum_path_steps() > 0);
    let first = entry(
        &fixture,
        EntryInput {
            key: "overflow-a",
            source: fixture.source,
            target: fixture.middle,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: u64::MAX,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let second = entry(
        &fixture,
        EntryInput {
            key: "overflow-b",
            source: fixture.middle,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    assert!(matches!(
        plan_migration(
            &CatalogSnapshot::new(vec![first, second]),
            &request(&fixture, "data-a", Vec::new())?,
            &policy(&fixture)?,
            limits()?,
        ),
        PlanningOutcome::InvalidCatalog {
            reason: InvalidCatalogReason::CostOverflow,
            ..
        }
    ));
    Ok(())
}

#[test]
fn missing_and_wrong_implementation_preconditions_fail() -> TestResult {
    // r[verify onix.schema_migration.preconditions.missing]
    // r[verify onix.schema_migration.preconditions.cohort]
    let fixture = fixture()?;
    debug_assert_ne!(fixture.preferred, fixture.fallback);
    debug_assert_ne!(fixture.source, fixture.target);
    let limit = TextLimit::new(TEXT_BYTES)?;
    let requirement = PreconditionRequirement::new(
        PreconditionId::new("implementation-ready", limit)?,
        PreconditionKind::Implementation,
    );
    let migration = entry(
        &fixture,
        EntryInput {
            key: "guarded",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: vec![requirement.clone()],
            reversibility: ReversibilityClass::Compensatable,
        },
    )?;
    let guarded_policy = PlanningPolicy::new(PlanningPolicyInput {
        required_preconditions: vec![PreconditionKind::Implementation],
        ..policy_input(&fixture)
    })?;
    let catalog = CatalogSnapshot::new(vec![migration]);
    assert!(matches!(
        plan_migration(
            &catalog,
            &request(&fixture, "data-a", Vec::new())?,
            &guarded_policy,
            limits()?
        ),
        PlanningOutcome::PreconditionDenied {
            reason: PreconditionDenial::ObservationMissing,
            ..
        }
    ));
    let wrong = PreconditionObservation::new(
        requirement,
        ObservationId::new("observed-implementation", limit)?,
        true,
        Some(fixture.fallback.clone()),
    );
    assert!(matches!(
        plan_migration(
            &catalog,
            &request(&fixture, "data-a", vec![wrong])?,
            &guarded_policy,
            limits()?
        ),
        PlanningOutcome::PreconditionDenied {
            reason: PreconditionDenial::ImplementationCohortMismatch,
            ..
        }
    ));
    Ok(())
}

#[test]
fn selected_plan_binds_effects_data_and_freshness() -> TestResult {
    // r[verify onix.schema_migration.effect_plan.valid]
    // r[verify onix.schema_migration.effect_plan.rollback]
    // r[verify onix.schema_migration.plan_identity.equal]
    // r[verify onix.schema_migration.plan_identity.data]
    // r[verify onix.schema_migration.freshness.valid]
    // r[verify onix.schema_migration.freshness.data]
    let fixture = fixture()?;
    let catalog = CatalogSnapshot::new(vec![direct_entry(&fixture)?]);
    let policy = policy(&fixture)?;
    let limits = limits()?;
    let first = plan_migration(
        &catalog,
        &request(&fixture, "data-a", Vec::new())?,
        &policy,
        limits,
    );
    let second = plan_migration(
        &catalog,
        &request(&fixture, "data-b", Vec::new())?,
        &policy,
        limits,
    );
    let repeated = plan_migration(
        &catalog,
        &request(&fixture, "data-a", Vec::new())?,
        &policy,
        limits,
    );
    let first_plan = selected(&first).ok_or("expected first plan")?;
    let second_plan = selected(&second).ok_or("expected second plan")?;
    let repeated_plan = selected(&repeated).ok_or("expected repeated plan")?;
    assert_eq!(first_plan.id(), repeated_plan.id());
    assert_ne!(first_plan.id(), second_plan.id());
    assert_eq!(first_plan.effects()[0].kind(), EffectKind::Load);
    assert!(
        first_plan
            .effects()
            .iter()
            .any(|effect| effect.kind() == EffectKind::Rollback && effect.is_conditional())
    );
    let current = FreshnessObservation::matching(first_plan);
    assert_eq!(
        check_freshness(first_plan, &current),
        FreshnessDecision::Eligible {
            plan_id: first_plan.id()
        }
    );
    let stale = FreshnessObservation::new(FreshnessInput {
        source_data: second_plan.source_data().clone(),
        source_schema: first_plan.source_schema(),
        target_schema: first_plan.target_schema(),
        catalog_id: first_plan.catalog_id(),
        policy_id: first_plan.policy_id(),
        limits_id: first_plan.limits_id(),
        observations_id: first_plan.observations_id(),
        implementations_id: first_plan.implementations_id(),
    });
    assert_eq!(
        check_freshness(first_plan, &stale),
        FreshnessDecision::Stale {
            reason: StaleReason::SourceData
        }
    );
    Ok(())
}

#[test]
fn receipt_is_metadata_only_and_keeps_non_claims() -> TestResult {
    // r[verify onix.schema_migration.receipts.valid]
    // r[verify onix.schema_migration.receipts.claim]
    let fixture = fixture()?;
    let outcome = plan_migration(
        &CatalogSnapshot::new(vec![direct_entry(&fixture)?]),
        &request(&fixture, "opaque-data-identity", Vec::new())?,
        &policy(&fixture)?,
        limits()?,
    );
    let receipt = project_receipt(&outcome);
    assert_eq!(receipt.decision(), PlanningDecisionClass::Selected);
    assert_eq!(receipt.step_count(), 1);
    assert_eq!(receipt.non_claims(), &RECEIPT_NON_CLAIMS);
    assert!(
        receipt
            .non_claims()
            .contains(&"a migration plan does not prove successful execution or atomicity")
    );
    Ok(())
}

#[test]
fn seeded_order_and_semantic_mutation_are_deterministic() -> TestResult {
    // r[verify onix.schema_migration.property_cases]
    // r[verify onix.schema_migration.property_cases.seed]
    // r[verify onix.schema_migration.property_cases.mutation]
    const SEED: u64 = 0x5a17_cafe;
    let fixture = fixture()?;
    let direct = direct_entry(&fixture)?;
    let alternate = entry(
        &fixture,
        EntryInput {
            key: "alternate",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.fallback,
            risk: DIRECT_RISK,
            effort: DIRECT_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let is_forward_order = (SEED & 1) == 1;
    let ordered = if is_forward_order {
        vec![direct.clone(), alternate.clone()]
    } else {
        vec![alternate.clone(), direct.clone()]
    };
    let replay = if is_forward_order {
        vec![direct.clone(), alternate.clone()]
    } else {
        vec![alternate, direct.clone()]
    };
    let left = admit_catalog(&CatalogSnapshot::new(ordered), limits()?)?;
    let right = admit_catalog(&CatalogSnapshot::new(replay), limits()?)?;
    assert_eq!(left.id(), right.id());
    let mutated = entry(
        &fixture,
        EntryInput {
            key: "direct",
            source: fixture.source,
            target: fixture.target,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: DIRECT_RISK.saturating_add(1),
            effort: DIRECT_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::SchemaRollback,
        },
    )?;
    let changed = admit_catalog(&CatalogSnapshot::new(vec![mutated]), limits()?)?;
    let original = admit_catalog(&CatalogSnapshot::new(vec![direct]), limits()?)?;
    assert_ne!(changed.id(), original.id());
    Ok(())
}
