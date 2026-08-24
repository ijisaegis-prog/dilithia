# Account–UTXO Pilot Common Semantic Case: Value-Effect Baseline

> **NON-NORMATIVE EXPERIMENTAL COMMON SEMANTIC CASE**
>
> **Status: FROZEN PILOT SEMANTIC CASE**
>
> This document defines one candidate-neutral experimental semantic case for the
> Dilithia Account/UTXO comparison methodology.
>
> It is not a protocol specification, transaction definition, state-model
> definition, authorization mechanism, monetary rule, benchmark result, candidate
> mapping, or candidate ranking.
>
> If this document conflicts with the Dilithia Constitution, Formal
> Specification, or ratified HIP / Super HIP material, the authoritative protocol
> material prevails.
>
> Within the non-normative Account/UTXO comparison methodology, this case is
> subordinate to:
>
> - `ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`;
> - `ACCOUNT_UTXO_WORKLOAD_MODEL.md`; and
> - `ACCOUNT_UTXO_CANDIDATE_MAPPING_REQUIREMENTS.md`.

## 1. Purpose

This file defines the first pilot common semantic case in the
`value-effect-baseline` workload family.

Its purpose is intentionally narrow:

- define one minimal candidate-neutral value-affecting semantic effect;
- preserve one declared amount while requiring a changed external semantic
  projection;
- prevent amount-preserving no-op satisfaction of the declared effect;
- avoid Account-native terminology;
- avoid UTXO-native terminology;
- avoid candidate-internal cardinality;
- avoid authorization-mechanism selection;
- avoid replay or lifecycle assumptions;
- avoid state-commitment assumptions;
- avoid resource-accounting assumptions; and
- provide one shared semantic case from which later Account and UTXO mappings
  can be constructed.

This case does not attempt to model a typical production transaction.

It does not claim workload frequency, workload importance, or candidate
preference.

## 2. Artifact-Layer Boundary

The experimental artifact flow is:

Common Semantic Case
→ Paired Comparison Manifest
→ Account Mapping / UTXO Mapping
→ Evidence

This file defines only the **Common Semantic Case** layer.

The following do not belong in this file:

- Account record count;
- UTXO record count;
- transaction input count;
- transaction output count;
- candidate state-key count;
- candidate state reads;
- candidate state writes;
- candidate storage layout;
- credential count;
- signature count;
- proof count;
- verification-operation count;
- candidate conflict footprint;
- candidate serialization;
- exact encoded bytes;
- candidate implementation algorithm;
- candidate preparation work;
- candidate optimization configuration;
- benchmark timing; or
- candidate result.

Those properties belong to later paired-manifest, candidate-mapping, or evidence
layers as appropriate.

## 3. Organizational Identity

The human-readable organizational alias for this frozen case is:

`pilot/value-effect-baseline/v1`

This alias is non-authoritative.

It is not:

- a transaction identifier;
- a replay identifier;
- a state key;
- a consensus commitment;
- a protocol object identifier; or
- a permanent semantic identity.

A content hash is **not embedded in this frozen file**.

This frozen case may receive an evidence-only content hash calculated over the
frozen artifact according to the reviewed experimental identity procedure.

The hash must not create protocol authority.

## 4. Workload Family

Workload family:

`value-effect-baseline`

The family label is organizational only.

It does not mean:

- ordinary transaction;
- typical transaction;
- common user behavior;
- production-frequency transaction;
- one-input transaction;
- one-output transaction;
- Account balance mutation;
- UTXO spend; or
- preferred state-model workload.

## 5. Candidate-Neutral Semantic Vocabulary

This case uses only abstract case-local semantic terms.

The case defines:

- one pre-effect value term;
- one post-effect value term;
- one case-local amount parameter;
- one value-preservation relation; and
- one required change between the pre-effect and post-effect external semantic
  projections.

The terms do not denote:

- Accounts;
- UTXOs;
- transaction inputs;
- transaction outputs;
- addresses;
- owners;
- persistent records;
- state keys;
- credentials;
- signatures;
- proofs;
- reusable identities; or
- one-use objects.

`pre_value_0` and `post_value_0` are externally distinguished case-local
semantic terms for this experimental contract.

Their distinction means only that the contract requires amount `A` to be
associated with `pre_value_0` in the pre-effect external projection and with
`post_value_0` in the required post-effect external projection.

The term labels are names for those two case-local contract terms. Their
distinction does not assign persistent identity, ownership, persistence,
lifecycle, replay, consumption, address, credential, or protocol-object
semantics to either term.

Term labels exist only inside this experimental case and carry no protocol
authority.

## 6. Parameter Slot

This case defines one semantic parameter:

`A`

`A` is a case-local abstract native-value amount.

For this case:

`A > 0`

`A` is not assigned a protocol integer width by this document.

This case selects no:

- `u64`;
- `u128`;
- `u256`;
- balance width;
- supply-counter width;
- fee width; or
- arithmetic storage representation.

A paired manifest must instantiate `A` explicitly before candidate mappings are
compared.

Both candidate mappings must receive the same instantiated value of `A`.

Changing only the manifest-bound value of `A` does not by itself redefine an
Account or UTXO candidate.

## 7. Pre-Effect Semantic Term

The common semantic case contains one pre-effect term:

`pre_value_0`

Its case-local semantic amount is:

`amount(pre_value_0) = A`

`pre_value_0` is an opaque semantic term used in the pre-effect external
semantic projection.

The fact that this case contains one pre-effect semantic term does not require
either candidate to use:

- one record;
- one state object;
- one input;
- one balance field;
- one key;
- one credential;
- one proof;
- one read; or
- one write.

Candidate-internal realization remains a mapping property or evidence output.

The case-local term label itself is not a persistent object identifier.

## 8. Post-Effect Semantic Term

The common semantic case contains one required post-effect term:

`post_value_0`

Its required case-local semantic amount is:

`amount(post_value_0) = A`

`post_value_0` is an opaque semantic term used in the required post-effect
external semantic projection.

Within this case's external semantic contract, `post_value_0` is distinguished
from `pre_value_0` only as a separate case-local semantic term whose association
with amount `A` is required after `E0`.

That distinction does not prescribe candidate record creation, mutation,
deletion, replacement, persistence, ownership, addressability, or lifecycle.

The case-local term label itself is not a persistent object identifier.

## 9. Semantic Effect

The case defines one abstract value-affecting experimental effect:

`E0`

The semantic requirement for `E0` is:

```text
before external semantic projection:
    term   = pre_value_0
    amount = A

E0

after external semantic projection:
    term   = post_value_0
    amount = A
```

`pre_value_0` and `post_value_0` are externally distinguished case-local
semantic terms under this contract.

The amount is preserved, but the required external semantic projection is not
identical before and after `E0`.

Therefore, equality of the numeric amount alone is not sufficient to satisfy
`E0`.

A candidate mapping that leaves the external semantic projection unchanged
after the attempted effect does not satisfy this case merely because the amount
still equals `A`.

Likewise, an evidence record cannot establish `E0` merely by changing a
before/after label while presenting candidate evidence that the frozen mapping
projects identically on both sides.

This relation is an experimental semantic predicate only.

It does not define:

- a transaction;
- a transfer;
- a spend;
- a balance mutation;
- an input-consumption operation;
- an output-creation operation;
- a state transition format; or
- consensus validity.

The required projection change does not require any particular candidate
storage mutation or record lifecycle. Candidate internals remain mapping
properties or evidence outputs.

## 10. Case-Local Conservation Assumption

This case uses the following explicit non-normative case-scoped conservation
assumption:

`post_effect_amount = pre_effect_amount`

Therefore:

`A_after = A_before`

for the semantic amount represented by this case.

This assumption exists only to define this experimental baseline.

It does **not** establish:

- universal DLTH conservation;
- issuance rules;
- reward rules;
- fee rules;
- burn rules;
- supply policy; or
- monetary policy.

No supply-changing behavior is included in this case.

## 11. Preconditions

The common semantic preconditions are:

1. `A` is instantiated by the paired manifest.
2. `A > 0`.
3. the pre-effect external semantic projection associates the case-local term
   `pre_value_0` with amount `A`.
4. the contract distinguishes `pre_value_0` from `post_value_0` as separate
   case-local semantic terms.
5. the experimental contract evaluates only the case-local semantic projection
   defined by this document.

No candidate-specific representation is a precondition.

The term distinction is not an Account record distinction, UTXO record
distinction, owner distinction, address distinction, credential distinction, or
protocol identity rule.

## 12. Required Postcondition

The required postcondition is:

```text
term   = post_value_0
amount = A
```

The post-effect external semantic projection must therefore satisfy both:

1. amount preservation; and
2. the required case-local change in association from `pre_value_0` to
   `post_value_0`.

An unchanged pre-effect projection does not satisfy the required postcondition.

In particular, the following is insufficient by itself:

`amount_after = A`

if the mapping has not defensibly realized the required post-effect external
semantic projection.

No candidate-native counter is part of this required postcondition.

The case does not require equal:

- record counts;
- state accesses;
- byte counts;
- credential counts;
- signature counts;
- proof counts;
- verification-operation counts; or
- storage changes

between Account and UTXO mappings.

## 13. External Semantic Projection

For this case, the external semantic projection contains only the case-local
term-and-amount association required to evaluate the declared semantic effect.

Conceptually:

Before:

```text
term   = pre_value_0
amount = A
```

After successful satisfaction of the experimental contract:

```text
term   = post_value_0
amount = A
```

The before and after projections are intentionally non-identical because the
case-local semantic term associated with amount `A` changes while the amount is
preserved.

This is a representation-neutral experimental distinction. It does not mean
that either candidate must create, delete, replace, consume, or mutate one
particular internal object.

A mapping may use any defensible candidate-specific realization permitted by
the frozen paired manifest and mapping requirements, but it must realize the
same changed external semantic projection.

The mapping and evidence methodology must not use an unconstrained
before/after phase label as the sole reason that otherwise identical mapped
candidate evidence projects to different case-local semantic terms.

Merely relabeling unchanged candidate evidence from `pre_value_0` to
`post_value_0` is not evidence that the mapping realized `E0`.

Information outside this declared case-local projection is not specified by this
case.

Candidate internals are excluded from the common projection.

## 14. Experimental-Contract Disposition

This case uses the workload-model experimental-contract disposition vocabulary.

If all declared case-local preconditions and required postcondition predicates
are satisfied, including the required changed external semantic projection, the
experimental-contract outcome is:

`CONTRACT_CONDITIONS_MET`

If a declared case-local condition or required predicate is not satisfied, the
experimental-contract outcome is:

`CONTRACT_CONDITIONS_NOT_MET`

In particular, an unchanged external semantic projection after the attempted
effect is `CONTRACT_CONDITIONS_NOT_MET` for this case even when the numeric
amount remains `A`.

These values describe only this non-normative experimental semantic contract.

They do not mean:

- protocol valid;
- protocol invalid;
- accept;
- reject;
- transaction valid;
- transaction invalid; or
- future Dilithia consensus behavior.

Protocol validity remains unspecified.

## 15. Authority-Relation Boundary

This baseline case does not make a candidate-specific authorization-mechanism
selection.

It does not define:

- keys;
- addresses;
- credentials;
- signatures;
- proofs;
- multisignature;
- threshold authorization;
- delegation;
- recovery; or
- stable identity.

No credential, signature, proof, or verification-operation cardinality is part
of this common case.

If a later paired experiment introduces a material authorization profile without
changing this case's external semantic contract, that profile belongs in the
paired comparison manifest.

If an independently required authority relation changes the external semantic
contract itself, that change requires separately reviewed case content.

## 16. Replay and Lifecycle Boundary

This case defines no:

- replay rule;
- nonce;
- one-use capability;
- record-consumption identity;
- transaction-hash replay identity;
- duplicate-presentation validity;
- reapplication validity;
- rollback rule;
- reorganization rule; or
- candidate record lifecycle.

The existence of `pre_value_0` and `post_value_0` does not imply that one
candidate object is consumed or another is created.

Canonical replay and reorganization semantics remain outside this case.

## 17. External Conflict Relation

This baseline case defines no intrinsic pairwise external semantic conflict edge
between multiple effects because it contains only one declared semantic effect.

Therefore the intrinsic external conflict relation for this case is empty.

This does not mean either candidate is conflict-free.

Candidate representation-induced:

- conflicts;
- contention;
- dependency breadth;
- scheduling restrictions; or
- conflict amplification

remain candidate outputs.

A later `semantic-conflict-pair` case will define an explicit shared external
conflict relation separately.

## 18. Ordering Boundary

This case contains no intrinsic external sequence or partial order beyond the
logical before/effect/after relationship required to describe `E0`.

It selects no consensus ordering mechanism.

It selects no:

- block ordering;
- transaction ordering;
- scheduler;
- parallel-execution rule;
- serialization rule; or
- conflict-resolution rule.

Any material experimental ordering profile that does not alter this case's
external semantics belongs in the paired comparison manifest.

## 19. Cryptographic Boundary

This case selects no cryptographic algorithm.

It contains no common-case assumption about:

- signature algorithm;
- proof system;
- key type;
- signature count;
- proof count;
- verification-operation count;
- cryptographic byte count; or
- Crypto Agility transition.

If cryptographic behavior is later measured, both Account and UTXO mappings in a
paired comparison must receive the same applicable algorithm profile.

## 20. State-Commitment Boundary

This case selects no state commitment.

It defines no:

- Merkle structure;
- authenticated tree;
- accumulator;
- proof format;
- commitment identifier;
- commitment update rule; or
- commitment-retention rule.

Commitment-dependent evidence belongs to a later explicitly scoped experimental
artifact or branch.

## 21. Resource Boundary

This case defines no canonical resource-accounting mechanism.

It does not select:

- gas;
- scalar resource score;
- attempt meter;
- no-refund accounting;
- fee constant;
- transaction resource limit;
- block resource limit;
- state-growth limit; or
- ingress-abuse mechanism.

Candidate mappings may later produce resource evidence only under explicitly
declared experimental assumptions.

No resource observation from this case becomes a consensus rule.

## 22. Arithmetic Boundary

The semantic equation in this common case is mathematical and
representation-neutral.

Candidate mappings that instantiate arithmetic must use deterministic,
host-independent arithmetic with explicit overflow and underflow behavior under
the applicable experimental methodology.

This case itself selects no candidate arithmetic representation.

Existing Dilithia serialization widths do not select the future state-model
balance representation.

## 23. Candidate-Neutrality Requirements

An Account mapping and a UTXO mapping constructed from this case must receive
the same:

- frozen case content;
- case identity;
- instantiated `A`;
- applicable paired-manifest profiles;
- applicable branch selections;
- external semantic preconditions;
- required postcondition;
- definition of the required external semantic projection change;
- experimental-contract disposition rules; and
- external semantic projection.

Neither candidate may rewrite this case to make its representation easier.

Neither candidate may satisfy the required projection change solely through an
evidence-only before/after label that is not grounded in its frozen mapping.

## 24. Prohibited Common-Case Inferences

The following inferences are prohibited:

1. one pre-effect semantic term means one Account;
2. one pre-effect semantic term means one UTXO;
3. one post-effect semantic term means one Account;
4. one post-effect semantic term means one UTXO;
5. one semantic effect means one transaction;
6. one semantic term means one state key;
7. one semantic term means one owner;
8. one semantic term means one address;
9. one semantic term means one credential;
10. one semantic term means one signature;
11. one semantic term means one proof;
12. distinction between `pre_value_0` and `post_value_0` means two persistent
    objects;
13. distinction between `pre_value_0` and `post_value_0` means two owners,
    addresses, credentials, or protocol identities;
14. amount preservation alone is sufficient to satisfy `E0`;
15. a before/after evidence label alone is sufficient to establish the required
    projection change;
16. the required projection change means a particular storage mutation;
17. the required projection change means a particular record lifecycle;
18. the absence of an intrinsic conflict edge means candidate-internal absence
    of conflict;
19. the absence of an authorization mechanism means authorization is globally
    unnecessary;
20. the absence of replay semantics means replay is permitted by a future
    protocol; or
21. success under this experimental contract means protocol acceptance.

## 25. Candidate Mapping Outputs Reserved for Later Layers

The following may be recorded later for each candidate mapping or its evidence,
but are deliberately absent from this common case:

- internal value representation;
- internal record cardinality;
- persistent logical-state shape;
- logical reads;
- logical writes;
- records created;
- records replaced;
- records retired;
- authorization evidence objects;
- cryptographic verification work;
- internal dependencies;
- candidate conflict realization;
- serialization work;
- exact bytes under an experimental schema;
- validation work;
- preparation work;
- state-growth consequences;
- migration work;
- implementation complexity;
- physical timing;
- CPU evidence;
- allocation evidence;
- database evidence; and
- other implementation-specific observations.

## 26. Missing or Unresolved Material

This common case intentionally leaves unresolved:

- Account representation;
- UTXO representation;
- hybrid representation;
- transaction format;
- state format;
- authorization architecture;
- stable identity;
- replay protection;
- nonce semantics;
- one-use semantics;
- reorganization semantics;
- cryptographic algorithm;
- state commitment;
- migration mechanism;
- resource accounting;
- numeric resource limits;
- fees;
- supply-changing behavior;
- consensus mechanism; and
- state-model selection.

None of these unresolved dimensions receives an implicit default through this
case.

## 27. Paired-Manifest Requirements for This Case

Before Account and UTXO mappings are constructed for comparison, a paired
comparison manifest must explicitly bind the material experimental assumptions
applicable to the comparison.

At minimum, the future manifest must bind:

- the frozen identity of this semantic case;
- the instantiated value of `A`;
- applicable assumption profile;
- applicable policy branches;
- applicable architecture branches;
- applicable algorithm profile;
- applicable ordering profile;
- applicable version profile;
- applicable resource-architecture profile, if measured;
- Account candidate mapping identity;
- UTXO candidate mapping identity; and
- any other material dimension required by the higher-level methodology.

No material profile receives an implicit default.

## 28. Evidence Boundary

No evidence is recorded in this file.

This file contains no:

- benchmark result;
- byte count;
- read count;
- write count;
- record count;
- signature count;
- proof count;
- verification count;
- latency result;
- throughput result;
- state-growth result; or
- candidate preference.

Evidence may be collected only after:

1. this common semantic case is reviewed and frozen;
2. the paired manifest is frozen;
3. both candidate mappings are frozen;
4. mapping equivalence is independently reviewed; and
5. the applicable evidence methodology is bound.

The frozen mapping and evidence methodology must define how each candidate
demonstrates the required non-identical pre-effect and post-effect external
semantic projections.

A before/after phase label, timestamp, run position, or evidence-record label
that is not grounded in the frozen candidate mapping is not sufficient evidence
of the required projection change.

## 29. Freeze and Change Control

This file is currently:

`FROZEN`

The freeze review for this artifact covers:

- candidate-label leakage;
- candidate-internal cardinality leakage;
- semantic ambiguity;
- no-op satisfaction or vacuous effect satisfaction;
- ungrounded evidence-only relabeling;
- hidden authorization assumptions;
- hidden replay assumptions;
- hidden lifecycle assumptions;
- hidden ordering assumptions;
- hidden cryptographic assumptions;
- hidden commitment assumptions;
- hidden resource assumptions;
- hidden monetary assumptions; and
- accidental protocol-rule creation.

After a reviewed version is frozen, a material change to its external:

- preconditions;
- required postconditions;
- semantic value relation;
- required external semantic projection change;
- authority relations;
- intrinsic conflict relation; or
- intrinsic sequence/partial order

creates new semantic case content and requires a new evidence identity.

## 30. Current Project Impact

Creation of this frozen common semantic case has the following project impact:

Account selected:
NO

UTXO selected:
NO

Hybrid selected:
NO

State-model decision:
NOT MADE

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
NOT JUSTIFIED BY THIS FROZEN CASE

PROJECT_STATE update:
NOT JUSTIFIED BY THIS FROZEN CASE

THREAT_MODEL update:
NOT JUSTIFIED BY THIS FROZEN CASE

Consensus implementation change:
NONE

Benchmark evidence collected:
NO

## 31. Next Gate

This common semantic case must not proceed directly to benchmark evidence.

The actual-file review, material-finding resolution, and semantic-case freeze gates
are complete for this artifact.

The remaining permitted workflow is:

1. establish its evidence-only content identity;
2. create one paired comparison manifest;
3. create one Account candidate mapping;
4. create one UTXO candidate mapping;
5. independently review mapping equivalence and fairness; and
6. only then consider evidence collection.

This file remains a non-normative experimental artifact. The remaining gates must pass before evidence collection.
