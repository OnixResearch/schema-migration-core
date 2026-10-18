let Contracts = import "../../source-contracts.ncl" in
let source = import "../../source-boundary.ncl" in
(source & { cargo.revision = "main" }) | Contracts.Source
