//! Negative construction tests for explicit resource limits.

#[test]
fn zero_limit_fails_before_search() {
    // r[verify onix.schema_migration.quality_gate.ambiguity]
    assert!(matches!(
        schema_migration_core::PlanningLimits::new(schema_migration_core::PlanningLimitValues {
            maximum_entries: 0,
            maximum_outgoing_edges: 1,
            maximum_path_steps: 1,
            maximum_candidate_paths: 1,
            maximum_search_states: 1,
            maximum_preconditions: 1,
            maximum_diagnostics: 1,
        }),
        Err(schema_migration_core::MigrationError::ZeroLimit { .. })
    ));
}
