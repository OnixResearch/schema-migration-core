//! Nickel-generated conformance vector tests.

#[test]
fn positive_and_negative_vectors_match_the_pure_core() -> Result<(), Box<dyn std::error::Error>> {
    // r[verify onix.schema_migration.conformance.positive]
    // r[verify onix.schema_migration.conformance.negative]
    schema_migration_conformance::verify_embedded_vectors()?;
    Ok(())
}

#[test]
fn unknown_scenario_fails_closed() {
    assert!(schema_migration_conformance::run_scenario("unknown-scenario").is_err());
}

#[test]
fn shared_consumer_fixture_is_effect_free_planning() -> Result<(), Box<dyn std::error::Error>> {
    // r[verify onix.schema_migration.consumer_pilots]
    // r[verify onix.schema_migration.consumer_pilots.valid]
    // r[verify onix.schema_migration.consumer_pilots.no_effect]
    let report = schema_migration_conformance::shared_fixture_report()?;
    assert_eq!(report.decision(), "selected");
    assert_eq!(report.step_count(), 1);
    assert!(!report.has_conditional_recovery());
    assert!(report.plan_id().is_some());
    Ok(())
}
