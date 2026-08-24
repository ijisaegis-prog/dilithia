# Account–UTXO Pilot Candidate Mapping: Value-Effect Baseline — Account Reference

> **NON-NORMATIVE EXPERIMENTAL CANDIDATE MAPPING**
>
> **Status: FROZEN PILOT CANDIDATE MAPPING — MAPPED**
>
> **Candidate family: Account**
>
> **Mapping maturity: REFERENCE**
>
> This document defines one Account-family reference mapping for the frozen
> `value-effect-baseline` common semantic case under the frozen paired comparison
> manifest identified below.
>
> It is not a protocol specification, transaction definition, state format,
> authorization mechanism, replay mechanism, cryptographic profile, resource
> architecture, benchmark result, or state-model selection.
>
> If this document conflicts with the Dilithia Constitution, Formal
> Specification, or ratified HIP / Super HIP material, the authoritative protocol
> material prevails.
>
> Within the non-normative Account/UTXO comparison methodology, this mapping is
> subordinate to:
>
> - `ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`;
> - `ACCOUNT_UTXO_WORKLOAD_MODEL.md`;
> - `ACCOUNT_UTXO_CANDIDATE_MAPPING_REQUIREMENTS.md`;
> - frozen `VALUE_EFFECT_BASELINE_CASE.md`; and
> - frozen `VALUE_EFFECT_BASELINE_PAIRED_MANIFEST.md`.

## 1. Purpose

This file defines one deliberately narrow Account-family **reference mapping**.

Its purpose is to demonstrate, without selecting future Dilithia protocol
semantics, one defensible way for an Account-family analytical candidate to
realize the frozen external semantic effect:

```text
before:
    term   = pre_value_0
    amount = 1

E0

after:
    term   = post_value_0
    amount = 1
```

The mapping must ground the required external projection change in an actual
candidate-specific logical-state change.

It must not satisfy the case through evidence-only relabeling.

This mapping is intended to validate the candidate-mapping methodology.

It is not intended to establish an optimized Account design or an Account-family
performance conclusion.

## 2. Mapping Record Summary

```text
mapping_format_version:
    pilot-account-utxo-mapping/v1

mapping_alias:
    pilot/value-effect-baseline/account/reference/v1

mapping_content_hash:
    NOT EMBEDDED — EXTERNAL EVIDENCE-ONLY CONTENT IDENTITY

candidate_family:
    Account

candidate_variant:
    persistent-logical-value-relations/two-relation-reference/v1

semantic_case_hash:
    sha256:1DB6FBE63C13BFB619BFECE7D61D0A9958F00E2BF8866331A4602BA0240B276D

paired_manifest_hash:
    sha256:219B286FD6D4EFB7ECE6F18B5872A34C22F6209E8CE45FA8BE96DD9AB1081D83

mapping_maturity:
    REFERENCE

optimization_configuration:
    REFERENCE / NO RESULT-DRIVEN PERFORMANCE TUNING

preparation_configuration:
    EXPLICIT INITIAL LOGICAL-STATE SETUP ONLY

experimental_schema_binding_if_any:
    NONE — LOGICAL MAPPING ONLY

mapping_status:
    MAPPED

author_or_generator_provenance:
    AI-ASSISTED MANUAL ARTIFACT DRAFTING; NO DETERMINISTIC MAPPING GENERATOR
```

The identifiers above are experimental evidence identifiers only.

They are not protocol identities, state keys, addresses, transaction
identifiers, replay identifiers, consensus commitments, or ownership
identifiers.

The mapping content hash is intentionally not embedded in this frozen artifact.

The exact frozen file receives an external evidence-only SHA-256 identity after
the installed file is verified byte-for-byte.

That hash provides experimental reproducibility only and creates no protocol
identity or authority.

## 3. Frozen Semantic Case Binding

This mapping binds exactly:

```text
semantic case alias:
    pilot/value-effect-baseline/v1

semantic case SHA-256:
    1DB6FBE63C13BFB619BFECE7D61D0A9958F00E2BF8866331A4602BA0240B276D
```

The mapping must not change the frozen case.

In particular, it must preserve:

- instantiated semantic amount `A = 1`;
- `pre_value_0` as the pre-effect external semantic term;
- `post_value_0` as the required post-effect external semantic term;
- preservation of amount `1`;
- the required non-identical external semantic projection;
- the empty intrinsic external conflict relation;
- the absence of a selected authorization mechanism;
- the absence of selected replay or one-use semantics;
- the absence of selected cryptographic behavior;
- the absence of a selected state commitment;
- the absence of selected resource-accounting semantics; and
- unspecified protocol validity.

A different semantic case hash is not this mapping.

## 4. Frozen Paired Manifest Binding

This mapping binds exactly:

```text
paired manifest alias:
    pilot/value-effect-baseline/paired-manifest/v1

paired manifest SHA-256:
    219B286FD6D4EFB7ECE6F18B5872A34C22F6209E8CE45FA8BE96DD9AB1081D83
```

The manifest binds this mapping to:

```text
A                                  = 1
ordering profile                   = NO_ORDER_ASSERTION
mapping maturity                   = REFERENCE
optimization rule                  = NO RESULT-DRIVEN PERFORMANCE TUNING
authorization architecture         = NOT SELECTED
replay architecture                = BLOCKED / NOT SELECTED
one-use architecture               = BLOCKED / NOT SELECTED
reorganization architecture        = BLOCKED / NOT SELECTED
migration architecture             = OUT OF SCOPE
commitment architecture            = OUT OF SCOPE
resource architecture              = OUT OF SCOPE FOR EVIDENCE
consensus mechanism                = OUT OF SCOPE
protocol validity                  = UNSPECIFIED
candidate ranking                  = NONE
supply-changing behavior           = OUT OF SCOPE
```

This mapping cannot override those selections or exclusions.

## 5. Account-Family Analytical Hypothesis

For this mapping only, the Account-family analytical hypothesis is:

> Native value is accounted for by changing quantities or availability in
> persistent logical value relation(s).

This is a non-normative analytical distinction.

It does not define:

- ownership;
- address semantics;
- credential placement;
- authorization count;
- signatures;
- proofs;
- replay exclusion;
- nonce behavior;
- one-use semantics;
- record consumption;
- transaction inputs;
- transaction outputs;
- stable identity;
- migration;
- recovery;
- cryptographic algorithm;
- state commitment;
- physical storage; or
- parallel execution.

The word **Account** in this artifact means only that this candidate mapping
represents the value effect through persistent logical value relations whose
mapped quantities change across the experimental effect.

## 6. Mapping-Local Terminology

This mapping introduces two opaque mapping-local logical relation handles:

```text
AR_PRE
AR_POST
```

They are names used only inside this candidate mapping.

They are not:

- protocol Account identifiers;
- user accounts;
- addresses;
- owners;
- state keys;
- credentials;
- transaction fields;
- replay identities;
- persistent protocol object identities; or
- proof identifiers.

The handles allow the mapping to describe two persistent logical value
relations without selecting a future Dilithia state schema.

## 7. Persistent Logical Value Relation Model

The reference mapping uses two persistent logical value relations.

Conceptually, the mapping-level state exposes a function:

```text
quantity(relation) -> non-negative case-local experimental amount
```

for the two mapping-local relations:

```text
quantity(AR_PRE)
quantity(AR_POST)
```

This function is a logical mapping construct only.

It does not define:

- a Rust type;
- a balance width;
- a map implementation;
- a database key;
- a serialized field;
- a consensus state object;
- zero-balance retention;
- pruning;
- deletion;
- account recreation; or
- physical persistence layout.

The two logical relations remain the same mapping-local relations across the
pre-effect and post-effect mapped states.

The mapping changes their quantities.

It does not replace them with new record identities.

## 8. Instantiated Amount

The paired manifest fixes:

```text
A = 1
```

This mapping therefore uses:

```text
experimental amount = 1
```

The number `1` is one case-local experimental semantic amount unit.

It is not:

- one DLTH;
- one future DLTH base unit;
- one protocol denomination;
- one byte;
- one record;
- one Account object;
- one transaction input; or
- one transaction output.

No protocol numeric width is selected.

## 9. Pre-Effect Account Mapping State

The mapped pre-effect logical state is:

```text
quantity(AR_PRE)  = 1
quantity(AR_POST) = 0
```

Interpretation:

- the case-local experimental amount is represented through the persistent
  logical value relation `AR_PRE`; and
- `AR_POST` carries no positive case-local experimental amount in the pre-effect
  mapped state.

The value `0` is a mapping-level mathematical quantity.

It does not decide whether a future Account design stores, omits, prunes, or
otherwise represents zero-valued state.

No physical record count is implied.

No ownership or authorization meaning is attached to either relation.

## 10. Pre-Effect External Projection Derivation

For this mapping, the frozen pre-effect external projection is derived from the
candidate state by the following mapping rule:

```text
if quantity(AR_PRE) = 1
and quantity(AR_POST) = 0

then project:
    term   = pre_value_0
    amount = 1
```

This projection rule is part of the candidate mapping.

It is not a protocol rule.

The case-local term `pre_value_0` does not become an Account identifier.

The mapping demonstrates only how the Account candidate state realizes the
frozen common semantic projection.

## 11. Candidate-Specific Realization of E0

The Account reference realization of `E0` changes the two persistent logical
value-relation quantities as follows:

```text
AR_PRE:
    1 -> 0

AR_POST:
    0 -> 1
```

Equivalently, the mapped state vector changes from:

```text
( quantity(AR_PRE), quantity(AR_POST) ) = (1, 0)
```

to:

```text
( quantity(AR_PRE), quantity(AR_POST) ) = (0, 1)
```

This is the candidate-specific logical realization of the frozen semantic
effect.

The mapping does not define:

- a transaction;
- a transfer instruction;
- debit or credit byte encoding;
- transaction inputs or outputs;
- a nonce;
- authorization;
- replay protection;
- state-key encoding;
- execution scheduling;
- a database update algorithm; or
- consensus acceptance.

## 12. Post-Effect Account Mapping State

The required mapped post-effect logical state is:

```text
quantity(AR_PRE)  = 0
quantity(AR_POST) = 1
```

Interpretation:

- `AR_PRE` no longer carries a positive case-local experimental amount; and
- the case-local experimental amount is represented through `AR_POST`.

Again, the logical value `0` does not select a zero-state retention policy.

The mapping does not define creation, deletion, recreation, or ownership
lifecycle of protocol objects.

## 13. Post-Effect External Projection Derivation

For this mapping, the required post-effect external projection is derived by:

```text
if quantity(AR_PRE) = 0
and quantity(AR_POST) = 1

then project:
    term   = post_value_0
    amount = 1
```

The frozen external semantic projection therefore changes from:

```text
(pre_value_0, 1)
```

to:

```text
(post_value_0, 1)
```

while the case-local amount remains preserved.

## 14. No-Op and Relabeling Rejection

This mapping cannot satisfy `E0` by leaving the Account logical state unchanged.

The following candidate state:

```text
before:
    quantity(AR_PRE)  = 1
    quantity(AR_POST) = 0

after:
    quantity(AR_PRE)  = 1
    quantity(AR_POST) = 0
```

does not satisfy this mapping merely because an evidence record writes
`post_value_0` after the attempted effect.

Likewise, the following is not sufficient:

```text
same mapped state
+ new timestamp
+ later run position
+ changed evidence label
```

The required Account mapping state itself must change from `(1, 0)` to `(0, 1)`.

Therefore the post-effect projection is grounded in candidate-specific mapped
state rather than evidence-only relabeling.

## 15. Amount Preservation

The mapped pre-effect available amount is:

```text
1 + 0 = 1
```

The mapped post-effect available amount is:

```text
0 + 1 = 1
```

Therefore this mapping preserves the frozen case-local conservation relation:

```text
A_after = A_before = 1
```

This is a case-scoped experimental relation only.

It does not define universal DLTH conservation or any issuance, reward, fee,
burn, or supply policy.

## 16. Logical Relation Cardinality

This reference mapping uses:

```text
persistent logical value relations = 2
```

That cardinality is a property of this named Account reference mapping.

It is not:

- a common workload input;
- a protocol requirement;
- a statement that every Account design needs two records;
- a statement that one semantic term universally equals one Account; or
- evidence that Account is better or worse than UTXO.

A materially different defensible Account mapping may use a different internal
structure.

Such a mapping requires a distinct mapping identity if used for comparative
evidence.

## 17. Physical Record Cardinality Boundary

This logical mapping does **not** fix:

```text
physical Account record count
physical database row count
physical key count
physical page count
serialized object count
```

The two logical relations may or may not correspond one-to-one with physical
records in a later experimental implementation.

That decision belongs to a separately reviewed candidate schema or
implementation artifact.

No physical-cardinality evidence may be inferred from this document alone.

## 18. Logical State-Key Boundary

`AR_PRE` and `AR_POST` are mapping-local logical handles.

This mapping does not define a future Dilithia state-key schema.

Therefore:

```text
protocol state-key count = NOT SELECTED
state-key encoding        = NOT SELECTED
key derivation            = NOT SELECTED
address-to-key mapping    = NOT SELECTED
```

A later implementation may map these logical relations onto candidate-specific
state locations, but those locations become implementation or experimental
schema evidence.

## 19. Logical Read/Write Measurement Boundary

The semantic mapping requires the resulting Account logical state to reflect:

```text
AR_PRE:  1 -> 0
AR_POST: 0 -> 1
```

This establishes two logical relation-state changes in the mapping description.

It does **not** assign benchmark counters such as:

```text
logical_reads  = 2
logical_writes = 2
```

because read/write measurement requires a frozen experimental schema,
measurement definition, metadata placement, validation workflow, and applicable
commitment assumptions.

Accordingly:

```text
logical read count metric  = NOT YET ASSIGNED
logical write count metric = NOT YET ASSIGNED
```

Later evidence may record those values only under a reviewed metric definition.

## 20. Internal Dependency Shape

This reference mapping has one mapping-level dependency:

> The post-effect projection is satisfied only when the candidate state reflects
> the required quantity reassignment from `AR_PRE` to `AR_POST`.

No broader dependency graph is fixed.

This document does not define:

- locking;
- scheduler dependencies;
- speculative execution;
- parallel-validation dependencies;
- database transaction dependencies; or
- block-level dependency rules.

## 21. External Conflict Preservation

The frozen common semantic case has:

```text
intrinsic external conflict relation = EMPTY
```

This mapping preserves that relation unchanged.

It does not infer that the Account candidate has zero candidate-induced
contention.

Candidate-specific conflict footprint, state-key overlap, scheduling
restrictions, or conflict amplification remain later mapping/schema/evidence
outputs.

The Account label itself implies none of them.

## 22. Ordering Boundary

The frozen paired manifest uses:

```text
ordering profile = NO_ORDER_ASSERTION
```

This mapping introduces no externally meaningful ordering relation beyond the
logical pre-effect / `E0` / post-effect structure already required by the case.

It defines no:

- transaction order;
- block order;
- scheduler;
- serial execution mandate;
- parallel execution guarantee; or
- conflict-resolution protocol.

Candidate-internal implementation ordering remains unresolved.

## 23. Authorization Boundary

The frozen paired manifest selects no authorization architecture.

This Account mapping therefore defines no:

- owner;
- address;
- key;
- credential;
- signature;
- proof;
- authorization state;
- account owner field;
- multisignature;
- threshold rule;
- delegation;
- recovery; or
- stable identity.

Neither `AR_PRE` nor `AR_POST` is an owner or address.

This mapping must not be used as evidence that Account-family value changes need
zero, one, or any particular number of authorizations.

## 24. Replay and One-Use Boundary

The paired manifest keeps replay, nonce, one-use, reapplication, and
reorganization semantics blocked or unselected.

This mapping therefore introduces no:

- nonce;
- sequence number;
- one-use token;
- record-consumption identity;
- transaction-hash replay identity;
- duplicate-presentation rule;
- reapplication rule;
- rollback rule; or
- chain-reorganization rule.

The fact that `AR_PRE` changes from `1` to `0` is not a canonical replay rule.

The mapping does not state that the same experimental capability cannot be
exercised again.

## 25. Lifecycle Boundary

The two persistent logical relations exist across this mapping's before and
after states.

This statement is limited to the mapping's Account-family value-accounting
representation.

It does not select protocol semantics for:

- Account creation;
- Account deletion;
- Account recreation;
- tombstones;
- pruning;
- archival history;
- inactivity;
- address reuse; or
- persistent identity.

The mapping uses quantity changes, not record-consumption semantics.

## 26. Cryptographic Boundary

The paired manifest uses:

```text
algorithm profile = pilot/cryptography-not-exercised/v1
```

This mapping therefore performs no cryptographic comparison.

It selects no:

- signature algorithm;
- proof system;
- public-key format;
- post-quantum primitive;
- aggregation;
- batching;
- verification-operation count; or
- cryptographic byte format.

The mapping must not be interpreted as an Account design that lacks
authorization or cryptography in a future protocol.

Those dimensions are simply outside this pilot configuration.

## 27. Crypto Agility Boundary

This reference mapping is only a logical value-accounting mapping.

It does not embed a cryptographic algorithm into the identity of `AR_PRE`,
`AR_POST`, or the Account-family hypothesis.

It therefore makes no claim that a future protocol is cryptographically
complete.

Any future candidate design used for architecture conclusions must not
structurally foreclose authoritative cryptographic evolution.

The mechanism remains unresolved here.

## 28. Migration Boundary

The paired manifest uses:

```text
migration architecture = OUT OF SCOPE
```

This mapping selects no:

- account migration;
- credential migration;
- global migration;
- spend-time migration;
- re-keying;
- version registry;
- compatibility path;
- dormant-state migration; or
- recovery path.

The logical relations `AR_PRE` and `AR_POST` are not stable migration identities.

Migration-dependent conclusions are prohibited under this mapping.

## 29. State-Commitment Boundary

The paired manifest uses:

```text
commitment architecture = OUT OF SCOPE
```

This mapping defines no:

- Merkle tree;
- authenticated map;
- accumulator;
- state root;
- proof path;
- commitment key;
- commitment update rule; or
- commitment retention rule.

The logical value relations are commitment-independent mapping constructs.

Commitment-dependent costs cannot be attributed to this mapping without a new
explicit experimental configuration.

## 30. Resource-Architecture Boundary

The paired manifest uses:

```text
resource architecture = OUT OF SCOPE FOR EVIDENCE
```

This mapping selects no:

- gas;
- scalar resource score;
- candidate-attempt meter;
- no-refund accounting;
- reservation;
- fee;
- transaction limit;
- block limit;
- state-growth limit; or
- ingress-abuse mechanism.

The state change described by this mapping is semantic structure, not a resource
measurement.

No resource advantage or disadvantage is claimed.

## 31. Failure-Atomicity Boundary

This pilot is not the dedicated `failure-atomic-external-projection` workload
family.

This mapping describes only the state required for successful satisfaction of
the frozen `value-effect-baseline` contract.

It does not select:

- rollback implementation;
- journal;
- database transaction;
- attempted-work accounting;
- refund behavior; or
- failure-state persistence.

A separate failure-atomic case and manifest are required before those properties
are compared.

## 32. Hostile-Validation Boundary

This mapping defines no hostile or malformed input grammar.

It does not define:

- adversarial candidate population;
- malformed byte input;
- late-failure stage;
- hostile authorization evidence;
- candidate validation budget; or
- denial-of-service benchmark.

The absence of hostile-validation evidence here is not evidence that such work
may be unbounded in a future protocol.

## 33. Arithmetic Boundary

The mapping uses only the case-local mathematical quantities:

```text
0
1
```

to describe the reference state relation.

No protocol balance width is selected.

No `u64`, `u128`, `u256`, signed integer type, floating-point type, or storage
representation is selected.

If an implementation later performs consensus-relevant arithmetic for this
mapping, that experimental implementation must use deterministic,
host-independent behavior with explicit overflow and underflow semantics.

Existing Dilithia serialization widths do not choose the future Account balance
representation.

## 34. Monetary and Supply Boundary

The only monetary relation bound here is the frozen case-local assumption:

```text
available amount before = 1
available amount after  = 1
```

This mapping defines no:

- issuance;
- reward;
- burn;
- fee;
- supply cap;
- inflation;
- deflation; or
- monetary schedule.

No universal DLTH conservation rule is created.

## 35. Exact-Byte and Serialization Boundary

This mapping is logical only.

Experimental schema binding:

```text
NONE
```

Therefore it defines no:

- transaction bytes;
- Account record bytes;
- state encoding;
- field order;
- canonical serialization;
- malformed-byte grammar;
- schema identifier; or
- exact-byte count.

Byte evidence is unavailable under this mapping until a separately reviewed
experimental schema is bound.

## 36. Physical Persistence Boundary

This mapping selects no physical database representation.

It defines no:

- database engine;
- table layout;
- page layout;
- index;
- cache;
- snapshot;
- compaction policy;
- history retention;
- pruning policy; or
- storage backend.

`AR_PRE` and `AR_POST` are logical relations only.

Physical persistence effects remain implementation evidence.

## 37. Preparation Configuration

Preparation configuration:

```text
EXPLICIT INITIAL LOGICAL-STATE SETUP ONLY
```

Required preparation is limited to defining the mapping-local initial state:

```text
quantity(AR_PRE)  = 1
quantity(AR_POST) = 0
```

and binding this mapping to the exact frozen case and manifest hashes.

No hidden preparation is assumed.

In particular, this reference mapping does not assume unreported:

- cache warming;
- indexing;
- preprocessing;
- metadata construction;
- proof preparation;
- state compaction;
- record consolidation; or
- database preparation.

If a later implementation uses such work, it must be disclosed before
comparative results are interpreted.

## 38. Optimization Configuration

Optimization configuration:

```text
REFERENCE / NO RESULT-DRIVEN PERFORMANCE TUNING
```

This mapping does not claim to be optimized.

It also must not contain an intentional artificial disadvantage.

No optional performance result has been observed while selecting this mapping.

This frozen reference mapping applies no special:

- batching;
- caching;
- indexing;
- deduplication;
- compact encoding;
- proof reuse;
- scheduler optimization;
- preprocessing optimization; or
- physical database optimization.

The two-relation structure is used to provide a direct candidate-native logical
grounding for the two externally distinguished semantic terms without adding a
separate non-value semantic-association mechanism.

A future optimized Account mapping requires a distinct mapping identity or a
new pre-result-frozen configuration as required by the methodology.

## 39. Anti-Strawman Statement

This reference mapping is not intended to make the Account candidate
artificially weak.

The mapping does not intentionally add:

- redundant logical relations beyond the two used by this reference mapping;
- redundant value movement;
- redundant serialization;
- duplicated authorization evidence;
- artificial cryptographic work;
- artificial conflict edges; or
- deliberately inefficient validation stages.

No claim is made that this is the only defensible Account mapping.

Any later conclusion from this mapping remains specific to this mapping and the
frozen manifest.

## 40. Alternative Defensible Account Mappings

Materially different Account-family mappings may be defensible.

Potential alternatives may differ in how persistent logical value relations are
organized, grouped, or represented while preserving the same frozen external
semantic contract.

This mapping does not pre-approve any particular alternative.

It also does not assume that an alternative with a different logical relation
count, state layout, or indirection strategy is better or worse.

If an alternative is used for evidence, it must receive:

- a distinct mapping identity;
- the same frozen semantic case;
- a compatible frozen paired manifest;
- explicit optimization and preparation configuration;
- independent review; and
- mapping-qualified reporting.

This first pilot does not support a broad Account-family conclusion from one
reference mapping.

## 41. Candidate-Specific Structural Outputs

This mapping currently discloses the following structural outputs:

```text
candidate family:
    Account

mapping variant:
    two persistent logical value relations

logical value relation count:
    2

pre-effect positive-quantity relations:
    1

post-effect positive-quantity relations:
    1

mapped quantity vector before:
    (1, 0)

mapped quantity vector after:
    (0, 1)

physical record count:
    UNRESOLVED / NOT FIXED BY LOGICAL MAPPING

protocol state-key count:
    UNRESOLVED / NOT FIXED BY LOGICAL MAPPING

logical read metric:
    NOT ASSIGNED

logical write metric:
    NOT ASSIGNED

exact bytes:
    UNAVAILABLE — NO EXPERIMENTAL SCHEMA

cryptographic work:
    NOT EXERCISED

resource evidence:
    NOT COLLECTED

physical timing:
    NOT COLLECTED
```

These values are not automatically comparable to candidate-native counters from
a UTXO mapping.

## 42. External Projection Checklist

Pre-effect mapping:

```text
Account candidate state:
    quantity(AR_PRE)  = 1
    quantity(AR_POST) = 0

projects to:
    term   = pre_value_0
    amount = 1
```

Required post-effect mapping:

```text
Account candidate state:
    quantity(AR_PRE)  = 0
    quantity(AR_POST) = 1

projects to:
    term   = post_value_0
    amount = 1
```

No-op rejection:

```text
(1, 0) -> (1, 0)
```

does not satisfy this mapping.

Evidence-only relabeling rejection:

```text
unchanged candidate state
+ changed semantic label
```

does not satisfy this mapping.

Amount preservation:

```text
1 -> 1
```

is satisfied.

## 43. Manifest-Compatibility Checklist

```text
frozen case hash matches:
YES

frozen paired manifest hash matches:
YES

A = 1:
YES

same external semantic contract preserved:
YES

required projection change preserved:
YES

intrinsic external conflict relation preserved:
YES — EMPTY

ordering profile preserved:
YES — NO_ORDER_ASSERTION

authorization mechanism introduced:
NO

replay mechanism introduced:
NO

one-use semantics introduced:
NO

cryptographic algorithm introduced:
NO

migration mechanism introduced:
NO

state commitment introduced:
NO

resource accounting introduced:
NO

supply-changing behavior introduced:
NO

protocol validity claimed:
NO

candidate ranking claimed:
NO
```

## 44. Unresolved Dependencies

The following remain unresolved for this mapping:

- future Account protocol state schema;
- physical record representation;
- state-key representation;
- zero-quantity storage behavior;
- transaction format;
- transaction validity;
- ownership semantics;
- authorization architecture;
- replay protection;
- nonce semantics;
- one-use semantics;
- reorganization behavior;
- cryptographic algorithm;
- Crypto Agility mechanism;
- migration mechanism;
- state commitment;
- resource architecture;
- numeric resource limits;
- fees;
- monetary policy beyond the case-local experimental assumption;
- exact-byte experimental schema;
- implementation language representation;
- database representation;
- benchmark measurement definitions;
- evidence methodology;
- alternative defensible Account mapping set.

None receives an implicit default from this artifact.

## 45. Mapping Status

Current mapping status:

```text
MAPPED
```

Reason:

- the standalone actual-file review is complete;
- no material standalone finding remains unresolved;
- the paired UTXO reference mapping exists;
- cross-mapping semantic-equivalence, fairness, maturity-parity, and
  anti-strawman review is complete;
- no material cross-mapping finding remains unresolved; and
- this mapping defensibly realizes the frozen semantic case under the frozen
  paired manifest.

`MAPPED` is an experimental mapping status only.

It does not mean:

- protocol valid;
- consensus adopted;
- Account selected;
- Account superior; or
- evidence collected.

Any material change after freeze creates distinguishable mapping content and
requires a new evidence-only content identity before new evidence is attributed
to it.

## 46. Evidence Status

Evidence collection status:

```text
EVIDENCE_NOT_COLLECTED
```

No comparative evidence exists in this file.

In particular, this mapping records no:

- benchmark timing;
- throughput;
- latency;
- serialized byte count;
- physical read count;
- physical write count;
- CPU measurement;
- allocation measurement;
- database measurement;
- commitment cost;
- cryptographic verification cost; or
- state-growth benchmark.

Missing evidence is not measured zero.

## 47. Review Gate

The independent actual-file review for this Account mapping covered:

- semantic drift from the frozen case;
- mismatch with the frozen manifest;
- Account-favoring hidden assumptions;
- artificial Account disadvantages;
- evidence-only relabeling;
- hidden ownership assumptions;
- hidden authorization assumptions;
- hidden replay assumptions;
- hidden stable identity;
- hidden lifecycle assumptions;
- hidden ordering assumptions;
- hidden cryptographic assumptions;
- hidden migration assumptions;
- hidden commitment assumptions;
- hidden resource assumptions;
- hidden monetary assumptions;
- unjustified internal cardinality;
- unreported preparation work;
- unreported optimization choices;
- premature logical read/write counts;
- candidate-native metric misuse; and
- unsupported Account-family generalization.

No material standalone finding remained unresolved at freeze.

## 48. Cross-Mapping Fairness Gate

Cross-mapping fairness review status:

```text
PASSED
```

The reviewed Account and UTXO reference mappings:

- use the same frozen semantic case hash;
- use the same frozen paired manifest hash;
- receive `A = 1`;
- preserve the same external semantic contract;
- ground the required projection change in candidate-specific mapped state;
- reject evidence-only relabeling;
- both use REFERENCE maturity;
- receive symmetric optimization permissions;
- disclose preparation work;
- preserve excluded dimensions symmetrically;
- keep candidate-native counters separate;
- do not intentionally weaken the Account persistent-relation mapping;
- do not intentionally weaken the UTXO replacement mapping; and
- remain qualified to their named mapping variants.

No material cross-mapping finding remained unresolved at freeze.

This Account mapping alone, and the paired reference set as a whole, still
cannot establish candidate superiority without the later evidence methodology
and decision process.

## 49. Current Project Impact

Creation of this frozen Account reference mapping has the following project
impact:

```text
Account selected:
NO

UTXO selected:
NO

Hybrid selected:
NO

State-model decision:
NOT MADE

Account reference mapping frozen:
YES — MAPPED

UTXO reference mapping frozen:
YES — MAPPED

Protocol validity defined:
NO

Transaction format selected:
NO

State format selected:
NO

Authorization mechanism selected:
NO

Replay mechanism selected:
NO

Cryptographic algorithm selected:
NO

State commitment selected:
NO

Resource-accounting mechanism selected:
NO

Monetary policy selected:
NO

Formal Specification update:
NOT JUSTIFIED BY THIS FROZEN MAPPING

PROJECT_STATE update:
NOT JUSTIFIED BY THIS FROZEN MAPPING

THREAT_MODEL update:
NOT JUSTIFIED BY THIS FROZEN MAPPING

Consensus implementation change:
NONE

Comparative evidence collected:
NO
```

## 50. Next Gate

This mapping must not proceed directly to comparative evidence.

The standalone actual-file review, cross-mapping fairness review, final mapping
status assignment, and mapping freeze gates are complete for this artifact.

The remaining permitted workflow is:

1. verify the installed frozen Account mapping byte-for-byte and establish its
   external evidence-only content identity;
2. verify the installed frozen UTXO mapping byte-for-byte and establish its
   external evidence-only content identity;
3. bind the applicable evidence methodology;
4. freeze the complete pre-result evidence configuration, including both exact
   mapping hashes;
5. only then collect comparative evidence; and
6. keep all conclusions mapping-qualified and non-normative unless later
   authority explicitly decides otherwise.

This file remains a non-normative experimental mapping. Its freeze does not
select a Dilithia state model or authorize protocol behavior.
