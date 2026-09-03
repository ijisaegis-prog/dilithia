# Package-Wide Check and Receipt Schema

> **SPECIFICATION AND BLANK RECEIPT ONLY.** No check execution or pass is
> claimed by this document.

## Scope binding

A future run must enumerate every `*.md` file directly in this run directory,
record the sorted relative-path inventory and SHA-256 of every file, and record
the SHA-256 of that inventory. The receipt itself is excluded from its input
inventory to avoid a self-hash; it must name that exclusion explicitly. Any
other exclusion is a failure. Adding, removing, renaming, or editing an input
invalidates the receipt.

## Required predicate groups

1. Re-run all predicates in `MECHANICAL_STRUCTURAL_CHECK_SPECIFICATION.md`.
2. Require every inventory entry expected by `EXECUTION_MANIFEST.md`; reject
   missing, extra, duplicate, or unhashed entries.
3. Validate every instantiated paired manifest against
   `PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md`: required fields, digest
   syntax, resolvable paths, recomputed hashes, co-equal Account/UTXO binding
   classes, and result/status consistency. Templates must remain identified as
   templates and are not failed merely because their fields are unresolved.
4. Detect prohibited selection language: scoring, weighting, ranking,
   preference, recommendation, adoption, or selection of a candidate. Maintain
   an explicit reviewed allowlist only for negated boundary statements,
   quotations of review findings, and task names that prohibit those acts.
5. Detect candidate-native metric leakage: a metric, threshold, workload, or
   success criterion stated for only Account or only UTXO. Shared external
   dimensions and mirrored candidate slots are permitted; candidate-specific
   measurements require paired scope and provenance and must not be normalized
   into a comparison score.
6. Detect unsupported evidence verbs such as `proves`, `verifies`,
   `establishes`, `demonstrates`, `passed`, or `executed`. Each non-negated
   evidence claim must cite a receipt and predicate/output identity in the same
   section. Template requirements and explicit retractions are permitted.
7. Detect missing provenance for claimed observations: exact executable
   command or script identity, working directory, bound inputs, stdout, stderr,
   exit status, and timestamps are required. Otherwise classify the statement
   as `EXECUTOR_REPORTED_OBSERVATION`, `PENDING`, or `BLOCKED`.
8. Validate status consistency across `README.md`, `EXECUTION_MANIFEST.md`,
   `SAFE_TASK_CLASSIFICATION.md`, `FIXED_POINT_REPORT.md`, and result records.

## Durable execution receipt — blank

| Field | Value |
|---|---|
| receipt status | `NOT_EXECUTED` |
| receipt ID and receipt-document SHA-256 | `UNRESOLVED` |
| durable binding location or mechanism | `UNRESOLVED` |
| checker script path and SHA-256 | `UNRESOLVED` |
| exact executable command and argument vector | `UNRESOLVED` |
| working directory | `UNRESOLVED` |
| operating-system and execution-environment identity | `UNRESOLVED` |
| relevant runtime and toolchain identities | `UNRESOLVED` |
| input inventory path and SHA-256 | `UNRESOLVED` |
| explicit excluded receipt path | `UNRESOLVED` |
| stdout path and SHA-256 | `UNRESOLVED` |
| stderr path and SHA-256 | `UNRESOLVED` |
| exit status | `UNRESOLVED` |
| start and end timestamps with timezone | `UNRESOLVED` |
| predicate-by-predicate output | `UNRESOLVED` |

The receipt may be marked current only when every required field is populated,
all identities recompute, every predicate output is captured, and the receipt
identity is fixed by the named durable binding mechanism. A mutable inline file
in an untracked directory is insufficient by itself. A successful package check
remains documentary validation, never candidate or Gate evidence.
