//! Adversarial bounded-search, receipt, effect-order, and freshness tests.

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
    reason = "adversarial cases exercise the public migration contract as an external consumer; owner: schema-migration-core"
)]
use schema_migration_core::*;
use support::*;

#[test]
fn ambiguous_receipt_keeps_context_and_candidate_exhaustion_is_distinct() -> TestResult {
    // r[verify onix.schema_migration.receipts.ambiguous]
    // r[verify onix.schema_migration.ambiguity.bound]
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
    let snapshot = CatalogSnapshot::new(vec![left, right]);
    let request = request(&fixture, "data-a", Vec::new())?;
    let policy = policy(&fixture)?;
    let outcome = plan_migration(&snapshot, &request, &policy, limits()?);
    let receipt = project_receipt(&outcome);
    assert_eq!(receipt.decision(), PlanningDecisionClass::Ambiguous);
    assert_eq!(receipt.source_data(), Some(request.source_data()));
    assert!(receipt.catalog_id().is_some());
    assert!(receipt.policy_id().is_some());
    assert!(receipt.limits_id().is_some());
    assert_eq!(receipt.candidates().len(), 2);

    let candidate_limit = PlanningLimits::new(PlanningLimitValues {
        maximum_entries: MAX_ENTRIES,
        maximum_outgoing_edges: MAX_OUTGOING,
        maximum_path_steps: MAX_STEPS,
        maximum_candidate_paths: 1,
        maximum_search_states: MAX_STATES,
        maximum_preconditions: MAX_PRECONDITIONS,
        maximum_diagnostics: MAX_DIAGNOSTICS,
    })?;
    assert!(matches!(
        plan_migration(&snapshot, &request, &policy, candidate_limit),
        PlanningOutcome::BoundExhausted {
            reason: BoundReason::CandidatePaths,
            ..
        }
    ));
    Ok(())
}

#[test]
fn selected_effect_order_is_explicit_and_catalog_drift_denies_handoff() -> TestResult {
    // r[verify onix.schema_migration.effect_plan.valid]
    // r[verify onix.schema_migration.freshness.catalog]
    let fixture = fixture()?;
    let request = request(&fixture, "data-a", Vec::new())?;
    let policy = policy(&fixture)?;
    let limits = limits()?;
    let selected_outcome = plan_migration(
        &CatalogSnapshot::new(vec![direct_entry(&fixture)?]),
        &request,
        &policy,
        limits,
    );
    let plan = selected(&selected_outcome).ok_or("expected selected plan")?;
    let kinds: Vec<_> = plan.effects().iter().map(EffectRequest::kind).collect();
    assert_eq!(
        kinds,
        vec![
            EffectKind::Load,
            EffectKind::ValidateInput,
            EffectKind::Checkpoint,
            EffectKind::Migrate,
            EffectKind::ValidateOutput,
            EffectKind::Commit,
            EffectKind::Rollback,
            EffectKind::Receipt,
        ]
    );

    let alternate = entry(
        &fixture,
        EntryInput {
            key: "alternate-catalog-entry",
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
    let alternate_outcome = plan_migration(
        &CatalogSnapshot::new(vec![alternate]),
        &request,
        &policy,
        limits,
    );
    let alternate_plan = selected(&alternate_outcome).ok_or("expected alternate plan")?;
    let stale = FreshnessObservation::new(FreshnessInput {
        source_data: plan.source_data().clone(),
        source_schema: plan.source_schema(),
        target_schema: plan.target_schema(),
        catalog_id: alternate_plan.catalog_id(),
        policy_id: plan.policy_id(),
        limits_id: plan.limits_id(),
        observations_id: plan.observations_id(),
        implementations_id: plan.implementations_id(),
    });
    assert_eq!(
        check_freshness(plan, &stale),
        FreshnessDecision::Stale {
            reason: StaleReason::Catalog
        }
    );
    Ok(())
}

#[test]
fn satisfied_implementation_precondition_selects_compensation_request() -> TestResult {
    // r[verify onix.schema_migration.preconditions.valid]
    let fixture = fixture()?;
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
    let observation = PreconditionObservation::new(
        requirement,
        ObservationId::new("reviewed-implementation-observation", limit)?,
        true,
        Some(fixture.preferred.clone()),
    );
    let guarded_policy = PlanningPolicy::new(PlanningPolicyInput {
        required_preconditions: vec![PreconditionKind::Implementation],
        ..policy_input(&fixture)
    })?;
    let outcome = plan_migration(
        &CatalogSnapshot::new(vec![migration]),
        &request(&fixture, "data-a", vec![observation])?,
        &guarded_policy,
        limits()?,
    );
    let plan = selected(&outcome).ok_or("expected guarded plan")?;
    assert!(
        plan.effects()
            .iter()
            .any(|effect| { effect.kind() == EffectKind::Compensation && effect.is_conditional() })
    );
    assert!(
        !plan
            .effects()
            .iter()
            .any(|effect| { effect.kind() == EffectKind::Rollback && effect.is_conditional() })
    );
    Ok(())
}

#[test]
fn cyclic_catalog_does_not_revisit_a_schema() -> TestResult {
    let fixture = fixture()?;
    let source_middle = entry(
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
    let middle_source = entry(
        &fixture,
        EntryInput {
            key: "middle-source",
            source: fixture.middle,
            target: fixture.source,
            direction: Direction::Forward,
            cohort: &fixture.preferred,
            risk: LOW_RISK,
            effort: LOW_EFFORT,
            preconditions: Vec::new(),
            reversibility: ReversibilityClass::Irreversible,
        },
    )?;
    let middle_target = entry(
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
    let outcome = plan_migration(
        &CatalogSnapshot::new(vec![source_middle, middle_source, middle_target]),
        &request(&fixture, "data-a", Vec::new())?,
        &policy(&fixture)?,
        limits()?,
    );
    let plan = selected(&outcome).ok_or("expected bounded cyclic plan")?;
    assert_eq!(plan.steps().len(), 2);
    assert_eq!(plan.source_schema(), fixture.source);
    assert_eq!(plan.target_schema(), fixture.target);
    Ok(())
}
