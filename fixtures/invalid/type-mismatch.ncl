let Contracts = import "../contracts.ncl" in
{
  schema = "schema-migration-conformance.v1",
  cases = [{ id = "bad", scenario = "direct", expected = { decision = "selected", steps = "one", conditional_recovery = false } }],
} | Contracts.Fixture
