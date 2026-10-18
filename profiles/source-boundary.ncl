# r[verify onix.schema_migration.source_revision]
# r[verify onix.schema_migration.source_revision.agree]
let Contracts = import "source-contracts.ncl" in
let Adapter = "https://seed.radicle.garden/z1C4YVMgDGyVdQa72uPNj3UDS5cY.git" in
let Revision = "4fe90e130f2871cf69a6febcdc70785adca98aea" in
{
  schema = "onix.schema-migration.source-boundary.v1",
  repository_id = "rad:z1C4YVMgDGyVdQa72uPNj3UDS5cY",
  acquisition_adapter = Adapter,
  reviewed_revision = Revision,
  cargo = { adapter = Adapter, revision = Revision },
  nix = { adapter = Adapter, revision = Revision },
  non_claims = [
    "publication does not prove remote availability",
    "publication does not prove delegate honesty",
    "publication does not prove consumer compatibility",
    "publication does not prove migration correctness or release eligibility",
  ],
} | Contracts.Source
