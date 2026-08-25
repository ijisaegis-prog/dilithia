# Account–UTXO Pilot Evidence Methodology: Value-Effect Baseline

> **NON-NORMATIVE EXPERIMENTAL EVIDENCE METHODOLOGY**
>
> **Status: FROZEN — EXTERNAL EVIDENCE-ONLY CONTENT IDENTITY**
>
> **Evidence scope: DETERMINISTIC STRUCTURAL / SEMANTIC-MAPPING EVIDENCE ONLY**
>
> This document defines the evidence methodology for the frozen
> `value-effect-baseline` Account/UTXO pilot.
>
> It is not a protocol specification, state-model decision, transaction or state
> format, authorization or replay mechanism, cryptographic profile, state
> commitment, resource-accounting rule, monetary policy, benchmark result, or
> candidate ranking.
>
> If this document conflicts with the Dilithia Constitution, Formal
> Specification, or ratified HIP / Super HIP material, the authoritative protocol
> material prevails.
>
> Within the non-normative Account/UTXO comparison methodology, this document is
> subordinate to:
>
> - `ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`;
> - `ACCOUNT_UTXO_WORKLOAD_MODEL.md`;
> - `ACCOUNT_UTXO_CANDIDATE_MAPPING_REQUIREMENTS.md`;
> - `BENCHMARK_METHODOLOGY.md`;
> - frozen `VALUE_EFFECT_BASELINE_CASE.md`;
> - frozen `VALUE_EFFECT_BASELINE_PAIRED_MANIFEST.md`;
> - frozen `VALUE_EFFECT_BASELINE_ACCOUNT_MAPPING.md`; and
> - frozen `VALUE_EFFECT_BASELINE_UTXO_MAPPING.md`.

## 1. Purpose

This methodology defines how evidence may be created and interpreted for the
first frozen `value-effect-baseline` pilot.

The first evidence stage is intentionally narrow. It evaluates only whether the
two frozen reference mappings:

1. bind the exact frozen case and manifest;
2. receive the same shared experimental conditions;
3. realize the required non-identical external semantic projection;
4. preserve the case-local amount relation;
5. ground the projection change in candidate-specific mapped logical state;
6. reject no-op and evidence-only relabeling;
7. preserve the manifest's exclusions and unresolved dimensions;
8. expose candidate-native structural facts without falsely equating unlike
   counters; and
9. remain reproducible and independently reviewable.

This methodology does **not** authorize timing, throughput, CPU, allocation,
database, exact-byte, state-growth, cryptographic-work, or canonical
resource-accounting comparisons.

Those dimensions require additional frozen prerequisites that do not exist for
this pilot configuration.

## 2. Evidence-Layer Position

The artifact chain for this pilot is:

```text
Frozen Common Semantic Case
            |
            v
Frozen Paired Comparison Manifest
            |
            +------------------------+
            |                        |
            v                        v
Frozen Account Mapping       Frozen UTXO Mapping
            |                        |
            +-----------+------------+
                        |
                        v
              Frozen Evidence Methodology
                        |
                        v
             Frozen Pre-Result Configuration
                        |
                        v
                  Evidence Records
                        |
                        v
                 Evidence Summary
```

This document defines only the **Evidence Methodology** layer.

It does not itself contain measured evidence.

A later pre-result configuration must bind this methodology's frozen content
identity before any evidence record is created.

## 3. Frozen Artifact Bindings

This methodology is designed for exactly the following frozen artifact set.

### Evidence Methodology Identity

Methodology format version:

`pilot-account-utxo-evidence-methodology/v1`

Methodology alias:

`pilot/value-effect-baseline/evidence-methodology/v1`

Methodology content hash:

```text
NOT EMBEDDED — EXTERNAL EVIDENCE-ONLY CONTENT IDENTITY
```

After this methodology is reviewed and frozen, its evidence-only SHA-256
identity is computed over the exact frozen file bytes. The resulting identity
is recorded externally and is bound by the later pre-result configuration.

The frozen file does not embed its own final content hash. This avoids a
self-referential identity.

Any byte change after freeze creates different methodology content and therefore
a different evidence-only identity.

### Common Semantic Case

File:

`VALUE_EFFECT_BASELINE_CASE.md`

Evidence-only content identity:

```text
sha256:1DB6FBE63C13BFB619BFECE7D61D0A9958F00E2BF8866331A4602BA0240B276D
```

### Paired Comparison Manifest

File:

`VALUE_EFFECT_BASELINE_PAIRED_MANIFEST.md`

Evidence-only content identity:

```text
sha256:219B286FD6D4EFB7ECE6F18B5872A34C22F6209E8CE45FA8BE96DD9AB1081D83
```

### Account Reference Mapping

File:

`VALUE_EFFECT_BASELINE_ACCOUNT_MAPPING.md`

Evidence-only content identity:

```text
sha256:94FD6D8C72137CBCB228C32C8D298930C36C99BEA1BB167FFAE747D69B57F600
```

Mapping alias:

`pilot/value-effect-baseline/account/reference/v1`

Mapping status:

`MAPPED`

Mapping maturity:

`REFERENCE`

### UTXO Reference Mapping

File:

`VALUE_EFFECT_BASELINE_UTXO_MAPPING.md`

Evidence-only content identity:

```text
sha256:3211A348EDA0AE97271F22C1D6C0622B29366BCDFACDE1196E73D350249265C3
```

Mapping alias:

`pilot/value-effect-baseline/utxo/reference/v1`

Mapping status:

`MAPPED`

Mapping maturity:

`REFERENCE`

A different content hash for any of the four artifacts creates a different
evidence configuration.

No file may be silently substituted.

## 4. Shared Experimental Contract

The paired manifest binds:

```text
A = 1
```

where `1` is one case-local experimental semantic amount unit.

The required external semantic projections are:

```text
before:
    term   = pre_value_0
    amount = 1
```

and:

```text
after:
    term   = post_value_0
    amount = 1
```

The required semantic properties are:

```text
amount preserved       = YES
external projection    = MUST CHANGE
no-op satisfaction     = PROHIBITED
evidence-only relabel  = PROHIBITED
```

Protocol validity remains unspecified.

## 5. Evidence Classification

Evidence produced under this methodology is classified into three categories.

### 5.1 Shared Contract Evidence

These facts use the same meaning for both candidate mappings and may be compared
directly:

- frozen case hash matches;
- frozen paired manifest hash matches;
- `A = 1` is used;
- required pre-effect projection is demonstrated;
- required post-effect projection is demonstrated;
- external projection changes from `pre_value_0` to `post_value_0`;
- amount remains `1`;
- candidate mapped state actually changes;
- no-op does not satisfy the mapping;
- evidence-only relabeling does not satisfy the mapping;
- intrinsic external conflict relation remains empty;
- ordering profile remains `NO_ORDER_ASSERTION`;
- excluded architecture and policy dimensions remain excluded;
- mapping maturity is `REFERENCE`; and
- mapping status is `MAPPED`.

These are deterministic structural or semantic-mapping checks.

### 5.2 Candidate-Native Structural Evidence

These facts describe one candidate mapping under its own vocabulary.

Examples include:

Account:

- logical value relation count;
- positive-quantity logical relation count;
- mapped quantity vector; and
- mapping-local dependency description.

UTXO:

- live logical value-record count;
- distinct mapping-local record-handle count;
- mapped live set;
- mapping-local replacement relation; and
- mapping-local dependency description.

Candidate-native structural values remain descriptive unless a separately
reviewed shared metric gives them the same meaning.

### 5.3 Unavailable or Excluded Evidence

Evidence is not collected for a dimension whose prerequisites are absent or
whose branch is excluded by the frozen manifest.

Unavailable or excluded evidence must remain visible.

It must not be converted to numeric zero.

## 6. Directly Comparable Evidence Dimensions

The following evidence dimensions have a shared reviewed meaning in this pilot.

| Dimension | Account expected result | UTXO expected result | Direct comparison meaning |
|---|---|---|---|
| Frozen semantic-case hash binding | MATCH | MATCH | Same case required |
| Frozen paired-manifest hash binding | MATCH | MATCH | Same manifest required |
| Instantiated `A` | `1` | `1` | Same semantic parameter |
| Mapping maturity | `REFERENCE` | `REFERENCE` | Maturity parity |
| Mapping status | `MAPPED` | `MAPPED` | Both represent frozen case |
| Pre-effect external projection | `(pre_value_0, 1)` | `(pre_value_0, 1)` | Same semantic projection |
| Post-effect external projection | `(post_value_0, 1)` | `(post_value_0, 1)` | Same semantic projection |
| Required projection change | SATISFIED | SATISFIED | Same semantic predicate |
| Amount preservation | SATISFIED | SATISFIED | Same case-local relation |
| Candidate mapped-state change | REQUIRED / PRESENT | REQUIRED / PRESENT | Prevents vacuous mapping |
| No-op satisfaction | REJECTED | REJECTED | Same anti-vacuity rule |
| Evidence-only relabeling | REJECTED | REJECTED | Same grounding rule |
| Intrinsic external conflict relation | EMPTY | EMPTY | Same shared relation |
| Ordering profile | `NO_ORDER_ASSERTION` | `NO_ORDER_ASSERTION` | Same ordering assumptions |
| Protocol validity | UNSPECIFIED | UNSPECIFIED | No validity inference |
| Candidate ranking | NONE | NONE | No winner inference |

A failure of a shared-contract dimension is material.

A shared-contract failure must not be compensated for by favorable
candidate-native structural or physical measurements.

## 7. Candidate-Native Evidence That Is Not Directly Comparable

The following values are deliberately **not** defined as a common scalar metric.

### Account Mapping

The frozen Account mapping discloses:

```text
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
```

### UTXO Mapping

The frozen UTXO mapping discloses:

```text
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
```

This methodology explicitly prohibits reasoning such as:

```text
Account logical relations = 2
UTXO live records         = 1

therefore:
UTXO wins
```

or:

```text
Account distinct handles = 2
UTXO distinct handles    = 2

therefore:
the candidates are tied
```

The counters name different candidate-native structures.

A direct quantitative comparison requires a future shared, reviewed metric
meaning and all prerequisites for that metric.

## 8. Account Mapping Evidence Rule

The Account mapping demonstrates the pre-effect state:

```text
quantity(AR_PRE)  = 1
quantity(AR_POST) = 0
```

and the post-effect state:

```text
quantity(AR_PRE)  = 0
quantity(AR_POST) = 1
```

Its candidate-specific `E0` realization is therefore:

```text
(1, 0) -> (0, 1)
```

An Account evidence record may report the mapping contract as satisfied only if
the evidence demonstrates those frozen mapping conditions or an exactly
equivalent derivation explicitly permitted by the frozen mapping.

The following does not satisfy the Account mapping:

```text
(1, 0) -> (1, 0)
```

even if the evidence record changes a phase name, timestamp, run index, or
external label.

## 9. UTXO Mapping Evidence Rule

The UTXO mapping demonstrates the pre-effect state:

```text
live_set = { UR_PRE }
amount(UR_PRE) = 1
```

and the post-effect state:

```text
live_set = { UR_POST }
amount(UR_POST) = 1
```

Its candidate-specific `E0` realization is therefore:

```text
{ UR_PRE : 1 } -> { UR_POST : 1 }
```

An UTXO evidence record may report the mapping contract as satisfied only if the
evidence demonstrates those frozen mapping conditions or an exactly equivalent
derivation explicitly permitted by the frozen mapping.

The following does not satisfy the UTXO mapping:

```text
{ UR_PRE : 1 } -> { UR_PRE : 1 }
```

even if the evidence record changes a phase name, timestamp, run index, or
external label.

The mapping-local replacement relation is not evidence of canonical replay,
one-use, or spent-output semantics.

## 10. Amount-Preservation Evidence Rule

For Account:

```text
before:
    1 + 0 = 1

after:
    0 + 1 = 1
```

For UTXO:

```text
before:
    sum(live amounts) = 1

after:
    sum(live amounts) = 1
```

The shared experimental conclusion may state only:

> Both frozen reference mappings satisfy the case-local value-preservation
> predicate for `A = 1`.

It may not state:

- DLTH has a universal conservation rule;
- issuance is forbidden;
- rewards are forbidden;
- burns are forbidden;
- fees are defined;
- supply policy is selected; or
- either state model has better monetary integrity.

## 11. Excluded Evidence Dimensions

The following evidence dimensions are excluded from this pilot methodology.

```text
benchmark wall-clock timing       = EXCLUDED
throughput                        = EXCLUDED
latency                           = EXCLUDED
CPU cycles                        = EXCLUDED
allocation counts                 = EXCLUDED
peak memory                       = EXCLUDED
database operations               = EXCLUDED
physical storage growth           = EXCLUDED
network measurements              = EXCLUDED
exact encoded bytes               = EXCLUDED
canonical state bytes             = EXCLUDED
logical read metric               = EXCLUDED / NOT DEFINED
logical write metric              = EXCLUDED / NOT DEFINED
record creation metric            = EXCLUDED / NOT DEFINED
record retirement metric          = EXCLUDED / NOT DEFINED
canonical resource units          = EXCLUDED / NOT DEFINED
gas                               = EXCLUDED / NOT SELECTED
candidate-attempt accounting      = EXCLUDED / NOT SELECTED
fees                              = EXCLUDED / NOT SELECTED
cryptographic verification work   = EXCLUDED / NOT EXERCISED
state commitment work             = EXCLUDED / NOT SELECTED
migration work                    = EXCLUDED / NOT EXERCISED
hostile-validation performance    = EXCLUDED / NOT DEFINED
replay behavior                   = EXCLUDED / BLOCKED
reorganization behavior           = EXCLUDED / BLOCKED
```

No excluded value may appear in a pilot comparison table as `0`.

## 12. Why Timing Is Excluded

The frozen candidate mappings are logical mappings.

They bind no executable Account or UTXO implementation.

They bind no:

- benchmark harness;
- candidate schema;
- transaction encoding;
- state encoding;
- database;
- allocator;
- compiler configuration for candidate execution;
- validation workflow;
- cryptographic algorithm;
- state commitment;
- resource-accounting architecture; or
- physical persistence model.

Therefore wall-clock timing would measure an arbitrary implementation choice
rather than the frozen mappings themselves.

Timing evidence is not merely absent.

It is **not methodologically authorized by this configuration**.

A later performance campaign requires new, reviewed, frozen implementation and
measurement prerequisites.

## 13. Why Exact Bytes Are Excluded

Neither mapping binds an experimental schema.

Therefore:

```text
experimental_schema_binding = NONE
```

Exact-byte claims require a separately reviewed experimental schema that is:

- explicitly non-normative;
- versioned;
- content-addressed;
- bound to the frozen semantic case;
- bound to the frozen paired manifest;
- bound to the exact candidate mapping;
- feature-symmetric for the compared candidates; and
- used only for evidence.

No existing Dilithia serialization primitive may silently select an Account or
UTXO transaction/state schema.

Under this frozen pilot, exact-byte evidence is already excluded by the paired
manifest and this methodology. Therefore the current scope disposition is:

```text
exact-byte evidence scope = EXCLUDED_BY_FROZEN_MANIFEST
```

If a later reviewed configuration explicitly brings exact-byte evidence into
scope but a required experimental schema does not exist or cannot be bound,
that later configuration uses:

```text
PROFILE_OR_SCHEMA_UNAVAILABLE
```

The two conditions must not be conflated.

## 14. Why Logical Read/Write Counts Are Excluded

A state transition description is not automatically a read/write metric.

For example, the Account mapping describes:

```text
AR_PRE:  1 -> 0
AR_POST: 0 -> 1
```

but this does not automatically mean:

```text
logical_reads  = 2
logical_writes = 2
```

Likewise, the UTXO mapping describes a replacement relation but this does not
automatically mean:

```text
logical_reads   = 1
logical_writes  = 2
records_created = 1
records_retired = 1
```

A directly comparable logical-access metric requires a frozen definition of:

- logical schema;
- access boundary;
- metadata placement;
- indirection;
- validation stages;
- commitment interaction where applicable;
- treatment of zero-valued or absent state;
- record creation/replacement/retirement semantics; and
- candidate-specific realization.

Those prerequisites are not frozen.

Therefore direct logical read/write comparison is prohibited.

## 15. Why Resource Scoring Is Excluded

This methodology defines no:

- resource weights;
- conversion ratios;
- gas schedule;
- execution-unit schedule;
- candidate-attempt meter;
- refund rule;
- reservation rule;
- state-growth price;
- fee schedule; or
- scalar aggregate cost.

A physical observation, candidate-native count, or benchmark duration must not
be converted into a protocol resource unit through this methodology.

Runtime consensus counts only resource units adopted by authoritative protocol
material.

This pilot has none.

## 16. Correctness Precedes Measurement

Correctness is a separate gate from performance.

For this pilot, a candidate evidence record must first pass all applicable
shared-contract checks.

If a candidate fails the frozen semantic contract, its status must not be
rewritten as a poor performance score.

The appropriate status is retained according to the workload-model taxonomy.

Performance evidence, if added by a future configuration, cannot compensate for
semantic failure.

A faster wrong mapping is not a successful mapping.

## 17. Evidence Record Logical Schema

Each evidence record produced under this methodology should contain, directly
or by immutable reference:

```text
evidence_record_format_version
evidence_record_alias
evidence_record_content_identity

evidence_methodology_hash
pre_result_configuration_hash

semantic_case_hash
paired_manifest_hash
candidate_mapping_hash

candidate_family
candidate_variant
mapping_maturity
mapping_status

shared_parameter_A

case_binding_check
manifest_binding_check
mapping_binding_check

pre_effect_projection
post_effect_projection
required_projection_change_check
amount_preservation_check
mapped_state_change_check
no_op_rejection_check
evidence_only_relabel_rejection_check

external_conflict_relation_check
ordering_profile_check
dimension_scope_dispositions

candidate_native_structural_observations

neutral_status_observations_if_applicable
run_lifecycle_status_if_run_based
deviations
exclusions
review_provenance
generator_or_analysis_provenance
```

`evidence_record_content_identity` is not a self-embedded final hash.

The exact evidence-record representation and its content-addressing procedure
must be frozen in the pre-result configuration before records are created. A
final evidence-record hash is then recorded externally or by a non-self-
referential immutable reference.

This is a logical evidence schema only.

It defines no JSON format, Rust type, wire format, protocol object, transaction
format, state format, or consensus record.

A future machine-readable schema may be created as a separate non-normative
evidence artifact.

## 18. Neutral Status Taxonomy and Field Separation

The existing neutral status taxonomy is reused without reinterpretation:

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

The meanings and required treatment of these labels are defined by
`ACCOUNT_UTXO_WORKLOAD_MODEL.md`.

This methodology does not restate, narrow, broaden, reorder, or replace those
meanings.

The taxonomy is not a single evidence-record lifecycle field.

In particular:

- `mapping_status` remains the mapping status supplied by the frozen mapping;
- deterministic shared checks report their own categorical outcomes such as
  `MATCH`, `SATISFIED`, or `REJECTED`;
- `neutral_status_observations_if_applicable` records only labels whose
  workload-model meanings are actually satisfied;
- explicitly excluded evidence dimensions are represented by the separate
  `dimension_scope_dispositions` field and are not mislabeled as merely
  uncollected evidence; and
- a run lifecycle status, when runs exist, is separate from this taxonomy.

This methodology does not invent an `EVIDENCE_COLLECTED` status.

Successful deterministic evidence is represented by the preserved check
outcomes, raw evidence record, and, when run-based, the run lifecycle status.

If more than one neutral status appears potentially applicable and the existing
workload-model meanings do not uniquely resolve the classification, the
ambiguity must be surfaced for review. It must not be resolved after results by
choosing whichever label makes a candidate appear better.

No status or scope disposition may be selected to improve a candidate's
apparent result.

## 19. Failure Visibility

Evidence records must retain:

- failed checks;
- incomplete checks;
- unsupported branches;
- unavailable schemas;
- uncollected evidence;
- infrastructure failures;
- deterministic mapping failures;
- deviations; and
- exclusions.

This methodology prohibits:

```text
if candidate evidence is unfavorable:
    remove evidence record
```

A failed or unavailable result remains visible with its correct status.

## 20. Exclusion Rules

The pre-result configuration must declare all permitted exclusions before
evidence records are produced.

For this pilot, an evidence record may be excluded from a summary only if:

1. the exclusion reason was predeclared;
2. the rule applies symmetrically;
3. the raw record remains retained;
4. the exclusion, scope disposition, and any applicable neutral status remain visible; and
5. included and excluded views remain reproducible.

An exclusion must not transform a failed semantic check into success.

Post-result candidate-specific exclusion is prohibited.

## 21. Deterministic Structural Evidence

The first evidence stage has no statistical sampling.

Each frozen mapping is evaluated against a finite set of deterministic checks.

For each check, the evidence outcome is categorical or structural.

Examples:

```text
MATCH
SATISFIED
REJECTED
EMPTY
NO_ORDER_ASSERTION
UNSPECIFIED
EXCLUDED
```

Repeated identical evaluations do not create extra decision weight.

A deterministic check repeated 1,000 times is still one semantic condition, not
1,000 votes.

## 22. Statistical Policy for This Pilot

Because the authorized evidence stage contains no timing or stochastic
measurement:

```text
warmup plan         = NOT APPLICABLE
sample count        = NOT APPLICABLE
confidence interval = NOT APPLICABLE
percentiles         = NOT APPLICABLE
outlier policy      = NOT APPLICABLE TO DETERMINISTIC CHECKS
thermal policy      = NOT APPLICABLE
cache policy        = NOT APPLICABLE
CPU affinity        = NOT APPLICABLE
```

This does not waive the statistics requirements of `BENCHMARK_METHODOLOGY.md`
for a future physical benchmark.

If a future configuration introduces physical measurement, its sampling,
stopping, exclusion, environment, and analysis procedures must be frozen before
results and must preserve raw samples and failures.

## 23. Candidate-Native Structural Disclosure

Candidate-native structural observations are recorded because hiding them would
make the mapping unreproducible.

They are not automatically metrics.

The evidence summary must present them in separate candidate-specific sections
unless a shared metric definition exists.

A summary must not place unrelated candidate-native counters into one numeric
ranking column merely because both are integers.

## 24. No Scalar Score

This pilot defines no scalar score.

It defines no weights that combine:

- semantic correctness;
- record counts;
- logical relation counts;
- mapping complexity;
- timing;
- bytes;
- memory;
- state growth; or
- any other dimension.

Shared-contract checks are gates or categorical evidence.

Candidate-native observations remain descriptive.

A future weighting or decision function requires separate authority and review.

## 25. No Winner by Case Count

This pilot contains one frozen semantic case.

Its existence gives it no production-frequency or decision weight.

The following claims are prohibited:

- Account wins one case;
- UTXO wins one case;
- one case implies global state-model superiority;
- one pilot is representative of production traffic; or
- equal contract satisfaction means the candidates are globally equivalent.

The pilot validates a comparison method for one frozen case and two named
reference mappings.

## 26. Mapping-Qualified Conclusions

Every conclusion under this methodology must remain qualified to:

- the frozen `value-effect-baseline` case;
- `A = 1`;
- the frozen paired manifest;
- the Account reference mapping hash;
- the UTXO reference mapping hash;
- REFERENCE maturity;
- the included evidence dimensions;
- the excluded dimensions; and
- the evidence methodology version/hash.

A permitted conclusion may say:

> Under the frozen value-effect-baseline pilot configuration, both reviewed
> reference mappings satisfy the same external semantic contract and amount
> preservation predicate while realizing the projection change through different
> candidate-native logical structures.

It may not say:

> Account is better.

or:

> UTXO is better.

or:

> Account and UTXO are equally efficient.

No efficiency evidence exists under this methodology.

## 27. Provenance Requirements

Every pre-result configuration and resulting evidence package must record:

- exact frozen semantic-case hash;
- exact frozen paired-manifest hash;
- exact Account mapping hash;
- exact UTXO mapping hash;
- exact externally recorded frozen evidence-methodology hash;
- exact pre-existing Dilithia source revision containing the frozen inputs used
  by the evidence procedure;
- repository clean/dirty state relative to that source revision;
- complete diff if a dirty tree is intentionally permitted;
- evidence generator or checking-tool source identity;
- analysis source identity;
- dependency identities where tooling uses dependencies;
- toolchain/runtime identity when tooling is executed;
- generated artifact hashes;
- reviewer provenance; and
- deviations and exclusions.

Provenance records provide evidence auditability only.

They create no protocol authority.

## 28. Evidence-Tool Integrity

If a script or other tool is used to create deterministic evidence records, the
tool must:

- have pinned or content-addressed source;
- have pinned dependencies or an immutable dependency record;
- validate all frozen input hashes before evaluating them;
- fail closed on a hash mismatch;
- use checked arithmetic for any counts it computes;
- detect and report parsing or schema failures;
- preserve candidate labels without deriving semantics from the labels alone;
- emit deterministic output for deterministic inputs;
- expose all failed checks;
- not delete unavailable fields;
- not convert missingness into zero; and
- be independently reviewable.

Where practical, a separately reviewed analysis path should be able to
reconstruct the summary from the raw evidence records.

## 29. Manual Evidence Boundary

Manual review may be used as review provenance.

Manual review alone must not silently replace a deterministic check that the
pre-result configuration declares as tool-generated.

Conversely, a tool result does not replace independent semantic review.

Human review and machine checking have different roles:

```text
semantic interpretation / fairness:
    independent review

mechanical hash / field / invariant verification:
    deterministic tool where configured

protocol adoption:
    authoritative process only
```

## 30. Pre-Result Configuration Requirements

Evidence collection is not authorized merely because this methodology exists.

A separate pre-result configuration must be created, independently reviewed,
and frozen.

It must bind at minimum:

```text
pre_result_configuration_format_version
pre_result_configuration_alias
pre_result_configuration_content_identity

semantic_case_hash
paired_manifest_hash
account_mapping_hash
utxo_mapping_hash
evidence_methodology_hash

evidence_source_revision
repository_clean_or_dirty_state
complete_dirty_diff_identity_if_applicable

evidence_record_format_version
evidence_record_content_addressing_procedure
evidence_generator_or_manual_procedure_identity
analysis_identity

exact_shared_checks
candidate_native_disclosure_fields
dimension_scope_dispositions

failure_handling
missingness_handling
status_assignment_rules_consistent_with_workload_model
exclusion_rules
deviation_rules

run_plan
review_plan
publication_or_registration_plan
```

`pre_result_configuration_content_identity` is:

```text
NOT EMBEDDED — EXTERNAL EVIDENCE-ONLY CONTENT IDENTITY
```

The pre-result configuration must not attempt to contain its own final hash.
After review and freeze, its exact frozen bytes receive an external
evidence-only content identity.

`evidence_source_revision` identifies a pre-existing immutable repository
revision containing the frozen artifact set and frozen evidence methodology used
for the evidence procedure. It is not the future commit hash that will contain
the pre-result configuration itself.

No field may receive an implicit result-dependent default.

## 31. Pre-Result Publication / Registration

Before evidence records intended to support a comparative conclusion are
created, the exact frozen pre-result configuration must receive an independently
verifiable publication or registration record.

The mechanism is not selected by this methodology.

Acceptable mechanism-neutral properties are:

- the exact configuration identity is externally verifiable;
- the publication occurred before evidence generation;
- the record is not silently replaceable;
- changed configurations receive new identities; and
- prior registered configurations remain discoverable.

A public version-control publication may participate only if the later
pre-result configuration explicitly documents how it satisfies the applicable
auditability requirements.

Registration establishes evidence timing and auditability only.

It creates no protocol authority.

## 32. Two-Phase Evidence Chain

The evidence chain must keep pre-result planning separate from post-result data.

### Phase A — Frozen Before Results

Contains:

- frozen source artifacts;
- frozen evidence methodology;
- frozen pre-result configuration;
- planned checks;
- planned exclusions;
- planned deviation handling;
- tool or manual procedure identities; and
- pre-result publication/registration receipt.

### Phase B — Created After Evidence Begins

Contains:

- immutable evidence record identities;
- completed checks;
- failed checks;
- deviations;
- exclusions;
- evidence-tool failures;
- raw evidence artifacts;
- summary artifacts; and
- independent review or reproduction records.

Phase B must reference Phase A.

Phase B must not mutate Phase A.

## 33. Run / Evidence Record Identity

If evidence generation is implemented as one or more runs, every started run
must receive a discoverable identity.

A run lifecycle may end as:

```text
COMPLETED
FAILED
ABORTED
SUPERSEDED
DEVIATED
```

This run lifecycle vocabulary is bookkeeping for evidence execution.

It does not replace or reinterpret the neutral workload-model status taxonomy.
For example, a run may have:

```text
run_lifecycle_status = FAILED
neutral_status_observation = RUN_INFRASTRUCTURE_FAILURE
```

or:

```text
run_lifecycle_status = COMPLETED
all deterministic shared checks = SATISFIED
```

The run identity and final lifecycle status remain retained.

A failed run must not disappear merely because a later run succeeds.

For purely deterministic document checks, one run per frozen pre-result
configuration may be sufficient if the configuration predeclares that plan.

## 34. Source-Tree Cleanliness

Evidence generation should normally use a clean repository state.

If a dirty tree is intentionally used:

- the complete diff must be preserved;
- the dirty-tree status must be recorded;
- the evidence record must bind the diff identity; and
- the reason must be disclosed.

Recording only:

```text
dirty = true
```

is insufficient for reproducible evidence.

## 35. Correctness Verification

The evidence procedure must verify at least:

### Shared Bindings

- semantic-case hash;
- paired-manifest hash;
- candidate mapping hash;
- mapping alias;
- mapping maturity;
- mapping status; and
- `A = 1`.

### Account Mapping

- pre-effect state equals the frozen Account mapping;
- post-effect state equals the frozen Account mapping;
- `(1, 0) -> (0, 1)` is demonstrated;
- amount preservation is demonstrated;
- no-op is rejected; and
- relabel-only satisfaction is rejected.

### UTXO Mapping

- pre-effect state equals the frozen UTXO mapping;
- post-effect state equals the frozen UTXO mapping;
- `{ UR_PRE : 1 } -> { UR_POST : 1 }` is demonstrated;
- amount preservation is demonstrated;
- no-op is rejected; and
- relabel-only satisfaction is rejected.

### Shared Exclusions

- no authorization mechanism is introduced;
- no replay mechanism is introduced;
- no one-use protocol semantics are introduced;
- no cryptographic algorithm is introduced;
- no commitment is introduced;
- no canonical resource accounting is introduced;
- no supply-changing behavior is introduced; and
- protocol validity remains unspecified.

## 36. Semantic-Drift Detection

Evidence generation must fail the shared-contract check if it observes or
requires any unreviewed change to:

- `A`;
- semantic case hash;
- paired manifest hash;
- candidate mapping hash;
- `pre_value_0`;
- `post_value_0`;
- amount-preservation predicate;
- required projection-change predicate;
- intrinsic external conflict relation;
- ordering profile;
- authorization scope;
- replay/one-use scope;
- cryptographic scope;
- commitment scope;
- resource scope; or
- supply-changing scope.

Such a change requires a new reviewed configuration.

It is not a deviation that may be silently tolerated.

## 37. Missingness, Scope, and Status Assignment

Missing evidence is never numeric zero.

Scope disposition is determined before evidence availability.

For this frozen pilot, the applicable scope vocabulary is:

```text
IN_SCOPE
EXCLUDED_BY_FROZEN_MANIFEST
NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE
```

For the currently blocked quantitative dimensions:

```text
timing:
    scope = EXCLUDED_BY_FROZEN_MANIFEST

exact bytes:
    scope = EXCLUDED_BY_FROZEN_MANIFEST

canonical resource metrics:
    scope = EXCLUDED_BY_FROZEN_MANIFEST
```

Those dimensions must not be relabeled `EVIDENCE_NOT_COLLECTED`, because this
configuration does not authorize collecting them.

Neutral statuses are assigned only according to the unchanged meanings in
`ACCOUNT_UTXO_WORKLOAD_MODEL.md`.

Examples of distinctions that must remain visible include:

- `CASE_UNDERDEFINED` concerns the common case and applies to both candidates as
  defined by the workload model;
- `BRANCH_NOT_IN_SCOPE` concerns a branch explicitly excluded before results;
- `PROFILE_OR_SCHEMA_UNAVAILABLE` concerns an in-scope required explicit profile
  or external experimental schema that cannot be bound;
- `BRANCH_UNSUPPORTED_BY_MAPPING` concerns a frozen selected branch that cannot
  be represented by a mapping;
- `SHARED_CONTRACT_UNSATISFIED` is determined only against the frozen common
  contract or independently reviewed semantic oracle;
- `DETERMINISTIC_MAPPING_FAILURE` concerns a frozen mapping that reproducibly
  fails under the frozen case and manifest independently of transient
  infrastructure;
- `RUN_INFRASTRUCTURE_FAILURE` remains an infrastructure failure; and
- `EVIDENCE_NOT_COLLECTED` applies only when the mapping may exist and the
  requested in-scope evidence has not been collected.

This methodology does not impose a new one-dimensional precedence over those
workload-model meanings.

The pre-result configuration must bind:

- the scope disposition for every planned evidence field;
- the field in which each applicable neutral status may be recorded;
- any pre-result disambiguation rule needed for that field; and
- the rule that unresolved classification ambiguity is surfaced rather than
  selected post-result.

If an ambiguity cannot be resolved from the frozen workload-model meanings and
pre-result rules, the record must preserve the ambiguity for independent review
or require a new reviewed configuration.

No operator may choose among multiple labels after observing which one makes a
candidate look better.

## 38. Deviation Rules

A deviation is a recorded departure from the frozen execution plan that does not
silently rewrite the frozen semantic contract.

Every deviation must include:

- affected run or evidence-record identity;
- exact deviation;
- reason;
- detection time;
- affected fields;
- reviewer;
- whether evidence remains usable; and
- whether a new pre-result configuration is required.

A material change to the shared contract, artifact binding, evidence methodology,
or comparison scope requires a new configuration.

It must not be handled as a minor deviation.

## 39. Evidence Summary Requirements

A summary produced under this methodology must show separately:

### Shared Contract Results

- binding checks;
- semantic projection checks;
- projection-change checks;
- amount-preservation checks;
- no-op rejection;
- relabeling rejection;
- branch/exclusion preservation; and
- failures or unavailable evidence.

### Account-Native Structural Disclosure

Only Account-native mapping facts.

### UTXO-Native Structural Disclosure

Only UTXO-native mapping facts.

### Excluded / Unavailable Dimensions

Explicit scope disposition for every excluded dimension and, where applicable, the unchanged neutral status defined by the workload model.

The summary must not collapse these sections into one candidate score.

## 40. Raw Evidence Preservation

The summary is not the raw evidence.

The final evidence package must preserve:

- raw evidence records;
- all deterministic check outputs;
- failed check outputs;
- tool logs where a tool is used;
- configuration hashes;
- source/tool identities;
- deviations;
- exclusions;
- review records; and
- the exact summary-generation procedure.

A reviewer must be able to reconstruct the published summary from the raw
records.

## 41. Independent Review

Before the pre-result configuration is frozen, independent review must attempt
to identify:

- hidden Account-favoring checks;
- hidden UTXO-favoring checks;
- asymmetric missingness treatment;
- false equivalence between candidate-native counters;
- candidate-specific exclusions;
- semantic drift;
- result-dependent edit paths;
- hidden performance assumptions;
- hidden serialization assumptions;
- hidden resource assumptions;
- hidden authorization assumptions;
- hidden replay assumptions;
- hidden one-use assumptions;
- hidden cryptographic assumptions;
- hidden commitment assumptions;
- hidden monetary assumptions;
- inadequate provenance;
- inadequate failure retention; and
- conclusions broader than the evidence supports.

Material findings block evidence collection.

## 42. Reproduction

The first deterministic structural evidence package should be reproducible by an
independent reviewer from the frozen artifacts and pre-result configuration.

Independent reproduction should verify:

- artifact hashes;
- shared contract results;
- candidate mapping transitions;
- amount preservation;
- no-op rejection;
- relabeling rejection;
- exclusion preservation; and
- summary reconstruction.

A reproduction report must identify its own tooling and artifact hashes.

Independent reproduction provides evidence confidence only.

It creates no protocol authority.

## 43. Relationship to `BENCHMARK_METHODOLOGY.md`

`BENCHMARK_METHODOLOGY.md` remains the general non-normative evidence framework
for benchmark provenance, reproducibility, formal campaign registration,
statistics, anti-gaming controls, raw-data preservation, and future physical
measurements.

This pilot methodology narrows that general framework to a case where:

- no performance benchmark is currently authorized;
- no statistical samples are currently required;
- no exact-byte schema exists;
- no canonical resource metric exists;
- no physical database mapping exists; and
- the first evidence is deterministic semantic/mapping evidence.

If this pilot later introduces physical measurement, every applicable general
benchmark requirement becomes binding for a campaign claiming conformance to
that non-normative methodology.

This pilot methodology cannot weaken the general benchmark requirements.

## 44. Benchmark-Gaming Defenses for This Pilot

Even without timing, evidence can still be gamed.

| Risk | Required defense |
|---|---|
| Favorable-only semantic checks | Freeze the complete shared-contract checklist before evidence |
| Candidate-specific check wording | Use the same external predicates for both mappings |
| Counter cherry-picking | Keep candidate-native counters descriptive and separate |
| Missingness hidden as zero | Use explicit scope dispositions plus the unchanged neutral status taxonomy where applicable |
| Failed evidence deleted | Retain every failed or incomplete record |
| Post-result scope changes | New pre-result configuration identity |
| Mapping substitution | Validate exact frozen mapping hashes |
| Evidence-only relabeling | Require mapped-state grounding |
| Artificial no-op success | Explicit no-op rejection checks |
| Broad candidate-family claims | Require mapping-qualified conclusions |
| Hidden performance claims | Performance dimensions explicitly excluded |
| Summary-only publication | Preserve raw evidence and reconstruction path |

## 45. Security Boundary

This evidence methodology does not select security-sensitive protocol
mechanisms.

It does not weaken existing security requirements.

Evidence under this pilot must not be used to infer security equivalence between
Account and UTXO.

In particular, this pilot provides no comparative evidence about:

- theft resistance;
- ownership safety;
- authorization coverage;
- key compromise;
- replay protection;
- double-spend rules;
- cryptographic forgery;
- cryptographic migration;
- state commitment;
- hostile validation;
- denial of service;
- failure atomicity;
- reorganization safety;
- persistent-state exhaustion; or
- supply security.

Those require separately scoped semantic cases, manifests, mappings, and
evidence.

## 46. Formal-Specification Boundary

Nothing produced under this methodology becomes a Formal Specification rule by
repetition or successful experimentation.

This methodology selects no:

- field;
- encoding;
- transaction structure;
- state structure;
- ownership rule;
- replay rule;
- lifecycle rule;
- commitment;
- resource counter;
- resource limit;
- fee;
- cryptographic algorithm; or
- consensus mechanism.

A future protocol rule requires the authoritative protocol process.

## 47. Change Control

This methodology is currently:

```text
FROZEN
```

The actual-file review, focused material-finding resolution, and freeze gate are
complete for this artifact.

After freeze, a material change to any of the following creates new methodology
content and requires a new evidence-only identity:

- frozen artifact bindings;
- authorized evidence classes;
- directly comparable dimensions;
- excluded dimensions;
- evidence record requirements;
- correctness checks;
- missingness rules;
- failure rules;
- exclusion rules;
- deviation rules;
- pre-result configuration requirements;
- registration requirements;
- reproduction requirements; or
- permitted conclusion scope.

Old evidence remains bound to the old methodology identity.

## 48. Current Project Impact

Creation of this frozen evidence methodology has the following project impact:

```text
Account selected:
NO

UTXO selected:
NO

Hybrid selected:
NO

State-model decision:
NOT MADE

Frozen semantic case changed:
NO

Frozen paired manifest changed:
NO

Frozen Account mapping changed:
NO

Frozen UTXO mapping changed:
NO

Evidence methodology frozen:
YES

Pre-result configuration exists:
NO

Comparative evidence collected:
NO

Physical benchmark authorized:
NO

Exact-byte evidence authorized:
NO

Logical read/write comparison authorized:
NO

Resource-score comparison authorized:
NO

Protocol validity defined:
NO

Formal Specification update:
NOT JUSTIFIED BY THIS FROZEN METHODOLOGY

PROJECT_STATE update:
NOT JUSTIFIED BY THIS FROZEN METHODOLOGY

THREAT_MODEL update:
NOT JUSTIFIED BY THIS FROZEN METHODOLOGY

Consensus implementation change:
NONE
```

## 49. Review Gate

The freeze review for this methodology covered:

1. all four frozen artifact hashes;
2. preservation of the exact frozen semantic contract;
3. symmetric Account/UTXO treatment at the shared-contract layer;
4. prevention of false common metrics from candidate-native counters;
5. blocking of timing, bytes, reads/writes, resource units, crypto, commitment,
   migration, and hostile-validation evidence;
6. separation of missingness from numeric zero;
7. retention of failed and deviating records;
8. prevention of post-result methodology editing;
9. reproducible pre-result provenance without a self-referential content hash or
   repository revision;
10. separation of scope dispositions, neutral statuses, deterministic check
    outcomes, and run lifecycle statuses without reinterpreting the workload-model
    taxonomy; and
11. avoidance of candidate-winner and protocol-adoption claims.

No unresolved Critical, High, Medium, or Low findings remain at this freeze gate.

## 50. Next Gate

This methodology does not yet authorize evidence collection.

The next permitted workflow is:

1. establish this frozen methodology's evidence-only content identity;
2. create the pilot pre-result configuration bound to that identity;
3. independently review and freeze that configuration;
4. establish the frozen pre-result configuration's external evidence-only content
   identity;
5. establish an independently verifiable pre-result publication or registration
   record;
6. only then create deterministic structural evidence records; and
7. preserve raw records, failures, deviations, and reconstruction provenance.

Until those gates pass, evidence collection remains:

```text
NOT AUTHORIZED
```
