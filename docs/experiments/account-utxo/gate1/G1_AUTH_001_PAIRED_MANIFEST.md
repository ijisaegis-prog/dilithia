# Dilithia G1-AUTH-001 Paired Manifest

**Alias:** `G1-AUTH-001-PAIR-001`
**Status:** NON-NORMATIVE EXPERIMENTAL PAIRED MANIFEST — REVISION 1 — FOCUSED RE-REVIEW PENDING
**Gate:** 1 — Ownership and Authorization
**Common case:** `G1-AUTH-001-ORDINARY-NATIVE-DLTH-AUTHORIZATION`
**Candidates:** Minimal Account / Minimal UTXO
**Manifest applies identically to both candidates:** YES
**State-model decision:** NOT MADE
**Account selected:** NO
**UTXO selected:** NO
**State-model ranking:** NONE
**Comparison scoring:** NOT STARTED
**Protocol adoption effect:** NONE
**Account mapping:** NOT YET CREATED
**UTXO mapping:** NOT YET CREATED
**Candidate evidence:** NOT YET GENERATED
**Prior focused verdict:** REVISION REQUIRED
**Corrections applied:** G1-AUTH-001-H-01 / G1-AUTH-001-H-02 / G1-AUTH-001-M-01
**Manifest content identity:** EXTERNAL SHA-256 AFTER BYTE FREEZE

---

## 1. Frozen Source Authority

This paired manifest instantiates the exact reviewed common case:

`docs/experiments/account-utxo/gate1/G1_AUTH_001_ORDINARY_NATIVE_DLTH_AUTHORIZATION_COMMON_CASE.md`

Common-case SHA-256:

`F52F3B81F47B49370F913F812829ACB1DAA8D1010E4E17AD7488B7E0BCC90AB3`

Common-case raw Git blob:

`27795f874ed13164eee6e56617c8a0c025c90682`

Focused re-review record:

`docs/experiments/account-utxo/gate1/G1_AUTH_001_FOCUSED_REREVIEW.md`

Review-record SHA-256:

`7DC6F8E0B57903C48A3B1FB4F07BDB2ED472257D3B2C476335283272252872B8`

Review-record raw Git blob:

`9fd47160d374770e8f743f1080ed0b0e4cef4da5`

The focused common-case re-review verdict is:

**PASS**

Paired-manifest creation is authorized.

Account and UTXO mappings remain unauthorized until this exact manifest
revision passes its own focused review.

---

## 2. Manifest Purpose

This artifact freezes one concrete external semantic instance of the reviewed
G1-AUTH-001 common case.

It supplies the same content-identified external problem to:

- Minimal Account; and
- Minimal UTXO.

The candidates may realize this problem differently internally.

They may not change the external problem.

This manifest is one experimental case only.

It does not assert that every native-DLTH effect must have the shape used in
this case.

After this manifest passes focused review, its bytes are intended to remain
immutable.

Later mapping results must not be written back into this manifest.

---

## 3. Case-Local Semantic Labels

This manifest uses the following opaque experimental labels.

### 3.1 `X`

`X` is a case-local external native-DLTH control/value scope.

`X` is not defined to be:

- an Account;
- an address;
- a stable protocol identity;
- a UTXO;
- an input;
- an output;
- a state key;
- a database record; or
- one candidate-native object.

### 3.2 `Y`

`Y` is a second case-local external native-DLTH control/value scope.

`Y` is distinct from `X` for this experimental case.

`Y` is not defined to be:

- an Account;
- an address;
- a stable protocol identity;
- a UTXO;
- an input;
- an output;
- a state key;
- a database record; or
- one candidate-native object.

### 3.3 `u`

`u` denotes one abstract native-DLTH semantic test unit.

It exists only to make the experimental value relation exact.

`u` does not select:

- a production denomination;
- decimals;
- a smallest protocol unit;
- serialization width;
- supply magnitude; or
- fee denomination.

### 3.4 `q`

For this paired case:

`q = 1u`

This value is an experimental case parameter only.

It is not a protocol constant.

---

## 4. Frozen Common External Contract

The exact common contract for this paired case is:

`C1-PAIR-001 = (P0-001, V-001, E1-001, A1-001, P1-001, M1-001)`

Every component above is supplied identically to both candidate mappings.

No candidate may alter one component without creating a different
content-identified paired case.

Candidate inability to satisfy a component does not modify the component.

It must remain visible as a mapping limitation, unsupported mapping, or failure
to satisfy this exact case.

---

## 5. `P0-001` — Shared External Precondition

The shared representation-neutral external precondition is:

1. `X` and `Y` are distinct case-local external semantic scopes.
2. `X` is associated with `2u` of the native-DLTH control/value facts relevant
   to this case.
3. `Y` is associated with `1u` of the native-DLTH control/value facts relevant
   to this case.
4. Therefore the affected case-local value total before `E1-001` is `3u`.
5. Exactly one external effect, `E1-001`, is proposed.
6. Exactly one independent required consensus authority relation, `A1-001`,
   applies to authorization of that effect.
7. The supplied positive-baseline authorization evidence satisfies `A1-001`
   for exactly `E1-001`.
8. No privileged administrator, founder, foundation, emergency key, recovery
   master key, or equivalent authority participates.
9. No alternate authority path participates.
10. The shared lifecycle/admissibility profile declares the affected value
    presently usable for this positive case.
11. The shared replay/history profile declares this case presently admissible
    and not a test of duplicate, stale, reverted, or cross-domain presentation.
12. The shared conflict/order profile contains no competing external effect
    whose ordering would change the intended positive authorization result.
13. The shared supply/conservation profile permits no issuance or destruction
    of native-DLTH as part of `E1-001`.
14. Consensus-visible fee/resource-economic side effects are governed only by
    the explicitly frozen economic-side-effect disposition in Section 15.
15. Every external semantic fact outside `V-001` and outside that explicitly
    excluded economic projection is declared unchanged.
16. Account and UTXO receive this exact same precondition.

`P0-001` is an external semantic predicate.

It is not candidate state.

---

## 6. `V-001` — Affected External Native-DLTH Facts

For this manifest, `V-001` contains only the external native-DLTH
control/value facts necessary to express the following case-local relation:

- before `E1-001`, `X` is associated with `2u`;
- before `E1-001`, `Y` is associated with `1u`;
- the affected pre-effect total is `3u`.

`V-001` does not specify how either candidate internally represents those
facts.

Minimal Account may require one or more candidate-native facts.

Minimal UTXO may require one or more candidate-native facts.

Those differences are candidate outputs.

Neither candidate receives credit merely for using fewer candidate-native
objects, records, accesses, credentials, signatures, proofs, verification
operations, or other native artifacts.

---

## 7. `E1-001` — Shared External Effect

`E1-001` is the following candidate-independent external semantic effect:

- reduce the native-DLTH value/control quantity associated with `X` by `q`;
- increase the native-DLTH value/control quantity associated with `Y` by the
  same `q`;
- create no issuance;
- create no burn;
- preserve every external semantic fact declared unchanged by this manifest.

Because `q = 1u`, the required external effect is:

- `X`: `2u -> 1u`;
- `Y`: `1u -> 2u`;
- affected total: `3u -> 3u`.

The separate economic-side-effect disposition in Section 15 is not part of
`V-001` or the success projection of `E1-001`.

No claim that such excluded economic effects are zero or absent follows from
this section.

This is an experimental external relation.

It does not select:

- sender or recipient protocol roles;
- debit/credit storage;
- balance fields;
- input/output grammar;
- transaction shape;
- source record count;
- destination record count;
- record consumption;
- replacement-object creation;
- transaction identity; or
- effect-identifier encoding.

---

## 8. `A1-001` — Shared External Authority Relation

For this paired case:

`N = 1 independent required consensus authority relation`

`A1-001` is the one external authority relation required to authorize the
declared reduction of `X` by `q` as part of exactly the indivisible external
effect `E1-001`.

For this case, the `Y`-side consequence of `E1-001` introduces no second
independent required external authority relation.

That statement is an external case property.

It does not state how either candidate internally represents, verifies, groups,
or derives authorization.

`A1-001` authorizes no unrelated effect merely by satisfying this case.

`A1-001` does not mean:

- one human;
- one owner identity;
- one Account;
- one UTXO;
- one input;
- one credential;
- one public key;
- one private key;
- one signature;
- one proof;
- one authorization artifact;
- one cryptographic verification;
- one state access;
- one address; or
- one state record.

Each candidate must later explain how its architecture realizes this same
external authority relation.

That realization is a candidate output.

---

## 9. Shared Authorization / Cryptographic Profile

The authorization profile for this positive baseline is:

`G1-AUTH-ABSTRACT-VALID-001`

This profile freezes only external properties:

1. authorization evidence is present;
2. the candidate must demonstrate that the evidence satisfies its declared
   realization of the same external `A1-001`;
3. the evidence is scoped to exactly the same `E1-001`;
4. the evidence is current for the shared profile used by this case;
5. no privileged authority path participates;
6. no alternate recovery or emergency authority path participates.

A candidate may not obtain success merely by declaring its evidence valid.

The mapping must expose the candidate-specific authorization realization and
the evidence needed to substantiate that realization.

This profile does not select:

- a cryptographic primitive;
- a PQ primitive;
- key format;
- credential format;
- signature format;
- proof format;
- artifact count;
- credential count;
- verification count;
- batching;
- aggregation;
- reuse;
- cache behavior; or
- byte size.

Those remain later candidate/profile outputs.

Success in this manifest gives no evidence about malformed, missing,
cryptographically invalid, stale, mismatched, or adversarial authorization
evidence.

---

## 10. Shared Domain and Protocol-Version Profile

The shared experimental profile identifier is:

`G1-AUTH-PAIR-DOMAIN-VERSION-001`

For this case, the property-level meaning of this profile is:

- both candidates interpret `E1-001` under the same external protocol-version
  context;
- both candidates interpret authorization under the same external domain
  context;
- the positive evidence is assumed to belong to that same context; and
- no domain/version mismatch is under test.

This is a comparison-profile label only.

It is not:

- a production `ChainId`;
- a production `NetworkId`;
- a domain-tag encoding;
- a signed-message layout;
- a transaction version number; or
- a consensus version selector.

No candidate may choose a different external domain/version meaning to improve
its mapping.

---

## 11. Shared Lifecycle / Admissibility Profile

The shared lifecycle profile is:

`G1-AUTH-LIFECYCLE-CURRENT-001`

For this positive baseline:

- the affected native-DLTH value/control facts are presently admissible;
- the required external authority relation is presently applicable;
- no lifecycle deletion is in progress;
- no credential migration is being tested;
- no cryptographic migration is being tested;
- no dormant-value recovery is being tested;
- no deprecated authorization mode is being tested.

This profile exists only to isolate ordinary authorization.

It selects no lifecycle mechanism.

---

## 12. Shared Replay / History Profile

The shared replay/history profile is:

`G1-AUTH-HISTORY-FRESH-001`

For this paired case:

- the proposed positive effect is treated as currently admissible;
- the positive authorization evidence is treated as current for this
  case-local history disposition;
- no duplicate presentation is being tested;
- no stale authorization is being tested;
- no reorganization is being tested;
- no reverted-history reapplication is being tested;
- no cross-network or cross-domain replay is being tested.

These cases remain deferred.

This profile selects no replay-prevention mechanism.

It selects no nonce, sequence counter, one-use encoding, transaction identity,
or historical proof mechanism.

---

## 13. Shared Conflict / Ordering Profile

The shared conflict/order profile is:

`G1-AUTH-ISOLATED-001`

For this paired case:

- exactly one external semantic effect is evaluated;
- there is no second competing external effect whose relative order changes the
  required authorization result;
- scheduling, parallel execution, fork choice, and finality are outside this
  case.

This profile selects no conflict key, scheduler, execution model, consensus
algorithm, fork-choice rule, or finality rule.

---

## 14. Shared Supply / Conservation Profile

The shared supply/conservation requirement is:

- affected value before: `3u`;
- affected value after successful `E1-001`: `3u`;
- native-DLTH issuance caused by `E1-001`: `0u`;
- native-DLTH burn caused by `E1-001`: `0u`.

This is a semantic test relation.

It does not define production supply size, denomination, accounting
representation, fee mechanism, fee payer, producer compensation, storage
charge, or resource pricing.

---

## 15. Shared Economic-Side-Effect Disposition

The shared property-level economic disposition is:

`G1-AUTH-ECON-EXCLUDED-001`

This paired case is an authorization-semantic baseline.

It does not evaluate or compare consensus-visible fee/resource-economic side
effects.

Accordingly:

1. no claim is made that consensus-visible fees or other economic side effects
   are zero;
2. no claim is made that such effects are absent;
3. no claim is made that such effects are unchanged;
4. such effects are excluded from the `V-001` and `E1-001` success projection;
5. such effects are excluded from the "declared unchanged external facts"
   requirement only to the extent that they do not alter a frozen element of
   `C1-PAIR-001`;
6. each candidate mapping must disclose whether its realization requires any
   consensus-visible economic side effect;
7. each candidate mapping must disclose whether such an effect changes `X`,
   `Y`, the `3u -> 3u` relation, `A1-001`, `P1-001`, or another frozen
   component of this manifest;
8. no candidate receives Gate-1 success credit merely from an economic side
   effect; and
9. no economic-side-effect difference may be used for comparison scoring in
   this case.

If a candidate's required economic side effect changes any frozen component of
`C1-PAIR-001`, that candidate may not silently reinterpret this manifest.

Its inability to realize this exact case must remain visible.

A materially different economic-bearing case requires a different
content-identified paired manifest.

This disposition selects no:

- fee mechanism;
- fee payer;
- fee recipient;
- producer compensation rule;
- burn rule;
- storage charge;
- rent mechanism;
- refund mechanism;
- pricing function;
- resource meter; or
- numeric economic parameter.

---

## 16. `P1-001` — Required Shared External Postcondition

A candidate mapping may report structural success for this paired case only if
its successful realization produces the following same external postcondition:

1. `X` is associated with `1u`;
2. `Y` is associated with `2u`;
3. the affected value total remains `3u`;
4. `E1-001` caused no issuance;
5. `E1-001` caused no burn;
6. every external semantic fact declared unchanged by this manifest remains
   unchanged;
7. the supplied authorization evidence satisfies the candidate realization of
   the same external `A1-001`;
8. that authorization applies to exactly `E1-001`;
9. satisfying `A1-001` for `E1-001` does not thereby authorize an unrelated
   external effect;
10. no privileged or alternate authority path contributes to success.

Consensus-visible economic side effects governed by
`G1-AUTH-ECON-EXCLUDED-001` are outside this success projection unless they
alter a frozen component above.

The candidate may reach `P1-001` using different internal state semantics.

It may not substitute another external postcondition.

---

## 17. `M1-001` — Additional Frozen Material Parameters

`M1-001` consists of the following content-identified case parameters:

- case alias: `G1-AUTH-001-PAIR-001`;
- semantic test unit: `u`;
- effect magnitude: `q = 1u`;
- external scope labels: `X`, `Y`;
- `X` pre-effect quantity: `2u`;
- `Y` pre-effect quantity: `1u`;
- `X` post-effect quantity: `1u`;
- `Y` post-effect quantity: `2u`;
- affected total before: `3u`;
- affected total after: `3u`;
- independent required authority relations: `N = 1`;
- authority relation: `A1-001`;
- authorization profile: `G1-AUTH-ABSTRACT-VALID-001`;
- domain/version profile: `G1-AUTH-PAIR-DOMAIN-VERSION-001`;
- lifecycle profile: `G1-AUTH-LIFECYCLE-CURRENT-001`;
- replay/history profile: `G1-AUTH-HISTORY-FRESH-001`;
- conflict/order profile: `G1-AUTH-ISOLATED-001`;
- supply/conservation disposition: `3u -> 3u`, issuance `0u`, burn `0u`;
- economic-side-effect disposition: `G1-AUTH-ECON-EXCLUDED-001`;
- unrelated external facts: unchanged except for the explicitly excluded
  economic projection described in Section 15;
- negative authorization cases: deferred;
- candidate evidence scoring: disabled;
- state-model ranking: none.

Every field above is identical for both candidate mappings.

A material change creates a different paired case.

No candidate mapping may rewrite `M1-001`.

---

## 18. Declared Unchanged External Facts

For this case, every external semantic fact outside:

- the declared `V-001` effect projection; and
- the explicitly excluded economic-side-effect projection defined by
  `G1-AUTH-ECON-EXCLUDED-001`

must remain unchanged.

This requirement is frozen before candidate mapping begins.

A candidate may not weaken, remove, reinterpret, or retroactively narrow this
requirement because its architecture cannot realize it.

If a candidate cannot preserve the required unchanged facts, that result must
remain visible as a limitation or failure to satisfy this exact paired case.

Changing the requirement creates a new content-identified paired case.

A candidate may not silently widen `E1-001`.

Candidate-internal housekeeping that has no independently meaningful external
semantic effect is not automatically a violation.

Each mapping must disclose which candidate-specific facts are treated as
internal housekeeping and why they do not alter the frozen external projection.

---

## 19. Deferred Negative and Extended Scenarios

This manifest does not provide evidence for:

- missing authorization evidence;
- malformed authorization evidence;
- cryptographically invalid evidence;
- evidence for another authority relation;
- evidence for another effect;
- domain mismatch;
- cross-network replay;
- duplicate presentation;
- stale authorization;
- changed authorization after lifecycle transition;
- reorganization;
- reverted-history reapplication;
- credential rotation;
- crypto migration;
- dormant-value migration;
- catastrophic cryptographic failure;
- delegation;
- recovery;
- multisignature;
- multiple independent required authorities;
- multiple concurrent effects;
- economic-profile comparison;
- fee-model comparison;
- resource-performance superiority; or
- hostile invalid-candidate work.

Success in `G1-AUTH-001-PAIR-001` cannot be reused as proof that any deferred
scenario passes.

---

## 20. Mapping Identity Binding Policy

This manifest contains no mutable mapping-identity slots.

After this manifest passes focused review, its bytes, SHA-256, and raw Git blob
must remain unchanged for evidence produced under this manifest identity.

Minimal Account and Minimal UTXO mapping artifacts, when later authorized, must
each reference:

- this manifest path;
- this exact manifest SHA-256; and
- this exact manifest raw Git blob.

The mapping artifacts must not be written back into this manifest.

After candidate mappings exist, their artifact identities must be recorded in a
separate binding, index, or evidence-layer artifact that references:

- this frozen manifest identity;
- the Account mapping identity; and
- the UTXO mapping identity.

That later binding artifact is process/evidence provenance only.

It is not a protocol identity mechanism.

The eventual mapping-identity requirement is therefore satisfied without
mutating this reviewed manifest and without creating a circular identity
dependency.

No mapping identity is available yet.

Account mapping:

**NOT YET CREATED**

UTXO mapping:

**NOT YET CREATED**

---

## 21. Candidate Output Requirements

When mapping is later authorized, both candidates must report equivalent
classes of output, including:

- interpretation of `P0-001`;
- realization of `V-001`;
- realization of `E1-001`;
- realization of `A1-001`;
- authorization evidence placement;
- realization of `P1-001`;
- logical authorization dependencies;
- current-state dependencies;
- authorization-related persistent facts;
- authorization-evidence reuse assumptions;
- credential reuse assumptions;
- authorization-condition reuse assumptions;
- verification-result reuse assumptions;
- implementation-local cache assumptions;
- candidate-native cardinalities where descriptive;
- consensus-visible economic-side-effect disclosure under
  `G1-AUTH-ECON-EXCLUDED-001`;
- whether any disclosed economic effect touches a frozen component of
  `C1-PAIR-001`;
- unresolved requirements;
- blocked claims; and
- evidence required to substantiate the mapping.

Candidate-native cardinalities are not automatically neutral comparison
metrics.

Economic-side-effect differences are not comparison metrics in this case.

---

## 22. Symmetry Rule

Account and UTXO must receive:

- the same exact manifest bytes;
- the same `P0-001`;
- the same `V-001`;
- the same `E1-001`;
- the same `A1-001`;
- the same `P1-001`;
- the same `M1-001`;
- the same authorization property profile;
- the same domain/version profile;
- the same lifecycle profile;
- the same history profile;
- the same conflict/order profile;
- the same conservation requirement;
- the same economic-side-effect disposition;
- the same unchanged-fact requirement; and
- the same deferred-scenario boundary.

Candidate-specific internal differences are outputs.

They may not be copied back into the shared manifest.

Candidate inability to satisfy a frozen field is evidence about the candidate,
not permission to change that field.

---

## 23. Explicit Non-Selections

This manifest selects no:

- Account design;
- UTXO design;
- hybrid design;
- production ownership representation;
- stable logical identity;
- production owner identity;
- address role;
- credential format;
- key format;
- signature format;
- proof format;
- cryptographic primitive;
- PQ primitive;
- authorization descriptor;
- authorization grouping;
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
- replay mechanism;
- state schema;
- state key;
- state commitment;
- proof system;
- snapshot trust model;
- light-client protocol;
- resource meter;
- gas;
- fee mechanism;
- fee payer;
- fee recipient;
- producer compensation rule;
- storage rent;
- storage charge;
- refund mechanism;
- numeric resource limit;
- consensus algorithm;
- fork-choice rule;
- finality rule;
- scheduler;
- parallel-execution mechanism; or
- state-model ranking.

The case-local labels `X`, `Y`, `u`, and `q` are experimental semantic
parameters only.

They are not selections of any item above.

---

## 24. Evidence and Process Status

Common case:

**REVIEWED — PASS**

Focused common-case review:

**PASS**

Paired manifest:

**REVISION 1 — NOT YET FOCUSED RE-REVIEWED**

G1-AUTH-001-H-01:

**CORRECTION APPLIED — RE-REVIEW PENDING**

G1-AUTH-001-H-02:

**CORRECTION APPLIED — RE-REVIEW PENDING**

G1-AUTH-001-M-01:

**CORRECTION APPLIED — RE-REVIEW PENDING**

Account mapping:

**NOT CREATED**

UTXO mapping:

**NOT CREATED**

Candidate evidence generated:

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

## 25. Next Step

The next step is:

> focused read-only re-review of this exact Revision-1 paired manifest.

That review should verify only:

- G1-AUTH-001-H-01 is fixed;
- G1-AUTH-001-H-02 is fixed;
- G1-AUTH-001-M-01 is fixed;
- no regression was introduced into previously passing X/Y, value-granularity,
  E1, A1, authorization, domain/version, lifecycle/history/conflict,
  supply/conservation, P1, candidate symmetry, metric-neutrality, or
  non-selection properties.

Only after that exact Revision-1 manifest passes focused re-review may it be
frozen and candidate mapping creation be authorized.

**STATE MODEL DECISION: NOT MADE**