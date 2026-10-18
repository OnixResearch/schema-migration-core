## Context

A migration reference proves that a named artifact is bound to one source and target schema. It does not choose a complete path or make execution safe.

The planner must consume Schema Identity Core contracts without duplicating their identity and compatibility decisions.

## Decisions

### Decision: Depend on published schema identity contracts

**Choice:** The core uses exact `TypeLineageId`, `SchemaId`, compatibility decision, and migration-reference types from one immutable Schema Identity Core revision.

**Rationale:** Two schema identity implementations can produce incompatible planning results.

### Decision: Admit immutable catalog snapshots

**Choice:** A catalog snapshot contains migration entries with source, target, direction, artifact, implementation cohort, profile, preconditions, reversibility class, cost facts, and limits.

**Rationale:** Path selection must not depend on a mutable ambient registry.

### Decision: Keep path policy explicit

**Choice:** Policy defines allowed directions, implementation cohorts, required preconditions, maximum steps, cost ordering, and tie behavior.

**Rationale:** The numerically shortest path is not always the safest or intended path.

### Decision: Reject unresolved ambiguity

**Choice:** If policy and cost facts leave multiple different paths equal, the core returns an ambiguity denial.

**Rationale:** Ambient catalog order must not select a migration path.

### Decision: Bind the data subject

**Choice:** A plan binds source data identity, source schema, target schema, catalog snapshot, policy, preconditions, limits, and selected steps.

**Rationale:** A path for one data snapshot must not authorize migration of another snapshot.

### Decision: Return typed effect plans

**Choice:** The core returns ordered load, validate, migrate, validate-output, checkpoint, commit, rollback, compensation, and receipt requests.

**Rationale:** The consumer shell owns migration code and every external effect.

### Decision: Separate rollback claims

**Choice:** Reversibility metadata can produce a rollback plan. It does not prove that rollback code exists, succeeds, or restores external effects.

**Rationale:** Schema reversal and operational recovery are different claims.

### Decision: Recheck freshness before handoff

**Choice:** Execution eligibility requires current source, target, catalog, policy, implementation, and precondition observations equal to the plan bindings.

**Rationale:** A valid old plan cannot authorize execution after relevant state changes.

### Decision: Keep planning bounded

**Choice:** Profiles bound catalog entries, outgoing edges, path steps, candidate paths, preconditions, diagnostics, and checked cost arithmetic.

**Rationale:** Adversarial or malformed catalogs must not cause unbounded search or overflow.

### Decision: Apply the complete Octet error gate

**Choice:** The Rust project will pin one reviewed immutable Octet revision through Nix and enable its complete lint catalog.

**Rationale:** Migration planning affects durable data and needs strict source and test checks.

## Risks / Trade-offs

- An admitted migration artifact can still contain incorrect code.
- Conservative ambiguity denial can require explicit operator policy.
- Cost metadata can be stale or inaccurate.
- A rollback plan cannot reverse unrelated external effects.
- A correct plan does not prove transactional or durable execution.
