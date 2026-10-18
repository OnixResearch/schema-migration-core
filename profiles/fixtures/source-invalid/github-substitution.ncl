let Contracts = import "../../source-contracts.ncl" in
let source = import "../../source-boundary.ncl" in
(source & { nix.adapter = "https://github.com/OnixResearch/schema-migration-core" }) | Contracts.Source
