# Architecture

## Functional core

`schema-migration-core` owns deterministic meaning for:

- immutable catalog admission and canonical identity;
- policy-directed bounded path selection;
- explicit no-op, selected, no-path, ambiguity, bound, precondition, and invalid results;
- complete plan and path identities;
- ordered typed effect requests;
- exact pre-handoff freshness checks;
- bounded metadata-only receipts.

The core receives all catalogs, policies, observations, limits, and identities as values. It performs no I/O.

## Consumer shell

Consumers own:

- catalog loading and currentness;
- data and migration artifact access;
- migration implementation review;
- authorization and approval;
- clocks and environment observations;
- transactions, checkpoints, commits, rollback, and compensation;
- persistence, publication, retention, and release decisions.

Effect requests are plans. They do not grant effect authority or report effect success.

## Dependency direction

Schema Migration Core consumes Schema Identity Core at one immutable revision. It does not implement schema identity, aliases, compatibility, or migration-reference semantics.

Content Identity Core supplies deterministic BLAKE3 framing. It does not own migration policy.

Kamacite and Molten adapt product facts at their boundaries. Their types and authority do not enter the shared core.
