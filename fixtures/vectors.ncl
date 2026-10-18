let Contracts = import "contracts.ncl" in

{
  schema = "schema-migration-conformance.v1",
  cases = [
    { id = "positive-no-op", scenario = "no-op", expected = { decision = "no-op", steps = 0, conditional_recovery = false } },
    { id = "positive-direct", scenario = "direct", expected = { decision = "selected", steps = 1, conditional_recovery = false } },
    { id = "positive-multi-step", scenario = "multi-step", expected = { decision = "selected", steps = 2, conditional_recovery = false } },
    { id = "positive-preferred-cohort", scenario = "preferred-cohort", expected = { decision = "selected", steps = 1, conditional_recovery = false } },
    { id = "positive-reversible", scenario = "reversible", expected = { decision = "selected", steps = 1, conditional_recovery = true } },
    { id = "negative-entry-limit", scenario = "entry-limit", expected = { decision = "invalid-catalog", steps = 0, conditional_recovery = false } },
    { id = "negative-duplicate", scenario = "duplicate", expected = { decision = "invalid-catalog", steps = 0, conditional_recovery = false } },
    { id = "negative-ambiguous", scenario = "ambiguous", expected = { decision = "ambiguous", steps = 0, conditional_recovery = false } },
    { id = "negative-no-path", scenario = "no-path", expected = { decision = "no-path", steps = 0, conditional_recovery = false } },
    { id = "negative-overflow", scenario = "overflow", expected = { decision = "invalid-catalog", steps = 0, conditional_recovery = false } },
    { id = "negative-precondition", scenario = "missing-precondition", expected = { decision = "precondition-denied", steps = 0, conditional_recovery = false } },
    { id = "negative-cohort", scenario = "wrong-cohort", expected = { decision = "precondition-denied", steps = 0, conditional_recovery = false } },
    { id = "negative-stale", scenario = "stale", expected = { decision = "selected", steps = 1, freshness = "stale-source-data", conditional_recovery = false } },
  ],
} | Contracts.Fixture
