let exact_fields = fun allowed value =>
  std.is_record value
  && std.array.length (std.record.fields value) == std.array.length allowed
  && std.array.all (fun field => std.array.elem field allowed) (std.record.fields value)
in
let field = fun name value => std.record.get name value in
let full_revision = fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{40}$" value
in
let Adapter = "https://seed.radicle.garden/z1C4YVMgDGyVdQa72uPNj3UDS5cY.git" in
let valid_consumer = fun value =>
  exact_fields ["adapter", "revision"] value
  && field "adapter" value == Adapter
  && full_revision (field "revision" value)
in
let valid_source = fun value =>
  exact_fields ["schema", "repository_id", "acquisition_adapter", "reviewed_revision", "cargo", "nix", "non_claims"] value
  && field "schema" value == "onix.schema-migration.source-boundary.v1"
  && field "repository_id" value == "rad:z1C4YVMgDGyVdQa72uPNj3UDS5cY"
  && field "acquisition_adapter" value == Adapter
  && full_revision (field "reviewed_revision" value)
  && valid_consumer (field "cargo" value)
  && valid_consumer (field "nix" value)
  && (field "cargo" value).revision == field "reviewed_revision" value
  && (field "nix" value).revision == field "reviewed_revision" value
  && field "non_claims" value == [
    "publication does not prove remote availability",
    "publication does not prove delegate honesty",
    "publication does not prove consumer compatibility",
    "publication does not prove migration correctness or release eligibility",
  ]
in
{
  Source = std.contract.from_predicate valid_source,
}
