let Contracts = import "../contracts.ncl" in
{ schema = "schema-migration-conformance.v1", cases = [{ id = "bad", scenario = "direct" }] } | Contracts.Fixture
