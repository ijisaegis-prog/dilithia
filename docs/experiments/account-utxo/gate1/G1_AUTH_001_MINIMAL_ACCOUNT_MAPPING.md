# G1-AUTH-001 Minimal Account Reference Mapping

**Status:** NON-NORMATIVE EXPERIMENTAL REFERENCE MAPPING — `MAPPED`; EVIDENCE NOT COLLECTED
**Mapping alias:** `G1-AUTH-001-MINIMAL-ACCOUNT-REFERENCE-001`
**Mapping format version:** `1`
**Candidate family / variant:** Minimal Account / deliberately simple reference mapping
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
reference mapping, candidate-local current state has one Account-style value
fact for `X` carrying `2u` and one for `Y` carrying `1u`. Their use is currently
admissible under the frozen lifecycle, history, domain/version, isolation, and
authorization profiles. This representation does not turn `X` or `Y` into a
protocol Account identity.

### `V-001` realization

The two candidate-local value facts project to `X = 2u`, `Y = 1u`, total `3u`.
The Account-style records, fields, keys, and storage layout are analytical
placeholders, not selected protocol schema.

### `E1-001` realization

One atomic candidate transition conditionally subtracts `1u` from the
candidate-local value fact projecting to `X` and adds `1u` to the fact
projecting to `Y`. It commits both changes together only after all conditions
succeed; otherwise neither candidate-local value change becomes canonical.
Arithmetic is exact over the case values and rejects underflow or overflow.
The resulting projection is `X = 1u`, `Y = 2u`, total `3u`, issuance `0u`, and
burn `0u`.

### `A1-001` realization and authorization evidence placement

The `X` value-reduction fact has one candidate-local authorization condition
whose satisfaction realizes the one external relation `A1-001`. The positive
evidence is placed on the proposed candidate transition, not inferred from an
Account label or external human identity. Validation relates that evidence,
the current `X` authorization condition, the frozen domain/version and current
profiles, and the exact indivisible `E1-001`. The `Y` increase adds no second
independent external authority relation. No privileged or alternate path is
used.

This is a logical placement only. It selects no transaction field, credential,
signature, proof, cryptographic primitive, PQ primitive, signing bytes, or
authorization descriptor.

### `P1-001` realization

After successful atomic application, the external projection is exactly
`X = 1u`, `Y = 2u`, total `3u`; issuance and burn are zero; the supplied
evidence satisfies the candidate realization of `A1-001` for exactly
`E1-001`; unrelated effects are not thereby authorized; and every frozen
external fact outside `V-001` and the excluded economic projection is
unchanged.

Candidate-local balance replacement and bookkeeping are internal housekeeping:
they only maintain the two Account-style current facts and have no independently
meaningful external effect beyond `E1-001`.

## 3. Dependencies and Persistent Facts

Logical authorization dependencies are: exact `E1-001` description -> frozen
domain/version and current-profile interpretation -> current authorization
condition for the `X` reduction -> supplied evidence satisfaction -> atomic
state transition. All are required; the arrows describe dependency, not a
selected validation order.

Current-state dependencies are the current candidate-local `X` and `Y` value
facts and the current authorization condition applicable to `X`. Historical
state, a stable identity, a nonce, a replay record, and a migration record are
not assumed.

Authorization-related persistent facts consist of one logical authorization
condition associated with the current `X` value fact. Whether it is embedded
in the same physical record, referenced indirectly, or encoded elsewhere is
unresolved. No separate persistent credential, key, history, registry,
tombstone, cache, or evidence object is selected. The post-effect `X` fact
continues to be governed by a logically equivalent condition solely as
reference-mapping housekeeping; persistence and update mechanics remain open.

## 4. Reuse Assumptions

- Authorization-evidence reuse: none. The positive evidence covers exactly
  this `E1-001`; reuse for another presentation, source, or effect is not
  claimed.
- Credential reuse: not assumed and not prohibited. The mapping does not fix
  credential cardinality.
- Authorization-condition reuse: the same logical condition governs the
  pre-effect `X` reduction and the remaining `X` value fact; this is a
  candidate-local descriptive assumption, not evidence reuse or a protocol
  selection.
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
| pre-effect logical Account-style value facts | 2 | one projecting to each of `X` and `Y` |
| post-effect logical Account-style value facts | 2 | the two current facts are atomically replaced/updated |
| logical value facts read | 2 | physical accesses are unspecified |
| logical value facts written | 2 | physical writes are unspecified |
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
and none is claimed to be zero, absent, or unchanged. Whether an eventual
Account realization requires one is unresolved. Any required economic effect
that changes `X`, `Y`, the `3u -> 3u` relation, `A1-001`, `P1-001`, or another
frozen `C1-PAIR-001` component would make this reference mapping unable to
satisfy this exact manifest; it could not be silently reinterpreted.

## 7. Unresolved Requirements and Candidate-Specific Open Questions

Unresolved requirements include the Account state and transaction schema,
physical authorization placement, ownership and authorization architecture,
credential multiplicity and format, exact authorization scope binding,
algorithm/profile/version, replay and historical interpretation, lifecycle,
conflict and ordering, failure-validation workflow, persistence and pruning,
migration, commitment, resource accounting, and economics.

Candidate-specific open questions are whether authorization is embedded or
indirect, whether the condition remains stable across value updates, how
deletion or recreation affects history, whether several logical Account facts
can share a condition, which metadata persists, and which physical reads and
writes the logical dependencies induce.

## 8. Blocked Claims and Evidence Required Later

Blocked claims include protocol validity, security sufficiency, deterministic
historical behavior, replay exclusion, exact credential/artifact/verification
counts, exact bytes, performance or resource cost, state-growth cost, economic
effects, production suitability, PQ advantage, comparison superiority, and any
Account-family generalization.

Later evidence must include a reviewed concrete mapping/schema binding;
authorization-coverage table; positive scenario trace and failure-atomicity
trace; proof that evidence is bound to exactly `E1-001` and the applicable
current state/domain/version; independently reproducible authorization outcome;
candidate-native state-access and persistent-fact accounting; explicit
credential, grouping, artifact, verification, and reuse profiles; and, for any
quantitative claim, frozen cryptographic, encoding, implementation, resource,
and provenance profiles. Negative, replay, migration, hostile, and economic
claims require their own expressly scoped cases and evidence.

The mapping itself is not candidate evidence.

## 9. Explicit Non-Selections

This mapping does not select Account or UTXO, a state-model decision, a
production Account design, transaction or replay mechanism, cryptographic or PQ
mechanism, fee or resource mechanism, commitment or proof mechanism, consensus
mechanism, recovery or alternate authority, migration mechanism, state schema,
stable identity, nonce, ordering rule, aggregation, batching, multisignature,
threshold authorization, score, rank, or winner.

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

