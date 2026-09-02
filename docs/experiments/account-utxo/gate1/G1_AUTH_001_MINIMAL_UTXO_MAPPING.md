# G1-AUTH-001 Minimal UTXO Reference Mapping

**Status:** NON-NORMATIVE EXPERIMENTAL REFERENCE MAPPING — `MAPPED`; EVIDENCE NOT COLLECTED
**Mapping alias:** `G1-AUTH-001-MINIMAL-UTXO-REFERENCE-001`
**Mapping format version:** `1`
**Candidate family / variant:** Minimal UTXO / deliberately simple reference mapping
**Mapping maturity:** `REFERENCE` — not optimized and not a family-wide claim
**Author or generator provenance:** hand-authored project analysis
**Mapping content hash:** unavailable until the mapping bytes are frozen
**Protocol adoption effect:** NONE

## 1. Frozen Binding and Scope

This mapping consumes, without modification:

- manifest: `docs/experiments/account-utxo/gate1/G1_AUTH_001_PAIRED_MANIFEST.md`;
- manifest SHA-256: `3456D4AB164DA7C6B4CB05282E2A14EE884187C374DA0996805AA44940E4555C`;
- manifest raw Git blob: `6bd8a59e640839d3a4402f6f67a5aa5cf45c6409`;
- case alias: `G1-AUTH-001-PAIR-001`;
- common-case SHA-256: `F52F3B81F47B49370F913F812829ACB1DAA8D1010E4E17AD7488B7E0BCC90AB3`; and
- common-case raw Git blob: `27795f874ed13164eee6e56617c8a0c025c90682`.

The optimization configuration is `NONE (REFERENCE)`. The preparation
configuration is `NONE DECLARED`. There is no experimental schema binding,
encoding, executable implementation, or evidence record.

`N = 1` means exactly one independent required **external consensus authority
relation**. It does not mean one Account, UTXO, input, source, credential,
signature, proof, artifact, verification, lookup, state access, record, or
persistent object.

## 2. Candidate Realization

### `P0-001` interpretation

Interpret `X` and `Y` only as the two frozen external semantic scopes. For this
reference mapping, candidate-local current state contains one unspent value
record carrying `2u` whose projection is associated with `X`, and one unspent
value record carrying `1u` whose projection is associated with `Y`. Their use
is currently admissible under the frozen lifecycle, history, domain/version,
isolation, and authorization profiles. This representation does not turn `X`
or `Y` into a protocol UTXO identity.

### `V-001` realization

The two candidate-local unspent value records project to `X = 2u`, `Y = 1u`,
total `3u`. Record identities, fields, keys, conditions, and storage layout are
analytical placeholders, not selected protocol schema.

### `E1-001` realization

One atomic candidate transition retires the `2u` record projecting to `X`,
creates one `1u` replacement record projecting to `X`, and creates one new `1u`
record projecting to `Y`. The existing `1u` record projecting to `Y` remains
current. This yields `X = 1u`, aggregate `Y = 2u`, total `3u`, issuance `0u`,
and burn `0u`. All retirement and creation occurs together only after all
conditions succeed; otherwise no candidate-local value change becomes
canonical. Arithmetic is exact over the case values and rejects underflow or
overflow.

### `A1-001` realization and authorization evidence placement

The retired `X`-projecting value record carries or logically references one
candidate-local authorization condition whose satisfaction realizes the one
external relation `A1-001`. The positive evidence is placed on the proposed
candidate transition and associated with its reference to that record; it is
not inferred from a UTXO/input label or external human identity. Validation
relates that evidence, the current referenced authorization condition, the
frozen domain/version and current profiles, and the exact indivisible
`E1-001`. Leaving the existing `Y` record intact and creating the additional
`Y`-projecting record adds no second independent external authority relation.
No privileged or alternate path is used.

This is a logical placement only. It selects no input/output grammar,
transaction field, credential, signature, proof, cryptographic primitive, PQ
primitive, signing bytes, authorization descriptor, or production consumption
mechanism.

### `P1-001` realization

After successful atomic application, the external projection is exactly
`X = 1u`, aggregate `Y = 2u`, total `3u`; issuance and burn are zero; the
supplied evidence satisfies the candidate realization of `A1-001` for exactly
`E1-001`; unrelated effects are not thereby authorized; and every frozen
external fact outside `V-001` and the excluded economic projection is
unchanged.

Retirement of the old `X` record, creation of its `1u` replacement, creation of
the additional `Y` record, and record-set maintenance are internal housekeeping
whose only external semantic projection is `E1-001`.

## 3. Dependencies and Persistent Facts

Logical authorization dependencies are: exact `E1-001` description -> frozen
domain/version and current-profile interpretation -> current existence and
authorization condition of the referenced `X` record -> supplied evidence
satisfaction -> atomic record-set transition. All are required; the arrows
describe dependency, not a selected validation order.

Current-state dependencies are the current unspent status, value, and
authorization condition of the referenced `X` record, plus the current
existing `Y` record needed to establish the frozen precondition and resulting
aggregate projection. Historical state, a stable identity, a nonce, a replay
record, and a migration record are not assumed.

Authorization-related persistent facts consist of one logical authorization
condition carried or referenced by the current `X` value record and a logically
equivalent condition on its replacement, plus the existing `Y` record's logical
control condition and a logically equivalent condition on the additional `Y`
record. The latter preserves both records' association with the same external
scope `Y`; it does not authorize the effect, participate in `A1-001`, or add a
second required authority relation. Direct attachment versus indirection is
unresolved. No separate persistent credential, key, history, registry,
tombstone, cache, or evidence object is selected.

## 4. Reuse Assumptions

- Authorization-evidence reuse: none. The positive evidence covers exactly
  this `E1-001`; reuse for another presentation, record, or effect is not
  claimed.
- Credential reuse: not assumed and not prohibited. The mapping does not fix
  credential cardinality.
- Authorization-condition reuse: a logically equivalent condition governs the
  retired and replacement `X` records, and the existing and additional `Y`
  records share a logically equivalent `Y` condition. These are candidate-local
  descriptive assumptions, not evidence reuse or protocol selections. No
  condition reuse across `X` and `Y` is assumed.
- Verification-result reuse: none claimed; the mapping does not state how many
  cryptographic verification operations occur.
- Implementation-local cache reuse: none assumed. A later implementation may
  evaluate a separately disclosed cache branch, but cache state cannot change
  consensus authorization.

## 5. Candidate-Native Cardinalities

These are descriptive mapping outputs only. They are not scores, evidence,
shared metrics, protocol requirements, or grounds for ranking.

| Candidate-native item | Reference output | Qualification |
|---|---:|---|
| pre-effect logical unspent value records | 2 | one projecting to each of `X` and `Y` |
| referenced/retired `X` value records | 1 | not an authority-count synonym |
| post-effect logical unspent value records | 3 | one for `X`, two aggregating to `Y` |
| logical current value records read | 2 | physical accesses are unspecified |
| logical value records retired | 1 | the pre-effect `X` record |
| logical value records created | 2 | one `X` replacement and one additional `Y` record |
| independent required external authority relations | 1 | frozen `N`, not a native-object count |
| logical authorization conditions consulted | 1 | applicable to the `X` reduction |
| transition-level authorization-evidence placements | 1 | a placement, not an artifact-count claim |
| credentials / signatures / proofs / artifacts | unresolved | no cryptographic or representation profile exists |
| cryptographic verification operations | unresolved | distinct from every count above |
| implementation-cache accesses | 0 assumed | not measured |

## 6. `G1-AUTH-ECON-EXCLUDED-001`

This mapping neither requires nor rules out a consensus-visible fee or other
resource-economic side effect because no such mechanism or profile is selected.
No economic effect is included in `V-001` or the `E1-001` success projection,
and none is claimed to be zero, absent, or unchanged. Whether an eventual UTXO
realization requires one is unresolved. Any required economic effect that
changes `X`, `Y`, the `3u -> 3u` relation, `A1-001`, `P1-001`, or another frozen
`C1-PAIR-001` component would make this reference mapping unable to satisfy this
exact manifest; it could not be silently reinterpreted.

## 7. Unresolved Requirements and Candidate-Specific Open Questions

Unresolved requirements include the UTXO state and transaction schema, record
identity and production consumption semantics, physical authorization
placement, ownership and authorization architecture, credential multiplicity
and format, exact authorization scope binding, algorithm/profile/version,
replay and historical interpretation, lifecycle, conflict and ordering,
failure-validation workflow, persistence and pruning, migration, commitment,
resource accounting, and economics.

Candidate-specific open questions are whether conditions are attached or
indirect, how old and replacement conditions remain interpretable, how the
additional `Y` record's logically equivalent condition is represented without
changing this case, whether such equivalence uses duplication or indirection,
which metadata persists, and which physical reads and writes the logical
dependencies induce.

## 8. Blocked Claims and Evidence Required Later

Blocked claims include protocol validity, security sufficiency, deterministic
historical behavior, replay exclusion, exact credential/artifact/verification
counts, exact bytes, performance or resource cost, state-growth cost, economic
effects, production suitability, PQ advantage, comparison superiority, and any
UTXO-family generalization.

Later evidence must include a reviewed concrete mapping/schema binding;
authorization-coverage table; positive scenario trace and failure-atomicity
trace; proof that evidence is bound to exactly `E1-001` and the applicable
current record/domain/version; independently reproducible authorization
outcome; candidate-native state-access and persistent-fact accounting; explicit
credential, grouping, artifact, verification, and reuse profiles; and, for any
quantitative claim, frozen cryptographic, encoding, implementation, resource,
and provenance profiles. Negative, replay, migration, hostile, and economic
claims require their own expressly scoped cases and evidence.

The mapping itself is not candidate evidence.

## 9. Explicit Non-Selections

This mapping does not select Account or UTXO, a state-model decision, a
production UTXO design, transaction or replay mechanism, cryptographic or PQ
mechanism, fee or resource mechanism, commitment or proof mechanism, consensus
mechanism, recovery or alternate authority, migration mechanism, state schema,
stable identity, input/output grammar, production consumption rule, ordering
rule, aggregation, batching, multisignature, threshold authorization, score,
rank, or winner.

## 10. Process Status

- Frozen paired manifest focused review: `PASS`.
- Mapping status: `MAPPED` as a deliberately simple `REFERENCE` mapping.
- Independent mapping review: `NOT STARTED`.
- Candidate evidence: `EVIDENCE_NOT_COLLECTED`.
- Comparison scoring: `NOT STARTED`.
- State-model ranking: `NONE`.
- State-model decision: `NOT MADE`.
- Account selected: `NO`.
- UTXO selected: `NO`.
- Formal Specification change: `NONE`.
- Consensus implementation change: `NONE`.
