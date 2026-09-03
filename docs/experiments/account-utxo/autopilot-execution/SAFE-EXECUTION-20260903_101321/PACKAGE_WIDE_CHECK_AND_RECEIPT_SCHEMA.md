# Package-Wide Check and Receipt Schema

> **SPECIFICATION ONLY.** This document is a governed checker input, not an
> execution receipt and not candidate or Gate evidence. A completed receipt is
> recorded separately in `PACKAGE_WIDE_CHECK_RECEIPT.md`.

## Scope binding

A future run must enumerate every `*.md` file directly in this run directory,
record the sorted relative-path inventory and SHA-256 of every file, and record
the SHA-256 of that inventory. The receipt itself is excluded from its input
inventory to avoid a self-hash; it must name that exclusion explicitly. Any
other exclusion is a failure. Adding, removing, renaming, or editing an input
invalidates the receipt.

## Required predicate groups

1. Re-run all predicates in `MECHANICAL_STRUCTURAL_CHECK_SPECIFICATION.md`.
2. Require every governed inventory entry expected by `EXECUTION_MANIFEST.md`;
   reject missing, extra, duplicate, or unhashed governed entries. The one
   explicitly excluded receipt is permitted in the physical directory but is
   not a governed input.
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

## Required durable execution-receipt fields

| Field | Value |
|---|---|
| receipt status | required |
| receipt ID and receipt-document identity | required; an explicitly defined, recomputable canonical receipt hash may blank only its own identity value |
| durable binding location or mechanism | required |
| checker script path and SHA-256 | required |
| exact executable command and argument vector | required |
| working directory | required |
| operating-system and execution-environment identity | required |
| relevant runtime and toolchain identities | required |
| input inventory path and SHA-256 | required |
| explicit excluded receipt path | required |
| stdout path and SHA-256 | required |
| stderr path and SHA-256 | required |
| exit status | required |
| start and end timestamps with timezone | required |
| predicate-by-predicate output | required |

The receipt may be marked current only when every required field is populated,
all identities recompute, every predicate output is captured, and the receipt
identity is fixed by the named durable binding mechanism. The receipt must
contain or identify the exact checker source and captured streams; a status
assertion without those materials is insufficient. A successful package check
remains documentary validation, never candidate or Gate evidence.
