## Context

schema-migration-core has existing owned source and historical source/license evidence. Operator authority selects AGPL-3.0-or-later for owned current/new material without revoking earlier grants.

## Goals / Non-Goals

Make package metadata and current grants consistent. Preserve third-party licenses and historical evidence. Do not change implementation, dependency cohorts, source identities, transport, or publication state.

## Decisions

Use AGPL-3.0-or-later in authoritative Cargo metadata and the complete GNU AGPL version 3 primary text. Use an explicit current-source grant plus notice preserving copyright and historical permissive terms. Child packages inherit the workspace license where applicable. Owned non-code material follows the same grant unless an explicit third-party or immutable-source notice governs it. Existing permissive texts remain historical, not a license offer for new contributions. Byte-identical content previously distributed under a permissive grant remains usable under that grant.

## Risks / Trade-offs

Changing working-tree licensing cannot relicense immutable dependency objects or revoke prior permissions. Publication requires separate reviewed source identity and operator action; no unpublished revision is inferred. The COB store has no declared remote; a separately supplied remote and authorization remain prerequisites.

## Migration Plan

Pass proposal, design, and tasks gates before product edits. Align only owned metadata and current licensing prose; preserve extraction facts and dependency identity. Retain historical license texts with explicit scope. Final coordinated licensing, projection, and packaging checks must run before verification is recorded.

## Validation Strategy

Inspect Cargo package license inheritance, complete primary license text, historical-grant boundaries, copyright notices, unchanged source/RID/revision inventories, and package contents using offline no-dependency metadata and no-verify package listing where available. No builds, lint, tests, formatters, or broad validation run during implementation.
