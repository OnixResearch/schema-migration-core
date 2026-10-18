let Contracts = import "../../source-contracts.ncl" in
let source = import "../../source-boundary.ncl" in
(source & { cargo.revision = "3f7b4315" }) | Contracts.Source
