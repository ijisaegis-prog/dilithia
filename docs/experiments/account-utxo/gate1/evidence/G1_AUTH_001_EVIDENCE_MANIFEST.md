# G1-AUTH-001 Structural Evidence Manifest

**Status:** PENDING DURABLE PROVENANCE; NOT ELIGIBLE FOR DIRECT GATE-1 CREDIT
**Evidence case:** `G1-AUTH-001-PAIR-001`
**Evidence scope:** exact frozen positive authorization baseline only
**Scoring/ranking/selection:** NOT OCCURRING

## 1. Frozen input identity binding

This package is downstream of, and is valid only for, the following exact
inputs. It does not alter or extend them.

| Input | Path | SHA-256 | Raw Git blob |
|---|---|---|---|
| Paired manifest | `docs/experiments/account-utxo/gate1/G1_AUTH_001_PAIRED_MANIFEST.md` | `3456D4AB164DA7C6B4CB05282E2A14EE884187C374DA0996805AA44940E4555C` | `6bd8a59e640839d3a4402f6f67a5aa5cf45c6409` |
| Minimal Account mapping | `docs/experiments/account-utxo/gate1/G1_AUTH_001_MINIMAL_ACCOUNT_MAPPING.md` | `C33C308A841A820312826C0A0C937E725D00589631382A04D6008DB7A376A17D` | `83073e5ff31d7cc042621827b1e08f8b3c46e231` |
| Minimal UTXO mapping | `docs/experiments/account-utxo/gate1/G1_AUTH_001_MINIMAL_UTXO_MAPPING.md` | `BD6FCFC9FDE0B84A3046F0E862287159013F3AA2FB2E4649B1237C6E7A01475B` | `55106e28f35a2868b1e993e33f2c0180095dc876` |
| Mapping identity binding | `docs/experiments/account-utxo/gate1/G1_AUTH_001_MAPPING_IDENTITY_BINDING.md` | `8B686C366F02979BAC7AA384D190650468AE94EB65AA5AFEA9ACB691D26C4745` | `ded0d2364296c1644b3f0b11f8fb7c78e8513701` |

Mapping freeze commit:
`ed51717583844eab4dcf2a2cc99d2a08c95ca42d`.

Binding freeze commit:
`3984d8a9d9983524abc5506802134e9c5284e40e`.

The focused candidate-mapping and binding reviews record `PASS`. Review status
is process provenance, not candidate or implementation evidence.

## 2. Package content binding

The identities below describe the four payloads before the provenance
correction prompted by review. They are retained only as audit history and do
not bind the current five-file package. The files were untracked, the stated
payload blobs were absent from the repository object database, and no durable
external receipt or evidence revision recorded the manifest identity.

| Included file | Role | SHA-256 | Raw Git blob |
|---|---|---|---|
| `G1_AUTH_001_ACCOUNT_STRUCTURAL_EVIDENCE.md` | Account positive structural trace and disclosures | `9E70AA48896787C79741488FF9ECC321E590AA08249EF13CEB89B4D4BA174A56` | `3c1bd65ad530e2db4cfc3869bd0c42bfdef8df24` |
| `G1_AUTH_001_UTXO_STRUCTURAL_EVIDENCE.md` | UTXO positive structural trace and disclosures | `D03AC3463CD635399A4A8CC04D3443AF2009FFB16502DA3981CA0C4D3E6ABFDA` | `7ebfa344e4994448781f19a8e5ce56a614abef3b` |
| `G1_AUTH_001_EXISTING_PILOT_REUSE_ASSESSMENT.md` | Eligibility determination for repository pilot evidence | `C7EB40A527AC459BF2DA977CE0308FAC9FA62564512CA953789AE1A2D3B0E520` | `4a5419fb6452032cb0d8de023190cd893acf2939` |
| `G1_AUTH_001_PAIRED_STRUCTURAL_EVIDENCE_SUMMARY.md` | Symmetric non-scoring paired summary | `30647D7A833F1527462743F494B226A37298B52716C5131033A1AACF2A7FC5EC` | `50745fb245eb78e0605e7ea2b64202d514021057` |
| `G1_AUTH_001_EVIDENCE_MANIFEST.md` | Package identity, scope, inclusions, and exclusions | **NOT RECORDED** | **NOT RECORDED** |

All included paths are relative to
`docs/experiments/account-utxo/gate1/evidence/`. Because the current manifest,
payloads, and governing versions lack one complete durable content binding,
this package is **PENDING EVIDENCE**, **NOT ELIGIBLE FOR DIRECT GATE-1 CREDIT**,
and **RE-EXECUTION REQUIRED**. The documentary derivations may be inspected,
but their substantive conclusions receive no evidence credit from this
package.

### 2.1 Governing-identity gap

The applicable methodology/requirements set includes at least
`docs/ACCOUNT_UTXO_GATE1_9_COMPARISON_EVIDENCE_MATRIX.md`,
`docs/ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`,
`docs/ACCOUNT_UTXO_WORKLOAD_MODEL.md`,
`docs/ACCOUNT_UTXO_CANDIDATE_MAPPING_REQUIREMENTS.md`,
`docs/OWNERSHIP_AUTHORIZATION_REQUIREMENTS.md`, and the applicable reviewed
Gate-1-through-9 requirement records. Exact content identities for that set
were not recorded with this package. Applicability and version identity are
therefore **PENDING EVIDENCE**, not inferred from current working-tree bytes.

Eligibility requires a future non-main evidence revision or durable external
receipt binding the final manifest, all four final payloads, the source
revision, and every applicable governing document identity. This record does
not claim that such a freeze or receipt exists.

## 3. Included evidence scope

Subject to the provenance limitation in Section 2, the draft documents only
the following independently checkable derivations for the exact supplied
positive baseline; it does not currently support Gate-1 evidence credit for
them:

- frozen identity and provenance checks;
- `P0-001`, `V-001`, `E1-001`, `A1-001`, and `P1-001` structural traces;
- positive authorization-coverage tables;
- candidate-local logical dependency and current-state traces;
- persistent authorization-fact and reuse disclosures;
- unchanged-external-fact and conservation checks;
- candidate-native cardinalities as descriptive mapping outputs only;
- the frozen economic-side-effect exclusion; and
- a direct-reuse eligibility assessment of the prior repository pilot.

Every candidate conclusion is either a direct frozen-artifact fact or a
deterministic structural derivation. Narrative and tables are not represented
as executed tests or independent implementations.

## 4. Excluded and pending evidence classes

The following are not generated by this package and remain **PENDING EVIDENCE**
unless a future separately scoped, content-bound record establishes them:

- missing, malformed, cryptographically invalid, stale, mismatched, duplicate,
  replayed, cross-domain, or adversarial authorization cases;
- failure traces and independently observed failure atomicity;
- migration, recovery, rotation, deprecation, coexistence, lifecycle-transition,
  version-transition, reorganization, and cryptographic-break cases;
- exact credentials, signatures, proofs, artifacts, verification operations,
  bytes, encodings, physical reads/writes, persistence, or state growth;
- quantitative timing, latency, throughput, CPU, memory, network, database,
  resource, fee, economic, or hostile-work benchmarks;
- implementation correctness, independent implementation outcomes, protocol
  validity, security sufficiency, production suitability, and family-wide
  Account or UTXO claims.

No benchmark was run. No exact-byte profile exists. No independent
implementation result exists. The prior pilot is not imported because its
case, mappings, authorization assumptions, and provenance identity do not
match this frozen evidence identity.

## 5. Comparison and decision boundary

This package performs no scalar scoring, weighting, normalization of
candidate-native counts, ranking, winner selection, or state-model
recommendation. It does not select Account, UTXO, an authorization or
cryptographic mechanism, a transaction/state schema, a replay mechanism, an
economic/resource mechanism, or any production mechanism. It changes no
Formal Specification, Constitution, or Threat Model.

## 6. Reproduction procedure

1. Run SHA-256 and `git hash-object` over each frozen input and compare with
   Section 1; verify both commit objects exist.
2. Run the same hashes over the four payloads and compare with Section 2.
3. Read the manifest, both mappings, mapping review, binding, binding review,
   and applicable methodology/requirements documents.
4. Reperform each paper derivation using only the frozen statements; verify the
   Account and UTXO documents use the same external case and disclosure classes.
5. Confirm the pilot assessment against the pilot evidence records' embedded
   case, manifest, mapping, authorization-profile, and source-revision values.
6. Confirm no claim crosses an exclusion in Section 4 and no native count is
   used as a score or state-model inference.

## 7. Process status

- Structural positive-baseline documentary draft: **GENERATED**
- Durable complete-package provenance: **PENDING EVIDENCE**
- Direct Gate-1 evidence credit: **NOT ELIGIBLE**
- Re-execution after complete content binding: **REQUIRED**
- Quantitative/performance evidence: **NOT GENERATED**
- Independent implementation evidence: **NOT GENERATED**
- Comparison scoring: **NOT STARTED**
- State-model ranking: **NONE**
- State-model decision: **NOT MADE**
- Account selected: **NO**
- UTXO selected: **NO**
- Main merge: **NOT DONE**
