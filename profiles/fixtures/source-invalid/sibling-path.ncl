# r[verify onix.schema_migration.source_revision.local]
let Contracts = import "../../source-contracts.ncl" in
let source = import "../../source-boundary.ncl" in
(source & { cargo.adapter = "../schema-migration-core" }) | Contracts.Source
