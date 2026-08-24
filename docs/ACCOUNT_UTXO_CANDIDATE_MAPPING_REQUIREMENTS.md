# Account–UTXO Candidate Mapping Requirements

> **NON-NORMATIVE EXPERIMENTAL MAPPING METHODOLOGY**
>
> This document defines requirements for constructing and reviewing
> Account-family and UTXO-family candidate mappings for comparative evidence.
> It does not define protocol behavior, select or rank a state model, or resolve
> any Formal Specification TBD.
>
> If this document conflicts with the Dilithia Constitution, Formal
> Specification, or a ratified HIP / Super HIP, the authoritative protocol
> material prevails. Within the non-normative comparison methodology,
> `ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md` and `ACCOUNT_UTXO_WORKLOAD_MODEL.md`
> govern the scopes they define.

## 1. Status, Purpose, and Scope

Dilithia remains Pre-Genesis.

Account-family and UTXO-family designs remain co-equal analytical candidates.

The purpose of this document is to ensure that future candidate mappings are:

- based on the same external semantic problem;
- bound to the same declared experimental conditions;
- resistant to semantic drift;
- resistant to strawman implementations;
- resistant to asymmetric optimization;
- explicit about missing, incomplete, or unsupported mappings; and
- reproducible enough for independent review.

Terms such as **must**, **required**, and **prohibited** below describe controls
for credible experimental comparison. They are not consensus rules.

This document selects no:

- Account design;
- UTXO design;
- hybrid design;
- transaction format;
- state format;
- authorization architecture;
- stable identity mechanism;
- replay mechanism;
- reorganization mechanism;
- ordering mechanism;
- cryptographic algorithm;
- migration mechanism;
- state commitment;
- resource-accounting mechanism;
- numeric resource limit;
- monetary mechanism; or
- candidate winner.

## 2. Relationship to Existing Comparison Artifacts

This document is subordinate to:

- `ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`; and
- `ACCOUNT_UTXO_WORKLOAD_MODEL.md`.

The required experimental artifact flow is:

```text
Common Semantic Case
        |
        v
Paired Comparison Manifest
        |
        +-------------------+
        |                   |
        v                   v
Account Mapping         UTXO Mapping
        |                   |
        +---------+---------+
                  |
                  v
             Evidence
```

Each layer has a separate responsibility.

Information owned by a later layer must not be moved into an earlier layer
merely to make one candidate easier to describe or measure.

## 3. Common Semantic Case Boundary

The common semantic case describes the external experimental problem.

Candidate mappings consume that case. They do not redefine it.

Candidate-specific properties such as the following do not belong in the common
case:

- Account record count;
- UTXO record count;
- transaction input count;
- transaction output count;
- state-key count;
- state reads;
- state writes;
- credential count;
- signature count;
- proof count;
- verification-operation count;
- internal dependency count;
- candidate conflict footprint; and
- candidate storage layout.

If the external semantic contract independently requires separateness or
multiplicity, the common case records only that representation-neutral semantic
distinction. Candidate-specific record, input/output, state-key/access,
credential, signature, proof, verification, dependency, conflict-footprint,
storage, and other internal cardinalities remain mapping properties or evidence
outputs.

## 4. Paired Comparison Manifest

Every paired comparison must bind both candidates to the same frozen manifest.

The manifest identifies the common case and all material experimental
assumptions required for that comparison.

Material selections may include:

- assumption profile;
- policy branch set;
- architecture branch set;
- authorization profile;
- algorithm profile;
- ordering profile;
- version profile;
- migration branch;
- resource-architecture profile;
- instantiated parameter values;
- suite identity; and
- candidate mapping identities.

Both candidates receive the same applicable semantic case hash, parameter
values, profile selections, and branch selections.

No material profile has an implicit default.

If a material dimension is unresolved and unbound, the comparison remains
conditional, unavailable, incomplete, or blocked as appropriate.

## 5. Pre-Result Freeze

The material comparison configuration must be frozen before comparative results
are used to prefer a candidate.

This includes, where applicable:

- semantic case;
- paired manifest;
- candidate mappings;
- allowed optimization classes;
- parameter ranges;
- measurement definitions;
- evidence methodology;
- corpus or suite composition; and
- exclusion rules.

A material post-result change creates a new experimental configuration.

Old and new evidence must remain distinguishable.

Results must not be used to choose favorable assumptions retroactively.

## 6. Semantic Preservation

A candidate mapping must preserve the frozen external semantic contract.

Preservation review must cover every applicable:

- precondition;
- required postcondition;
- semantic effect relation;
- authority relation;
- external conflict relation;
- intrinsic sequence;
- intrinsic partial order;
- case-local monetary or conservation assumption; and
- instantiated semantic parameter.

Internal mechanisms may differ between candidates. External requirements may
not.

A mapping that changes the external problem is not a mapping of the same case.

If the external semantic contract itself changes, the corrected workload case
must receive new content and a new case identity according to the workload
methodology.

## 7. Semantic Drift Prohibition

A candidate mapping must not gain advantage by silently:

- weakening an authority requirement;
- adding an authority requirement to the competitor;
- changing required postconditions;
- changing the shared conflict relation;
- adding externally meaningful ordering constraints;
- removing externally meaningful ordering constraints;
- adding candidate-specific success conditions;
- removing failure conditions;
- changing semantic value effects; or
- assuming candidate-specific external information.

Such changes constitute semantic drift.

Semantic drift invalidates a claim that the two mappings implement the same
experimental case.

## 8. Candidate-Label Neutrality

The labels **Account** and **UTXO** do not by themselves imply:

- ownership structure;
- authorization structure;
- credential placement;
- replay protection;
- nonce behavior;
- one-use semantics;
- record lifecycle;
- deletion or pruning;
- concurrency;
- conflict granularity;
- cryptographic algorithm;
- proof structure;
- state commitment;
- migration behavior;
- recovery;
- stable identity; or
- physical database layout.

A mapping must state candidate-specific choices rather than deriving them from
the candidate label.

## 9. Authority Relations

Where a workload uses `N`, it means:

> `N` independently required consensus authority relations in the experimental
> semantic workload.

`N` is not automatically the number of:

- humans;
- owners;
- organizations;
- accounts;
- addresses;
- keys;
- credentials;
- signatures;
- proofs;
- witnesses;
- threshold shares; or
- cryptographic verification operations.

A mapping records how the selected candidate realizes the authority requirement.

It must not redefine `N` to suit the candidate.

Whether native support for `N > 1` exists remains an explicit architecture
branch unless resolved through the authoritative protocol process.

## 10. Value and Internal Cardinality

Experimental pre-effect and post-effect value terms are semantic terms.

They are not automatically:

- Account balances;
- UTXOs;
- inputs;
- outputs;
- owners;
- addresses;
- persistent records; or
- reusable identities.

Internal source or record cardinality belongs to the candidate mapping or
evidence. If the external case independently requires semantic separateness or
multiplicity, the common case records only that representation-neutral semantic
distinction; it still does not prescribe candidate internal source or record
cardinality.

Therefore a candidate's fragmentation, consolidation, splitting, grouping, or
internal indirection must not be silently promoted into a shared workload
requirement.

## 11. Conflict Preservation

Both candidate mappings receive the same frozen external semantic conflict
relation.

A candidate mapping must not redefine that relation.

Candidate-specific effects such as internal contention, conflict amplification,
dependency fan-out, state-key contention, record overlap, or scheduling
restrictions may be recorded as candidate outputs.

They do not modify the common conflict relation.

## 12. Ordering Boundary

This document selects no ordering mechanism.

Where the common case or paired manifest contains an ordering relation, both
candidates must preserve the same applicable relation.

Candidate-specific internal scheduling may differ.

A candidate mapping must not introduce or remove externally meaningful ordering
constraints merely to improve its comparative result.

## 13. Replay, Reorganization, and Lifecycle Boundary

This document selects no canonical:

- replay rule;
- nonce;
- one-use mechanism;
- consumption rule;
- duplicate-presentation rule;
- reorganization behavior; or
- lifecycle mechanism.

A mapping must not silently assume any of these.

Experimental duplicate-presentation or reapplication behavior may be supplied
only through an explicit compatible optional profile branch. Under the current
workload methodology, canonical replay protection, nonce semantics, one-use
capability, record-consumption identity, transaction-hash replay identity, and
reorganization semantics remain blocked unless and until higher-authority rules
make them available.

## 14. Authorization and Migration Boundary

Candidate mappings must respect the applicable ownership, authorization, and
migration decision requirements.

A candidate may not gain an apparent advantage by assuming:

- hidden administrative authority;
- privileged rescue;
- unselected recovery;
- unselected delegation;
- unselected native multisignature;
- consensus knowledge of external human intent;
- stable identity not otherwise selected; or
- an alternate authority path unavailable under the shared manifest.

Where migration behavior is tested, the applicable migration branch belongs in
the paired manifest.

The candidate mapping does not choose it independently.

## 15. Cryptographic Profile Boundary

This document selects no cryptographic algorithm.

Where cryptography affects a paired comparison, both mappings receive the same
selected algorithm profile. If algorithm choice itself is an explicitly
controlled variable, each algorithm branch forms a separate paired comparison,
and both candidate mappings within that branch receive the same algorithm
profile.

Candidate-specific counts of signatures, proofs, evidence objects, verification
operations, and cryptographic bytes remain outputs. An explicit shared
experimental profile may fix a representation-neutral external evidence
requirement, but it does not convert candidate-specific counts into common-case
semantics.

Crypto Agility experiments must remain branch-specific and must not silently
turn one algorithm profile into a protocol default.

## 16. State Commitment Boundary

This document selects no state commitment.

A candidate mapping must not hide a commitment construction inside the ordinary
candidate definition and then attribute its effects to Account or UTXO as a
whole.

Commitment-dependent evidence requires an explicitly scoped later artifact or
comparison branch.

## 17. Resource and Failure-Atomicity Boundary

This document selects no canonical resource-accounting mechanism.

It does not mandate:

- scalar gas;
- an attempt meter;
- no-refund accounting;
- fee constants;
- transaction limits;
- block limits; or
- state-growth limits.

Where a resource architecture materially affects comparison, it must be selected
explicitly in the paired manifest.

Failure-atomic external semantic behavior must remain separate from
attempted-work accounting.

A failed experimental effect must not leave an externally committed partial
semantic effect when the frozen case requires unchanged external projection.

## 18. Arithmetic and Monetary Boundary

Experimental mappings must use deterministic, host-independent arithmetic with
explicit overflow and underflow behavior.

This document selects no balance width, fee width, resource-counter width, or
supply-counter representation.

Existing serialization widths do not by themselves determine those arithmetic
domains.

Every experimental value-affecting transition must respect the authoritative
monetary and supply constraints applicable to that transition.

Supply-changing experiments remain blocked or branch-specific until sufficient
authoritative rules exist.

## 19. Anti-Strawman Requirement

Neither candidate may be used as an intentionally weak comparator.

A meaningful paired comparison requires comparable engineering effort.

A mapping intended to support architecture conclusions must not contain obvious
unnecessary disadvantages such as:

- redundant state accesses;
- redundant records;
- unnecessary serialization;
- unnecessary evidence duplication;
- artificial conflicts;
- intentionally disabled obvious optimization; or
- deliberately inefficient validation structure.

A deliberately simple reference mapping may exist for explanation or
attribution, but its reference status must remain explicit.

It must not be presented as an optimized candidate comparison.

## 20. Symmetric Optimization

Both candidates receive equivalent methodological freedom to use
semantics-preserving candidate-native optimizations under the same branch
assumptions.

Potential optimization categories include:

- batching;
- grouping;
- aggregation;
- caching;
- indexing;
- deduplication;
- evidence reuse;
- metadata indirection;
- compact representation;
- scheduling;
- preprocessing; and
- candidate-native specialization.

Symmetry does not require identical internal techniques.

It requires equal opportunity, comparable maturity, equivalent correctness
review, and disclosure of candidate-specific advantages or unavailable
optimizations.

An optimization available to only one mapping may be legitimate evidence.

It must not automatically be generalized into an inherent property of the
entire candidate family.

## 21. Multiple Defensible Mappings

Where materially different credible mappings exist for one candidate, a
comparison intended to support a broad conclusion should evaluate more than one
or explain why omitted mappings cannot affect that conclusion.

A result from one mapping is a result about that mapping.

It must not automatically become a universal statement about all Account-family
or all UTXO-family designs.

Conclusions remain mapping-qualified.

## 22. Preparation and Hidden Work

Material preparation work must be disclosed.

A candidate must not appear cheaper merely because meaningful work occurred
outside the observed interval.

Relevant preparation may include:

- index construction;
- cache warming;
- preprocessing;
- proof preparation;
- metadata construction;
- state compaction;
- record consolidation; and
- database preparation.

Whether a particular measurement includes such work depends on the declared
evidence methodology.

The work must not be hidden.

## 23. Mapping Record

A candidate mapping should identify, directly or by evidence reference:

```text
mapping_format_version
mapping_alias
mapping_content_hash

candidate_family
candidate_variant

semantic_case_hash
paired_manifest_hash

mapping_description
mapping_maturity

optimization_configuration
preparation_configuration

experimental_schema_binding_if_any

unresolved_dependencies
mapping_status

author_or_generator_provenance
```

This is a logical evidence schema only.

It defines no JSON schema, Rust type, wire format, state format, or protocol
object.

Aliases and content hashes provide organization and reproducibility only.

They are not protocol identities.

## 24. Mapping Maturity

A campaign may use descriptive mapping maturity labels such as:

```text
REFERENCE
OPTIMIZED
EXPERIMENTAL
```

These labels are not scores.

A comparison must not hide a material mismatch such as:

```text
Account: OPTIMIZED
UTXO: REFERENCE
```

and then present the result as intrinsic state-model superiority.

Any maturity asymmetry must remain explicit.

## 25. Mapping and Missingness Status

The workload-model status taxonomy is reused without reinterpretation:

```text
MAPPED
MAPPING_INCOMPLETE
BRANCH_NOT_IN_SCOPE
BRANCH_UNSUPPORTED_BY_MAPPING
SHARED_CONTRACT_UNSATISFIED
CASE_UNDERDEFINED
PROFILE_OR_SCHEMA_UNAVAILABLE
EVIDENCE_NOT_COLLECTED
RUN_INFRASTRUCTURE_FAILURE
DETERMINISTIC_MAPPING_FAILURE
```

The status meanings defined in `ACCOUNT_UTXO_WORKLOAD_MODEL.md` are reused
unchanged within this non-normative experimental methodology.

A mapping must not select whichever status makes its candidate appear better.

In particular:

- incomplete work must not be reported as success;
- unsupported branches must not be silently removed;
- infrastructure failure must remain distinct from deterministic mapping
  failure;
- an underdefined shared case applies to both candidates; and
- missing evidence is not measured zero.

## 26. Failure Visibility and Exclusion Rules

A comparison must not use the rule:

```text
if candidate fails:
    remove case
```

Failed, incomplete, unsupported, unavailable, and uncollected results remain
visible according to the workload methodology.

Any exclusion from analysis must be governed by a predeclared rule that applies
symmetrically.

Post-result exclusion designed to improve one candidate's apparent performance
is prohibited.

## 27. Candidate Outputs and Metric Comparability

Candidate mappings and later evidence may expose descriptive outputs such as:

- logical state reads;
- logical state writes;
- internal record counts;
- records created, replaced, or retired;
- authorization evidence objects;
- verification operations;
- dependency structure;
- conflict amplification;
- state-growth projection;
- migration work;
- preparation work;
- validation work; and
- physical benchmark observations.

These quantities are not automatically directly comparable.

For example:

```text
Account logical writes = 2
UTXO created records = 3
```

does not establish that Account is better because `2 < 3`.

A direct comparison requires a shared, reviewed metric meaning.

Candidate-native counters otherwise remain candidate-native evidence.

## 28. Exact Bytes and Experimental Schemas

This document defines no Account or UTXO transaction/state encoding.

Exact-byte claims require an explicit experimental schema that satisfies the
comparison framework's evidence-only and candidate-neutrality requirements.

Malformed byte corpora, canonical transaction encodings, and commitment-specific
representations belong to later scoped artifacts.

Existing Dilithia serialization primitives do not select future Account or UTXO
transaction/state structure.

## 29. Evidence Separation

The following concepts must remain distinct:

1. experimental-contract outcome;
2. candidate mapping status;
3. evidence/run availability; and
4. future protocol validity.

A successful experimental mapping does not prove future protocol validity.

A mapping failure does not by itself prove future protocol invalidity.

A missing run does not prove either semantic success or failure.

Protocol validity remains governed only by the authoritative protocol process.

## 30. Weighting and Ranking Boundary

This document defines no scalar candidate score.

It assigns no weights to:

- bytes;
- reads;
- writes;
- state growth;
- verification work;
- contention;
- latency;
- throughput;
- migration burden; or
- implementation complexity.

It also does not permit:

- winner by case count;
- winner by parameter density;
- treating repeated cases as extra votes;
- cross-family averaging to declare a winner; or
- assuming production workload frequency without evidence.

Those controls remain governed by the workload model and comparison framework.

Where weights do not exist, descriptive or Pareto-style evidence remains
preferable to invented scalar ranking.

## 31. Mapping Freeze and Reproducibility

A material mapping used for comparison should be reproducibly identifiable.

Once evidence has been attributed to a mapping, a material change to semantic
mapping, representation, optimization configuration, preparation configuration,
manifest binding, or experimental schema binding must produce a distinguishable
mapping or experiment identity.

Historical evidence must remain attributable to the configuration that produced
it.

## 32. Deterministic Mapping Generation

A future generator may automate mechanical candidate mapping.

Automation must not resolve an unresolved protocol decision.

If deterministic generation is claimed, identical declared inputs should
produce identical mapping artifacts.

A deterministic mapping failure must not be mislabeled as transient
infrastructure failure.

Non-deterministic generation must record its selection procedure sufficiently to
prevent result cherry-picking.

## 33. Independent Review Gate

Before candidate-mapping evidence is used for architecture conclusions, an
independent review should attempt to identify:

- Account-favouring assumptions;
- UTXO-favouring assumptions;
- semantic drift;
- hidden profile defaults;
- result-dependent manifest changes;
- asymmetric optimization;
- artificial de-optimization;
- hidden preparation work;
- authority-relation drift;
- conflict-relation drift;
- missingness manipulation;
- candidate-native metric misuse; and
- unsupported generalization.

Material findings must be resolved or explicitly represented before the affected
evidence is treated as decision-relevant.

## 34. Security Gate

Mappings touching security-sensitive areas require review appropriate to the
affected area.

Examples include:

- ownership;
- authorization;
- cryptographic evolution;
- migration;
- failure atomicity;
- hostile validation;
- persistent state growth; and
- monetary or supply behavior.

Experimental convenience cannot override applicable Dilithia constitutional or
security requirements.

## 35. Mapping-Qualified Conclusions

Every comparison conclusion must identify the configuration to which it applies.

At minimum, a decision-relevant claim should be attributable to:

```text
semantic case
paired manifest
candidate mapping
material branches
parameter point or sweep
evidence methodology
implementation or generator version
```

A conclusion about one frozen mapping pair must not automatically be generalized
to the entire Account or UTXO family.

## 36. Prohibited Practices

The following practices are prohibited under this methodology:

1. changing the shared semantic problem for only one candidate;
2. selecting a material profile after observing results to favor a candidate;
3. hiding incomplete or unsupported mappings;
4. deleting unfavorable cases after results are known;
5. treating candidate-internal structure as shared semantics rather than
   expressing any independently required external distinction in
   representation-neutral semantic terms;
6. equating authority relations with signature or credential counts;
7. redefining the shared external conflict relation;
8. silently assuming replay, nonce, one-use, or reorg semantics;
9. silently selecting authorization, cryptographic, migration, commitment, or
   resource architecture;
10. intentionally weakening one candidate;
11. optimizing only the preferred candidate without disclosure;
12. hiding material preparation work;
13. comparing unrelated candidate-native counters as if they were one metric;
14. treating case count or sampling density as candidate votes;
15. treating content hashes as protocol identities;
16. treating benchmark results as consensus rules;
17. inferring production frequency from an experimental suite; and
18. treating one mapping as proof of an entire candidate family.

## 37. Pilot Mapping Campaign

The first mapping campaign should remain small.

Its purpose is to validate the methodology before attempting a broad state-model
comparison.

A pilot may use a limited set of already permitted candidate-neutral workload
families, for example:

- value-effect baseline;
- authority-relation multiplicity;
- semantic conflict;
- failure-atomic external projection;
- semantic population or state-growth sequence; and
- dormancy under an explicit profile.

The pilot does not define a canonical corpus or production workload
distribution.

## 38. Pilot Readiness Gate

Before expanding the campaign, review should confirm that:

- both candidates received the same semantic cases;
- material profile choices were frozen;
- mappings preserved external semantics;
- unsupported mappings remained visible;
- optimization treatment was symmetric;
- preparation work was disclosed;
- candidate-native metrics were not falsely equated;
- missingness statuses were applied consistently;
- evidence remained reproducible; and
- no hidden ranking function emerged.

Failure of these controls requires methodology correction before broader
candidate conclusions are drawn.

## 39. State-Model Decision Boundary

Successful candidate mapping does not select a state model.

Mapping evidence is only one input to a later state-model decision.

A later decision must still satisfy the applicable requirements for:

- correctness;
- deterministic behavior;
- security;
- authorization;
- migration;
- Crypto Agility;
- state growth;
- conflict behavior;
- resource behavior;
- implementation independence;
- reproducibility; and
- unresolved architecture branches.

No metric in this document is declared controlling.

## 40. Current Project Impact

Creation of this document alone has the following effect:

```text
Account selected:
NO

UTXO selected:
NO

Hybrid selected:
NO

State-model decision:
NOT MADE

Formal Specification update:
NOT READY

PROJECT_STATE update:
NOT JUSTIFIED

THREAT_MODEL update:
NOT JUSTIFIED

Consensus implementation change:
NONE
```

The next permitted analytical step is to construct a small set of frozen paired
manifests and candidate mappings under these requirements, independently review
them, and only then begin collecting comparison evidence.
