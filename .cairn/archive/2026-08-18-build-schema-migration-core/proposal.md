## Why

Schema Identity Core owns exact schema identities, lineages, aliases, migration references, and compatibility decisions. It intentionally does not select or execute migration paths.

Kamacite, Molten, and future configuration stores need deterministic plans across exact schema versions. Local planners can select ambiguous paths or execute stale plans.

## What Changes

- Create a product-neutral `no_std + alloc` schema migration planning core.
- Admit exact migration catalogs bound to schema identities, artifact identities, directions, profiles, preconditions, and implementation cohorts.
- Select paths through explicit policy precedence, costs, limits, and tie behavior.
- Fail closed when valid candidates remain ambiguous under the selected policy.
- Bind each plan to source data identity, source schema, target schema, catalog snapshot, policy, preconditions, resource bounds, and plan identity.
- Emit typed step, execution, checkpoint, rollback, compensation, and recovery requests without executing them.
- Recheck catalog, source, target, policy, and precondition freshness before execution handoff.
- Add positive, negative, property, mutation, and stale-plan fixtures.
- Pin the full Octet gate before Rust implementation enters the checked path.

## Impact

- **Producer contract**: Schema Identity Core
- **Consumers**: Kamacite, Molten, configuration stores, artifact metadata stores, and runtime registries
- **New component**: `schema-migration-core`
- **Authority**: consumers retain migration code, data access, authorization, transactions, effects, rollback, and release policy
- **Evidence**: a plan records one deterministic selection and does not prove successful migration
