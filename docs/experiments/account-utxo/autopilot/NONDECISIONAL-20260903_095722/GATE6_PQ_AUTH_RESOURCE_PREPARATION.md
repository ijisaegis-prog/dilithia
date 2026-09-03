# Gate 6 — PQ Authorization and Resource Preparation

> **NON-NORMATIVE PREPARATION.** No cryptographic/PQ primitive, authorization
> architecture, artifact, aggregation, batching, or resource unit is selected.

## Frozen requirement and status

The comparison must not equate Account with one signature, UTXO with one
signature per input, one owner with one credential, one authorization object
with one verification, or either candidate with one crypto version. Workload
must derive from explicit authority, coverage, artifact, reuse, and algorithm
profiles. Protocol-visible reuse and local caching are distinct; shared-case
capabilities are offered symmetrically. Source: matrix §§10.1–10.5 and
`PQ_AUTHORIZATION_COUNT_ARTIFACT_ASSUMPTIONS_REQUIREMENTS.md`. Process
requirements: **SATISFIED**. Evidence: **PENDING**.

## Neutral profile axes to prepare

- independent authorizer count and authority relation graph;
- logical source/effect count, coverage and grouping;
- credential multiplicity and placement left abstract;
- artifact class/count/size and verification operations;
- public-key, credential, condition, evidence, proof/signature, verification
  result, and implementation-cache reuse as separate fields;
- invalidation conditions and protocol-visible/local classification;
- single/mixed crypto versions, coexistence, migration, historical validation;
- batching and aggregation as separate optional branches;
- valid, malformed, invalid, mixed-version, and fallback workloads.

Use symbolic profile IDs and unresolved values; do not invent bytes or operation
counts. Exact bytes require a frozen experimental algorithm/schema profile.

## Tests and stop points

Schema lint can verify decomposition, equal common-case capability, and absence
of signature-count stereotypes. Artifact generation, verification counts,
invalid/fallback execution, timings, and persistent metadata measurement are
**REQUIRES IMPLEMENTATION**. Any crypto/PQ choice, artifact format, signing
context, grouping/scope rule, reuse rule, aggregation/batching mechanism, or
numeric meter is **REQUIRES OWNER DECISION**.

Candidate outputs may differ; comparisons require the same external authority
relations and frozen profile. Candidate-native credentials, signatures, proofs,
or inputs are not neutral scores.

Scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**; Account **NO**;
UTXO **NO**; main merge **NOT DONE**.
