## ADDED Requirements

### Requirement: Current owned-source licensing
r[owned_source_licensing.current] Current and new owned packages, code, contracts, schemas, fixtures, templates, documentation, and policy MUST be offered under AGPL-3.0-or-later, with complete primary license text and consistent package metadata.

#### Scenario: Current source package
r[owned_source_licensing.current.scenario.package]
- GIVEN current owned source is prepared for distribution
- WHEN the package license and accompanying notice are inspected
- THEN they identify AGPL-3.0-or-later without offering permissive terms for new contributions

### Requirement: Historical and third-party boundaries
r[owned_source_licensing.history] Original copyright attribution, extraction authorization, prior grants, third-party terms, and exact immutable dependency/source identities MUST remain unchanged. Prior grants covering byte-identical content remain effective.

#### Scenario: Retained immutable dependency
r[owned_source_licensing.history.scenario.immutable]
- GIVEN a consumer retains an earlier source object or a third-party dependency
- WHEN owned-source licensing changes
- THEN its original terms and exact object/RID identities remain unchanged

### Requirement: Evidence before completion
r[owned_source_licensing.verification] Licensing preparation MUST NOT claim publication, release readiness, or completed verification without observed evidence and MUST NOT archive this change before final checks.

#### Scenario: Preparation only
r[owned_source_licensing.verification.scenario.pending]
- GIVEN only lifecycle gates and source preparation have run
- WHEN the change is handed off
- THEN final licensing, projection, and packaging checks remain explicitly pending
