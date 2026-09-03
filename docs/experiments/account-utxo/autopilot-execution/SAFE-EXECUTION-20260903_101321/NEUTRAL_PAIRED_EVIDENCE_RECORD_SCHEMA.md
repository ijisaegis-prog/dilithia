# Neutral Paired Evidence-Record Schema

> **TEMPLATE ONLY — NOT EVIDENCE.** This is a candidate-neutral documentary
> schema. Filling a field, passing a structural lint, or assigning an identity
> does not establish candidate behavior or Gate credit.

## Record envelope

Every later record must contain all fields below. `UNRESOLVED` is a stop marker.
A material field change creates a new record identity.

| Field | Required value shape |
|---|---|
| `record_id` | unique non-result identifier |
| `record_status` | `TEMPLATE`, `REGISTERED_PRE_RESULT`, or an evidence-class-specific later state |
| `evidence_class` | structural, executable, formal, quantitative, benchmark, independent, or historical |
| `claim_scope` | exact proposition; `UNRESOLVED` in this template |
| `case_id` / `case_sha256` | shared case identity and content digest |
| `source_commit` | exact source revision |
| `governing_sources` | ordered paths plus SHA-256 and Git blob/object identity where available |
| `profile_set_id` / `profile_set_sha256` | identity of every material profile value and unresolved branch |
| `generator_or_author` | identity, version, invocation, and provenance |
| `generation_determinism` | deterministic procedure or recorded seed/selection procedure |
| `created_utc` | timestamp with timezone |
| `environment` | OS, architecture, toolchain, dependency lock, locale, and relevant environment variables |
| `corpus` | source, version, content identity, selection rule, seed, and exclusions |
| `oracle` | external semantic predicate and oracle implementation identity |
| `commands` | exact ordered commands and working directory |
| `raw_outputs` | planned paths, formats, SHA-256 values, exit status, stdout, and stderr |
| `summary_derivation` | exact procedure/tool identity and output identity |
| `review` | reviewer identity, procedure, input identities, output, and date |
| `registration_receipt` | pre-result manifest identity and durable receipt; `UNRESOLVED` here |
| `reuse_parent` | exact parent identity or `NONE`; similarity is insufficient |
| `exclusions` | explicit cases and claims not covered |

## Shared external contract

The following fields occur once, outside both candidate slots:

| Field | Template value |
|---|---|
| `prior_authoritative_state` | `UNRESOLVED` |
| `required_absences` | `UNRESOLVED` |
| `authority_relations` | `UNRESOLVED_PROTECTED_DECISION` |
| `proposed_external_effect` | `UNRESOLVED_PROTECTED_DECISION` |
| `external_preconditions` | `UNRESOLVED` |
| `external_postconditions` | `UNRESOLVED` |
| `unchanged_external_facts` | `UNRESOLVED` |
| `outcome_class` | `UNRESOLVED` (`success`, `rejection`, or owner-defined accepted-unsuccessful only) |
| `rejection_canonical_effect` | `ZERO_REQUIRED_IF_REJECTION` |
| `rollback_prior_state` | `UNRESOLVED` |
| `reapplication_context` | `UNRESOLVED` |
| `historical_interpretation` | `UNRESOLVED` |

## Material profile block

Every profile is an externally supplied input, a co-equal explicit branch, or
`UNRESOLVED`. This schema supplies no default.

| Profile field | Template value |
|---|---|
| semantic/effect identity | `UNRESOLVED_PROTECTED_DECISION` |
| domain and protocol version | `UNRESOLVED_PROTECTED_DECISION` |
| replay and currentness | `UNRESOLVED_PROTECTED_DECISION` |
| lifecycle and history | `UNRESOLVED_PROTECTED_DECISION` |
| semantic conflict and canonical order | `UNRESOLVED_PROTECTED_DECISION` |
| authorization coverage and credential properties | `UNRESOLVED_PROTECTED_DECISION` |
| crypto/PQ artifact and operation profile | `UNRESOLVED_PROTECTED_DECISION` |
| migration, participation, acceptance, dormancy, retrofit, recovery | `UNRESOLVED_PROTECTED_DECISION` |
| authenticated-state claims, trust, scale, and history | `UNRESOLVED_PROTECTED_DECISION` |
| resource dimensions, measurement meaning, and physical environment | `UNRESOLVED_METHOD_PREREQUISITE` |
| consensus, governance, emergency, and reorganization inputs | `UNRESOLVED_PROTECTED_DECISION` |

## Mirrored candidate slots

The two slots have exactly the same required keys. Candidate-native values are
descriptive outputs and are never neutral scores.

### Account slot

| Key | Value |
|---|---|
| `candidate_slot` | `ACCOUNT` |
| `mapping_id` / `mapping_sha256` | `UNRESOLVED_IMPLEMENTATION` |
| `realization_id` / `realization_sha256` | `UNRESOLVED_IMPLEMENTATION` |
| `shared_case_id` / `shared_profile_set_id` | `UNRESOLVED` |
| `declared_dependencies` | `UNRESOLVED` |
| `declared_absence_dependencies` | `UNRESOLVED` |
| `declared_internal_effects` | `UNRESOLVED` |
| `declared_external_projection` | `UNRESOLVED` |
| `outcome_observed` | `NO_RESULT` |
| `canonical_effect_observed` | `NO_RESULT` |
| `rollback_observed` / `reapplication_observed` | `NO_RESULT` |
| `candidate_native_resource_observations` | `NO_RESULT_NOT_A_SCORE` |
| `raw_output_identities` | `NO_RESULT` |
| `missing_evidence` | `IMPLEMENTATION_AND_EXECUTION` |

### UTXO slot

| Key | Value |
|---|---|
| `candidate_slot` | `UTXO` |
| `mapping_id` / `mapping_sha256` | `UNRESOLVED_IMPLEMENTATION` |
| `realization_id` / `realization_sha256` | `UNRESOLVED_IMPLEMENTATION` |
| `shared_case_id` / `shared_profile_set_id` | `UNRESOLVED` |
| `declared_dependencies` | `UNRESOLVED` |
| `declared_absence_dependencies` | `UNRESOLVED` |
| `declared_internal_effects` | `UNRESOLVED` |
| `declared_external_projection` | `UNRESOLVED` |
| `outcome_observed` | `NO_RESULT` |
| `canonical_effect_observed` | `NO_RESULT` |
| `rollback_observed` / `reapplication_observed` | `NO_RESULT` |
| `candidate_native_resource_observations` | `NO_RESULT_NOT_A_SCORE` |
| `raw_output_identities` | `NO_RESULT` |
| `missing_evidence` | `IMPLEMENTATION_AND_EXECUTION` |

## Structural predicates

A mechanical schema check may establish only that both slots exist, their key
sets match, both reference one shared case/profile set, required provenance
fields exist, and prohibited result values are absent from a template. It
cannot establish that future values are truthful, semantically equivalent, or
supported by execution.
