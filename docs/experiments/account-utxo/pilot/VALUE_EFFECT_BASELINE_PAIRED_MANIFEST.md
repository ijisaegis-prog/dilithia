# Account–UTXO Pilot Paired Comparison Manifest: Value-Effect Baseline

> **NON-NORMATIVE EXPERIMENTAL PAIRED COMPARISON MANIFEST**
>
> **Status: FROZEN PILOT PAIRED COMPARISON MANIFEST**
>
> This document binds the frozen `value-effect-baseline` common semantic case to
> one explicit, candidate-neutral pilot comparison configuration.
>
> It is not a protocol specification, transaction definition, state-model
> definition, authorization mechanism, cryptographic profile for production,
> resource-accounting rule, benchmark result, or candidate ranking.
>
> If this document conflicts with the Dilithia Constitution, Formal
> Specification, or ratified HIP / Super HIP material, the authoritative protocol
> material prevails.
>
> Within the non-normative Account/UTXO comparison methodology, this manifest is
> subordinate to:
>
> - `ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`;
> - `ACCOUNT_UTXO_WORKLOAD_MODEL.md`;
> - `ACCOUNT_UTXO_CANDIDATE_MAPPING_REQUIREMENTS.md`; and
> - the frozen `VALUE_EFFECT_BASELINE_CASE.md` identified below.

## 1. Purpose

This file defines one paired comparison manifest for the first
`value-effect-baseline` pilot case.

Its purpose is intentionally narrow:

- bind both candidate mappings to the same frozen common semantic case;
- instantiate the same semantic amount parameter for both candidates;
- make every material in-scope experimental assumption explicit;
- explicitly mark material dimensions that are excluded or blocked;
- prevent hidden defaults;
- reserve one Account mapping identity and one UTXO mapping identity;
- restrict this first pilot to reference mappings;
- preserve the common case's required non-identical external semantic
  projection; and
- prepare for candidate-mapping review without collecting comparative evidence.

This manifest does not select Account, UTXO, or Hybrid as the Dilithia state
model.

It does not claim that this case is typical, common, production-representative,
or decision-controlling.

## 2. Artifact-Layer Boundary

The experimental artifact flow for this pilot is:

```text
Frozen Common Semantic Case
            |
            v
Paired Comparison Manifest
            |
            +--------------------+
            |                    |
            v                    v
   Account Mapping          UTXO Mapping
            |                    |
            +---------+----------+
                      |
                      v
                   Evidence
```

This file defines only the **Paired Comparison Manifest** layer.

Candidate-specific representation, internal record cardinality, state access,
storage shape, preparation work, validation workflow, conflict realization,
exact bytes, and implementation measurements do not become shared assumptions
merely by appearing in a later mapping.

## 3. Manifest Identity

Human-readable organizational alias:

`pilot/value-effect-baseline/paired-manifest/v1`

Current status:

`FROZEN`

This alias is non-authoritative.

It is not:

- a transaction identifier;
- a replay identifier;
- a state key;
- a consensus commitment;
- a protocol object identifier; or
- a permanent state-model identity.

A content hash is not embedded in this frozen file.

This frozen manifest may receive an evidence-only content hash calculated over
the frozen artifact according to the reviewed experimental identity procedure.

That hash creates no protocol authority.

## 4. Frozen Common Semantic Case Binding

Common semantic case file:

`VALUE_EFFECT_BASELINE_CASE.md`

Common semantic case organizational alias:

`pilot/value-effect-baseline/v1`

Frozen case evidence-only content identity:

`sha256:1DB6FBE63C13BFB619BFECE7D61D0A9958F00E2BF8866331A4602BA0240B276D`

Both candidate mappings must bind exactly that frozen case content identity.

A different case hash is a different semantic case for evidence purposes and
must not be silently substituted into this manifest.

The case hash is not a transaction hash, replay identity, consensus commitment,
or protocol object identity.

## 5. Shared Semantic Contract

Both candidate mappings receive the same frozen external semantic contract.

The required pre-effect external semantic projection is:

```text
term   = pre_value_0
amount = A
```

The required post-effect external semantic projection is:

```text
term   = post_value_0
amount = A
```

The common semantic effect is `E0`.

The amount is preserved.

The required external semantic projection changes.

An unchanged external semantic projection does not satisfy the case merely
because the numeric amount remains equal to `A`.

Neither candidate may satisfy the required projection change solely by
relabeling otherwise unchanged mapped evidence.

This manifest does not change, weaken, or extend that frozen semantic contract.

## 6. Instantiated Parameter Manifest

The only semantic parameter instantiated by this pilot manifest is:

```text
A = 1
```

Interpretation:

`1` means one **case-local experimental semantic amount unit**.

It does not mean:

- one DLTH;
- one future DLTH base unit;
- one atom;
- one coin;
- one protocol balance unit;
- one byte;
- one record;
- one Account;
- one UTXO; or
- one transaction input or output.

No protocol denomination is selected.

Both candidate mappings receive exactly the same instantiated `A = 1`.

No parameter sweep is part of this manifest.

A later amount sweep requires a separately frozen manifest or suite artifact
with predeclared parameter points.

## 7. Assumption Profile

Assumption profile identifier:

`pilot/value-effect-baseline/assumptions/semantic-mapping-only/v1`

This identifier is local to this non-normative pilot.

The assumption profile binds the following shared conditions:

1. the comparison is a semantic-mapping pilot, not a protocol-validity test;
2. both candidates receive the same frozen case hash;
3. both candidates receive `A = 1`;
4. candidate-specific internal cardinality is an output, not a shared input;
5. candidate-specific storage shape is an output, not a shared input;
6. candidate-specific state-access shape is an output, not a shared input;
7. no candidate-specific external information may be assumed;
8. no production-frequency claim is made;
9. no candidate winner is defined by this manifest;
10. no benchmark evidence is collected under this frozen manifest; and
11. higher-authority Dilithia constraints remain unaffected.

Protocol validity is:

`UNSPECIFIED BY THIS MANIFEST`

## 8. Experimental-Contract Outcome Binding

This manifest reuses the frozen case's experimental-contract dispositions.

If the frozen case-local conditions and required changed external projection are
satisfied:

`CONTRACT_CONDITIONS_MET`

If a required case-local condition or predicate is not satisfied:

`CONTRACT_CONDITIONS_NOT_MET`

These are experimental-contract outcomes only.

They do not mean protocol `VALID`, `INVALID`, `ACCEPT`, or `REJECT`.

Mapping status, protocol validity, and evidence availability remain separate
concepts.

## 9. External Conflict Binding

Frozen intrinsic external conflict relation:

`EMPTY`

Reason:

The frozen case contains one semantic effect and defines no intrinsic pairwise
external conflict edge.

This does not mean either candidate has no internal conflicts.

Candidate-specific contention, dependency breadth, key overlap, record overlap,
or scheduling restrictions remain mapping properties or evidence outputs.

Neither candidate may invent a different shared external conflict relation.

## 10. Ordering Profile

Ordering profile identifier:

`NO_ORDER_ASSERTION`

This manifest introduces no ordering relation beyond the logical
before/`E0`/after relation already intrinsic to the frozen semantic case.

It selects no:

- consensus ordering mechanism;
- transaction ordering rule;
- block ordering rule;
- scheduler;
- parallel execution rule;
- serialization order; or
- conflict-resolution mechanism.

Candidate-internal execution or scheduling structure may differ and must not be
promoted into shared external semantics.

## 11. Version Profile

Version profile identifier:

`pilot/no-protocol-version-semantics/v1`

Meaning:

- this pilot does not bind a future Dilithia transaction/state protocol version;
- no activation behavior is selected;
- no version-transition semantic relation is introduced; and
- implementation or document versions may later be recorded as provenance
  without becoming protocol-version semantics.

This is an explicit experimental exclusion, not an implicit default.

If future comparison behavior materially depends on a protocol-version or
version-transition profile, a new manifest is required.

## 12. Authorization Profile

Authorization profile identifier:

`pilot/authorization-not-bound-no-case-authority-relation/v1`

The frozen common semantic case does not declare a required consensus authority
relation.

This manifest therefore does not select an authorization mechanism.

It does not assert that future Dilithia value effects require no authorization.

It does not define:

- owner;
- address;
- key;
- credential;
- signature;
- proof;
- multisignature;
- threshold rule;
- delegation;
- recovery;
- stable identity; or
- verification-operation count.

No value of `N` is inferred from the absence of a declared authority relation in
this pilot case.

If an authority relation or authorization mechanism becomes material to a
future comparison, it must be introduced through separately reviewed case or
manifest content as required by the higher-level methodology.

## 13. Algorithm Profile

Algorithm profile identifier:

`pilot/cryptography-not-exercised/v1`

This pilot does not collect cryptographic evidence and does not select a
cryptographic algorithm.

The identifier means only:

`CRYPTOGRAPHIC BEHAVIOR NOT EXERCISED BY THIS PILOT CONFIGURATION`

It does not mean cryptography is unnecessary for a future Dilithia protocol.

No signature algorithm, proof system, key type, aggregation mechanism, batching
mechanism, post-quantum primitive, or verification count is selected here.

If cryptographic behavior becomes part of the comparison, both candidates must
receive the same applicable explicit algorithm profile in a new frozen manifest.

## 14. Replay, Duplicate, One-Use, and Reorganization Branches

The following canonical protocol semantics remain outside this pilot:

```text
canonical replay protection          = BLOCKED / NOT SELECTED
nonce semantics                       = BLOCKED / NOT SELECTED
one-use capability                    = BLOCKED / NOT SELECTED
record-consumption replay identity    = BLOCKED / NOT SELECTED
transaction-hash replay identity      = BLOCKED / NOT SELECTED
canonical reapplication semantics     = BLOCKED / NOT SELECTED
rollback semantics                    = BLOCKED / NOT SELECTED
chain reorganization semantics        = BLOCKED / NOT SELECTED
```

No candidate may gain an advantage by silently assuming any of these semantics.

This explicit exclusion is not permission for a future protocol to omit replay
or reorganization handling.

## 15. Migration Branch

Migration branch identifier:

`pilot/migration-out-of-scope/v1`

Migration behavior is not exercised by this pilot.

This manifest selects no:

- credential migration;
- state migration;
- record migration;
- key migration;
- cryptographic migration;
- compatibility mechanism;
- recovery path; or
- activation transition.

A future migration-dependent comparison requires a separately frozen manifest.

## 16. Commitment Branch

Commitment branch identifier:

`pilot/state-commitment-out-of-scope/v1`

State commitment is not exercised by this pilot.

This manifest selects no:

- Merkle structure;
- authenticated tree;
- accumulator;
- commitment format;
- commitment identifier;
- proof format;
- commitment update rule; or
- commitment retention rule.

Candidate mappings must not introduce a commitment structure and then treat it
as a shared assumption of this manifest.

Commitment-dependent comparison requires a separate explicit branch or
manifest.

## 17. Resource-Architecture Profile

Resource-architecture profile identifier:

`pilot/resource-evidence-not-collected/v1`

No resource architecture is selected for comparative evidence in this pilot
manifest.

This manifest selects no:

- gas;
- scalar resource score;
- candidate-attempt meter;
- monotonic attempt accounting;
- no-refund rule;
- reservation rule;
- transaction resource limit;
- block resource limit;
- state-growth limit;
- fee rule; or
- ingress-abuse mechanism.

Candidate mappings may describe structural work necessary to explain their
mapping, but no such description becomes a canonical resource metric.

Comparative resource evidence requires a separately reviewed and frozen
evidence configuration.

## 18. Monetary and Supply Branch

Monetary branch identifier:

`pilot/case-local-value-preservation-only/v1`

This manifest binds only the frozen common case's case-scoped conservation
assumption:

```text
A_after = A_before
```

with:

```text
A = 1
```

This does not establish universal DLTH conservation.

The following remain outside this pilot:

- issuance;
- block reward;
- burn;
- fee transfer;
- fee destruction;
- inflation;
- deflation;
- supply cap;
- supply schedule; and
- monetary policy.

No supply-changing case is included.

## 19. Failure-Atomicity Scope

This is not the `failure-atomic-external-projection` workload family.

This manifest therefore does not add a failure-atomicity workload beyond the
disposition behavior already defined by the frozen semantic case.

It selects no rollback implementation, journal, database transaction, attempt
meter, reservation, or refund mechanism.

A dedicated failure-atomic pilot case must be separately defined and frozen
before such evidence is compared.

## 20. Hostile-Validation Scope

Hostile-validation evidence is:

`NOT COLLECTED BY THIS MANIFEST`

This manifest does not define an adversarial semantic population, malformed
evidence grammar, hostile byte input, or candidate validation budget.

No candidate may infer from this exclusion that rejected or hostile validation
can be unbounded in a future protocol.

A hostile-validation pilot requires a separately scoped case, manifest, and
evidence configuration.

## 21. Exact-Byte and Serialization Scope

Exact-byte comparison is:

`OUT OF SCOPE`

This manifest defines no:

- transaction encoding;
- state encoding;
- candidate schema;
- malformed-byte grammar;
- wire format;
- candidate-specific serialization schema; or
- exact-byte metric.

Existing Dilithia canonical serialization work does not select the future state
model's transaction or state representation through this manifest.

If exact-byte evidence is later required, it must use a separately reviewed,
versioned, explicitly non-normative experimental schema.

## 22. Architecture Branch Set

For this pilot, the shared architecture branch set is:

```text
authorization architecture   = NOT SELECTED
replay architecture          = BLOCKED / NOT SELECTED
one-use architecture         = BLOCKED / NOT SELECTED
reorganization architecture  = BLOCKED / NOT SELECTED
migration architecture       = OUT OF SCOPE
commitment architecture      = OUT OF SCOPE
resource architecture        = OUT OF SCOPE FOR EVIDENCE
consensus mechanism          = OUT OF SCOPE
physical database layout     = CANDIDATE OUTPUT / OUT OF COMMON SCOPE
```

The Account and UTXO candidate families themselves are not entries in this
shared branch set.

They are the two candidate mappings being paired.

No branch receives an implicit default.

## 23. Policy Branch Set

For this pilot, the shared policy branch set is:

```text
protocol validity            = UNSPECIFIED
candidate ranking            = NONE
production-frequency claim   = NONE
suite weighting              = NONE
supply-changing behavior     = OUT OF SCOPE
case-local amount relation   = VALUE PRESERVING
authorization policy         = NOT SELECTED
recovery policy              = NOT SELECTED
migration policy             = OUT OF SCOPE
```

These are non-normative experimental controls only.

They create no Dilithia protocol policy.

## 24. Candidate Mapping Identities

The following mapping identities are reserved for this paired comparison.

Account mapping identity:

`pilot/value-effect-baseline/account/reference/v1`

Expected Account mapping file:

`VALUE_EFFECT_BASELINE_ACCOUNT_MAPPING.md`

UTXO mapping identity:

`pilot/value-effect-baseline/utxo/reference/v1`

Expected UTXO mapping file:

`VALUE_EFFECT_BASELINE_UTXO_MAPPING.md`

The mapping files do not exist at the time this frozen manifest is established.

This manifest therefore records only the following **pre-mapping readiness
snapshot**:

```text
Account reserved mapping readiness = MAPPING_ARTIFACT_NOT_YET_CREATED
UTXO reserved mapping readiness    = MAPPING_ARTIFACT_NOT_YET_CREATED
```

These readiness labels are local manifest bookkeeping only. They are not entries
in the workload-model mapping-status taxonomy and are not evidence against
either candidate.

This frozen manifest, once created, must not be edited merely because a reserved
mapping is later completed.

The authoritative experimental mapping status for each candidate belongs to the
candidate mapping artifact and later evidence record. A completed mapping may
therefore later report `MAPPED`, `MAPPING_INCOMPLETE`,
`BRANCH_UNSUPPORTED_BY_MAPPING`, or another applicable workload-model status
without changing this manifest.

Before evidence collection, the evidence configuration must bind the exact
frozen mapping artifact hashes and their then-applicable mapping statuses.

## 25. Mapping Maturity and Optimization Configuration

This first pilot binds both candidates to the same mapping maturity class:

`REFERENCE`

The `REFERENCE` label is descriptive only. It is not a score and does not by
itself define an optimization policy.

Optimization-configuration rule for this manifest:

`REFERENCE / NO RESULT-DRIVEN PERFORMANCE TUNING`

The purpose is methodology validation, not an optimized performance comparison.

A reference mapping may use semantics-preserving candidate-native structural
choices needed to produce a credible, non-artificial mapping. It must not
contain an obvious unnecessary disadvantage merely to keep the mapping simple.

Every material optional optimization or deliberately omitted obvious
optimization must be disclosed in that mapping's `optimization_configuration`
before comparative results are examined.

Both candidates receive equivalent methodological freedom under the same branch
assumptions. Symmetry does not require identical internal techniques.

An optimization available only to one candidate may be represented if it is
genuinely candidate-native and permitted by the shared assumptions, but that
fact must remain explicit and may not be generalized into an inherent property
of the entire candidate family.

Optimized and experimental mapping variants are outside this first paired
manifest unless a separately identified and pre-result-frozen mapping
configuration is created.

The omission of optimized variants means that conclusions from this pilot are
reference-mapping-qualified and must not be generalized to an entire candidate
family.

## 26. Symmetric Treatment Requirement

Both mappings must receive the same:

- frozen common semantic case hash;
- `A = 1`;
- assumption profile;
- policy branch set;
- architecture branch set;
- ordering profile;
- version profile;
- authorization scope;
- cryptographic scope;
- migration scope;
- commitment scope;
- resource scope;
- monetary case-scoped assumption;
- mapping maturity class;
- optimization-configuration rule;
- evidence exclusion rules; and
- review criteria.

Candidate-native internal structures need not be identical.

Symmetry means equal external requirements and equal permission, not forced
internal sameness.

## 27. Candidate-Specific Information Reserved for Mapping Layer

The following are not fixed to equal values by this paired manifest:

- Account record count;
- UTXO record count;
- candidate internal source count;
- candidate internal post-effect record count;
- candidate state-key count;
- candidate logical reads;
- candidate logical writes;
- candidate internal dependencies;
- candidate conflict footprint;
- candidate storage shape;
- candidate evidence placement;
- candidate preparation work;
- candidate validation workflow;
- candidate implementation complexity; and
- candidate physical measurements.

Those properties belong in candidate mappings or later evidence.

The manifest must not be retroactively changed to force favorable internal
cardinalities after those outputs are known.

## 28. Required Mapping Demonstration

Each candidate mapping must explain, using its own candidate-specific
representation, how it realizes the frozen common semantic contract.

At minimum, each mapping must demonstrate:

1. how its mapped pre-effect state projects to:

   ```text
   term   = pre_value_0
   amount = 1
   ```

2. what candidate-specific mapped effect is associated with `E0`;

3. how its mapped post-effect state projects to:

   ```text
   term   = post_value_0
   amount = 1
   ```

4. why the post-effect external semantic projection is not merely an
   evidence-only relabeling of unchanged mapped evidence;

5. how amount preservation is maintained under the mapping;

6. which internal structures are candidate-specific rather than shared
   semantics; and

7. which material mapping choices remain unresolved.

A mapping that cannot defensibly demonstrate the required projection change
must not be labeled `MAPPED`.

## 29. Requested Mapping Output Categories

Before comparative evidence collection, each mapping should disclose at least:

- mapping identity;
- frozen semantic case content hash;
- frozen paired manifest content hash;
- candidate family;
- mapping maturity label;
- mapping description;
- pre-effect candidate representation;
- post-effect candidate representation;
- candidate-specific realization of `E0`;
- external projection derivation;
- internal cardinality;
- logical state-access shape where applicable;
- internal dependency shape where applicable;
- preparation work;
- optimization configuration;
- unresolved dependencies;
- mapping status; and
- justification for any unsupported or inapplicable category.

These disclosures are mapping records, not protocol rules.

No candidate-native counter is automatically comparable merely because both
mappings report a field with a similar name.

## 30. Mapping and Missingness Status Vocabulary

Mappings and later evidence must use the existing neutral status taxonomy,
including where applicable:

- `MAPPED`;
- `MAPPING_INCOMPLETE`;
- `BRANCH_NOT_IN_SCOPE`;
- `BRANCH_UNSUPPORTED_BY_MAPPING`;
- `SHARED_CONTRACT_UNSATISFIED`;
- `CASE_UNDERDEFINED`;
- `PROFILE_OR_SCHEMA_UNAVAILABLE`;
- `EVIDENCE_NOT_COLLECTED`;
- `RUN_INFRASTRUCTURE_FAILURE`; and
- `DETERMINISTIC_MAPPING_FAILURE`.

A status must not be hidden because it is unfavorable.

This manifest does not freeze a future mapping status for either reserved
mapping identity.

Each mapping artifact must report its own applicable status from this taxonomy.
That status is then bound by the mapping artifact and later evidence
configuration, not retroactively written into this paired manifest.

## 31. Suite Identity

This manifest is a single-case pilot configuration.

Suite manifest identity:

`NONE — SINGLE-CASE PILOT`

No fixed corpus or suite weighting is defined.

This case carries no vote or decision weight merely because it exists.

No production-frequency inference is permitted.

If this case is later included in a multi-case suite, that suite requires its
own separately frozen identity and composition controls.

## 32. Sweep and Generator Scope

Parameter sweep:

`NONE`

Deterministic case generator:

`NONE`

Random seed:

`NOT APPLICABLE`

Sampling distribution:

`NONE`

The absence of a sweep is intentional for this first pilot.

A later sweep must use identical parameter points for both candidate mappings
and must be frozen before comparative results are examined.

## 33. Evidence Collection Status

Comparative evidence collection is:

`NOT YET AUTHORIZED — MAPPING AND EVIDENCE GATES PENDING`

No benchmark, timing, byte, resource, database, or state-growth result belongs
in this file.

Evidence collection may begin only after:

1. this paired manifest is independently reviewed and frozen;
2. the Account mapping is completed and frozen;
3. the UTXO mapping is completed and frozen;
4. mapping equivalence and fairness receive independent review;
5. material findings are resolved;
6. the applicable evidence methodology is explicitly bound; and
7. the pre-result configuration is frozen.

## 34. Pre-Result Freeze Controls

Before results are examined, the following applicable items must be frozen:

- common semantic case identity;
- paired manifest;
- candidate mapping identities;
- candidate mapping contents;
- mapping maturity class;
- optimization-configuration rule;
- parameter value;
- material branch selections;
- material profile selections;
- requested mapping-output definitions;
- evidence methodology, if any;
- exclusion rules; and
- later suite composition, if any.

Material post-result changes create a new experimental configuration.

Old and new configurations must remain distinguishable.

Results must not be used to choose more favorable assumptions retroactively.

## 35. Exclusion Rules

This pilot explicitly excludes comparative claims that depend on:

- protocol transaction validity;
- authorization evidence;
- cryptographic evidence;
- replay or reorganization behavior;
- migration behavior;
- commitment behavior;
- exact-byte representation;
- canonical resource accounting;
- numeric resource limits;
- hostile-validation limits;
- supply-changing behavior;
- physical performance;
- production workload frequency; or
- cross-family aggregate ranking.

An excluded dimension may not be silently reintroduced by one candidate
mapping.

If a mapping requires an excluded dimension to become material, the manifest
must be revised before results and reviewed as a new configuration.

## 36. Anti-Strawman Controls

Both candidate mappings must be defensible reference mappings.

The following are prohibited:

- intentionally weakening one candidate;
- forcing Account to imitate UTXO internals;
- forcing UTXO to imitate Account internals;
- assuming one semantic term equals one candidate record;
- forcing equal internal record counts;
- forcing equal state-access counts;
- forcing equal evidence-object counts;
- denying one candidate a structural mechanism permitted to the other under the
  same external assumptions;
- hiding necessary preparation work; and
- selecting candidate-specific assumptions after results are known.

If more than one materially different reference mapping is defensible for a
candidate, the omission of alternatives must be documented before any
candidate-family conclusion is attempted.

This first pilot does not attempt a candidate-family conclusion.

## 37. Semantic-Drift Controls

Neither mapping may silently:

- change `A`;
- change the common case hash;
- remove the required projection change;
- replace `pre_value_0` or `post_value_0` semantics;
- add candidate-specific external information;
- add a candidate-specific success condition;
- add or remove a shared conflict relation;
- introduce an externally meaningful ordering relation;
- introduce an authority relation;
- introduce replay or one-use semantics;
- introduce supply-changing behavior; or
- convert an excluded material dimension into an implicit assumption.

Such a change is not a mapping of this frozen paired configuration.

## 38. Independent Review Gate

The independent actual-file freeze review for this manifest covered:

- Account-favoring assumptions;
- UTXO-favoring assumptions;
- semantic drift;
- hidden profile defaults;
- hidden candidate-specific information;
- result-dependent future edit paths;
- asymmetric optimization permissions;
- artificial de-optimization;
- hidden preparation work;
- hidden authorization assumptions;
- hidden replay assumptions;
- hidden ordering assumptions;
- hidden cryptographic assumptions;
- hidden commitment assumptions;
- hidden resource assumptions;
- hidden monetary assumptions;
- misuse of candidate-native metrics; and
- unsupported candidate-family generalization.

No material finding remained unresolved at freeze.

Any later material change to the frozen case binding, parameter value, profile
or branch selection, mapping identity reservation, optimization-configuration
rule, exclusion rule, or shared comparison requirement creates distinguishable
manifest content and requires a new evidence-only manifest identity before new
results are attributed to it.

## 39. Security Boundary

This pilot manifest intentionally avoids selecting security-sensitive protocol
mechanisms.

That exclusion does not weaken applicable Dilithia security requirements.

If candidate mappings later require assumptions involving ownership,
authorization, cryptographic evolution, migration, failure atomicity, hostile
validation, persistent state growth, or monetary behavior, those assumptions
require review appropriate to the affected area before evidence becomes
decision-relevant.

Experimental convenience cannot create protocol authority.

## 40. Reproducibility Requirements

The frozen paired configuration must be reproducible from:

- frozen case hash;
- frozen manifest hash;
- exact parameter value;
- exact profile identifiers;
- exact branch selections;
- exact mapping identities;
- exact mapping artifact hashes after mapping freeze; and
- exact evidence methodology identity if evidence is later collected.

A future evidence package must identify the exact frozen configuration used.

A human-readable alias alone is insufficient.

## 41. Current Project Impact

Creation of this frozen paired manifest has the following project impact:

```text
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
NOT JUSTIFIED BY THIS FROZEN MANIFEST

PROJECT_STATE update:
NOT JUSTIFIED BY THIS FROZEN MANIFEST

THREAT_MODEL update:
NOT JUSTIFIED BY THIS FROZEN MANIFEST

Consensus implementation change:
NONE

Comparative evidence collected:
NO
```

## 42. Next Gate

This manifest must not proceed directly to comparative evidence.

The actual-file review, material-finding resolution, and paired-manifest freeze
gates are complete for this artifact.

The remaining permitted workflow is:

1. establish this frozen manifest's evidence-only content identity;
2. create the reserved Account reference mapping;
3. create the reserved UTXO reference mapping;
4. independently review mapping equivalence, fairness, and anti-strawman
   treatment;
5. resolve material mapping findings;
6. freeze both mappings;
7. bind the applicable evidence methodology;
8. freeze the complete pre-result evidence configuration; and
9. only then consider comparative evidence collection.

This file remains a non-normative experimental artifact. Its freeze does not
authorize evidence collection or select a Dilithia state model.
