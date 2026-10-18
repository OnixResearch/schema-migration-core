# Preimplementation licensing gate evidence

All commands ran from /home/brittonr/git/OnixResearch/schema-migration-core
using /tmp/kiln-agpl-cairn-bfa-target/debug/cairn, built by the parent offline
with --locked from unmodified exact Cairn
bfa88df5b2ac5f05e4a977bb8d4b8248d4511449. Parent-provided source NAR:
sha256-nQ/JSSXUQ5u1NvTQeIaBAO2koGlPPaLa/ol2NdUgha8=.
Parent-provided executable BLAKE3:
4b6b5110e762277ca8d1bd4643754407b3cbd91ad84fbe2dfec812950acbe8af.

The complete unchanged owning policy was explicitly selected with
--root . --policy cairn-policy/generated/cairn-policy.json. Policy hash:
f9e85c6afeb40f628a4baa5885fccf7768fff418975b35bbfba5c8d8cc62f9d7. No policy was weakened or replaced.

| Command | Observed result | Receipt hash |
|---|---|---|
| cairn gate proposal owned-source-agpl | PASS, valid true, no issues | cc6074bece6de6bf4e326fc9b6063ff9a15ea6b4812e8f5bc5652491addda639 |
| cairn gate design owned-source-agpl | PASS, valid true, no issues | 94eb8b22af00cc2954b38d5e30aa7a982c43fe877fa15ae53d079c95020e80a2 |
| cairn gate tasks owned-source-agpl | PASS, valid true, no issues | b608930b0e1332bf56aaf6c173ef200febaf8a4562161ceb7fb5bd7204fd6a2b |

All gates ran before product metadata changes. The workspace now selects
AGPL-3.0-or-later and both existing Cargo members inherit it. Root and
package-local LICENSE, LICENSE-MIT, and NOTICE carry the full current grant,
original historical MIT text, attribution, and third-party boundaries. Source
identities, immutable dependency objects, extraction facts, and prior grants
remain intact. No dependency, pin, algorithm, API, registry, or publication
changed. Owned policy/fixture licensing is stated in the current-source grant;
no executable license projection or fixture expectations required regeneration.

These gates establish structural preimplementation admission only, not legal
correctness, publication, or release readiness. Final coordinated licensing
and packaging verification remains parent-owned and pending. No builds, tests,
lint, formatters, or broad validation ran during worker implementation. The
change remains open with verification unchecked.
