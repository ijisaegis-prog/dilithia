# Dilithia Gate-1 Common Semantic Case G1-AUTH-001

**Alias:** `G1-AUTH-001-ORDINARY-NATIVE-DLTH-AUTHORIZATION`
**Status:** NON-NORMATIVE EXPERIMENTAL COMMON SEMANTIC CASE — REVISION 1 — FOCUSED RE-REVIEW PENDING
**Gate:** 1 — Ownership and Authorization
**Candidate families:** Minimal Account / Minimal UTXO
**State-model decision:** NOT MADE
**Account selected:** NO
**UTXO selected:** NO
**State-model ranking:** NONE
**Comparison scoring:** NOT STARTED
**Protocol adoption effect:** NONE
**Prior focused review:** REVISION REQUIRED
**Prior findings addressed:** G1-AUTH-001-001 / G1-AUTH-001-002
**Case content identity:** EXTERNAL SHA-256 AFTER BYTE FREEZE
**Paired manifest:** NOT YET CREATED
**Account mapping:** NOT YET CREATED
**UTXO mapping:** NOT YET CREATED
**Candidate evidence:** NOT YET GENERATED

> This artifact defines one shared external semantic problem.
>
> It does not describe how Account or UTXO internally represents that problem.
>
> Candidate-specific representation, source count, credential count, signature
> count, proof count, state access, record count, conflict footprint, storage
> layout, and implementation behavior are not common-case assumptions.
>
> Material case parameters that remain variable here must be instantiated once
> in the paired manifest and then held identical for both candidate mappings.

---

## 1. Purpose

This case is the first Gate-1 paired-comparison baseline.

It asks one representation-neutral question:

> Can the candidate represent deterministic authorization of one ordinary
> native-DLTH effect such that only evidence satisfying the required authority
> relation for the exact shared external effect can satisfy the authorization
> condition?

This case defines the external problem through:

- shared external precondition predicate `P0`;
- affected native-DLTH semantic facts `V`;
- one externally meaningful native-DLTH effect `E1`;
- one independently required consensus authority relation `A1`; and
- shared required external postcondition predicate `P1`.

The paired manifest must instantiate all material case-local parameters used by
`P0`, `V`, `E1`, `A1`, and `P1` identically before either candidate mapping is
created.

This case is intentionally limited to the positive ordinary-authorization
baseline.

It does not itself exercise:

- multiple independent required authorizers;
- credential rotation;
- cryptographic migration;
- dormant-value migration;
- catastrophic cryptographic failure;
- recovery;
- delegation;
- multisignature;
- negative authorization evidence;
- cross-network replay;
- historical authorization change;
- reorganization;
- authenticated-state proof construction; or
- resource-performance superiority.

Those remain separate scenarios or profile branches.

---

## 2. Authority and Methodology Boundary

This common case is subordinate to the repository's authoritative protocol
material and reviewed state-model decision requirements.

Material comparison methodology includes:

- `docs/ACCOUNT_UTXO_GATE1_9_COMPARISON_EVIDENCE_MATRIX.md`;
- `docs/ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`;
- `docs/ACCOUNT_UTXO_WORKLOAD_MODEL.md`;
- `docs/ACCOUNT_UTXO_CANDIDATE_MAPPING_REQUIREMENTS.md`;
- `docs/OWNERSHIP_AUTHORIZATION_REQUIREMENTS.md`; and
- applicable reviewed Gate-1-through-9 requirement records.

This artifact creates no consensus rule.

If this artifact conflicts with superior protocol authority, superior authority
controls.

---

## 3. Common Semantic Vocabulary

### 3.1 `P0`

`P0` denotes the shared representation-neutral external precondition predicate
for this case.

`P0` is not candidate state.

It does not mean:

- an Account database state;
- a UTXO set;
- a state root;
- a state commitment;
- a transaction;
- an input set;
- an Account record; or
- an implementation snapshot.

The paired manifest must instantiate every material case-local parameter needed
to make `P0` reproducible.

At minimum, the manifest must bind or explicitly mark not applicable:

1. the affected external native-DLTH control/value facts;
2. any external effect magnitude or value relation material to the case;
3. the current lifecycle/admissibility condition required for the positive case;
4. the protocol-version context;
5. the domain context;
6. the current replay/history disposition needed to make this a presently
   admissible positive case;
7. any conflict/order disposition material to the positive case;
8. the required supply/conservation disposition;
9. the external facts required to remain unchanged; and
10. every other non-authorization precondition that could materially change the
    meaning or outcome of the paired comparison.

A material field may be parameterized here.

It may not be chosen differently by Account and UTXO.

### 3.2 `V`

`V` denotes exactly the native-DLTH control/value facts whose permitted
candidate-independent external change from `P0` to `P1` constitutes `E1`.

`V` therefore receives its semantic role from the shared `P0`/`P1` relation.

`V` is not assumed to be one candidate-native entity.

`V` does not mean:

- an Account record;
- an Account balance field;
- a persistent account identity;
- a UTXO;
- an input;
- an output;
- a state key;
- a database record;
- a physical storage unit; or
- a transaction field.

The paired manifest must bind the same external `V` semantics for both
candidates before candidate mappings begin.

### 3.3 `E1`

`E1` denotes exactly one externally meaningful native-DLTH effect represented
by the shared relation:

```text
E1 : P0 -> P1
```

`E1` is the candidate-independent external effect being authorized.

The internal realization of `E1` is candidate-specific.

The common case does not define:

- transaction structure;
- debit structure;
- credit structure;
- input/output grammar;
- balance mutation;
- record consumption;
- record creation;
- state mutation layout;
- transaction identity; or
- effect-identifier encoding.

The paired manifest must instantiate enough external case parameters that two
reviewers can determine whether the Account and UTXO mappings are attempting to
realize the same `E1`.

A candidate mapping may not redefine `E1` after seeing its own internal
architecture.

### 3.4 `P1`

`P1` denotes the shared representation-neutral required external postcondition
predicate for successful `E1`.

For this positive baseline, `P1` must establish at property level that:

1. the exact external effect declared as `E1` occurred;
2. the affected native-DLTH control/value facts `V` changed only as declared by
   `E1`;
3. the required candidate-independent supply/conservation disposition holds;
4. external facts declared unchanged by the manifest remain unchanged;
5. authorization of `E1` did not silently grant authority over unrelated
   effects; and
6. both candidate mappings are evaluated against the same observable success
   projection.

`P1` does not define how a candidate stores or proves those facts internally.

### 3.5 `A1`

`A1` denotes one independently required consensus authority relation for `E1`.

For this baseline:

```text
N = 1 independent required consensus authority relation
```

`A1` is an opaque case-local semantic authority relation.

`A1` is not assumed to equal:

- one human;
- one external owner identity;
- one account;
- one UTXO;
- one input;
- one credential;
- one key;
- one signature;
- one proof;
- one authorization artifact;
- one cryptographic verification;
- one state lookup;
- one address; or
- one state record.

The candidate mapping must determine how its architecture realizes `A1`.

The paired manifest must hold the external authority scope of `A1` identical for
both candidates.

### 3.6 Authorization evidence

Authorization evidence means only:

> evidence supplied to protocol interpretation that is evaluated against the
> applicable authorization rule for `A1` and `E1`.

This case does not define:

- evidence encoding;
- cryptographic primitive;
- signature count;
- proof count;
- credential count;
- artifact count;
- verification count;
- grouping;
- batching; or
- reuse policy.

Those remain candidate/profile outputs.

---

## 4. Frozen External Contract

Before Account or UTXO mapping begins, the paired manifest must bind one common
external contract.

That contract consists of:

```text
C1 = (P0, V, E1, A1, P1, M1)
```

where `M1` is the set of all additional material case parameters required to
make the positive baseline reproducible.

`M1` may contain only external semantic or profile context.

It must not contain candidate-native architecture chosen merely to make one
candidate favorable.

The same content-identified `C1` must be supplied to both candidate mappings.

A material change to `C1` creates a different paired evidence case.

---

## 5. Frozen External Preconditions

The common positive-baseline preconditions are:

1. `P0` is satisfied under the case parameters fixed by the paired manifest;
2. `V` refers to the same external native-DLTH control/value facts for both
   candidates;
3. exactly one externally meaningful effect `E1` is proposed;
4. `E1` has the same external meaning for both candidates;
5. `E1` requires exactly one independent consensus authority relation `A1`;
6. `A1` has the same external scope for both candidates;
7. the experimental authorization evidence satisfies `A1` for `E1`;
8. the protocol-version and domain context are identical for both candidates;
9. any replay/history disposition required to make the positive case currently
   admissible is identical;
10. any lifecycle, conflict, ordering, supply, or conservation precondition
    material to this positive case is identical;
11. all external facts declared unrelated to `E1` are held unchanged;
12. no privileged administrator, founder, foundation, emergency key, recovery
    master key, or equivalent authority participates;
13. no alternate authority path is assumed;
14. no stable external human identity is assumed; and
15. no Account-specific or UTXO-specific representation is part of the common
    external contract.

If an additional non-authorization precondition can materially change candidate
behavior or evidence interpretation, it must be declared in the paired manifest
before mappings begin.

It cannot be silently chosen separately by the two candidates.

These are experimental comparison preconditions.

They do not define production transaction validity.

---

## 6. Required External Authorization Outcome

For the frozen positive case, both mappings are evaluated against the same
property-level result:

```text
shared_external_precondition(P0) = SATISFIED
required_authority_relation(A1, E1) = SATISFIED
authorization_condition_for(E1) = SATISFIED
required_external_postcondition(P1) = SATISFIED
```

A candidate receives structural success for this baseline only if its mapping
shows that:

1. the same `P0` is interpreted as the required starting external condition;
2. supplied evidence satisfies the candidate's realization of `A1`;
3. that authorization applies to the exact shared semantic scope of `E1`;
4. the candidate's successful realization produces the same required external
   `P1`; and
5. no unrelated external authority or value/control effect is silently added.

The candidate may realize the transition differently internally.

It may not substitute a different external transition.

---

## 7. Scope-Safety Obligation

Each candidate mapping must state explicitly:

1. which candidate-specific logical facts are needed to realize `P0`;
2. how candidate-specific state realizes the external `V`;
3. which candidate-specific logical facts are consulted to interpret `A1`;
4. how supplied evidence is associated with the permitted semantic scope of
   `E1`;
5. how candidate-specific behavior realizes the required external `P1`;
6. what candidate-specific assumptions are necessary;
7. whether internal authorization state is required;
8. whether authorization evidence, condition, credential, or verification
   result is reused;
9. whether such reuse is protocol-visible or implementation-local; and
10. which unresolved assumptions prevent a stronger claim.

These are candidate outputs.

They must not be copied backward into the common external contract.

---

## 8. Exact-Effect Boundary

This baseline establishes authorization only for the shared `E1`.

Evidence satisfying `A1` for `E1` is not assumed to authorize another effect
`E2`.

The required property is:

```text
scope(A1, evidence, E1) permits E1
scope(A1, evidence, E1) does not thereby permit unrelated E2
```

This is a property-level requirement.

It does not select:

- transaction identifiers;
- effect identifiers;
- signed-message bytes;
- nonce;
- sequence counter;
- UTXO one-use semantics;
- Account authorization descriptors; or
- any scope encoding.

A later negative companion case must test mismatched effect scope.

---

## 9. Positive / Negative Case Boundary

This case deliberately freezes valid authorization evidence for the positive
baseline.

It does not claim evidence for:

- missing evidence;
- malformed evidence;
- cryptographically invalid evidence;
- valid evidence for another authority;
- valid evidence for another semantic effect;
- domain mismatch;
- cross-network mismatch;
- stale historical authorization;
- authorization after a lifecycle change; or
- any other negative case.

Those cases remain required where applicable.

Success in G1-AUTH-001 cannot be used as evidence that those negative cases pass.

---

## 10. Failure and Atomicity Boundary

The inherited Gate-1 requirement remains:

> authorization failure must not create a partial canonical effect.

G1-AUTH-001 does not evidence that failure property.

A later negative companion case must exercise failing or mismatched
authorization.

No validation order is selected.

No rollback implementation is selected.

Local temporary verification work, implementation caches, and consensus-visible
canonical effects remain distinct.

---

## 11. Domain, Replay, and Historical Boundary

This baseline may assume one internally consistent domain/version/history
profile only to isolate ordinary positive authorization.

The paired manifest must content-identify that shared profile.

Separate cases remain required, where applicable, for:

- mismatched effect scope;
- mismatched domain;
- cross-network replay;
- duplicate presentation;
- historical versus current authorization interpretation;
- lifecycle changes affecting authorization; and
- reapplication after reverted authoritative history.

This case selects no:

- `ChainId` value;
- `NetworkId` discriminant;
- domain-tag encoding;
- signing-message layout;
- replay identity;
- nonce;
- sequence counter; or
- one-use mechanism.

---

## 12. Cryptographic Boundary

No cryptographic primitive or parameter set is selected.

No assumption is made that:

- Account uses one signature;
- UTXO uses one signature per input;
- one authority relation uses one credential;
- one credential requires one evidence artifact;
- one evidence artifact requires one verification;
- one external authority maps to one candidate-native ownership condition;
- verification results must be reused; or
- verification results must not be reused.

Cryptographic artifact counts, sizes, verification operations, grouping,
batching, reuse, invalidation, coexistence, and implementation caching remain
candidate/profile outputs governed by the applicable Gate-6 framework.

---

## 13. Candidate-Neutrality Guard

Neither candidate may redefine the common external contract:

```text
C1 = (P0, V, E1, A1, P1, M1)
```

after candidate mapping begins.

The Account mapping must not receive credit merely because Account vocabulary
describes some internal realization compactly.

The UTXO mapping must not receive credit merely because UTXO vocabulary
describes some internal realization naturally.

Candidate-native differences are allowed and expected.

They are outputs of the mapping.

They are not allowed to change the shared external problem.

Candidate-native cardinalities may be recorded descriptively.

They are not automatically common comparison metrics.

---

## 14. Candidate Outputs Requested Later

Both mappings must later report the same classes of evidence, where applicable:

- mapping status;
- realization of shared `P0`;
- interpretation of `V`;
- realization of `A1`;
- authorization evidence placement;
- realization of `E1`;
- realization of required `P1`;
- exact-effect coverage relation;
- logical authorization dependencies;
- current-state dependencies;
- authorization-related persistent facts;
- evidence reuse assumptions;
- credential reuse assumptions;
- ownership-condition or authorization-condition reuse assumptions;
- verification-result reuse assumptions;
- implementation-local cache assumptions;
- unresolved requirements;
- blocked claims; and
- evidence needed to support the mapping.

Equivalent evidence classes are required from both candidates.

Candidate-native internal structures need not be identical.

---

## 15. Mapping Success Boundary

A mapping of G1-AUTH-001 is not evidence that the candidate is globally
superior.

A mapping is not comparison scoring.

A mapping is not protocol adoption.

For this case, a candidate mapping is structurally usable only if it:

1. preserves the exact shared `C1`;
2. realizes the same external `P0`;
3. identifies how `A1` controls authorization of the same `E1`;
4. realizes the same external `P1`;
5. does not rely on privileged authority;
6. does not silently assume unselected recovery, delegation, multisignature,
   stable identity, replay, or migration mechanisms;
7. records all material candidate-specific assumptions;
8. keeps candidate-native architecture outside the common contract; and
9. remains reproducible enough for independent review.

Failure to produce a valid mapping must remain visible as evidence.

---

## 16. Explicit Non-Selections

This common case selects no:

- Account design;
- UTXO design;
- hybrid design;
- ownership representation;
- stable logical identity;
- owner identity;
- address role;
- credential format;
- key format;
- signature format;
- proof format;
- cryptographic primitive;
- PQ primitive;
- authorization descriptor;
- authorization grouping rule;
- multisignature;
- threshold mechanism;
- delegation;
- recovery;
- alternate authority;
- key rotation;
- migration mechanism;
- transaction format;
- transaction identifier;
- effect identifier;
- input structure;
- output structure;
- Account nonce;
- sequence counter;
- UTXO one-use encoding;
- state schema;
- state key;
- state commitment;
- proof system;
- snapshot trust model;
- light-client protocol;
- resource meter;
- gas;
- fee mechanism;
- storage rent;
- refund mechanism;
- numeric resource limit;
- consensus algorithm;
- fork-choice rule;
- finality rule;
- scheduler;
- parallel-execution mechanism; or
- state-model ranking.

---

## 17. Paired-Manifest Minimum Requirements

The paired manifest created after this common case passes focused review must
content-identify at minimum:

1. this exact common-case artifact identity;
2. the complete shared `C1` parameter instantiation;
3. concrete case-local `P0` parameters;
4. concrete case-local `V` semantics;
5. concrete case-local `E1` external semantics;
6. the external scope and `N = 1` meaning of `A1`;
7. required external `P1`;
8. domain and protocol-version profile;
9. applicable authorization/cryptographic profile;
10. lifecycle disposition;
11. replay/history disposition;
12. conflict/order disposition if material;
13. supply/conservation disposition;
14. declared unchanged external facts;
15. excluded and deferred negative scenarios; and
16. eventual Account and UTXO mapping artifact identities.

A paired manifest may instantiate parameters.

It may not invent different external semantics for the two candidates.

---

## 18. Evidence Status

Common semantic case:

**REVISION 1 — NOT YET FOCUSED RE-REVIEWED**

Prior focused review:

**REVISION REQUIRED**

G1-AUTH-001-001:

**CORRECTION APPLIED IN REVISION 1 — RE-REVIEW PENDING**

G1-AUTH-001-002:

**CORRECTION APPLIED IN REVISION 1 — RE-REVIEW PENDING**

Paired manifest:

**NOT YET CREATED**

Minimal Account mapping:

**NOT YET CREATED**

Minimal UTXO mapping:

**NOT YET CREATED**

New Gate-1 candidate evidence generated under this case:

**NO**

Comparison scoring:

**NOT STARTED**

State-model ranking:

**NONE**

State-model decision:

**NOT MADE**

Account selected:

**NO**

UTXO selected:

**NO**

---

## 19. Next Step

The next step is not paired-manifest creation yet.

The next step is:

> focused read-only re-review of this exact Revision-1 common semantic case,
> limited to G1-AUTH-001-001, G1-AUTH-001-002, and regression against the
> previously passing neutrality and boundary checks.

Only after that focused re-review passes may the paired manifest be created.

Account and UTXO mappings remain prohibited until the reviewed paired manifest
exists.

**STATE MODEL DECISION: NOT MADE**