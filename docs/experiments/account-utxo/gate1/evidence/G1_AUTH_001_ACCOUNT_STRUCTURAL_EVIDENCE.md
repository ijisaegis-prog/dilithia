# G1-AUTH-001 Minimal Account Structural Evidence

**Status:** PENDING DURABLE PACKAGE PROVENANCE; NOT ELIGIBLE FOR DIRECT GATE-1 CREDIT
**Candidate:** Minimal Account reference mapping
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
| Account mapping | `C33C308A841A820312826C0A0C937E725D00589631382A04D6008DB7A376A17D` | `83073e5ff31d7cc042621827b1e08f8b3c46e231` |
| Mapping identity binding | `8B686C366F02979BAC7AA384D190650468AE94EB65AA5AFEA9ACB691D26C4745` | `ded0d2364296c1644b3f0b11f8fb7c78e8513701` |

**FACT FROM FROZEN ARTIFACT:** mapping freeze commit: `ed51717583844eab4dcf2a2cc99d2a08c95ca42d`.
Binding freeze commit: `3984d8a9d9983524abc5506802134e9c5284e40e`.
The mapping pair and binding focused reviews both record `PASS`.
**UNSUPPORTED CLAIM:** neither mapping nor review constitutes independent
implementation evidence.

## 2. Positive-baseline trace

| Stage | Account realization | Classification |
|---|---|---|
| `P0-001` interpretation | Current logical Account-style facts project to `X = 2u` and `Y = 1u`; the frozen current lifecycle, fresh-history, common domain/version, isolated-effect, valid-authorization, and conservation profiles apply. | **FACT FROM FROZEN ARTIFACT** |
| `V-001` realization | Two logical value facts project to `(X, Y, total) = (2u, 1u, 3u)`. | **FACT FROM FROZEN ARTIFACT** |
| `E1-001` trace | One conditional atomic transition subtracts `1u` from the `X` fact and adds `1u` to the `Y` fact; commit occurs only after all declared conditions succeed. | **FACT FROM FROZEN ARTIFACT** |
| `A1-001` coverage | The `X` reduction has one logical authorization condition. Supplied positive evidence is related to that current condition, the frozen profiles, and exactly the indivisible `E1-001`. The `Y` increase adds no second independent external authority relation. | **FACT FROM FROZEN ARTIFACT** |
| `P1-001` result | Successful application projects to `X = 1u`, `Y = 2u`, total `3u`, issuance `0u`, burn `0u`; unrelated effects are not thereby authorized. | **DERIVED STRUCTURAL CLAIM** |

**DERIVED STRUCTURAL CLAIM:** the `P1-001` row follows arithmetically from the frozen transition:
`2u - 1u = 1u`, `1u + 1u = 2u`, and `1u + 2u = 3u`.
It is a paper trace of the exact positive path, not an executed validation result.

## 3. Authorization coverage

| Required element | Frozen Account realization | Coverage disposition |
|---|---|---|
| One independent external relation `A1-001` | One logical condition applicable to the `X` reduction | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Exact effect scope | Evidence is placed on the proposed transition and related to exact indivisible `E1-001` | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Current authorization facts | Current `X` authorization condition is consulted | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Domain/version context | Frozen `G1-AUTH-PAIR-DOMAIN-VERSION-001` participates | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| Current lifecycle/history | Frozen current and fresh profiles participate | **DERIVED STRUCTURAL CLAIM — COVERED structurally** |
| No extra `Y` authority | The `Y` increase introduces no second independent relation | **FACT FROM FROZEN ARTIFACT** |
| No privileged/alternate path | Neither participates | **FACT FROM FROZEN ARTIFACT** |
| Concrete evidence validation | No schema, evidence bytes, primitive, or executable validator exists | **PENDING EVIDENCE** |

**DERIVED STRUCTURAL CLAIM:** thus the maximum claim is structural coverage of the declared positive mapping.
Cryptographic validity, implementation correctness, and resistance to invalid
evidence are **PENDING EVIDENCE**.

## 4. Facts, dependencies, and preservation

**FACT FROM FROZEN ARTIFACT:** logical authorization dependencies are:

`exact E1-001 description -> frozen domain/version and current profiles -> current X authorization condition -> supplied evidence satisfaction -> atomic transition`

**FACT FROM FROZEN ARTIFACT:** the arrows are dependencies, not a required execution order. Current-state
dependencies are the current `X` and `Y` value facts and the current condition
applicable to `X`. Historical state, stable identity, nonce, replay record, and
migration record are **PENDING EVIDENCE** and are not assumed.

**FACT FROM FROZEN ARTIFACT:** persistent authorization facts comprise one logical condition associated with
the current `X` value fact. The post-effect `X` fact retains a logically
equivalent condition as mapping-local housekeeping. Physical co-location,
indirection, encoding, separate credentials, keys, histories, registries,
tombstones, caches, and evidence objects are **PENDING EVIDENCE**.

| Preservation obligation | Deterministic check | Result |
|---|---|---|
| `P0-001` | Uses exactly the frozen quantities, one effect, `N = 1`, and frozen positive profiles | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `V-001` | Candidate facts project to `2u + 1u = 3u` | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `E1-001` | Only the projected `1u` reassignment occurs; transition is indivisible | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `A1-001` | One external relation is realized by the `X` condition; no extra relation is inferred | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| `P1-001` | Projection is `1u, 2u, 3u`, with issuance/burn `0u` | **DERIVED STRUCTURAL CLAIM — PRESERVED** |
| Unchanged external facts | Account-local replacement/bookkeeping is declared internal and introduces no other external projection | **DERIVED STRUCTURAL CLAIM — PRESERVED under the mapping declaration** |

## 5. Reuse taxonomy

**FACT FROM FROZEN ARTIFACT:** the frozen mapping states the following reuse assumptions.

| Reuse class | Frozen assumption |
|---|---|
| Authorization-evidence reuse | None; evidence covers exactly this `E1-001`. |
| Credential reuse | Neither assumed nor prohibited; cardinality unresolved. |
| Authorization-condition reuse | Same logical condition governs pre-effect and remaining `X` facts; descriptive only. |
| Verification-result reuse | None claimed; operation count unresolved. |
| Implementation-cache reuse | None assumed; any future cache branch must be disclosed and cannot alter consensus authorization. |

## 6. Candidate-native descriptive cardinalities

**FACT FROM FROZEN ARTIFACT:** these mapping outputs are descriptive mapping
outputs, not measurements or comparison metrics.

| Item | Output |
|---|---:|
| Pre-effect logical Account-style value facts | 2 |
| Post-effect logical Account-style value facts | 2 |
| Logical value facts read | 2 |
| Logical value facts written | 2 |
| Independent required external authority relations | 1 |
| Logical authorization conditions consulted | 1 |
| Transition-level evidence placements | 1 |
| Credentials/signatures/proofs/artifacts | unresolved |
| Cryptographic verification operations | unresolved |
| Implementation-cache accesses | 0 assumed, not measured |

**UNSUPPORTED CLAIM:** any Account advantage, cost, or family-wide property
inferred from these counts.

## 7. Economic-side-effect disposition

**FACT FROM FROZEN ARTIFACT:** `G1-AUTH-ECON-EXCLUDED-001` applies. No fee or other resource-economic effect
is included in `V-001` or the success projection, and none is claimed zero,
absent, or unchanged. **PENDING EVIDENCE:** whether a concrete Account
realization requires one is unresolved. **DERIVED STRUCTURAL CLAIM:** any
required effect that changes a frozen contract component would
make this mapping unable to satisfy this exact case.

## 8. Pending and unsupported claims

**PENDING EVIDENCE:** concrete schema and validator; executable positive result;
failure atomicity trace; missing, malformed, invalid, stale, mismatched, replay,
cross-domain, migration, transition, reorganization, hostile-work, and
cryptographic-break cases; exact credential/artifact/verification counts;
exact bytes; physical accesses and persistence; benchmarks, timing, resources,
economics, security sufficiency, historical behavior, and production fitness.

**UNSUPPORTED CLAIM:** that this trace is independent implementation evidence;
that it establishes protocol validity; that the abstract evidence is a
signature or PQ artifact; that one Account means one authorizer; that the
descriptive counts establish cost, superiority, ranking, or a general Account
property.

## 9. Reproduction/check procedure

1. Hash the paired manifest, Account mapping, and binding with SHA-256 and
   `git hash-object`; compare them with Section 1.
2. Confirm both freeze commits exist with `git cat-file -e <commit>^{commit}`.
3. In the frozen manifest, verify `q = 1u`, pre-state `(2u,1u)`, post-state
   `(1u,2u)`, total `3u`, `N = 1`, and the named profiles.
4. In the Account mapping, independently locate the `P0/V/E1/A1/P1`, dependency,
   persistence, reuse, cardinality, economic, and blocked-claim statements.
5. Recompute the arithmetic and check every coverage/preservation row solely
   against those statements. Do not treat the result as an executed test.
