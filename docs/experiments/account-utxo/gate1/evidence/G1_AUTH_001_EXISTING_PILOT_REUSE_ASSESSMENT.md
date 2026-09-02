# G1-AUTH-001 Existing Pilot Reuse Assessment

**Disposition:** NOT ELIGIBLE FOR DIRECT G1 CREDIT; PENDING RE-EXECUTION

**Current-package provenance:** PENDING EVIDENCE. The current five-file
G1-AUTH-001 evidence package has no durable identity binding its manifest, all
final payloads, source revision, and exact governing methodology/requirements
versions. This assessment therefore remains a documentary draft and is itself
**NOT ELIGIBLE FOR DIRECT G1 CREDIT** until re-executed under a complete durable
package binding. This does not make the older pilot eligible and applies no
candidate-specific treatment.

## 1. Scope and rule

Repository pilot artifacts under
`docs/experiments/account-utxo/pilot/` were inspected for exact compatibility
with the frozen `G1-AUTH-001-PAIR-001` case, profiles, candidate meanings, and
provenance. Direct reuse requires every material dimension to align and be
content-addressed. Similar value preservation is insufficient.

## 2. Pilot evidence located

The repository contains a Phase-B deterministic structural package including a
final evidence manifest, Account and UTXO evidence records, deterministic
summary, review records, registration receipt, and procedure provenance. Its
manifest records source revision
`ba768dbfc4189987f1ea8231f3b1c3ec442f5fda` and hashes for its methodology,
configuration, records, reviews, provenance, and summary. It is genuine
content-addressed evidence for its own pilot identity.

Material pilot bindings include:

| Dimension | Pilot binding |
|---|---|
| Semantic case SHA-256 | `1DB6FBE63C13BFB619BFECE7D61D0A9958F00E2BF8866331A4602BA0240B276D` |
| Paired manifest SHA-256 | `219B286FD6D4EFB7ECE6F18B5872A34C22F6209E8CE45FA8BE96DD9AB1081D83` |
| Account mapping SHA-256 | `94FD6D8C72137CBCB228C32C8D298930C36C99BEA1BB167FFAE747D69B57F600` |
| UTXO mapping SHA-256 | `3211A348EDA0AE97271F22C1D6C0622B29366BCDFACDE1196E73D350249265C3` |
| Authorization profile | `pilot/authorization-not-bound-no-case-authority-relation/v1` |
| Algorithm profile | `pilot/cryptography-not-exercised/v1` |

## 3. Eligibility comparison

| Requirement | G1-AUTH-001 frozen identity | Pilot | Result |
|---|---|---|---|
| Exact case semantics | `X:2u->1u`, `Y:1u->2u`, `N=1`, authorization evidence satisfying exact `E1-001` | One-unit value-effect baseline; no case authority relation | **MISMATCH** |
| Exact manifest | SHA-256 `3456D4...55C` | SHA-256 `219B28...1D83` | **MISMATCH** |
| Exact Account mapping | SHA-256 `C33C30...A17D` | SHA-256 `94FD6D...F600` | **MISMATCH** |
| Exact UTXO mapping | SHA-256 `BD6FCF...475B` | SHA-256 `3211A3...65C3` | **MISMATCH** |
| Authorization assumptions | Positive evidence; one exact external authority relation; current condition and profiles | Authorization not bound; no case authority relation | **MATERIAL MISMATCH** |
| Candidate meaning | Two Account facts before/after; UTXO two records before and three after | Account two quantity relations; UTXO one-to-one record replacement | **MATERIAL MISMATCH** |
| Provenance | Must bind the G1 manifest, both G1 mappings, mapping freeze, binding, and binding freeze | Well-recorded, but binds the older pilot inputs/source revision | **INCOMPATIBLE IDENTITY** |

## 4. Reuse disposition

**DERIVED STRUCTURAL CLAIM:** the older package demonstrates a useful procedure
pattern—hash checking, pre-result configuration, separate candidate records,
review provenance, exclusions, and deterministic summary—for its own case.

**PENDING EVIDENCE:** any new execution or independent review using the exact
G1-AUTH-001 identities and authorization-bearing semantics.

**UNSUPPORTED CLAIM:** that pilot results validate `A1-001`, the current G1
Account or UTXO mappings, their cardinalities, their positive authorization
path, or any negative/failure/performance proposition.

Therefore no pilot result is imported as evidence credit. The pilot may inform
procedure design only. If reuse is desired later, re-execute under a frozen G1
configuration with exact input hashes and new provenance; do not relabel the
old records.
