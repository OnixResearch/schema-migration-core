let Contracts = import "../../source-contracts.ncl" in
let source = import "../../source-boundary.ncl" in
(source & { nix.revision = "0000000000000000000000000000000000000000" }) | Contracts.Source
