# Source inventory

## Schema Identity Core

Canonical repository: `rad:z6gGpUJtzdVBCCtZTzh4cV1skv4H`

Reviewed revision: `2562c8aa38a034061f9af9f3e17280494a5b8de2`

Consumed contracts:

- `TypeLineageId`
- `SchemaId`
- `MigrationReference`
- `MigrationReferenceId`
- `MigrationArtifactId`
- `PolicyProfileId`

Schema Migration Core does not duplicate schema descriptors, schema hashing, alias decisions, or compatibility classification.

## Content Identity Core

Canonical repository: `rad:zA14jhTdwST4PxPBdsryc4zS7Kpq`

Reviewed revision: `7f55597b5dc879b7601856e8d7fd0dbacaa2a498`

It supplies versioned deterministic BLAKE3 framing only.

## Consumer observations

Molten has product-owned migration recipes, storage mutation, system-extension generation changes, checkpoints, and receipts.

Kamacite has product-owned schema projections, adapter identities, diagnostics, and interchange receipts.

These product semantics remain in their repositories. The shared component owns planning mechanics only.
