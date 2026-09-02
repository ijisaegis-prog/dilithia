# G1-AUTH-001 Minimal UTXO Structural Evidence

**Status:** PENDING DURABLE PACKAGE PROVENANCE; NOT ELIGIBLE FOR DIRECT GATE-1 CREDIT
**Candidate:** Minimal UTXO reference mapping
**Case:** `G1-AUTH-001-PAIR-001`
**Evidence kind:** documentary, deterministic derivation from frozen artifacts
**Independent implementation evidence:** NOT GENERATED

The derivations below are preserved as a documentary draft. The five-file
package, including its manifest and applicable methodology/requirements
versions, has no complete durable content identity. Accordingly every
substantive structural conclusion below is **PENDING EVIDENCE** for Gate-1
credit and **RE-EXECUTION REQUIRED** after complete package binding. This
provenance disposition applies equally to the Account and UTXO records and
does not change their frozen candidate meanings.

## 1. Claim vocabulary and provenance

The labels below apply to every evidence-bearing claim in this record. A label
on a section-introductory sentence or paragraph applies to the table or
propositions it introduces unless a row or later paragraph carries a different
label. Procedural instructions in Section 9 are not evidence claims.

| Label | Meaning |
|---|---|
| **FACT FROM FROZEN ARTIFACT** | Directly stated by a content-identified frozen input. |
| **DERIVED STRUCTURAL CLAIM** | Checkable consequence of those statements; not an implementation observation. |
| **PENDING EVIDENCE** | Evidence is required but is absent or outside this positive-baseline case. |
| **UNSUPPORTED CLAIM** | The available artifacts do not justify the proposition. |

**FACT FROM FROZEN ARTIFACT:** the frozen inputs and identities are:

| Artifact | SHA-256 | Raw Git blob |
|---|---|---|
| Paired manifest | `3456D4AB164DA7C6B4CB05282E2A14EE884187C374DA0996805AA44940E4555C` | `6bd8a59e640839d3a4402f6f67a5aa5cf45c6409` |
| UTXO mapping | `BD6FCFC9FDE0B84A3046F0E862287159013F3AA2FB2E4649B1237C6E7A01475B` | `55106e28f35a2868b1e993e33f2c0180095dc876` |
| Mapping identity binding | `8B686C366F02979BAC7AA384D190650468AE94EB65AA5AFEA9ACB691D26C4745` | `ded0d2364296c1644b3f0b11f8fb7c78e8513701` |

**FACT FROM FROZEN ARTIFACT:** mapping freeze commit: `ed51717583844eab4dcf2a2cc99d2a08c95ca42d`.
Binding freeze commit: `3984d8a9d9983524abc5506802134e9c5284e40e`.
The mapping pair and binding focused reviews both record `PASS`.
**UNSUPPORTED CLAIM:** neither mapping nor review constitutes independent
implementation evidence.

## 2. Positive-baseline trace

| Stage | UTXO realization | Classification |
|---|---|---|
| `P0-001` interpretation | One current unspent record projects to `X = 2u`; one projects to `Y = 1u`; the frozen current lifecycle, fresh-history, common domain/version, isolated-effect, valid-authorization, and conservation profiles apply. | **FACT FROM FROZEN ARTIFACT** |
| `V-001` realization | Two logical records project to `(X, Y, total) = (2u, 1u, 3u)`. | **FACT FROM FROZEN ARTIFACT** |
| `E1-001` trace | One atomic transition retires the `2u` `X` record, creates a `1u` `X` replacement and a new `1u` `Y` record, and leaves the existing `1u` `Y` record current. | **FACT FROM FROZEN ARTIFACT** |
| `A1-001` coverage | The retired `X` record carries or references one logical condition. Evidence on the proposed transition is related to that record, current condition, frozen profiles, and exact indivisible `E1-001`. Neither retained nor new `Y` value adds another independent relation. | **FACT FROM FROZEN ARTIFACT** |
| `P1-001` result | Successful application projects to `X = 1u`, aggregate `Y = 2u`, total `3u`, issuance `0u`, burn `0u`; unrelated effects are not thereby authorized. | **DERIVED STRUCTURAL CLAIM** |

**DERIVED STRUCTURAL CLAIM:** the last row follows from the record projection: `X = 1u`; `Y = 1u + 1u =
2u`; total `3u`. It is a paper trace, not an executed validator result.

## 3. Authorization coverage

| Required element | Frozen UTXO realization | Coverage disposition |
|---|---|---|
| One independent external relation `A1-001` | One logical condition on/referenced by the retired `X` record | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Exact effect scope | Evidence is associated with the transition's reference to that record and exact indivisible `E1-001` | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Current authorization facts | Current existence, unspent status, and current condition of referenced `X` record participate | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Domain/version context | Frozen `G1-AUTH-PAIR-DOMAIN-VERSION-001` participates | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Current lifecycle/history | Frozen current and fresh profiles participate | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| No extra `Y` authority | Existing and additional `Y` records do not participate in `A1-001` | **FACT FROM FROZEN ARTIFACT** |
| No privileged/alternate path | Neither participates | **FACT FROM FROZEN ARTIFACT** |
| Concrete evidence validation | No grammar, evidence bytes, primitive, or executable validator exists | **PENDING EVIDENCE** |

**DERIVED STRUCTURAL CLAIM:** the maximum claim is structural coverage of the
declared positive mapping. Cryptographic validity, implementation correctness,
and invalid-case behavior are **PENDING EVIDENCE**.

## 4. Facts, dependencies, and preservation

**FACT FROM FROZEN ARTIFACT:** logical authorization dependencies are:

`exact E1-001 description -> frozen domain/version and current profiles -> current existence and authorization condition of referenced X record -> supplied evidence satisfaction -> atomic record-set transition`

**FACT FROM FROZEN ARTIFACT:** the arrows are dependencies, not a required validation order. Current-state
dependencies are the referenced `X` record's current unspent status, value, and
condition, plus the existing `Y` record used for precondition and aggregate
projection. Historical state, stable identity, nonce, replay record, and
migration record are **PENDING EVIDENCE** and are not assumed.

**FACT FROM FROZEN ARTIFACT:** persistent authorization facts are a logical condition carried or referenced
by current `X` and a logically equivalent condition on its replacement, plus
logically equivalent `Y` conditions on existing and additional `Y` records.
The `Y` conditions preserve association and do not authorize `E1-001` or add an
external authority relation. Attachment, indirection, encoding, credentials,
keys, histories, registries, tombstones, caches, and evidence objects remain
**PENDING EVIDENCE**.

| Preservation obligation | Deterministic check | Result |
|---|---|---|
| `P0-001` | Uses exactly the frozen quantities, one effect, `N = 1`, and frozen positive profiles | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `V-001` | Two current records project to `2u + 1u = 3u` | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `E1-001` | Retirement/creation projects only to the `1u` reassignment and is indivisible | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `A1-001` | One external relation is realized by the referenced `X` condition; record count is not authority count | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `P1-001` | Projection is `1u, 2u, 3u`, with issuance/burn `0u` | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| Unchanged external facts | Retirement, creation, and record-set maintenance are declared internal with no other external projection | **DERIVED STRUCTURAL CLAIM — PRESERVED under the mapping declaration** |

## 5. Reuse taxonomy

**FACT FROM FROZEN ARTIFACT:** the frozen mapping states the following reuse assumptions.

| Reuse class | Frozen assumption |
|---|---|
| Authorization-evidence reuse | None; evidence covers exactly this `E1-001`. |
| Credential reuse | Neither assumed nor prohibited; cardinality unresolved. |
| Authorization-condition reuse | Equivalent conditions span old/replacement `X` and existing/additional `Y`; no cross-`X`/`Y` reuse assumed; descriptive only. |
| Verification-result reuse | None claimed; operation count unresolved. |
| Implementation-cache reuse | None assumed; a disclosed future cache branch cannot alter consensus authorization. |

## 6. Candidate-native descriptive cardinalities

**FACT FROM FROZEN ARTIFACT:** these mapping outputs are descriptive mapping
outputs, not measurements or comparison metrics.

| Item | Output |
|---|---:|
| Pre-effect logical unspent value records | 2 |
| Referenced/retired `X` records | 1 |
| Post-effect logical unspent value records | 3 |
| Logical current value records read | 2 |
| Logical value records retired | 1 |
| Logical value records created | 2 |
| Independent required external authority relations | 1 |
| Logical authorization conditions consulted | 1 |
| Transition-level evidence placements | 1 |
| Credentials/signatures/proofs/artifacts | unresolved |
| Cryptographic verification operations | unresolved |
| Implementation-cache accesses | 0 assumed, not measured |

**UNSUPPORTED CLAIM:** any UTXO advantage, cost, or family-wide property inferred
from these counts.

## 7. Economic-side-effect disposition

**FACT FROM FROZEN ARTIFACT:** `G1-AUTH-ECON-EXCLUDED-001` applies. No fee or other resource-economic effect
is in `V-001` or the success projection, and none is claimed zero, absent, or
unchanged. **PENDING EVIDENCE:** whether a concrete UTXO realization requires
one is unresolved. **DERIVED STRUCTURAL CLAIM:** any required effect changing
a frozen contract component would make this mapping
unable to satisfy the exact case.

## 8. Pending and unsupported claims

**PENDING EVIDENCE:** concrete schema and validator; executable positive result;
failure atomicity trace; missing, malformed, invalid, stale, mismatched, replay,
cross-domain, migration, transition, reorganization, hostile-work, and
cryptographic-break cases; exact credential/artifact/verification counts;
exact bytes; physical accesses and persistence; benchmarks, timing, resources,
economics, security sufficiency, historical behavior, and production fitness.

**UNSUPPORTED CLAIM:** that this trace is independent implementation evidence;
that it establishes protocol validity; that abstract evidence is a signature or
PQ artifact; that input/record count implies authorizer, artifact, signature,
or verification count; that counts establish cost, superiority, ranking, or a
general UTXO property.

## 9. Reproduction/check procedure

1. Hash the paired manifest, UTXO mapping, and binding with SHA-256 and
   `git hash-object`; compare them with Section 1.
2. Confirm both freeze commits exist with `git cat-file -e <commit>^{commit}`.
3. In the manifest verify `q = 1u`, pre-state `(2u,1u)`, post-state `(1u,2u)`,
   total `3u`, `N = 1`, and the named profiles.
4. In the UTXO mapping independently locate the `P0/V/E1/A1/P1`, dependency,
   persistence, reuse, cardinality, economic, and blocked-claim statements.
5. Recompute the projection/arithmetic and check all coverage/preservation rows
   solely against those statements. Do not treat the result as an executed test.
