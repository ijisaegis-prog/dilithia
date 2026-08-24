# Dilithia Account/UTXO Workload Model

> **NON-NORMATIVE EXPERIMENTAL WORKLOAD METHODOLOGY**
>
> This document defines a candidate-neutral method for preparing future
> Account-family and UTXO-family comparison evidence. It does not define
> protocol behavior, protocol validity, a transaction or state model, or a
> candidate ranking. If it conflicts with the Dilithia Constitution, Formal
> Specification, or a ratified HIP/Super HIP, the authoritative protocol
> documents take precedence.

## 1. Status, Authority, and Scope

Dilithia is Pre-Genesis. The Formal Specification's transaction, state,
consensus, Crypto Agility, governance, and mechanical HIP / Super HIP sections
remain pending. The current implementation contains canonical serialization
work but no decision-ready transaction, state, authorization, replay,
commitment, resource-accounting, or consensus implementation.

The authority order applied by this document is:

1. Dilithia Technical Constitution
2. Dilithia Formal Specification
3. Ratified HIP / Super HIP material, if any
4. Normatively adopted conformance material, where applicable
5. Implementation
6. `PROJECT_STATE.md`
7. Non-normative design, comparison, threat, resource, and benchmark documents
8. AI or other design discussion

Terms such as **must**, **required**, and **prohibited** below describe controls
for a credible experiment under this non-normative methodology. They do not
create current consensus rules or protocol requirements.

This document complements and is subordinate to
`ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`. It defines semantic workload and suite
methodology only. The comparison framework remains the source for candidate-
family definitions, model-label prohibitions, anti-strawman controls, symmetric
optimization permissions, hostile-work and failure-atomicity boundaries,
arithmetic and monetary boundaries, Crypto Agility, Article 7 and migration
boundaries, and future ranking interpretation.

## 2. Purpose and Core Direction

The purpose is to describe the same external experimental semantic problem for
both candidate families without prescribing either candidate's internal
representation.

The required direction is:

```text
common semantic template or case
    -> paired comparison manifest
    -> candidate-specific mapping
    -> evidence record
```

It is not an Account workload compared with a separately constructed UTXO
workload. Both mappings begin from the identical content-addressed semantic
case, paired profile selections, and parameter values.

The methodology may organize candidate-neutral semantic templates, cases,
suites, profile bindings, mappings, and evidence. It does not select a state
model, establish a production workload distribution, or authorize protocol
adoption.

## 3. Protocol-Authority Boundary

An experimental semantic contract may define only non-normative predicates over
abstract, case-local semantic terms. It must not define:

- protocol byte validity;
- state-layout validity;
- authorization-evidence formats;
- replay validity;
- consensus-ordering validity;
- resource-limit validity;
- transaction or state-transition formats; or
- protocol activation behavior.

Protocol validity is **UNSPECIFIED BY THIS WORKLOAD MODEL**. A result under an
experimental contract is not evidence that the same object would be accepted
or rejected by a future Dilithia protocol.

No workload, profile, mapping, suite, generator, oracle, corpus, hash, or result
defined under this methodology gains authority through implementation,
benchmark success, repetition, publication, or adoption by an evidence
campaign.

## 4. Candidate-Neutral Semantic Vocabulary

The core semantic vocabulary is deliberately representation-neutral.

**Pre-effect value term:** a case-local pair consisting conceptually of an
opaque label and an amount, used only inside one experimental semantic equation.

**Post-effect value term:** a case-local term used only to express a required
postcondition.

**Semantic effect equation:** a declared relation between pre-effect terms and
post-effect terms under the experimental contract.

**External semantic projection:** the abstract information observed by the
experimental contract before or after an effect, excluding candidate internals.

A pre-effect or post-effect term is not:

- a persistent record;
- an Account or UTXO;
- an owner or address;
- a transaction input or output;
- a candidate state key; or
- a reusable protocol identity.

Term labels are local to one case and carry no identity, ownership, lifecycle,
replay, or consumption semantics. The workload model does not use contribution,
source, recipient, destination, fan-in, fan-out, consolidation, balance
mutation, nonce churn, or value position as core workload primitives.

## 5. Experimental-Contract Disposition and Status Separation

The experimental-contract disposition vocabulary is:

| Disposition | Meaning inside the frozen experimental contract |
|---|---|
| `CONTRACT_CONDITIONS_MET` | The declared case-local conditions and required predicates are satisfied. |
| `CONTRACT_CONDITIONS_NOT_MET` | At least one declared case-local condition is not satisfied. |
| `CONTRACT_OUTCOME_CONDITIONAL` | The contract outcome depends on an explicitly selected profile or branch. |
| `CONTRACT_OUTCOME_UNSPECIFIED` | The experimental contract intentionally provides no outcome. |
| `BLOCKED_BY_AUTHORITY_TBD` | A higher-authority unresolved decision prevents a defensible experimental outcome. |

These values do not describe protocol acceptance or rejection. In particular,
this document does not use `VALID`, `INVALID`, `ACCEPT`, `REJECT`,
`EXPECTED_ACCEPT_UNDER_PROFILE`, or `EXPECTED_REJECT_UNDER_PROFILE` as workload
dispositions.

Every evidence package keeps four concepts separate:

1. **Experimental-contract outcome:** one disposition from the table above.
2. **Candidate mapping status:** whether and how the mapping represents the
   frozen semantic case.
3. **Protocol validity:** always unspecified by this methodology.
4. **Evidence or run availability:** whether evidence exists and whether its
   collection completed.

A condition not met under the semantic contract is not a candidate mapping
failure. A mapping failure is not a statement of future protocol validity. An
unavailable run is not evidence that contract conditions were or were not met.

## 6. Four-Layer Artifact Architecture

The methodology preserves four distinct layers:

1. Common semantic template or content-addressed case
2. Paired comparison manifest
3. Candidate-specific mapping
4. Evidence record

Each layer has a separate identity and responsibility. A field owned by a later
layer must not be copied into an earlier layer merely for convenience. In
particular, optional architecture and policy selections cannot become immutable
properties of a supposedly candidate-neutral semantic template.

## 7. Common Semantic Template or Case

The common layer describes external semantics only. Its logical,
non-serialized schema may include fields conceptually equivalent to:

- `non_authoritative_alias`;
- `workload_model_version`;
- `case_content_hash`;
- `experimental_semantic_contract_id`;
- `precondition_terms`;
- `required_postcondition_predicate`;
- `required_authority_relations`;
- `intrinsic_external_conflict_relations`;
- `parameter_slots`;
- `optional_intrinsic_sequence_or_partial_order`;
- `experimental_contract_disposition`; and
- `external_projection_predicates`.

An external conflict relation or sequence belongs in the case only when it is
intrinsic to the content-hashed semantic contract. A profile-dependent version
belongs in the paired manifest. Changing an intrinsic relation changes the case
content and hash.

The common layer must not contain:

- Account or UTXO records;
- transaction inputs, outputs, or fields;
- a nonce or one-use identifier;
- candidate state keys or state layouts;
- credential placement;
- credential, signature, proof, or verification-operation counts;
- candidate internal source counts;
- candidate state reads or writes;
- candidate conflict sets;
- candidate schema identifiers;
- algorithm selections;
- resource architecture;
- migration policy;
- commitment structure; or
- implementation measurements.

This schema is descriptive only. It defines no JSON representation, Rust type,
wire encoding, binary layout, or protocol schema.

## 8. Paired Comparison Manifest

The paired manifest binds the common case to explicit experimental assumptions.
Its logical schema may include:

- `case_content_hash`;
- `assumption_profile_id`;
- `policy_branch_set`;
- `architecture_branch_set`;
- `algorithm_profile_id`;
- `ordering_profile_id`;
- `version_profile_id`;
- `instantiated_parameter_manifest`;
- `candidate_mapping_ids`; and
- `suite_manifest_id`.

Both candidate mappings receive the identical case hash, selected profiles,
branch set, and parameter values.

Every material profile or branch must be explicit. This document defines no
preferred default algorithm, migration, authorization, conflict, ordering,
state-growth, commitment, or resource profile. An unbound material dimension
means that comparison is not ready under that dimension; it does not activate
an implicit default.

## 9. Candidate-Specific Mapping

A mapping records how one candidate represents the frozen common case under the
paired manifest. Its logical schema may include:

- `mapping_id`;
- `candidate_family`;
- mapping description;
- optimization configuration;
- optional binding to an external experimental schema; and
- requested candidate-output categories.

Candidate-specific representation, internal cardinality, state access, record
shape, maintenance, conflict realization, evidence placement, and validation
workflow belong here or in later evidence—not in the common case.

Mappings must satisfy the anti-strawman and symmetric-optimization controls in
`ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`. They must be frozen before comparative
results are examined. Materially different defensible mappings are evaluated or
their omission is justified. Conclusions remain mapping-qualified.

## 10. Evidence Record

An evidence record binds observed or proved information to a frozen case,
manifest, and mapping. Its logical schema may include:

- case content hash;
- paired-manifest hash;
- mapping identifier;
- experimental-contract outcome;
- mapping and missingness status;
- observed candidate outputs;
- proof, test, or benchmark provenance;
- run status; and
- environment linkage where applicable.

Evidence records retain failed, incomplete, unsupported, unavailable, and
deviating outcomes. A missing or unfavorable result must not be silently removed
from a suite or report.

## 11. Evidence-Only Identity and Versioning

Workload identities are non-authoritative evidence identifiers. A safe identity
uses:

- a human-readable organizational alias;
- workload-model version;
- case content hash;
- suite-manifest hash; and
- optional generator identity, version, and hash.

These identifiers are not transaction identifiers, replay identifiers, state
keys, chain objects, consensus commitments, or protocol object identities.

Immutability means evidence reproducibility only. If a registered case changes:

- the old content and hash remain discoverable;
- corrected content receives a new hash;
- an alias may remain an organizational catalog label; and
- campaigns bind the exact content hash, not the mutable meaning of an alias.

No protocol immutability, semantic permanence, or activation consequence is
implied.

A change to external preconditions, required postconditions, authority
relations, or intrinsic external conflict semantics creates new case content and
a new hash. A change only to algorithm profile, candidate representation,
optimization, resource architecture, or evidence mechanism uses the same
semantic case with a different paired manifest. Ordering, migration, or
authorization-profile changes create a new case only when they alter the
external semantic contract. Profile variants do not gain extra ranking weight.

## 12. Parameter Classification and Term Multiplicity

The following classifications prevent candidate internals from becoming common
workload assumptions.

| Classification | Parameters or outputs |
|---|---|
| SAFE COMMON PARAMETER | Semantic effect count; authority-relation count `N`; repetition count; externally defined conflict density; adversarial semantic population |
| SAFE COMMON ONLY WHEN EXTERNALLY DISTINGUISHABLE | Pre-effect term count; post-effect term count |
| PROFILE PARAMETER | Credential multiplicity; dormancy population; crypto-version proportions; lifecycle or logical-step length; oracle-relative condition-decision position |
| CANDIDATE OUTPUT | Internal source count; fragmentation; candidate validation-stage position; reads; writes; records; signatures; proofs; internal conflicts; bytes; performance |
| BLOCKED | Generic semantic contribution count; numeric resource limits |

Multiple pre-effect terms may be common semantic inputs only when their
separateness is externally meaningful independently of Account or UTXO.
Potentially defensible grounds include independently required authority
relations, separate temporal obligations, or separately declared semantic
contract conditions.

If separateness is not externally justified, the common case uses an aggregate
semantic amount. Candidate internal source count and fragmentation remain
outputs, while consolidation or comparable upkeep remains candidate-specific
maintenance. A later experiment may measure the maintenance needed to sustain
the same external workload without forcing either candidate to emulate the
other's internal cardinality.

"Early" and "late" refer only to a position in a frozen abstract semantic
workflow. Candidate implementation-pipeline position is an output.

## 13. Consensus Authority Relations

The conceptual boundary is:

```text
N = number of independently required consensus authority relations
```

Each relation may expose only:

- an opaque, case-local relation identifier;
- the experimental semantic predicate it independently gates; and
- whether that experimental condition is met.

`N` is not the number of humans, owners, organizations, credentials, keys,
signatures, proofs, threshold shares, or verification operations. The statement
that all required relation conditions are met defines no evidence structure.

Credential multiplicity is not part of the common case. Native support for
`N > 1` remains an architecture branch in the paired manifest. This document
defines no multisignature, k-of-m rule, threshold cryptography, evidence
grouping, or signing mechanism.

## 14. Shared External Conflict and Ordering

Both candidates receive the identical external semantic conflict relation. An
edge means only:

> Two abstract semantic effects cannot coexist within the same versioned
> experimental semantic contract under the declared paired-manifest
> assumptions.

Pure mutual-exclusion edges are symmetric. Order-sensitive cases use an
explicit sequence or partial-order assumption. No consensus conflict rule is
inferred, and candidate mappings may not redefine external conflict. Candidate
representation-induced conflict breadth or amplification is an output.

No candidate stereotype such as local UTXO conflict or broad Account contention
is permitted.

No ordering profile is a default. Experimental ordering vocabulary may include:

- `NO_ORDER_ASSERTION`;
- `DECLARED_SEQUENCE`; and
- `DECLARED_PARTIAL_ORDER`.

Material ordering selection belongs in the paired manifest unless the relation
is intrinsic to the content-hashed semantic case. A serial experimental oracle
does not imply serial consensus execution, block ordering, or a consensus
ordering protocol.

## 15. Repetition, Duplicate Presentation, Replay, and Reorganization

The workload model keeps these concepts distinct:

- separately declared repeated independent semantic effects;
- duplicate presentation;
- replay;
- one-use capability;
- reapplication;
- rollback; and
- chain reorganization.

Safe common cases may contain repeated independent effects. They may also
describe duplicate presentation only when its protocol outcome remains
unspecified.

Experimental duplicate or reapplication behavior is permitted only as an
explicit optional profile branch.

Canonical replay protection, nonce semantics, one-use capability,
record-consumption identity, transaction-hash replay identity, and reorganization
semantics are blocked. The common semantic contract must not state that an
already exercised capability cannot be exercised again.

## 16. Failure Atomicity and Hostile Validation

The safe abstract failure-atomic form is:

```text
initial external semantic projection
    -> attempted experimental effect whose CONTRACT CONDITIONS ARE NOT MET
    -> unchanged external semantic projection
```

This defines no rollback implementation, journal, database transaction,
candidate-attempt meter, reservation, refund rule, or local ingress rule.

The only universal hostile-work property used here is:

> Untrusted validation, including rejected or unsatisfied cases, must have
> bounded worst-case resource exposure under future authoritative resource
> rules.

Candidate-attempt meters, monotonic accounting, conservative reservation,
no-refund behavior, counter architecture, and numeric limits are resource-
architecture branches, not universal requirements of this workload model.

Safe abstract hostile shapes may include large semantic-clause populations,
many authority relations, conflict-heavy semantic graphs, adversarial semantic
sequences, and oracle-relative condition-decision positions. Valid-case limits
alone are not evidence that hostile unsatisfied cases are bounded.

## 17. Malformed Evidence and Experimental Grammar Boundary

The common workload may express only whether an authority-relation condition is
met. It does not define actual malformed authorization evidence.

Malformed-evidence experiments require a separately reviewed, explicitly
non-normative evidence-grammar profile containing:

- a version;
- a content hash;
- an algorithm profile;
- a syntactic or cryptographic classification; and
- identical paired-comparison assumptions.

Syntactic malformation, cryptographic malformation, unsupported algorithm
version, and an unsatisfied cryptographic proof must not be conflated. Until the
required grammar and profiles exist, these experiments are deferred. No
signature or credential format is invented here.

## 18. Exact Bytes, Representation, and Commitment Boundary

This workload model defines no exact-byte workload family, malformed-byte case,
candidate transaction encoding, or candidate state encoding.

Exact-byte, malformed-representation, or schema-dependent experiments belong in
later candidate-mapping or registered-campaign artifacts. They require an
explicitly non-normative, versioned, content-addressed experimental schema bound
to the candidate mapping, semantic profile, assumption profile, and symmetric
feature contract. Repeated use creates no Formal Specification precedent.

This workload model also contains no commitment-specific common case.
Commitment comparison belongs in a separate later artifact. Deferring commitment
evidence does not imply that Dilithia will lack a commitment.

## 19. Dormancy, Crypto Evolution, and Catastrophic Boundaries

Dormancy may be modeled only as:

> A case-local semantic value cohort for which no declared authority relation is
> exercised over `L` logical steps under an explicit profile.

This implies no persistent owner, Account identity, UTXO identity, recovery
metadata, or stable protocol object. `L` has no default or normative range.

Level C remains an unresolved policy branch. Level D remains a conditional
branch. Catastrophic primitive failure is an information boundary only.
Independent pre-existing distinguishing information or authority may be stated
as a condition but is not a selected mechanism. No case assumes stable identity,
alternate credentials, recovery architecture, or human intent known to
consensus.

The workload model selects no cryptographic algorithm. Algorithm profiles belong
in paired manifests, and both mappings receive the same profile. Optional
batching, aggregation, or recovery features appear only where the declared
algorithm actually supports them and are evaluated as separate branches.
Unequal benefit under symmetric availability is legitimate, algorithm-feature-
dependent evidence.

Candidate artifact, proof, and verification-operation counts remain outputs
unless an explicit architecture profile fixes an external evidence requirement.
Identical internal counts are not required.

## 20. State-Growth Workloads

Safe common state-growth inputs are limited to:

- semantic population size;
- an external semantic sequence;
- active or dormant case-local cohorts; and
- a version-transition semantic relation only when it is intrinsic to the
  content-hashed external semantic case.

Selected version-transition branches and version profiles are not common
semantic inputs. They belong in the paired comparison manifest unless the
version relation changes the external semantic contract itself, in which case
the changed external semantics produce new case content and a new case hash.

Candidate outputs may later include logical record population, persistent
logical bytes, metadata, created, removed, or replaced records, historical
retention, logical reads or writes, and physical evidence.

The common workload assumes no Account persistence, UTXO deletion, pruning,
history retention, commitment retention, or relative record count. Logical
measurements become candidate outputs only after equivalent experimental
mappings and their definitions are frozen.

## 21. Arithmetic and Monetary Boundaries

Every experimental mapping uses deterministic, host-independent arithmetic
domains with explicit overflow and underflow behavior. This methodology selects
no balance width, counter width, counter representation, or counter lifecycle.

Every value-affecting experimental transition must satisfy the authoritative
monetary and supply invariants applicable to that transition. A value-preserving
case may use an explicit, non-normative, case-scoped conservation assumption.
That assumption does not create a universal DLTH conservation rule.

Issuance, reward, burn, fee, and other supply-changing cases remain blocked or
explicitly branch-specific until authoritative monetary rules support them.
This document defines no monetary policy.

## 22. Workload Family Taxonomy

Family names are organizational labels only. They provide no protocol or
ranking authority and do not describe deployment frequency.

Safe neutral family labels include:

- `value-effect-baseline`;
- `post-term-multiplicity`;
- `externally-distinguished-pre-term-multiplicity`;
- `repeated-semantic-effects`;
- `value-magnitude-sweep`;
- `authority-relation-multiplicity`;
- `semantic-conflict-pair`;
- `semantic-conflict-density`;
- `semantically-independent-effect-set`;
- `semantic-sequence`;
- `contract-unsatisfied`;
- `failure-atomic-external-projection`;
- `semantic-population-sequence`; and
- `hostile-exposure`.

Multiple pre-effect terms are included only when externally grounded under
Section 12.

Branch-specific families may explore grouped authorization, mixed cryptographic
versions, dormancy, authority participation, Level C, Level D or catastrophic
information boundaries, and metadata or versioning profiles.

Malformed-evidence grammar, exact-byte, malformed-representation, and commitment
experiments move to later artifacts. Canonical replay, reapplication, and
reorganization cases remain blocked.

Family names such as ordinary, input-heavy, recipient-count, consolidation,
balance mutation, and nonce churn are not used. No family is preferred, and the
taxonomy need not be exhaustive.

## 23. Suite Hierarchy and No Fixed Corpus

The artifact hierarchy is:

```text
workload model
    -> common semantic family or template
    -> content-addressed semantic case
    -> paired comparison manifest
    -> candidate-specific mapping
    -> evidence record or campaign
```

The workload model owns semantic case schemas, neutral family taxonomy,
suite-manifest schemas, pairing rules, sweep rules, missingness, and suite-
weighting controls.

This document defines no fixed, canonical, required, or production-frequency
corpus. It may provide templates, neutral named families, and explicitly non-
binding examples. Examples carry no decision weight and do not estimate
deployment prevalence.

## 24. Suite Weighting and Reporting Controls

The following are prohibited:

- declaring a winner by case count;
- cross-family aggregate candidate preference;
- treating denser parameter sampling as greater importance;
- treating repeated variants as additional decision weight;
- treating example count as workload frequency;
- using ungrounded production-frequency assumptions; and
- averaging heterogeneous families, profiles, or branches to declare a winner.

Descriptive aggregation is permitted only within one frozen homogeneous
combination of family, profile, and branch, and only for evidence summarization.
It must not become a ranking function.

Reports preserve results by family, profile, branch, and parameter point. Suite
composition is frozen before results. Duplicated or correlated cases are
disclosed. Alternative defensible suites may be used as sensitivity analysis.

The document does not characterize any workload as typical, common, normal
usage, representative of most users, or a realistic distribution without a
future registered evidence source.

A future weighting or decision function requires a separate authoritative or
otherwise explicitly adopted ranking methodology. The absence of a scalar score
today is a methodology choice, not a constitutional or permanent prohibition.

## 25. Parameter Sweeps and Deterministic Generators

A paired sweep requires:

- identical parameter points for both mappings;
- the identical generator where applicable;
- the same case hash;
- the same paired profile selections;
- predeclared sweep ranges and distributions;
- preservation of failed and unsupported points;
- separate result families when branch selections change;
- a new evidence or campaign identity for post-result range changes; and
- no interpretation of sampling density as importance.

This document chooses no numerical range.

Future deterministic-generator metadata may include generator version and hash,
workload-model version, profile identifiers, parameter manifest, seed, generated
case hashes, and suite hash. No generator is implemented here. A generation
distribution is an explicit suite-weighting choice and does not describe
deployment frequency.

## 26. Mapping, Missingness, and Run-Status Taxonomy

The neutral status taxonomy is:

| Status | Meaning and required treatment |
|---|---|
| `MAPPED` | The mapping represents the frozen case under the paired manifest. |
| `MAPPING_INCOMPLETE` | Work remains incomplete; a reason is required, material use requires independent review, and the case cannot support ranking or coverage claims. |
| `BRANCH_NOT_IN_SCOPE` | The branch was explicitly excluded before results; the status is retained and is not candidate failure. |
| `BRANCH_UNSUPPORTED_BY_MAPPING` | The frozen selected branch cannot be represented by that mapping; this is branch-qualified evidence, not automatic global candidate failure. |
| `SHARED_CONTRACT_UNSATISFIED` | The frozen common contract or oracle determines that the shared experimental requirements are not satisfied. |
| `CASE_UNDERDEFINED` | The common case cannot support comparison; both candidate results are invalidated for this case. |
| `PROFILE_OR_SCHEMA_UNAVAILABLE` | A required explicit profile or external experimental schema does not exist or cannot be bound. |
| `EVIDENCE_NOT_COLLECTED` | The mapping may exist, but the requested evidence has not been collected. |
| `RUN_INFRASTRUCTURE_FAILURE` | Evidence collection failed because of campaign, host, tool, or environment failure. |
| `DETERMINISTIC_MAPPING_FAILURE` | The frozen mapping reproducibly fails under the frozen case and manifest independently of transient run infrastructure. |

`MAPPING_INCOMPLETE` cannot remain indefinitely when a frozen selected branch is
repeatedly unsupported. Persistent inability must be classified as
`BRANCH_UNSUPPORTED_BY_MAPPING`.

`SHARED_CONTRACT_UNSATISFIED` is determined only against the frozen common
contract or independently reviewed semantic oracle. It cannot be assigned from
candidate preference.

`CASE_UNDERDEFINED` applies to both candidates, cannot be assigned selectively
after results are seen, and requires new case content and a new hash when fixed.

Infrastructure failure remains distinct from deterministic mapping failure.
Every status remains visible in evidence reports. Failure to implement an
unselected optional branch is not global candidate failure, while failure to
satisfy an applicable shared hard requirement cannot be hidden as missing data.

## 27. Mapping and Algorithm Fairness

The workload model locally requires identical case hashes, profile selections,
and parameter values, plus preservation of mapping and missingness statuses. It
references `ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md` for mapping freeze,
comparable engineering effort, symmetric optimization, multiple defensible
mappings, and mapping-qualified conclusions.

Algorithm profiles can themselves bias results. Both mappings therefore receive
the same algorithm profile and the same permission to use an optional feature
when semantically applicable and actually supported. Feature-enabled and
feature-disabled configurations are separate branches. A feature need not be
forced onto an algorithm that lacks it, and internal artifact counts need not
be identical.

Unequal benefit from a symmetrically available feature is recorded as branch-
and algorithm-dependent evidence. It is not generalized automatically to an
entire candidate family.

## 28. Expected Outcomes, Oracle, and Benchmark Boundaries

Safe common experimental outcomes include:

- contract conditions met or not met;
- required pre-effect and post-effect semantic predicates;
- authority-relation predicate results;
- the shared external conflict relation; and
- an unchanged external semantic projection when contract conditions are not
  met.

Candidate reads, writes, records, credentials, signatures, proofs, exact bytes,
internal conflicts, timing, and mapping support are not expected common outputs.

This document defines only an abstract boundary for a possible future semantic
oracle. An oracle implementation belongs in a separately reviewed artifact. It
must be explicitly non-normative, candidate-neutral, limited to semantic
projection, versioned, content-addressed, evidence-only, and independently
reviewed. Repeated use creates no protocol or conformance authority.

`BENCHMARK_METHODOLOGY.md` remains the owner of campaign registration, pre-run
receipts, run manifests, hardware and environment records, statistical methods,
provenance, raw-result retention, and physical benchmark reporting. Those
controls provide auditability only and gain no protocol authority through this
document.

## 29. TBD and Non-Selection Register

This workload model does not select or define:

- Account, UTXO, or a hybrid model;
- state layout or transaction format;
- balance, ownership, or authorization representation;
- credential placement or signing mechanics;
- replay mechanism, nonce, one-use capability, or record-consumption semantics;
- conflict protocol or consensus ordering;
- commitment structure;
- migration, recovery, Level C, or Level D mechanisms or policies;
- cryptographic algorithm or registry structure;
- resource meter, unit, counter architecture, or limit;
- database or storage backend;
- benchmark hardware or production workload frequency;
- fixed corpus, corpus weights, scalar score, candidate ranking, or protocol
  adoption.

All higher-authority TBDs remain unresolved. In particular, transaction, state,
consensus, Crypto Agility, governance, HIP / Super HIP mechanics, `ChainId`,
`NetworkId` discriminants, domain tags, resource accounting, resource limits,
fees, ownership, authorization, replay, commitments, migration, and recovery
remain outside this document.

Project-document impact is:

- `PROJECT_STATE` update: **NOT JUSTIFIED NOW**
- `THREAT_MODEL` update: **NOT JUSTIFIED NOW**
- Formal Specification update: **NOT READY**

This artifact is non-normative workload and evidence methodology only. It
establishes no implementation milestone, new generic protocol attack class, or
state, transaction, authorization, replay, conflict, commitment, migration,
resource, monetary, or consensus rule.

## 30. Readiness and Conclusion

The methodology is ready for later experimental case drafting only after every
case demonstrates:

- candidate-neutral external semantics;
- correct four-layer placement;
- explicit material profiles without defaults;
- externally justified term multiplicity;
- shared conflict and ordering assumptions;
- visible mapping and missingness status;
- no hidden suite weighting or frequency claim;
- no schema, commitment, replay, or resource-rule leakage; and
- independent review appropriate to the intended evidence claim.

Before any actual workload artifact, suite, mapping, or result is used for a
candidate-ranking claim, it must also satisfy the applicable comparison-
framework and benchmark-methodology gates.

Account and UTXO remain co-equal analytical native-value-accounting hypotheses.
This document defines a method for constructing shared experimental semantic
problems and evidence; it supplies no candidate result, preference, winner, or
protocol direction.
