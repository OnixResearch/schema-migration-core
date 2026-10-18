# Schema Migration Core

Schema Migration Core provides product-neutral, deterministic planning between exact schema identities.

The `no_std + alloc` core admits immutable migration catalogs. It selects policy-approved paths and emits typed execution and recovery requests.

It does not load data, execute migration code, mutate storage, authorize changes, open transactions, or prove migrated data correctness.

## Components

- `schema-migration-core`: pure catalog admission, bounded path selection, plan identity, effect requests, freshness, and receipt projection.
- `schema-migration-conformance`: Nickel-generated positive and negative vectors for consumers.

## Planning contract

Each plan binds the source data identity, lineage, exact source and target schemas, catalog snapshot, policy, limits, observations, implementations, and path.

Equal candidates produce an ambiguity result. Resource exhaustion is different from no path. Stale observations deny execution handoff.

## Checks

Run the complete local gate with:

```text
nix flake check -L
```

Focused checks are:

```text
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build -p schema-migration-core --target wasm32-unknown-unknown
```

## Claim boundary

A valid plan records one deterministic selection from supplied facts.

It does not prove migration correctness, successful execution, atomicity, rollback success, restored external effects, authorization, or release eligibility.

## License

Current and new owned core and conformance packages, code, contracts, schemas, fixtures, templates, documentation, and policy are licensed under **AGPL-3.0-or-later**. See `LICENSE` for the complete license and `NOTICE` for attribution and historical/third-party boundaries. Both Cargo members inherit this workspace license.

`LICENSE-MIT` preserves earlier grants; it does not offer MIT terms for new owned contributions. Prior grants remain effective for content they already cover, including byte-identical retained content. The source identities and immutable dependency objects in `SOURCE.md` retain their original terms. This source preparation does not publish a revision or change consumer pins.

## References

- [Schema Identity Core](https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git) — exact schema, lineage, compatibility, and migration-reference contracts.
- [Kamacite](https://github.com/OnixResearch/kamacite) — independent interchange consumer pilot.
- [Molten](https://github.com/brittonr/aspen) — independent runtime planning consumer pilot.
