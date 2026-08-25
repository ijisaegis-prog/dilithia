# Account–UTXO Pilot Pre-Result Configuration: Value-Effect Baseline

> **NON-NORMATIVE EXPERIMENTAL PRE-RESULT CONFIGURATION**
>
> **Status: FROZEN — EXTERNAL EVIDENCE-ONLY CONTENT IDENTITY**
>
> **Evidence scope: DETERMINISTIC STRUCTURAL / SEMANTIC-MAPPING EVIDENCE ONLY**
>
> This document freezes the pre-result plan for the first deterministic evidence
> stage of the `value-effect-baseline` Account/UTXO pilot.
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
> - frozen `VALUE_EFFECT_BASELINE_ACCOUNT_MAPPING.md`;
> - frozen `VALUE_EFFECT_BASELINE_UTXO_MAPPING.md`; and
> - frozen `VALUE_EFFECT_BASELINE_EVIDENCE_METHODOLOGY.md`.

## 1. Purpose

This configuration defines the exact pre-result plan that must be frozen before
any evidence record is generated for the first `value-effect-baseline` pilot
evidence stage.

It binds, before results:

1. the exact frozen semantic case;
2. the exact frozen paired manifest;
3. the exact frozen Account reference mapping;
4. the exact frozen UTXO reference mapping;
5. the exact frozen evidence methodology;
6. the immutable source revision containing those frozen artifacts;
7. the permitted evidence-record representation;
8. the deterministic shared checks;
9. the candidate-native disclosure fields;
10. the scope disposition of every planned evidence dimension;
11. failure, missingness, status, exclusion, and deviation handling;
12. the run plan;
13. the review and reproduction plan; and
14. the pre-result publication / registration plan.

This configuration does not itself contain evidence results.

## 2. Configuration Identity

Pre-result configuration format version:

```text
pilot-account-utxo-pre-result-configuration/v1
```

Pre-result configuration alias:

```text
pilot/value-effect-baseline/pre-result-configuration/v1
```

Pre-result configuration content identity:

```text
NOT EMBEDDED — EXTERNAL EVIDENCE-ONLY CONTENT IDENTITY
```

After independent actual-file review and freeze, the exact frozen bytes of this
file receive an external SHA-256 evidence-only identity.

That final SHA-256 identity is not embedded into this file.

Any byte change after freeze creates a different pre-result configuration
identity and therefore a different evidence configuration.

## 3. Frozen Input Bindings

### 3.1 Common Semantic Case

File:

```text
docs/experiments/account-utxo/pilot/VALUE_EFFECT_BASELINE_CASE.md
```

Evidence-only content identity:

```text
sha256:1DB6FBE63C13BFB619BFECE7D61D0A9958F00E2BF8866331A4602BA0240B276D
```

### 3.2 Paired Comparison Manifest

File:

```text
docs/experiments/account-utxo/pilot/VALUE_EFFECT_BASELINE_PAIRED_MANIFEST.md
```

Evidence-only content identity:

```text
sha256:219B286FD6D4EFB7ECE6F18B5872A34C22F6209E8CE45FA8BE96DD9AB1081D83
```

### 3.3 Account Reference Mapping

File:

```text
docs/experiments/account-utxo/pilot/VALUE_EFFECT_BASELINE_ACCOUNT_MAPPING.md
```

Evidence-only content identity:

```text
sha256:94FD6D8C72137CBCB228C32C8D298930C36C99BEA1BB167FFAE747D69B57F600
```

Mapping alias:

```text
pilot/value-effect-baseline/account/reference/v1
```

Mapping status:

```text
MAPPED
```

Mapping maturity:

```text
REFERENCE
```

### 3.4 UTXO Reference Mapping

File:

```text
docs/experiments/account-utxo/pilot/VALUE_EFFECT_BASELINE_UTXO_MAPPING.md
```

Evidence-only content identity:

```text
sha256:3211A348EDA0AE97271F22C1D6C0622B29366BCDFACDE1196E73D350249265C3
```

Mapping alias:

```text
pilot/value-effect-baseline/utxo/reference/v1
```

Mapping status:

```text
MAPPED
```

Mapping maturity:

```text
REFERENCE
```

### 3.5 Frozen Evidence Methodology

File:

```text
docs/experiments/account-utxo/pilot/VALUE_EFFECT_BASELINE_EVIDENCE_METHODOLOGY.md
```

Evidence methodology format version:

```text
pilot-account-utxo-evidence-methodology/v1
```

Evidence methodology alias:

```text
pilot/value-effect-baseline/evidence-methodology/v1
```

External evidence-only content identity:

```text
sha256:E3969792F70B0F61BD2A2C4130F27C934CA29FF2C8CA063990C1172D7C53B312
```

No frozen input may be silently substituted.

Any mismatch in one of these five identities blocks evidence generation under
this configuration.

## 4. Evidence Source Revision

The pre-existing immutable Dilithia source revision for the evidence procedure is:

```text
ba768dbfc4189987f1ea8231f3b1c3ec442f5fda
```

This revision contains the complete frozen input set and the frozen evidence
methodology.

It is intentionally not the future commit that may contain this pre-result
configuration.

The evidence procedure must use the frozen inputs from exactly this source
revision.

## 5. Repository Cleanliness and Source Materialization

Evidence source execution must use a dedicated checkout whose `HEAD` is exactly:

```text
ba768dbfc4189987f1ea8231f3b1c3ec442f5fda
```

The dedicated source checkout is separate from the working tree that contains
this later pre-result configuration.

The source checkout is considered clean only when both of the following are true:

```text
git rev-parse HEAD
```

writes exactly the following 40 lowercase hexadecimal digits followed by one LF
to standard output:

```text
ba768dbfc4189987f1ea8231f3b1c3ec442f5fda
```

and:

```text
git status --porcelain=v1 --untracked-files=all
```

writes zero bytes to standard output. An interactive shell prompt is not command
output and is not part of this check.

This definition includes untracked files. A checkout with any tracked
modification, staged change, deletion, rename, or untracked file is not clean for
this evidence configuration.

Dirty source trees are not permitted.

Therefore:

```text
repository_clean_or_dirty_state = CLEAN_REQUIRED
complete_dirty_diff_identity_if_applicable = NOT_APPLICABLE_DIRTY_TREE_PROHIBITED
```

The evidence procedure must hash the five frozen bound files from this exact clean
checkout and compare them with Section 3 before candidate evaluation begins.

The pre-result configuration itself is a separate Phase A artifact. After freeze
and registration it is supplied to the procedure by its own external SHA-256
identity and does not need to be copied into the bound source checkout.

The filesystem path of the dedicated checkout is operational provenance only and
is not part of the evidence identity.

A later repository checkout must not be treated as equivalent merely because the
five filenames appear unchanged.

## 6. Shared Experimental Contract

The paired manifest binds:

```text
A = 1
```

Required external projections:

```text
before:
    term   = pre_value_0
    amount = 1

after:
    term   = post_value_0
    amount = 1
```

Required predicates:

```text
amount preserved       = YES
external projection    = MUST CHANGE
no-op satisfaction     = PROHIBITED
evidence-only relabel  = PROHIBITED
```

Protocol validity remains:

```text
UNSPECIFIED
```

## 7. Canonical Evidence Artifact Encoding

The first evidence stage uses non-run-based deterministic document evaluation.

Primary candidate evidence record format version:

```text
pilot-account-utxo-deterministic-evidence-record/v1
```

Candidate evidence record filename pattern:

```text
VALUE_EFFECT_BASELINE_EVIDENCE_<CANDIDATE>.txt
```

where `<CANDIDATE>` is exactly one of:

```text
ACCOUNT
UTXO
```

Every content-addressed text artifact whose schema is defined by this
configuration uses the canonical key-value byte grammar below unless this
configuration explicitly identifies the artifact as preserved raw attempt bytes.

The canonical grammar applies to:

- finalized candidate evidence records;
- procedure-incident artifacts;
- candidate-review artifacts;
- the registration-receipt artifact;
- the procedure-provenance artifact;
- the deterministic evidence summary; and
- the final evidence manifest.

Canonical byte grammar:

```text
encoding = UTF-8
UTF-8 BOM = PROHIBITED
line ending = LF only
final LF = EXACTLY ONE
blank lines = PROHIBITED
tab characters = PROHIBITED
trailing spaces = PROHIBITED
leading spaces before key = PROHIBITED
field delimiter = the first "=" byte on the line
field order = FIXED BY THIS CONFIGURATION
duplicate keys = PROHIBITED
unknown keys = PROHIBITED
omitted required keys = PROHIBITED
empty values = PROHIBITED
```

A field line is exactly:

```text
<key>=<value>\n
```

Key grammar is:

```text
[A-Za-z0-9_.]+
```

The exact artifact schemas below still prohibit unknown keys and freeze every
permitted key spelling and position. The grammar permits the uppercase `A`
needed by the frozen key `shared_parameter_A`; it does not authorize additional
keys.

Values are single-line UTF-8 strings. A value must not contain CR, LF, or TAB.
The first `=` byte separates key and value. Additional `=` bytes inside the value
are permitted and are data. A parser must split only on the first `=` byte.

Leading or trailing ASCII spaces in a value are prohibited. Internal ASCII spaces
are permitted when the applicable exact field rule permits them.

No parser may trim, case-fold, Unicode-normalize, reorder, infer, or substitute a
field.

Hashes use exactly:

```text
sha256:<64 UPPERCASE HEX DIGITS>
```

Git revision fields use exactly 40 lowercase hexadecimal digits.

UTC provenance timestamps use exactly:

```text
YYYY-MM-DDTHH:MM:SSZ
```

with zero-padded Gregorian date/time fields, UTC only, seconds precision, no
fractional seconds, and no numeric UTC offset.

Where this configuration freezes `NONE`, `NOT_APPLICABLE_NON_RUN_BASED`,
`NOT_AVAILABLE_CANDIDATE_RECORD_NOT_FINALIZED`,
`NOT_APPLICABLE_CANDIDATE_RECORD_NOT_FINALIZED`, or another uppercase token,
that spelling is exact.

A comma-separated hash list contains either `NONE` or one or more external
`sha256:` identities sorted lexicographically with no spaces.

Preserved raw candidate-record attempt bytes are an explicit exception to the
canonical key-value grammar because the purpose of that artifact is to retain the
exact incomplete or malformed bytes that existed when finalization failed. Those
raw bytes are hashed exactly as preserved and are never normalized before hashing.

This representation is evidence-only and non-normative. It defines no protocol
object, wire format, transaction format, state format, or consensus record.

## 8. Evidence Record Content Addressing

For each finalized candidate evidence record:

1. construct the exact record using Section 7 and the exact field order in
   Section 9;
2. populate only values permitted by this configuration;
3. encode as UTF-8 without BOM;
4. use LF line endings only;
5. require exactly one final LF;
6. perform no byte rewrite after finalization;
7. compute SHA-256 over the exact final file bytes;
8. represent the external identity as:

```text
sha256:<64 UPPERCASE HEX DIGITS>
```

9. do not embed the final record SHA-256 into the hashed record itself; and
10. preserve the exact record bytes together with the externally recorded hash.

The field:

```text
evidence_record_content_identity
```

therefore contains exactly:

```text
NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
```

Any byte change creates a different evidence-record identity.

## 9. Required Candidate Evidence Record Field Order

Each candidate evidence record contains exactly the following top-level fields in
this order before the candidate-native fields defined by Section 16:

```text
evidence_record_format_version
evidence_record_alias
evidence_record_content_identity
evidence_methodology_hash
pre_result_configuration_hash
evidence_source_revision
semantic_case_hash
paired_manifest_hash
candidate_mapping_hash
candidate_family
candidate_variant
mapping_maturity
mapping_status
shared_parameter_A
pre_effect_projection
post_effect_projection
case_binding_check
manifest_binding_check
mapping_binding_check
shared_parameter_check
mapping_maturity_check
mapping_status_check
pre_effect_projection_check
post_effect_projection_check
required_projection_change_check
amount_preservation_check
mapped_state_change_check
no_op_rejection_check
evidence_only_relabel_rejection_check
external_conflict_relation_check
ordering_profile_check
protocol_validity_check
excluded_dimensions_preservation_check
candidate_ranking_check
dimension_scope_dispositions
```

The candidate-native fields then appear in the exact candidate-specific order
defined by Section 16.

After the candidate-native fields, the record ends with exactly:

```text
neutral_status_observations_if_applicable
run_lifecycle_status_if_run_based
deviations
exclusions
review_provenance
generator_or_analysis_provenance
```

Static shared values are:

```text
evidence_record_format_version=pilot-account-utxo-deterministic-evidence-record/v1
evidence_record_content_identity=NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
evidence_methodology_hash=sha256:E3969792F70B0F61BD2A2C4130F27C934CA29FF2C8CA063990C1172D7C53B312
evidence_source_revision=ba768dbfc4189987f1ea8231f3b1c3ec442f5fda
semantic_case_hash=sha256:1DB6FBE63C13BFB619BFECE7D61D0A9958F00E2BF8866331A4602BA0240B276D
paired_manifest_hash=sha256:219B286FD6D4EFB7ECE6F18B5872A34C22F6209E8CE45FA8BE96DD9AB1081D83
mapping_maturity=REFERENCE
mapping_status=MAPPED
shared_parameter_A=1
pre_effect_projection=(pre_value_0,1)
post_effect_projection=(post_value_0,1)
dimension_scope_dispositions=frozen_artifact_binding_checks:IN_SCOPE,shared_parameter_A:IN_SCOPE,pre_effect_projection:IN_SCOPE,post_effect_projection:IN_SCOPE,required_projection_change:IN_SCOPE,amount_preservation:IN_SCOPE,candidate_mapped_state_change:IN_SCOPE,no_op_rejection:IN_SCOPE,evidence_only_relabel_rejection:IN_SCOPE,intrinsic_external_conflict_relation:IN_SCOPE,ordering_profile_preservation:IN_SCOPE,protocol_validity_unspecified_preservation:IN_SCOPE,candidate_native_structural_disclosure:IN_SCOPE,frozen_exclusion_preservation:IN_SCOPE,benchmark_wall_clock_timing:EXCLUDED_BY_FROZEN_MANIFEST,throughput:EXCLUDED_BY_FROZEN_MANIFEST,latency:EXCLUDED_BY_FROZEN_MANIFEST,cpu_cycles:EXCLUDED_BY_FROZEN_MANIFEST,allocation_counts:EXCLUDED_BY_FROZEN_MANIFEST,peak_memory:EXCLUDED_BY_FROZEN_MANIFEST,database_operations:EXCLUDED_BY_FROZEN_MANIFEST,physical_storage_growth:EXCLUDED_BY_FROZEN_MANIFEST,network_measurements:EXCLUDED_BY_FROZEN_MANIFEST,exact_encoded_bytes:EXCLUDED_BY_FROZEN_MANIFEST,canonical_state_bytes:EXCLUDED_BY_FROZEN_MANIFEST,logical_read_metric:EXCLUDED_BY_FROZEN_MANIFEST,logical_write_metric:EXCLUDED_BY_FROZEN_MANIFEST,record_creation_metric:EXCLUDED_BY_FROZEN_MANIFEST,record_retirement_metric:EXCLUDED_BY_FROZEN_MANIFEST,canonical_resource_units:EXCLUDED_BY_FROZEN_MANIFEST,gas:EXCLUDED_BY_FROZEN_MANIFEST,candidate_attempt_accounting:EXCLUDED_BY_FROZEN_MANIFEST,fees:EXCLUDED_BY_FROZEN_MANIFEST,cryptographic_verification_work:EXCLUDED_BY_FROZEN_MANIFEST,state_commitment_work:EXCLUDED_BY_FROZEN_MANIFEST,migration_work:EXCLUDED_BY_FROZEN_MANIFEST,hostile_validation_performance:EXCLUDED_BY_FROZEN_MANIFEST,replay_behavior:EXCLUDED_BY_FROZEN_MANIFEST,reorganization_behavior:EXCLUDED_BY_FROZEN_MANIFEST,benchmark_warmup:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,stochastic_sample_count:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,confidence_intervals:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,percentiles:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,thermal_policy:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,cache_policy:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,cpu_affinity:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,outlier_policy_for_deterministic_checks:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE
run_lifecycle_status_if_run_based=NOT_APPLICABLE_NON_RUN_BASED
deviations=NONE
exclusions=PREDECLARED_SCOPE_ONLY_NO_RESULT_BEARING_EXCLUSION
review_provenance=EXTERNAL_REVIEW_RECORD_REQUIRED_BY_EVIDENCE_RECORD_HASH
generator_or_analysis_provenance=pilot/value-effect-baseline/manual-deterministic-evidence-procedure/v1
```

`pre_result_configuration_hash` is the external frozen SHA-256 established after
this configuration passes its freeze gate.

`neutral_status_observations_if_applicable` is either:

```text
NONE
```

or a comma-separated list of applicable workload-model labels with no spaces,
ordered according to the taxonomy order in Section 18. No other ordering is
permitted.

A finalized candidate evidence record may use `deviations=NONE` only. If a
deviation occurs before finalization, the candidate record is not silently
repaired; a procedure-incident artifact is retained under Section 25 and the
evidence attempt is handled under Sections 20 and 23.

## 10. Candidate Record Fixed Identities

### 10.1 Account

Account evidence-record alias and exact fixed candidate identity values:

```text
evidence_record_alias=pilot/value-effect-baseline/evidence/account/reference/v1
candidate_mapping_hash=sha256:94FD6D8C72137CBCB228C32C8D298930C36C99BEA1BB167FFAE747D69B57F600
candidate_family=Account
candidate_variant=persistent-logical-value-relations/two-relation-reference/v1
```

### 10.2 UTXO

UTXO evidence-record alias and exact fixed candidate identity values:

```text
evidence_record_alias=pilot/value-effect-baseline/evidence/utxo/reference/v1
candidate_mapping_hash=sha256:3211A348EDA0AE97271F22C1D6C0622B29366BCDFACDE1196E73D350249265C3
candidate_family=UTXO
candidate_variant=discrete-value-records/one-to-one-replacement-reference/v1
```

These aliases and candidate identities are evidence-only and are not protocol
identifiers.

## 11. Evidence Generator or Manual Procedure Identity

Evidence generation mode:

```text
NON_RUN_BASED_MANUAL_DETERMINISTIC_DOCUMENT_EVALUATION_WITH_INDEPENDENT_SEMANTIC_REVIEW
```

Procedure identity:

```text
pilot/value-effect-baseline/manual-deterministic-evidence-procedure/v1
```

The complete procedure definition is embedded in this pre-result configuration.
The frozen configuration identity therefore binds this procedure definition.

The procedure must:

1. independently obtain the frozen pre-result configuration bytes and verify the
   registered external SHA-256;
2. materialize the dedicated source checkout defined by Sections 4 and 5;
3. verify the exact source revision and exact clean-check result;
4. compute and verify all five frozen input SHA-256 identities before either
   candidate record is started;
5. evaluate the shared checks in the exact order frozen in Section 13;
6. transcribe only facts directly supported by the frozen case, manifest, and
   candidate mapping;
7. use the exact byte grammar and field order frozen in Sections 7 through 10;
8. use the exact candidate-native values and order frozen in Section 16;
9. retain every failed or incomplete check;
10. retain explicit missingness and scope dispositions;
11. avoid converting missingness or exclusion to numeric zero;
12. finalize and hash each candidate record that can be finalized; when a
    candidate record cannot be finalized, content-address any emitted raw attempt
    bytes, create the required procedure incident, and establish the exact
    candidate-record and review absence state before creating the evidence summary;
13. never edit a finalized record in place;
14. create a new record identity if a permitted non-material re-evaluation is
    necessary and retain the earlier record or incident;
15. create the independent review artifacts required by Section 25; and
16. construct the final evidence manifest only after its bound artifacts exist.

This configuration declares no semantic shared check as tool-generated.

Mechanical SHA-256 implementations are interchangeable only with respect to the
standard SHA-256 algorithm. The exact implementation name, version if available,
runtime, and command or API used must be recorded in provenance. A hash used in
the evidence package must be independently recomputable from the preserved exact
bytes.

A SHA-256 implementation mismatch is treated as an evidence-procedure failure,
not as candidate evidence.

## 12. Analysis Identity

Analysis / summary procedure identity:

```text
pilot/value-effect-baseline/deterministic-summary-procedure/v1
```

The summary procedure is embedded in this configuration.

It may only:

1. reproduce shared-contract results from finalized candidate evidence records
   when they exist and preserve the exact frozen absence state otherwise;
2. present Account-native observations in an Account-native section when a
   finalized Account record exists and preserve the exact absence sentinel
   otherwise;
3. present UTXO-native observations in a UTXO-native section when a finalized
   UTXO record exists and preserve the exact absence sentinel otherwise;
4. present exclusions and unavailable evidence separately;
5. retain failures, procedure incidents, raw record attempts, deviations, and
   exclusions;
6. state the exact evidence methodology hash;
7. state the exact pre-result configuration hash;
8. state the exact candidate evidence-record hashes or frozen absence sentinels;
9. state the exact independent-review artifact hashes or frozen absence sentinels;
   and
10. make only mapping-qualified conclusions permitted by the frozen methodology.

The summary must be created only after all available finalized candidate records
are content-addressed, every required review artifact for those finalized records
exists, every required raw attempt and procedure incident for an unfinalized
candidate is content-addressed, and the registration receipt and procedure-
provenance artifact exist.

The summary itself receives an external SHA-256 over its exact final bytes. Its
hash is bound by the final evidence manifest.

It may not create a scalar score, candidate rank, efficiency claim, protocol
selection, or production-frequency inference.

## 13. Exact Shared Checks and Record Mapping

The following shared checks are frozen before evidence and are evaluated in this
exact order:

```text
SC01 case_binding_check
SC02 manifest_binding_check
SC03 mapping_binding_check
SC04 shared_parameter_check
SC05 mapping_maturity_check
SC06 mapping_status_check
SC07 pre_effect_projection_check
SC08 post_effect_projection_check
SC09 required_projection_change_check
SC10 amount_preservation_check
SC11 mapped_state_change_check
SC12 no_op_rejection_check
SC13 evidence_only_relabel_rejection_check
SC14 external_conflict_relation_check
SC15 ordering_profile_check
SC16 protocol_validity_check
SC17 excluded_dimensions_preservation_check
SC18 candidate_ranking_check
```

Every SC01 through SC18 field is mandatory in each candidate evidence record.

Permitted outcomes are frozen as follows:

```text
SC01 MATCH | MISMATCH
SC02 MATCH | MISMATCH
SC03 MATCH | MISMATCH
SC04 SATISFIED | UNSATISFIED
SC05 SATISFIED | UNSATISFIED
SC06 SATISFIED | UNSATISFIED
SC07 SATISFIED | UNSATISFIED
SC08 SATISFIED | UNSATISFIED
SC09 SATISFIED | UNSATISFIED
SC10 SATISFIED | UNSATISFIED
SC11 PRESENT | ABSENT
SC12 REJECTED | ACCEPTED
SC13 REJECTED | ACCEPTED
SC14 EMPTY | NONEMPTY
SC15 NO_ORDER_ASSERTION | MISMATCH
SC16 UNSPECIFIED | SPECIFIED_OR_INFERRED
SC17 SATISFIED | UNSATISFIED
SC18 NONE | CLAIM_PRESENT
```

A successful record therefore uses:

```text
case_binding_check=MATCH
manifest_binding_check=MATCH
mapping_binding_check=MATCH
shared_parameter_check=SATISFIED
mapping_maturity_check=SATISFIED
mapping_status_check=SATISFIED
pre_effect_projection_check=SATISFIED
post_effect_projection_check=SATISFIED
required_projection_change_check=SATISFIED
amount_preservation_check=SATISFIED
mapped_state_change_check=PRESENT
no_op_rejection_check=REJECTED
evidence_only_relabel_rejection_check=REJECTED
external_conflict_relation_check=EMPTY
ordering_profile_check=NO_ORDER_ASSERTION
protocol_validity_check=UNSPECIFIED
excluded_dimensions_preservation_check=SATISFIED
candidate_ranking_check=NONE
```

No failed shared check may be compensated for by a favorable candidate-native
observation.

SC18 evaluates only whether the evidence record itself contains or implies a
candidate-ranking claim. It does not evaluate global candidate superiority.

## 14. Account Mapping Realization

The Account record must verify:

```text
pre:
    quantity(AR_PRE)  = 1
    quantity(AR_POST) = 0

post:
    quantity(AR_PRE)  = 0
    quantity(AR_POST) = 1
```

Required mapped transition:

```text
(1, 0) -> (0, 1)
```

Rejected no-op:

```text
(1, 0) -> (1, 0)
```

A phase-name, timestamp, run-index, or external-label change does not satisfy the
mapped-state change.

## 15. UTXO Mapping Realization

The UTXO record must verify:

```text
pre:
    live_set = { UR_PRE }
    amount(UR_PRE) = 1

post:
    live_set = { UR_POST }
    amount(UR_POST) = 1
```

Required mapped transition:

```text
{ UR_PRE : 1 } -> { UR_POST : 1 }
```

Rejected no-op:

```text
{ UR_PRE : 1 } -> { UR_PRE : 1 }
```

A phase-name, timestamp, run-index, or external-label change does not satisfy the
mapped-state change.

The replacement relation is not interpreted as canonical replay, one-use, or
spent-output semantics.

## 16. Candidate-Native Disclosure Fields

Candidate-native observations are descriptive and are not directly comparable
common metrics.

The fields below appear immediately after
`dimension_scope_dispositions` in the exact listed order.

### 16.1 Account

Required Account-native field order:

```text
logical_value_relation_count
pre_effect_positive_quantity_relation_count
post_effect_positive_quantity_relation_count
mapped_quantity_vector_before
mapped_quantity_vector_after
mapping_local_dependency_description
```

Exact frozen evidence values:

```text
logical_value_relation_count=2
pre_effect_positive_quantity_relation_count=1
post_effect_positive_quantity_relation_count=1
mapped_quantity_vector_before=(1,0)
mapped_quantity_vector_after=(0,1)
mapping_local_dependency_description=The post-effect projection is satisfied only when the candidate state reflects the required quantity reassignment from AR_PRE to AR_POST.
```

The dependency sentence is an evidence-only canonical transcription of the
mapping-level dependency frozen in the Account reference mapping. It adds no
scheduler, database, locking, or protocol dependency semantics.

### 16.2 UTXO

Required UTXO-native field order:

```text
pre_effect_live_logical_value_record_count
post_effect_live_logical_value_record_count
distinct_mapping_local_record_handle_count
mapped_live_set_before
mapped_live_set_after
mapping_local_replacement_relation
mapping_local_dependency_description
```

Exact frozen evidence values:

```text
pre_effect_live_logical_value_record_count=1
post_effect_live_logical_value_record_count=1
distinct_mapping_local_record_handle_count=2
mapped_live_set_before={UR_PRE:1}
mapped_live_set_after={UR_POST:1}
mapping_local_replacement_relation={UR_PRE:1}->{UR_POST:1}
mapping_local_dependency_description=The post-effect projection is satisfied only when the mapped live record set reflects the required replacement from UR_PRE to UR_POST.
```

The dependency sentence is an evidence-only canonical transcription of the
mapping-level dependency frozen in the UTXO reference mapping. It adds no
scheduler, input-graph, database, one-use, replay, or protocol dependency
semantics.

These values must not be converted into a shared ranking merely because they are
counts or integers.

## 17. Dimension Scope Dispositions

The scope vocabulary is exactly:

```text
IN_SCOPE
EXCLUDED_BY_FROZEN_MANIFEST
NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE
```

### 17.1 In Scope

```text
scope = IN_SCOPE
```

applies to:

- frozen artifact binding checks;
- `A = 1`;
- pre-effect and post-effect external projections;
- required external projection change;
- amount preservation;
- candidate mapped-state change;
- no-op rejection;
- evidence-only relabel rejection;
- intrinsic external conflict relation;
- ordering-profile preservation;
- protocol-validity-unspecified preservation;
- candidate-native structural disclosure defined here; and
- preservation of frozen exclusions.

### 17.2 Excluded by Frozen Manifest

```text
scope = EXCLUDED_BY_FROZEN_MANIFEST
```

applies to:

- benchmark wall-clock timing;
- throughput;
- latency;
- CPU cycles;
- allocation counts;
- peak memory;
- database operations;
- physical storage growth;
- network measurements;
- exact encoded bytes;
- canonical state bytes;
- logical read metric;
- logical write metric;
- record creation metric;
- record retirement metric;
- canonical resource units;
- gas;
- candidate-attempt accounting;
- fees;
- cryptographic verification work;
- state commitment work;
- migration work;
- hostile-validation performance;
- replay behavior; and
- reorganization behavior.

These dimensions must not be represented as numeric zero or relabeled
`EVIDENCE_NOT_COLLECTED`.

### 17.3 Not Applicable to This Evidence Stage

```text
scope = NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE
```

applies to:

- benchmark warmup;
- stochastic sample count;
- confidence intervals;
- percentiles;
- thermal policy;
- cache policy;
- CPU affinity; and
- outlier policy for deterministic checks.

### 17.4 Canonical Serialized Scope Map

The `dimension_scope_dispositions` field in each candidate evidence record and
the deterministic evidence summary contains the complete explicit scope map,
not a pointer or an implicit reference.

Its exact value is:

```text
frozen_artifact_binding_checks:IN_SCOPE,shared_parameter_A:IN_SCOPE,pre_effect_projection:IN_SCOPE,post_effect_projection:IN_SCOPE,required_projection_change:IN_SCOPE,amount_preservation:IN_SCOPE,candidate_mapped_state_change:IN_SCOPE,no_op_rejection:IN_SCOPE,evidence_only_relabel_rejection:IN_SCOPE,intrinsic_external_conflict_relation:IN_SCOPE,ordering_profile_preservation:IN_SCOPE,protocol_validity_unspecified_preservation:IN_SCOPE,candidate_native_structural_disclosure:IN_SCOPE,frozen_exclusion_preservation:IN_SCOPE,benchmark_wall_clock_timing:EXCLUDED_BY_FROZEN_MANIFEST,throughput:EXCLUDED_BY_FROZEN_MANIFEST,latency:EXCLUDED_BY_FROZEN_MANIFEST,cpu_cycles:EXCLUDED_BY_FROZEN_MANIFEST,allocation_counts:EXCLUDED_BY_FROZEN_MANIFEST,peak_memory:EXCLUDED_BY_FROZEN_MANIFEST,database_operations:EXCLUDED_BY_FROZEN_MANIFEST,physical_storage_growth:EXCLUDED_BY_FROZEN_MANIFEST,network_measurements:EXCLUDED_BY_FROZEN_MANIFEST,exact_encoded_bytes:EXCLUDED_BY_FROZEN_MANIFEST,canonical_state_bytes:EXCLUDED_BY_FROZEN_MANIFEST,logical_read_metric:EXCLUDED_BY_FROZEN_MANIFEST,logical_write_metric:EXCLUDED_BY_FROZEN_MANIFEST,record_creation_metric:EXCLUDED_BY_FROZEN_MANIFEST,record_retirement_metric:EXCLUDED_BY_FROZEN_MANIFEST,canonical_resource_units:EXCLUDED_BY_FROZEN_MANIFEST,gas:EXCLUDED_BY_FROZEN_MANIFEST,candidate_attempt_accounting:EXCLUDED_BY_FROZEN_MANIFEST,fees:EXCLUDED_BY_FROZEN_MANIFEST,cryptographic_verification_work:EXCLUDED_BY_FROZEN_MANIFEST,state_commitment_work:EXCLUDED_BY_FROZEN_MANIFEST,migration_work:EXCLUDED_BY_FROZEN_MANIFEST,hostile_validation_performance:EXCLUDED_BY_FROZEN_MANIFEST,replay_behavior:EXCLUDED_BY_FROZEN_MANIFEST,reorganization_behavior:EXCLUDED_BY_FROZEN_MANIFEST,benchmark_warmup:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,stochastic_sample_count:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,confidence_intervals:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,percentiles:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,thermal_policy:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,cache_policy:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,cpu_affinity:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,outlier_policy_for_deterministic_checks:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE
```

This serialization order is frozen. No item may be omitted, reordered, renamed,
or assigned a different disposition under this configuration.

## 18. Neutral Status Taxonomy Binding

This configuration does not redefine the workload-model taxonomy.

Only these existing labels may be recorded as neutral status observations:

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

Their meanings and required treatment remain exactly those defined by
`ACCOUNT_UTXO_WORKLOAD_MODEL.md`.

No new precedence order is created.

No `EVIDENCE_COLLECTED` status is invented.

## 19. Status Assignment Rules

- `mapping_status` remains the frozen mapping status.
- Deterministic shared checks use their own categorical outcomes.
- `neutral_status_observations_if_applicable` contains only workload-model labels
  whose unchanged meanings are satisfied.
- explicitly excluded dimensions use `dimension_scope_dispositions`, not
  `EVIDENCE_NOT_COLLECTED`;
- run lifecycle status is separate from neutral workload-model status.

If classification remains ambiguous under the frozen workload-model meanings,
the ambiguity is retained for independent review.

No post-result label choice may be used to make a candidate appear better.

## 20. Failure Handling

The first evidence stage is non-run-based. Candidate semantic failure, procedure
failure, record-attempt failure, and workload-model status remain separate.

Every candidate-record creation that emits any bytes creates a record attempt.
If the attempt is not finalized as a canonical candidate evidence record, the
exact last preserved attempt bytes must be retained unchanged and externally
SHA-256 content-addressed as a raw candidate-record-attempt artifact.

A raw attempt artifact is evidence-procedure provenance. It is not a valid
candidate evidence record and must never be summarized as if it were finalized
evidence.

### 20.1 Frozen Input Hash Mismatch

If any of the five frozen input hashes mismatches:

```text
candidate semantic evaluation = NOT STARTED
candidate evidence record creation = NOT STARTED
candidate evidence record finalization = PROHIBITED
```

A procedure-incident artifact must be created and retained under Section 25.4.

Where the mismatch is caused by checkout, host, verification procedure, or
tooling, the applicable neutral observation is:

```text
RUN_INFRASTRUCTURE_FAILURE
```

The label is retained as the unchanged workload-model classification even though
this evidence stage is not organized as a run.

It is not converted into candidate failure.

### 20.2 Source Revision or Cleanliness Failure

If the exact source revision cannot be materialized or the exact cleanliness rule
in Section 5 is not satisfied:

```text
candidate semantic evaluation = NOT STARTED
candidate evidence record creation = NOT STARTED
candidate evidence record finalization = PROHIBITED
```

A procedure-incident artifact remains discoverable.

No candidate result is inferred.

### 20.3 Shared Contract Failure

If exact frozen inputs are verified but the shared contract is not satisfied, the
candidate evidence record must still be finalized when its byte representation
can be completed.

The failed shared-check outcome remains visible in that finalized candidate
record.

Status classification follows the unchanged workload-model meaning, including
`SHARED_CONTRACT_UNSATISFIED` where applicable.

A failed finalized record must not be deleted because its result is unfavorable.

### 20.4 Deterministic Mapping Failure

If a frozen mapping reproducibly fails under the verified frozen case and
manifest independently of transient procedure infrastructure, the applicable
neutral classification is:

```text
DETERMINISTIC_MAPPING_FAILURE
```

The failed finalized candidate record remains retained when finalization is
possible.

If finalization itself fails, the raw record-attempt bytes and corresponding
procedure incident are retained instead.

### 20.5 Record Representation Failure

If the operator cannot create a candidate record conforming exactly to Sections
7 through 10:

```text
candidate evidence record finalization = PROHIBITED
```

If any candidate-record bytes were emitted, the exact last preserved bytes are
content-addressed without normalization as a raw candidate-record-attempt
artifact.

A procedure-incident artifact is then created and binds that attempt hash, or the
exact token `NONE` when no candidate-record bytes were ever emitted.

The operator must not silently repair a finalized record. If a representation
problem is discovered only after a record was content-addressed, that finalized
record remains retained and any corrected record receives a different content
identity.

### 20.6 Candidate Pair Incompleteness

The evidence summary must not present a normal paired-success conclusion unless
both finalized candidate evidence records required by this configuration exist
and all applicable shared-contract outcomes are visible.

If one candidate record cannot be finalized:

- the missing finalized-record hash uses the exact sentinel defined in Section
  25.8 and propagated by Section 25.9;
- any raw record-attempt artifact remains retained;
- the applicable procedure-incident hash remains retained;
- the missing candidate review uses the exact review sentinel defined in Section
  25.8 and propagated by Section 25.9; and
- the summary reports no normal paired-success conclusion.

Failure-path packaging is therefore still content-addressable and reconstructible
without inventing a successful candidate record.

## 21. Missingness Handling

Missing evidence is never numeric zero.

Every planned field must preserve either:

1. the actual in-scope evidence value;
2. its predeclared scope disposition; or
3. an applicable neutral workload-model status observation.

An excluded field remains excluded even if it could be measured after results are
seen.

An in-scope requested field that was not collected is not silently omitted and
may use `EVIDENCE_NOT_COLLECTED` only when that unchanged workload-model meaning
is satisfied.

No missing value is silently replaced by `0`, `false`, an empty string, or an
empty list.

## 22. Exclusion Rules

Summary-level result-bearing exclusions are prohibited:

```text
post_result_candidate_specific_exclusion = PROHIBITED
result_bearing_record_exclusion = PROHIBITED
failed_record_exclusion = PROHIBITED
```

Every finalized candidate record remains in the evidence package whether
successful or unfavorable.

Every started-but-unfinalized candidate record attempt that emitted bytes remains
in the evidence package as an externally content-addressed raw attempt artifact.

Procedure incidents remain retained independently of whether a later retry
succeeds.

Predeclared dimension exclusions remain visible through the complete canonical
`dimension_scope_dispositions` value.

No exclusion may transform failure into success.

## 23. Deviation Rules

A material deviation always requires a new pre-result configuration identity.

The following are material:

- frozen input hash change;
- evidence methodology hash change;
- source revision change;
- source-cleanliness definition change;
- shared parameter change;
- shared check or permitted-outcome change;
- candidate-native disclosure field or canonical value change;
- scope-disposition change;
- status-assignment rule change;
- exclusion-rule change;
- failure-handling change;
- evidence-record byte grammar or field-order change;
- evidence-record content-addressing change;
- generator/manual procedure change;
- analysis or summary procedure change;
- change from non-run-based execution to run-based execution; or
- publication / registration plan change.

A material deviation stops evidence generation under this identity.

A non-material operational incident that does not change any frozen semantic,
classification, field, byte-format, summary-generation, or candidate-treatment
rule may be retried under the same frozen configuration only if:

1. the incident is content-addressed and retained;
2. any raw candidate-record attempt bytes are content-addressed and retained;
3. any already finalized evidence record remains retained;
4. the reason for retry is recorded before replacement candidate-record bytes are
   finalized; and
5. the retry does not overwrite any earlier artifact.

Procedure-incident artifacts use the exact schema in Section 25.4.

No incident field may be invented after results to convert a material deviation
into a non-material one.

## 24. Run Plan

Execution mode for this first deterministic evidence stage is:

```text
NON_RUN_BASED_DETERMINISTIC_DOCUMENT_EVALUATION
```

Therefore:

```text
run_plan=NOT_RUN_BASED
run_lifecycle_status_if_run_based=NOT_APPLICABLE_NON_RUN_BASED
```

No run identity, run count, candidate execution order, benchmark iteration,
sampling loop, warmup, or timing lifecycle is created by this configuration.

The evidence procedure evaluates two deterministic candidate documents under one
frozen Phase A configuration.

This choice is deliberate because no executable candidate implementation,
physical benchmark, stochastic measurement, or timing evidence is authorized.

If a later evidence stage becomes run-based, it requires a new reviewed
pre-result configuration that freezes run identity, lifecycle, failure, retry,
and provenance rules before results.

## 25. Review Plan and Phase-B Evidence Package

### 25.1 Freeze Review Completed

The completed freeze review covered:

1. full actual-file review;
2. adversarial candidate-symmetry review;
3. hash-binding review;
4. status and scope-taxonomy review;
5. evidence-record byte-grammar review;
6. shared-check-to-record-field completeness review;
7. source-materialization and cleanliness review;
8. failure-path raw-attempt preservation review;
9. result-dependent edit-path review;
10. Phase-B provenance-DAG review;
11. deterministic-summary procedure review;
12. publication / registration auditability review; and
13. a focused freeze gate after corrections.

No unresolved Critical, High, Medium, or Low finding remains at this freeze gate.

### 25.2 Before Evidence Generation

After freeze and registration, an independent reviewer must verify:

- exact frozen configuration SHA-256;
- exact five frozen input identities;
- exact source revision;
- exact clean-source definition;
- exact registration receipt;
- non-run-based execution mode;
- exact shared-check list and permitted outcomes;
- candidate symmetry;
- candidate-native canonical values;
- complete canonical scope map;
- missingness, exclusion, deviation, failure, and raw-attempt retention rules;
- candidate evidence-record byte grammar and field order; and
- exact summary and final-manifest schemas.

### 25.3 Raw Candidate-Record Attempt Artifacts

If candidate-record creation emits bytes but cannot produce a finalized canonical
candidate evidence record, the exact last preserved byte sequence is retained as
a raw candidate-record-attempt artifact.

The raw attempt artifact:

- is hashed exactly as preserved;
- is not normalized, repaired, trimmed, or canonicalized before hashing;
- may be incomplete or malformed;
- is not a valid candidate evidence record;
- is not interpreted as successful candidate evidence; and
- is referenced by the corresponding procedure-incident artifact and the final
  evidence manifest.

If no bytes were emitted for a candidate before the incident, the applicable
attempt-hash field uses:

```text
NONE
```

If more than one raw attempt exists for one candidate, every attempt hash is
retained. Hash lists are lexicographically sorted with no spaces.

### 25.4 Procedure-Incident Artifact

Procedure-incident artifact format version is exactly:

```text
pilot-account-utxo-procedure-incident/v1
```

It uses the canonical key-value grammar in Section 7 and contains exactly these
fields in this order:

```text
procedure_incident_format_version
procedure_incident_content_identity
pre_result_configuration_hash
incident_sequence
incident_time_utc
incident_kind
affected_candidate
neutral_status_observations_if_applicable
description
disposition
record_attempt_hash_if_any
prior_finalized_record_hash_if_any
```

Static values:

```text
procedure_incident_format_version=pilot-account-utxo-procedure-incident/v1
procedure_incident_content_identity=NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
```

`incident_sequence` is a base-10 positive integer without leading zeroes and is
monotonic within this evidence procedure.

`affected_candidate` is exactly one of:

```text
ACCOUNT
UTXO
BOTH
NONE
```

`neutral_status_observations_if_applicable` is exactly `NONE` or a
comma-separated list of applicable Section 18 workload-model labels with no
spaces, serialized in Section 18 taxonomy order. The field does not redefine any
status meaning and must not be chosen to improve a candidate's apparent result.

`record_attempt_hash_if_any` is either `NONE` or one external SHA-256 identity.

`prior_finalized_record_hash_if_any` is either `NONE` or one external SHA-256
identity.

The exact final incident artifact bytes receive an external SHA-256.

### 25.5 Candidate Evidence Review Artifact

Each finalized candidate evidence record receives one independent review artifact
that references the exact candidate record SHA-256.

Candidate-review artifact format version is exactly:

```text
pilot-account-utxo-candidate-evidence-review/v1
```

It uses the canonical key-value grammar in Section 7 and contains exactly these
fields in this order:

```text
review_artifact_format_version
review_artifact_content_identity
pre_result_configuration_hash
candidate_evidence_record_hash
candidate_family
reviewer_identity
review_time_utc
hash_binding_review
semantic_contract_review
candidate_native_transcription_review
scope_and_status_review
finding_summary
review_disposition
```

Static values include:

```text
review_artifact_format_version=pilot-account-utxo-candidate-evidence-review/v1
review_artifact_content_identity=NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
```

`candidate_family` is exactly `Account` or `UTXO`.

Each review-result field is exactly one of:

```text
PASS
FAIL
```

`review_disposition` is exactly one of:

```text
ACCEPTED
REJECTED
```

`finding_summary` is a single-line UTF-8 value under Section 7. `NONE` is used
only when the independent review found no finding to report.

A review finding does not authorize editing a finalized candidate record in
place. Any replacement record receives a different identity and the earlier
record and review artifact remain discoverable.

The exact final review artifact bytes receive an external SHA-256.

### 25.6 Registration Receipt Artifact

Before evidence generation, the independently verified public registration is
captured in one registration-receipt artifact.

Registration-receipt format version is exactly:

```text
pilot-account-utxo-pre-result-registration-receipt/v1
```

It uses the canonical key-value grammar in Section 7 and contains exactly these
fields in this order:

```text
registration_receipt_format_version
registration_receipt_content_identity
pre_result_configuration_hash
public_repository_identity
pull_request_number
source_commit
merge_commit
public_merge_event_time_utc
verified_configuration_path
verified_configuration_sha256
independent_verification_status
verifier_identity
verification_time_utc
```

Static values include:

```text
registration_receipt_format_version=pilot-account-utxo-pre-result-registration-receipt/v1
registration_receipt_content_identity=NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
public_repository_identity=github.com/ijisaegis-prog/dilithia
verified_configuration_path=docs/experiments/account-utxo/pilot/VALUE_EFFECT_BASELINE_PRE_RESULT_CONFIGURATION.md
independent_verification_status=VERIFIED
```

`pull_request_number` is a base-10 positive integer without leading zeroes.

`source_commit` and `merge_commit` use the exact Git revision grammar from
Section 7.

`verified_configuration_sha256` must equal the external frozen
pre-result-configuration SHA-256.

The exact receipt bytes receive an external SHA-256.

The receipt records evidence timing and auditability only.

### 25.7 Procedure Provenance Artifact

Before the deterministic summary is finalized, create one procedure-provenance
artifact.

Procedure-provenance format version is exactly:

```text
pilot-account-utxo-procedure-provenance/v1
```

It uses the canonical key-value grammar in Section 7 and contains exactly these
fields in this order:

```text
procedure_provenance_format_version
procedure_provenance_content_identity
pre_result_configuration_hash
evidence_source_revision
source_head_verification
source_cleanliness_verification
five_input_hash_verifications
sha256_implementation_identity
sha256_runtime_identity
sha256_command_or_api
operator_or_procedure_identity
procedure_start_time_utc
procedure_end_time_utc
```

Static values include:

```text
procedure_provenance_format_version=pilot-account-utxo-procedure-provenance/v1
procedure_provenance_content_identity=NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
evidence_source_revision=ba768dbfc4189987f1ea8231f3b1c3ec442f5fda
```

`source_head_verification` is exactly:

```text
VERIFIED
```

or:

```text
FAILED
```

`source_cleanliness_verification` is exactly `VERIFIED` or `FAILED`.

`five_input_hash_verifications` is exactly `VERIFIED` only when all five Section
3 identities match; otherwise it is `FAILED`.

The exact artifact bytes receive an external SHA-256. This artifact records
execution provenance only and does not create candidate evidence.

### 25.8 Deterministic Evidence Summary Artifact

The evidence summary is a deterministic content-addressed artifact, not free-form
post-result prose.

Summary format version is exactly:

```text
pilot-account-utxo-deterministic-evidence-summary/v1
```

It uses the canonical key-value grammar in Section 7 and contains exactly these
fields in this order:

```text
summary_format_version
summary_content_identity
evidence_methodology_hash
pre_result_configuration_hash
evidence_source_revision
registration_receipt_hash
procedure_provenance_hash
account_evidence_record_hash
utxo_evidence_record_hash
account_shared_check_results
utxo_shared_check_results
account_native_disclosure
utxo_native_disclosure
dimension_scope_dispositions
neutral_status_observations
procedure_incident_hashes
account_record_attempt_hashes
utxo_record_attempt_hashes
account_review_artifact_hash
utxo_review_artifact_hash
deviations
exclusions
mapping_qualified_conclusion
```

Static values include:

```text
summary_format_version=pilot-account-utxo-deterministic-evidence-summary/v1
summary_content_identity=NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
evidence_methodology_hash=sha256:E3969792F70B0F61BD2A2C4130F27C934CA29FF2C8CA063990C1172D7C53B312
evidence_source_revision=ba768dbfc4189987f1ea8231f3b1c3ec442f5fda
dimension_scope_dispositions=frozen_artifact_binding_checks:IN_SCOPE,shared_parameter_A:IN_SCOPE,pre_effect_projection:IN_SCOPE,post_effect_projection:IN_SCOPE,required_projection_change:IN_SCOPE,amount_preservation:IN_SCOPE,candidate_mapped_state_change:IN_SCOPE,no_op_rejection:IN_SCOPE,evidence_only_relabel_rejection:IN_SCOPE,intrinsic_external_conflict_relation:IN_SCOPE,ordering_profile_preservation:IN_SCOPE,protocol_validity_unspecified_preservation:IN_SCOPE,candidate_native_structural_disclosure:IN_SCOPE,frozen_exclusion_preservation:IN_SCOPE,benchmark_wall_clock_timing:EXCLUDED_BY_FROZEN_MANIFEST,throughput:EXCLUDED_BY_FROZEN_MANIFEST,latency:EXCLUDED_BY_FROZEN_MANIFEST,cpu_cycles:EXCLUDED_BY_FROZEN_MANIFEST,allocation_counts:EXCLUDED_BY_FROZEN_MANIFEST,peak_memory:EXCLUDED_BY_FROZEN_MANIFEST,database_operations:EXCLUDED_BY_FROZEN_MANIFEST,physical_storage_growth:EXCLUDED_BY_FROZEN_MANIFEST,network_measurements:EXCLUDED_BY_FROZEN_MANIFEST,exact_encoded_bytes:EXCLUDED_BY_FROZEN_MANIFEST,canonical_state_bytes:EXCLUDED_BY_FROZEN_MANIFEST,logical_read_metric:EXCLUDED_BY_FROZEN_MANIFEST,logical_write_metric:EXCLUDED_BY_FROZEN_MANIFEST,record_creation_metric:EXCLUDED_BY_FROZEN_MANIFEST,record_retirement_metric:EXCLUDED_BY_FROZEN_MANIFEST,canonical_resource_units:EXCLUDED_BY_FROZEN_MANIFEST,gas:EXCLUDED_BY_FROZEN_MANIFEST,candidate_attempt_accounting:EXCLUDED_BY_FROZEN_MANIFEST,fees:EXCLUDED_BY_FROZEN_MANIFEST,cryptographic_verification_work:EXCLUDED_BY_FROZEN_MANIFEST,state_commitment_work:EXCLUDED_BY_FROZEN_MANIFEST,migration_work:EXCLUDED_BY_FROZEN_MANIFEST,hostile_validation_performance:EXCLUDED_BY_FROZEN_MANIFEST,replay_behavior:EXCLUDED_BY_FROZEN_MANIFEST,reorganization_behavior:EXCLUDED_BY_FROZEN_MANIFEST,benchmark_warmup:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,stochastic_sample_count:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,confidence_intervals:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,percentiles:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,thermal_policy:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,cache_policy:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,cpu_affinity:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE,outlier_policy_for_deterministic_checks:NOT_APPLICABLE_TO_THIS_EVIDENCE_STAGE
deviations=NONE
exclusions=PREDECLARED_SCOPE_ONLY_NO_RESULT_BEARING_EXCLUSION
```

For a finalized candidate record, the corresponding
`*_evidence_record_hash` is its external SHA-256 identity.

If no finalized candidate record exists, that field is exactly:

```text
NOT_AVAILABLE_CANDIDATE_RECORD_NOT_FINALIZED
```

If no finalized candidate record exists, the corresponding review field is
exactly:

```text
NOT_APPLICABLE_CANDIDATE_RECORD_NOT_FINALIZED
```

Otherwise the review field is the external review-artifact SHA-256.

`account_shared_check_results` and `utxo_shared_check_results` serialize SC01
through SC18 in exact SC order as comma-separated `SCnn:OUTCOME` pairs with no
spaces.

If no finalized record exists for a candidate, that candidate shared-check field
is exactly:

```text
NOT_AVAILABLE_CANDIDATE_RECORD_NOT_FINALIZED
```

For any finalized Account record, `account_native_disclosure` is constructed
only from the five named Account-native record fields shown below, in this exact
order, as comma-separated `key:value` pairs with no spaces:

```text
logical_value_relation_count:2,pre_effect_positive_quantity_relation_count:1,post_effect_positive_quantity_relation_count:1,mapped_quantity_vector_before:(1,0),mapped_quantity_vector_after:(0,1)
```

The values must be copied exactly from the finalized Account record. They must
not be upgraded or rewritten because another shared-contract field succeeded or
failed.

For any finalized UTXO record, `utxo_native_disclosure` is constructed only from
the six named UTXO-native record fields shown below, in this exact order, as
comma-separated `key:value` pairs with no spaces:

```text
pre_effect_live_logical_value_record_count:1,post_effect_live_logical_value_record_count:1,distinct_mapping_local_record_handle_count:2,mapped_live_set_before:{UR_PRE:1},mapped_live_set_after:{UR_POST:1},mapping_local_replacement_relation:{UR_PRE:1}->{UR_POST:1}
```

The values must be copied exactly from the finalized UTXO record. They must not
be upgraded or rewritten because another shared-contract field succeeded or
failed.

If no finalized candidate record exists, the corresponding
`account_native_disclosure` or `utxo_native_disclosure` field is exactly:

```text
NOT_AVAILABLE_CANDIDATE_RECORD_NOT_FINALIZED
```

`neutral_status_observations` is `NONE` or the union of applicable unchanged
workload-model labels present in finalized candidate-record
`neutral_status_observations_if_applicable` fields and procedure-incident
`neutral_status_observations_if_applicable` fields, serialized in the Section 18
taxonomy order with no spaces. Free-form incident descriptions do not create
status labels.

`procedure_incident_hashes`, `account_record_attempt_hashes`, and
`utxo_record_attempt_hashes` use the canonical hash-list rule from Section 7.

A normal paired-success summary is permitted only when both finalized candidate
records exist, all SC01 through SC18 outcomes are their successful values, both
candidate review dispositions are `ACCEPTED`, and no material deviation exists.

Only in that case,
`mapping_qualified_conclusion` contains exactly:

```text
Both frozen reference mappings satisfy the case-local value-preservation predicate for A = 1.
```

Otherwise it contains exactly:

```text
NO_NORMAL_PAIRED_SUCCESS_CONCLUSION_DUE_TO_VISIBLE_FAILURE_OR_MISSINGNESS
```

The exact finalized summary bytes receive an external SHA-256.

The summary does not embed its own final SHA-256. Its external SHA-256 is recorded
by the final evidence manifest.

### 25.9 Final Evidence Manifest

After all available candidate records, raw record-attempt artifacts, procedure
incidents, applicable review artifacts, the registration receipt, the
procedure-provenance artifact, and the deterministic summary exist, create one
final evidence manifest using the canonical key-value grammar in Section 7.

Final evidence manifest format version:

```text
pilot-account-utxo-final-evidence-manifest/v1
```

Exact field order is:

```text
final_evidence_manifest_format_version
final_evidence_manifest_content_identity
evidence_methodology_hash
pre_result_configuration_hash
evidence_source_revision
registration_receipt_hash
procedure_provenance_hash
account_evidence_record_hash
utxo_evidence_record_hash
account_record_attempt_hashes
utxo_record_attempt_hashes
account_review_artifact_hash
utxo_review_artifact_hash
procedure_incident_hashes
summary_hash
comparative_conclusion_scope
```

Static values include:

```text
final_evidence_manifest_format_version=pilot-account-utxo-final-evidence-manifest/v1
final_evidence_manifest_content_identity=NOT_EMBEDDED_EXTERNAL_EVIDENCE_ONLY_CONTENT_IDENTITY
evidence_methodology_hash=sha256:E3969792F70B0F61BD2A2C4130F27C934CA29FF2C8CA063990C1172D7C53B312
evidence_source_revision=ba768dbfc4189987f1ea8231f3b1c3ec442f5fda
comparative_conclusion_scope=MAPPING_QUALIFIED_DETERMINISTIC_STRUCTURAL_EVIDENCE_ONLY
```

Finalized candidate record hashes and review hashes use the same absence sentinels
defined in Section 25.8 when a candidate record was not finalized.

Attempt and incident hash fields use the canonical hash-list rule from Section 7.

The final evidence manifest receives an external SHA-256 over its exact bytes and
is the root identity of the Phase B evidence package.

The deterministic summary must not contain the final evidence-manifest hash. The
final manifest contains the summary hash. This one-way ordering prevents a
content-addressing cycle.

### 25.10 Reproduction

Independent review should reconstruct:

- the five frozen input hash verifications;
- every finalized candidate evidence record;
- every preserved raw candidate-record attempt;
- all SC01 through SC18 outcomes that exist;
- candidate-native disclosures;
- scope and neutral-status treatment;
- procedure incidents if any;
- procedure provenance;
- the deterministic summary; and
- the final evidence-manifest bindings.

Independent review creates evidence confidence only and no protocol authority.

## 26. Publication / Registration Plan

The selected pre-result publication mechanism is a dedicated public GitHub pull
request merged into the public Dilithia repository before evidence generation.

This plan uses the public version-control record as a third-party-visible
publication event and then captures an independently verified registration
receipt under Section 25.6.

Procedure:

1. freeze this configuration;
2. compute its external evidence-only SHA-256 over exact frozen bytes;
3. commit only the intended frozen configuration change on its dedicated branch;
4. push that branch to the public Dilithia repository;
5. create a dedicated public pull request recording:
   - exact configuration path;
   - exact external configuration SHA-256;
   - bound evidence-source revision; and
   - that evidence remains unauthorized until merge and independent verification;
6. require repository CI to pass;
7. perform final Files-changed review;
8. merge the pull request to public `main`;
9. record the pull-request number, source commit, merge commit, and public merge
   event time;
10. independently retrieve the public merge commit and verify that the canonical
    path contains the exact frozen configuration bytes whose SHA-256 was
    registered;
11. create and content-address the registration receipt required by Section 25.6;
12. preserve the public identifiers and receipt bytes with the evidence package;
    and
13. only then permit evidence generation.

The registration is considered independently verifiable only after step 11 is
complete.

A public branch deletion after merge does not invalidate the registration when
the merged commit, pull-request record, exact configuration hash, and retained
receipt continue to identify the registered bytes.

A repository rewrite, deletion, or later replacement does not authorize silent
identity reuse. Any discrepancy detected later remains visible through the
preserved content identities and receipt.

A changed configuration requires a new external SHA-256 and a new public
registration record. The earlier configuration identity and receipt must remain
retained with any evidence that references them.

Registration establishes evidence timing and auditability only.

It creates no protocol authority.

## 27. Evidence Start Gate

Evidence generation is prohibited until all are `YES`:

```text
frozen_semantic_case_hash_verified = YES
frozen_paired_manifest_hash_verified = YES
frozen_account_mapping_hash_verified = YES
frozen_utxo_mapping_hash_verified = YES
frozen_evidence_methodology_hash_verified = YES

pre_result_configuration_review_complete = YES
pre_result_configuration_frozen = YES
pre_result_configuration_external_sha256_established = YES

public_pre_result_registration_complete = YES
public_pre_result_registration_independently_verified = YES
registration_receipt_artifact_hash_established = YES

evidence_source_revision_verified = YES
clean_source_checkout_verified = YES

non_run_based_execution_mode_verified = YES
candidate_record_byte_grammar_verified = YES
shared_check_field_completeness_verified = YES
explicit_scope_map_verified = YES
failure_path_raw_attempt_preservation_verified = YES
ancillary_artifact_schemas_verified = YES
deterministic_summary_schema_verified = YES
phase_b_manifest_plan_verified = YES
```

Otherwise:

```text
EVIDENCE GENERATION = NOT AUTHORIZED
```

## 28. Evidence Summary Plan

The exact deterministic summary-generation procedure and byte schema are frozen
in Section 25.8.

The summary must separately preserve, through its exact fields:

1. frozen binding verification;
2. shared-contract results for SC01 through SC18;
3. Account-native structural disclosure;
4. UTXO-native structural disclosure;
5. the complete explicit dimension-scope map;
6. neutral status observations if applicable;
7. procedure incidents if any;
8. raw candidate-record attempt identities if any;
9. deviations;
10. exclusions;
11. registration and procedure-provenance identities;
12. finalized candidate evidence-record identities or exact absence sentinels;
13. independent review artifact identities or exact absence sentinels; and
14. the mapping-qualified conclusion token or sentence permitted by Section 25.8.

The summary is not free-form post-result prose.

It must not state:

```text
Account wins
UTXO wins
Account is more efficient
UTXO is more efficient
the candidates are globally equivalent
the pilot is production-representative
a state model is selected
a protocol rule is adopted
```

The summary does not and cannot embed its own final SHA-256.

Its external SHA-256 is computed only after the exact summary bytes are finalized
and is then recorded by the final evidence manifest.

The summary is not the package root. The final evidence manifest defined in
Section 25.9 is the Phase B root.

## 29. No Scalar Score

This configuration defines no scalar score and no weights.

Shared checks are categorical gates or evidence.

Candidate-native observations remain descriptive.

## 30. Security Boundary

This configuration selects no ownership, authorization, replay, one-use,
commitment, cryptographic, migration, resource-accounting, fee, supply,
transaction-structure, state-structure, or consensus mechanism.

No evidence under this configuration may infer security equivalence between
Account and UTXO.

## 31. Formal-Specification Boundary

This configuration is non-normative.

Successful evidence does not become a Formal Specification rule by repetition.

A future protocol rule requires the authoritative Dilithia protocol process.

## 32. Project Impact

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

Frozen evidence methodology changed:
NO

Pre-result configuration exists:
FROZEN

Pre-result configuration frozen:
YES

Pre-result registration complete:
NO

Evidence execution mode:
NON_RUN_BASED_DETERMINISTIC_DOCUMENT_EVALUATION

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
NOT JUSTIFIED BY THIS FROZEN CONFIGURATION

PROJECT_STATE update:
NOT JUSTIFIED BY THIS FROZEN CONFIGURATION

THREAT_MODEL update:
NOT JUSTIFIED BY THIS FROZEN CONFIGURATION

Consensus implementation change:
NONE
```

## 33. Change Control

This pre-result configuration is currently:

```text
FROZEN
```

The actual-file review, adversarial review, material-finding resolution, and
focused freeze gate are complete for this artifact.

Any material byte change after freeze creates a new pre-result configuration
identity.

A changed plan must never reuse an old identity.

## 34. Freeze Review Gate

The completed freeze review established that:

1. all five frozen artifact identities are exact;
2. the source revision is the correct pre-existing immutable revision;
3. the source-cleanliness policy is reproducible;
4. the canonical key grammar admits every frozen schema key and the candidate
   evidence-record representation is byte-exact;
5. content addressing is non-self-referential;
6. the manual procedure is complete and symmetric;
7. the analysis procedure cannot introduce ranking;
8. shared checks preserve the frozen methodology;
9. candidate-native fields are descriptive and not falsely equated;
10. every planned evidence dimension has an explicit serialized scope disposition;
11. unchanged workload-model status meanings are preserved;
12. failure and missingness remain visible;
13. every started-but-unfinalized candidate-record attempt that emitted bytes is
    retained by exact external content identity;
14. post-result candidate-specific exclusion is impossible;
15. material deviations require a new identity;
16. the non-run-based choice removes unresolved run-identity lifecycle ambiguity;
17. procedure-incident, review, registration-receipt, procedure-provenance,
    deterministic-summary, and final-manifest schemas are frozen before evidence;
18. failure-path absence sentinels allow the Phase B package to remain complete
    without inventing successful candidate records;
19. candidate records, raw attempts, incidents, review artifacts, registration
    receipt, procedure provenance, summary, and final evidence manifest form a
    non-circular provenance DAG;
20. the deterministic summary is reconstructible from preserved raw and reviewed
    artifacts, including candidate-record absence paths, without free-form
    post-result ranking discretion;
21. neutral status observations used by the summary are carried in explicit
    frozen fields rather than inferred from free-form incident text;
22. publication / registration satisfies pre-result auditability;
23. no hidden performance, serialization, resource, crypto, replay, commitment,
    migration, or monetary assumption is introduced; and
24. no protocol-adoption claim is created.

No unresolved Critical, High, Medium, or Low finding remains at this freeze gate.

## 35. Next Gate

The permitted workflow from this frozen configuration is:

1. establish and independently verify the frozen configuration's external
   evidence-only SHA-256 over the exact canonical bytes;
2. commit, push, review, and merge the frozen configuration through its dedicated
   public pull request;
3. independently verify that the public merge commit contains the exact registered
   frozen configuration bytes;
4. create and content-address the public pre-result registration receipt;
5. verify every Evidence Start Gate requirement in Section 27; and
6. only then begin the non-run-based deterministic evidence procedure.

Until those gates pass:

```text
EVIDENCE GENERATION = NOT AUTHORIZED
```
