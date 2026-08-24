# Account–UTXO Pilot Candidate Mapping: Value-Effect Baseline — UTXO Reference

> **NON-NORMATIVE EXPERIMENTAL CANDIDATE MAPPING**
>
> **Status: FROZEN PILOT CANDIDATE MAPPING — MAPPED**
>
> **Candidate family: UTXO**
>
> **Mapping maturity: REFERENCE**
>
> This document defines one UTXO-family reference mapping for the frozen
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

This file defines one deliberately narrow UTXO-family **reference mapping**.

Its purpose is to demonstrate, without selecting future Dilithia protocol
semantics, one defensible way for a UTXO-family analytical candidate to realize
the frozen external semantic effect:

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

It is not intended to establish an optimized UTXO design or a UTXO-family
performance conclusion.

## 2. Mapping Record Summary

```text
mapping_format_version:
    pilot-account-utxo-mapping/v1

mapping_alias:
    pilot/value-effect-baseline/utxo/reference/v1

mapping_content_hash:
    NOT EMBEDDED — EXTERNAL EVIDENCE-ONLY CONTENT IDENTITY

candidate_family:
    UTXO

candidate_variant:
    discrete-value-records/one-to-one-replacement-reference/v1

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

They are not protocol identities, transaction identifiers, replay identifiers,
consensus commitments, ownership identifiers, or canonical output identifiers.

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

## 5. UTXO-Family Analytical Hypothesis

For this mapping only, the UTXO-family analytical hypothesis is:

> Native value is accounted for through a replacement relation from a set of
> live discrete value records to a resulting set.

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
- one-use capability;
- canonical record-consumption identity;
- transaction inputs;
- transaction outputs;
- stable identity;
- migration;
- recovery;
- cryptographic algorithm;
- state commitment;
- physical storage; or
- parallel execution.

The word **UTXO** in this artifact means only that this candidate mapping
represents the value effect through a replacement relation involving discrete
mapping-local value records.

The replacement relation in this mapping is **not** itself a canonical replay or
one-use rule.

## 6. Mapping-Local Terminology

This mapping introduces two opaque mapping-local logical value-record handles:

```text
UR_PRE
UR_POST
```

They are names used only inside this candidate mapping.

They are not:

- protocol UTXO identifiers;
- transaction output identifiers;
- transaction inputs;
- transaction outputs;
- addresses;
- owners;
- state keys;
- credentials;
- replay identities;
- canonical one-use identifiers;
- commitment leaves; or
- proof identifiers.

The handles allow the mapping to describe a replacement relation between
discrete logical value records without selecting a future Dilithia transaction
or state schema.

## 7. Discrete Logical Value Record Model

The reference mapping uses a mapping-local set of live discrete value records.

Conceptually, the mapping-level state exposes:

```text
live_set
amount(record) -> positive case-local experimental amount
```

for mapping-local records that are live in the mapped state.

The mapping uses:

```text
UR_PRE
UR_POST
```

as two distinct mapping-local logical value-record handles.

This is a logical mapping construct only.

It does not define:

- a Rust type;
- a protocol output type;
- a transaction output;
- an outpoint;
- a transaction hash;
- an index;
- a state key;
- a database key;
- a serialized field;
- a canonical record identifier;
- a consumption proof;
- a tombstone;
- pruning behavior; or
- physical persistence layout.

The mapping changes which logical value record is present in the live set.

That replacement is the UTXO-family analytical representation used for this
pilot only.

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
- one transaction input;
- one transaction output;
- one address; or
- one authorization relation.

No protocol numeric width is selected.

## 9. Pre-Effect UTXO Mapping State

The mapped pre-effect logical state is:

```text
live_set = { UR_PRE }

amount(UR_PRE) = 1
```

`UR_POST` is not a member of the pre-effect live set.

Interpretation:

- the case-local experimental amount is represented by one live discrete
  mapping-local value record, `UR_PRE`; and
- `UR_POST` is not part of the mapped pre-effect live record set.

This is a candidate-specific mapping choice.

It does not imply that the frozen common semantic case contains one source
record.

It does not imply that all UTXO-family designs require one input record for this
semantic case.

No ownership or authorization meaning is attached to `UR_PRE`.

## 10. Pre-Effect External Projection Derivation

For this mapping, the frozen pre-effect external projection is derived from the
candidate state by the following mapping rule:

```text
if live_set = { UR_PRE }
and amount(UR_PRE) = 1

then project:
    term   = pre_value_0
    amount = 1
```

This projection rule is part of the candidate mapping.

It is not a protocol rule.

The case-local term `pre_value_0` does not become a UTXO identifier.

The mapping demonstrates only how the UTXO candidate state realizes the frozen
common semantic projection.

## 11. Candidate-Specific Realization of E0

The UTXO reference realization of `E0` changes the mapping-local live record set
as follows:

```text
before:
    live_set = { UR_PRE }
    amount(UR_PRE) = 1

E0 mapping relation:
    remove UR_PRE from the mapped live set
    add UR_POST to the mapped live set
    amount(UR_POST) = 1

after:
    live_set = { UR_POST }
    amount(UR_POST) = 1
```

Equivalently, the candidate-specific replacement relation is:

```text
{ UR_PRE : 1 }
    ->
{ UR_POST : 1 }
```

This is the candidate-specific logical realization of the frozen semantic
effect.

The words `remove` and `add` here describe only the mapping-local before/after
membership relation.

They do not define a transaction instruction, storage mutation opcode, canonical
deletion operation, transaction input/output encoding, replay rule, or
one-use capability.

The mapping does not define:

- a transaction;
- a spend instruction;
- transaction inputs or outputs;
- an outpoint;
- a transaction hash;
- authorization;
- replay protection;
- state-key encoding;
- execution scheduling;
- a database update algorithm; or
- consensus acceptance.

## 12. Post-Effect UTXO Mapping State

The required mapped post-effect logical state is:

```text
live_set = { UR_POST }

amount(UR_POST) = 1
```

`UR_PRE` is not a member of the post-effect live set.

Interpretation:

- the case-local experimental amount is represented by `UR_POST` in the
  post-effect mapping state; and
- `UR_PRE` is no longer part of that mapped live record set.

This does not define a protocol rule that `UR_PRE` can never reappear, cannot be
referenced again, or is canonically consumed.

Those replay and lifecycle semantics remain blocked or unselected by the paired
manifest.

## 13. Post-Effect External Projection Derivation

For this mapping, the required post-effect external projection is derived by:

```text
if live_set = { UR_POST }
and amount(UR_POST) = 1

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

This mapping cannot satisfy `E0` by leaving the UTXO logical state unchanged.

The following candidate state:

```text
before:
    live_set = { UR_PRE }
    amount(UR_PRE) = 1

after:
    live_set = { UR_PRE }
    amount(UR_PRE) = 1
```

does not satisfy this mapping merely because an evidence record writes
`post_value_0` after the attempted effect.

Likewise, the following is not sufficient:

```text
same mapped live set
+ new timestamp
+ later run position
+ changed evidence label
```

The required UTXO mapping state itself must change from:

```text
{ UR_PRE : 1 }
```

to:

```text
{ UR_POST : 1 }
```

Therefore the post-effect projection is grounded in candidate-specific mapped
state rather than evidence-only relabeling.

## 15. Amount Preservation

The mapped pre-effect available amount is:

```text
sum(amount(r) for r in live_set) = 1
```

The mapped post-effect available amount is:

```text
sum(amount(r) for r in live_set) = 1
```

For this one-record reference mapping:

```text
before = 1
after  = 1
```

Therefore this mapping preserves the frozen case-local conservation relation:

```text
A_after = A_before = 1
```

This is a case-scoped experimental relation only.

It does not define universal DLTH conservation or any issuance, reward, fee,
burn, or supply policy.

## 16. Logical Record Cardinality

This reference mapping uses:

```text
pre-effect live logical value records  = 1
post-effect live logical value records = 1
distinct mapping-local record handles  = 2
```

Those cardinalities are properties of this named UTXO reference mapping.

They are not:

- common workload inputs;
- protocol requirements;
- statements that every UTXO design needs exactly one source record;
- statements that every UTXO design needs exactly one resulting record;
- statements that one semantic term universally equals one UTXO; or
- evidence that UTXO is better or worse than Account.

A materially different defensible UTXO mapping may use a different internal
record decomposition while preserving the same frozen external semantic
contract.

Such a mapping requires a distinct mapping identity if used for comparative
evidence.

## 17. Physical Record Cardinality Boundary

This logical mapping does **not** fix:

```text
physical UTXO record count
physical database row count
physical key count
physical page count
serialized transaction-output count
```

The mapping-local live logical value records may or may not correspond one-to-one
with physical records in a later experimental implementation.

That decision belongs to a separately reviewed candidate schema or
implementation artifact.

No physical-cardinality evidence may be inferred from this document alone.

## 18. Logical State-Key Boundary

`UR_PRE` and `UR_POST` are mapping-local logical handles.

This mapping does not define a future Dilithia state-key or outpoint schema.

Therefore:

```text
protocol state-key count = NOT SELECTED
outpoint structure        = NOT SELECTED
state-key encoding        = NOT SELECTED
key derivation            = NOT SELECTED
transaction index field   = NOT SELECTED
```

A later implementation may map these logical records onto candidate-specific
state locations, but those locations become implementation or experimental
schema evidence.

## 19. Logical Read/Write Measurement Boundary

The semantic mapping requires the resulting UTXO logical state to reflect:

```text
before live set:
    { UR_PRE : 1 }

after live set:
    { UR_POST : 1 }
```

This establishes a mapping-level replacement relation.

It does **not** assign benchmark counters such as:

```text
logical_reads   = 1
logical_writes  = 2
records_removed = 1
records_created = 1
```

because those metric meanings require a frozen experimental schema,
measurement definition, metadata placement, validation workflow, and applicable
commitment assumptions.

Accordingly:

```text
logical read count metric       = NOT YET ASSIGNED
logical write count metric      = NOT YET ASSIGNED
record-retirement metric        = NOT YET ASSIGNED
record-creation metric          = NOT YET ASSIGNED
```

Later evidence may record such values only under reviewed metric definitions.

## 20. Internal Dependency Shape

This reference mapping has one mapping-level dependency:

> The post-effect projection is satisfied only when the mapped live record set
> reflects the required replacement from `UR_PRE` to `UR_POST`.

No broader dependency graph is fixed.

This document does not define:

- locking;
- scheduler dependencies;
- speculative execution;
- parallel-validation dependencies;
- database transaction dependencies;
- input-dependency graphs; or
- block-level dependency rules.

## 21. External Conflict Preservation

The frozen common semantic case has:

```text
intrinsic external conflict relation = EMPTY
```

This mapping preserves that relation unchanged.

It does not infer that the UTXO candidate has zero candidate-induced
contention.

Candidate-specific record overlap, state-key overlap, scheduling restrictions,
dependency breadth, or conflict amplification remain later
mapping/schema/evidence outputs.

The UTXO label itself implies none of them.

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

This UTXO mapping therefore defines no:

- owner;
- address;
- key;
- credential;
- signature;
- proof;
- ownership condition;
- output locking condition;
- multisignature;
- threshold rule;
- delegation;
- recovery; or
- stable identity.

Neither `UR_PRE` nor `UR_POST` is an owner, address, key, or credential.

This mapping must not be used as evidence that UTXO-family value changes need
zero, one, per-record, or any particular number of authorizations.

## 24. Replay and One-Use Boundary

The paired manifest keeps replay, nonce, one-use, reapplication, and
reorganization semantics blocked or unselected.

This mapping therefore introduces no:

- nonce;
- sequence number;
- canonical one-use capability;
- canonical record-consumption identity;
- transaction-hash replay identity;
- duplicate-presentation rule;
- reapplication rule;
- rollback rule; or
- chain-reorganization rule.

The fact that `UR_PRE` is absent from the mapping's post-effect live set is not a
canonical replay rule.

The mapping does not state that `UR_PRE` can never reappear, can never be
referenced again, or represents a protocol-level spent marker.

The mapping-local replacement relation must not be interpreted as silently
selecting one-use semantics.

## 25. Lifecycle Boundary

This reference mapping describes before/after membership in a mapping-local live
record set.

That statement is limited to the UTXO-family analytical value-accounting
representation.

It does not select protocol semantics for:

- output creation;
- output consumption;
- output deletion;
- spent-output retention;
- tombstones;
- pruning;
- archival history;
- historical lookup;
- output recreation; or
- persistent protocol identity.

A future normative UTXO design would require explicit lifecycle and replay
semantics elsewhere.

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

The mapping must not be interpreted as a UTXO design that lacks authorization
or cryptography in a future protocol.

Those dimensions are simply outside this pilot configuration.

## 27. Crypto Agility Boundary

This reference mapping is only a logical value-accounting mapping.

It does not embed a cryptographic algorithm into the identity of `UR_PRE`,
`UR_POST`, or the UTXO-family hypothesis.

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

- output-condition migration;
- credential migration;
- global migration;
- spend-time migration;
- re-keying;
- version registry;
- compatibility path;
- dormant-state migration; or
- recovery path.

The logical record handles `UR_PRE` and `UR_POST` are not stable migration
identities.

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

The logical live value records are commitment-independent mapping constructs.

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

The replacement relation described by this mapping is semantic structure, not a
resource measurement.

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

The mapping uses only the case-local mathematical quantity:

```text
1
```

to describe the reference live-record amounts.

No protocol amount width is selected.

No `u64`, `u128`, `u256`, signed integer type, floating-point type, or storage
representation is selected.

If an implementation later performs consensus-relevant arithmetic for this
mapping, that experimental implementation must use deterministic,
host-independent behavior with explicit overflow and underflow semantics.

Existing Dilithia serialization widths do not choose the future UTXO amount
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
- output-record bytes;
- input-reference bytes;
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

`UR_PRE` and `UR_POST` are logical value-record handles only.

Physical persistence effects remain implementation evidence.

## 37. Preparation Configuration

Preparation configuration:

```text
EXPLICIT INITIAL LOGICAL-STATE SETUP ONLY
```

Required preparation is limited to defining the mapping-local initial state:

```text
live_set = { UR_PRE }
amount(UR_PRE) = 1
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
- grouping;
- aggregation;
- caching;
- indexing;
- deduplication;
- compact encoding;
- proof reuse;
- scheduler optimization;
- preprocessing optimization; or
- physical database optimization.

The one-to-one replacement structure is used as a direct UTXO-family reference
mapping for the two externally distinguished semantic terms without importing
additional source fragmentation or fan-out assumptions.

A future optimized UTXO mapping requires a distinct mapping identity or a new
pre-result-frozen configuration as required by the methodology.

## 39. Anti-Strawman Statement

This reference mapping is not intended to make the UTXO candidate artificially
weak.

The mapping does not intentionally add:

- fragmented source records;
- multiple resulting records;
- redundant logical records;
- redundant value movement;
- redundant serialization;
- duplicated authorization evidence;
- artificial cryptographic work;
- artificial conflict edges; or
- deliberately inefficient validation stages.

No claim is made that this is the only defensible UTXO mapping.

Any later conclusion from this mapping remains specific to this mapping and the
frozen manifest.

## 40. Alternative Defensible UTXO Mappings

Materially different UTXO-family mappings may be defensible.

Potential alternatives may differ in:

- internal value-record decomposition;
- grouping;
- replacement cardinality;
- metadata organization;
- indirection; or
- other candidate-native structures

while preserving the same frozen external semantic contract.

This mapping does not pre-approve any particular alternative.

It also does not assume that an alternative with a different logical record
count, state layout, or indirection strategy is better or worse.

If an alternative is used for evidence, it must receive:

- a distinct mapping identity;
- the same frozen semantic case;
- a compatible frozen paired manifest;
- explicit optimization and preparation configuration;
- independent review; and
- mapping-qualified reporting.

This first pilot does not support a broad UTXO-family conclusion from one
reference mapping.

## 41. Candidate-Specific Structural Outputs

This mapping currently discloses the following structural outputs:

```text
candidate family:
    UTXO

mapping variant:
    one-to-one discrete logical value-record replacement

pre-effect live logical value records:
    1

post-effect live logical value records:
    1

distinct mapping-local record handles:
    2

mapped live set before:
    { UR_PRE : 1 }

mapped live set after:
    { UR_POST : 1 }

physical record count:
    UNRESOLVED / NOT FIXED BY LOGICAL MAPPING

protocol state-key count:
    UNRESOLVED / NOT FIXED BY LOGICAL MAPPING

logical read metric:
    NOT ASSIGNED

logical write metric:
    NOT ASSIGNED

record-retirement metric:
    NOT ASSIGNED

record-creation metric:
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
an Account mapping.

## 42. External Projection Checklist

Pre-effect mapping:

```text
UTXO candidate state:
    live_set = { UR_PRE }
    amount(UR_PRE) = 1

projects to:
    term   = pre_value_0
    amount = 1
```

Required post-effect mapping:

```text
UTXO candidate state:
    live_set = { UR_POST }
    amount(UR_POST) = 1

projects to:
    term   = post_value_0
    amount = 1
```

No-op rejection:

```text
{ UR_PRE : 1 } -> { UR_PRE : 1 }
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

canonical one-use semantics introduced:
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

- future UTXO protocol state schema;
- future UTXO transaction schema;
- physical record representation;
- state-key or outpoint representation;
- canonical live/spent lifecycle;
- transaction format;
- transaction validity;
- ownership semantics;
- authorization architecture;
- replay protection;
- nonce semantics if any;
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
- alternative defensible UTXO mapping set.

None receives an implicit default from this artifact.

## 45. Mapping Status

Current mapping status:

```text
MAPPED
```

Reason:

- the standalone actual-file review is complete;
- no material standalone finding remains unresolved;
- the paired Account reference mapping exists;
- cross-mapping semantic-equivalence, fairness, maturity-parity, and
  anti-strawman review is complete;
- no material cross-mapping finding remains unresolved; and
- this mapping defensibly realizes the frozen semantic case under the frozen
  paired manifest.

`MAPPED` is an experimental mapping status only.

It does not mean:

- protocol valid;
- consensus adopted;
- UTXO selected;
- UTXO superior; or
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

The independent actual-file review for this UTXO mapping covered:

- semantic drift from the frozen case;
- mismatch with the frozen manifest;
- UTXO-favoring hidden assumptions;
- artificial UTXO disadvantages;
- evidence-only relabeling;
- hidden ownership assumptions;
- hidden authorization assumptions;
- hidden replay assumptions;
- hidden one-use assumptions;
- hidden record-identity assumptions;
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
- unsupported UTXO-family generalization.

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

This UTXO mapping alone, and the paired reference set as a whole, still cannot
establish candidate superiority without the later evidence methodology and
decision process.

## 49. Current Project Impact

Creation of this frozen UTXO reference mapping has the following project impact:

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
