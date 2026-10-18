# Conformance

The conformance crate consumes `fixtures/generated/vectors.json`. Nickel 1.17.0 generates this file from `fixtures/vectors.ncl`.

Positive vectors cover no-op, direct, multi-step, preferred-cohort, and reversible planning.

Negative vectors cover entry limits, duplicate entries, ambiguity, no path, checked overflow, missing preconditions, cohort mismatch, and stale plans.

The shared consumer fixture returns one plan identity. It executes no migration and mutates no product storage.
