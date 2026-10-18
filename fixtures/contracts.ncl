let ExpectedContract = {
  decision | String,
  steps | Number,
  freshness | String | optional,
  conditional_recovery | Bool,
} in

let CaseContract = {
  id | String,
  scenario | String,
  expected | ExpectedContract,
} in

let FixtureContract = {
  schema | String,
  cases | Array CaseContract,
} in

{
  Expected = ExpectedContract,
  Case = CaseContract,
  Fixture = FixtureContract,
}
