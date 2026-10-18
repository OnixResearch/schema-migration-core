# Agent Notes

- Keep the component product-neutral. Schema meaning, migration code, data access, authorization, transactions, and rollback execution stay in consumers.
- Preserve a pure deterministic planning core without filesystem, registry, clock, process, or storage effects.
- Consume published schema-identity contracts through immutable revisions. Do not duplicate schema identity or compatibility semantics.
- Make every catalog snapshot, step, precondition, path, bound, freshness fact, and execution request explicit.
- Add positive and negative tests together, including ambiguous-path and stale-plan cases.
- Use named constants for every nontrivial numeric value.
- Prefer typed Nickel for human-authored profiles and contracts.
- Pin cross-repository dependencies to immutable published revisions.
- Apply the full pinned Octet error gate when Rust implementation starts.
- Use native `.cairn/` lifecycle packages and validate every changed package.
- Do not push unless the user explicitly requests it.
