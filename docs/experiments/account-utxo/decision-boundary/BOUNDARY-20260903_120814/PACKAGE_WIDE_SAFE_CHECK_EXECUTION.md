# Package-Wide Safe Check Execution

## Corrected disposition

`INVENTORY_MANIFEST_RESULT: FAIL_SET_MISMATCH`

This record corrects the earlier execution account. The retained checker now
implements all eight predicate groups in the governing schema, including
explicit reviewed language dispositions and observation-level provenance
classification. It is locally present and executable. The inspected package
still fails group 2, so no package-wide pass or durable old-package receipt is
claimed. This boundary run records a receipt-excluded content binding below;
external durable retention is outside its authorization and is not claimed.

## Corrected execution identity

| Field | Value |
|---|---|
| inspected package | `docs/experiments/account-utxo/autopilot-execution/SAFE-EXECUTION-20260903_101321` |
| retained checker | `PACKAGE_WIDE_SAFE_CHECK.ps1` |
| checker SHA-256 | `af844b0472a1c1c4a889d83be2a135ebe7655800559da7df1fdfc8c8ecaf6f8b` |
| exact invocation | `powershell.exe -NoProfile -ExecutionPolicy Bypass -File "C:\Users\PC\AppData\Local\Temp\Dilithia_Decision_Boundary_Worktree_20260903_120814\docs\experiments\account-utxo\decision-boundary\BOUNDARY-20260903_120814\PACKAGE_WIDE_SAFE_CHECK.ps1" -PackagePath "C:\Users\PC\AppData\Local\Temp\Dilithia_Decision_Boundary_Worktree_20260903_120814\docs\experiments\account-utxo\autopilot-execution\SAFE-EXECUTION-20260903_101321"` |
| working directory | `C:\Users\PC\AppData\Local\Temp\Dilithia_Decision_Boundary_Worktree_20260903_120814` |
| environment | Windows PowerShell 5.1 on Microsoft Windows NT 10.0.26200.0 |
| started | `2026-09-03T14:17:28.0652691+09:00` |
| ended | `2026-09-03T14:17:29.0508496+09:00` |
| exit status | `1` |
| captured stdout SHA-256 | `9ae974132f81b6b0d2527a05c18da6df93ed043872a2dbca730930d05a547d6b` |
| stderr | empty |
| stderr SHA-256 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

The stdout hash identifies the recorded PowerShell `Out-String` capture,
including its final platform newline. It does not provide Git durability.

## Correction-verification rerun

After reconciling the checker-retention and provenance wording, the checker was
run again without modifying the inspected package or the checker.

| Field | Value |
|---|---|
| started | `2026-09-03T14:25:23.9626194+09:00` |
| ended | `2026-09-03T14:25:24.9460476+09:00` |
| exit status | `1` |
| captured stdout SHA-256 | `fe4e2888d8b9ddc01887fe89db174491e2e068b45a57d6561dcb0e2fcf348ad7` |
| stderr | empty |
| result | `FAIL_SET_MISMATCH` |

The rerun reproduced the 25 Markdown files, 24 governed inputs, 24 manifest
declarations, legacy literal-backtick-`t` inventory digest
`c2f6ac436a63d8c1e6ddb15f9f2130126a42cb67ce4999bff2fea2d16823391e`,
the wrongly declared excluded receipt, and the omitted final dual review. This
is a locally captured observation, not a durable Git-bound receipt.

## Input inventory and actual defect

The governing schema names
`PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md` as the receipt and requires it to be
excluded from the input inventory. The checker explicitly excluded that one
file and no other file.

| Field | Result |
|---|---|
| Markdown files physically present | 25 |
| excluded receipt | `PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md` |
| actual input files after exclusion | 24 |
| manifest declarations | 24 |
| legacy literal-backtick-`t` inventory SHA-256 | `c2f6ac436a63d8c1e6ddb15f9f2130126a42cb67ce4999bff2fea2d16823391e` (not tab-separated; superseded) |
| wrongly declared but excluded from inputs | `PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md` |
| actual input omitted from declarations | `AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md` |
| duplicate declarations | none |

Thus equal counts do not imply equality. The manifest and governed input sets
differ by one member in each direction. The previous “25 actual versus 24
declared” and “one undeclared extra only” explanations are superseded.

The inventory digest is SHA-256 over the retained checker's sorted records,
each encoded in UTF-8 without BOM as `relative-name`, U+0009, lowercase file
SHA-256, and U+000A, including the final U+000A. The 24 records and individual hashes are listed in
`PACKAGE_INVENTORY_PROVENANCE_DEFECT_RECORD.md`.

The earlier digest
`c2f6ac436a63d8c1e6ddb15f9f2130126a42cb67ce4999bff2fea2d16823391e`
was produced with literal backtick-plus-`t` characters by a single-quoted
PowerShell format string. Its description as tab-separated is retracted.

## Explicit selection-language review

The retained pattern produced 39 line matches. Every match is explicitly
disposed below. `ALLOWLIST_NEGATED_BOUNDARY` and `ALLOWLIST_PROHIBITIVE_TASK`
are the only allowlist classes. `OUT_OF_SCOPE_TERM` is not allowlisted; it
records a homonym that does not describe candidate selection.

| Disposition | Match IDs |
|---|---|
| `ALLOWLIST_NEGATED_BOUNDARY` | `AUTOPILOT_EXECUTION_BOUNDARY.md:14`, `AUTOPILOT_EXECUTION_BOUNDARY.md:35`, `AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md:7`, `:8`, `:10`, `:11`, `DECISION_QUEUE_UNCHANGED.md:3`, `:4`, `:8`, `:22`, `EXECUTION_MANIFEST.md:74`, `FIXED_POINT_REPORT.md:43`, `:48`, `:49`, `GATE1_RERUN_PRE_RESULT_MANIFEST_TEMPLATE.md:99`, `:100`, `PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md:5`, `README.md:4`, `:39`, `:40`, `:42`, `:43`, `RESUME_CODE_HEALTH_RECEIPT.md:257`, `:258`, `:260`, `:261` |
| `ALLOWLIST_PROHIBITIVE_TASK` | `BLOCKED_WORK_REGISTRY.md:18`, `FIXED_POINT_REPORT.md:9`, `:36`, `NEXT_SAFE_AUTOMATION_QUEUE.md:26`, `PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md:25`, `:26`, `:33`, `SAFE_TASK_CLASSIFICATION.md:17`, `:37` |
| `OUT_OF_SCOPE_TERM` | `GATE1_DEFERRED_CASE_MANIFESTS.md:4` (a chosen profile combination), `GATE1_NEUTRAL_PROFILE_MATRIX.md:4` (same), `NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:23` (seed procedure), `:26` (corpus sampling rule) |

Review result: no line makes a candidate choice. The earlier unexplained count
of 44 is not relied upon; its match set was not preserved and is superseded by
this reproducible 39-match scan and disposition.

## Explicit evidence-verb review

The retained pattern produced 45 line matches. Each is disposed below.

| Disposition | Match IDs |
|---|---|
| `NEGATED_OR_RETRACTED` | `AUTOPILOT_EXECUTION_BOUNDARY.md:32`, `CROSS_GATE_EXECUTED_CHECKS.md:5`, `:9`, `:20`, `DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md:3`, `:70`, `EXECUTION_MANIFEST.md:20`, `:21`, `FIXED_POINT_REPORT.md:14`, `GATE1_DEFERRED_CASE_MANIFESTS.md:8`, `GATE1_EXECUTED_STRUCTURAL_CASES.md:25`, `:31`, `GATE1_PAIRED_CASE_RECORD_SKELETONS.md:89`, `MECHANICAL_STRUCTURAL_CHECK_RESULTS.md:3`, `NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:5`, `:122`, `README.md:13`, `REPOSITORY_VERIFICATION_RESULTS.md:24`, `SAFE_TASK_CLASSIFICATION.md:39`, `:51` |
| `SPECIFICATION_TEMPLATE_OR_HEADING` | `AUTOPILOT_EXECUTION_BOUNDARY.md:30`, `EXECUTION_MANIFEST.md:15`, `GATE1_EXECUTED_STRUCTURAL_CASES.md:1`, `:18`, `MECHANICAL_STRUCTURAL_CHECK_SPECIFICATION.md:46`, `:50`, `NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:119`, `PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md:34`, `:35`, `PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md:51`, `:52` |
| `EXPLICITLY_UNBOUND_HISTORICAL_TRANSCRIPT` | `REPOSITORY_VERIFICATION_RESULTS.md:17`, `RESUME_CODE_HEALTH_RECEIPT.md:82`, `:88`, `:94`, `:100`, `:105`, `:111`, `:161`, `:166`, `:171`, `:176`, `:181`, `:186`, `:191` |
| `EXECUTOR_REPORTED_OR_PENDING_CLASSIFICATION` | `SAFE_TASK_CLASSIFICATION.md:13` |

Review result: no match is treated as current candidate evidence. The previous
unexplained count of 39 is not relied upon; it is superseded by this
reproducible 45-match scan and per-match disposition.

## Claim boundary

The corrected inventory set mismatch is sufficient to prevent a package-wide
PASS. This record does not assert that it is the sole possible failure of a
complete schema implementation. It creates no candidate evidence and makes no
state-model or protocol-mechanism choice.

## Final-review collision repair execution (superseding result)

The checker was corrected to give Group 2's manifest/input missing set and
Group 7's per-observation missing-provenance fields separate variables. The
checker derives both sets from current inputs; neither result is hard-coded.
Its explicit reviewed allowlists accept only the schema-permitted boundary/task
cases; any new selection-language or evidence hit fails until reviewed.
Evidence observations lacking complete provenance remain explicitly
`EXECUTOR_REPORTED_OBSERVATION` or `PENDING` and receive no evidence credit.
Classification is per lexical observation, not document-wide.

| Field | Value |
|---|---|
| checker SHA-256 | `0121c84c6d9d47aba09fa095ed71557ac297960ca2038c23cbf8bece5a611663` |
| started | `2026-09-03T15:07:51.6277037+09:00` |
| ended | `2026-09-03T15:07:53.0843420+09:00` |
| exit status | `1` |
| captured stdout SHA-256 | `5593c4c3ae13d73211285b15862fb74203a606e10a495d329e4bf2a79ef6a725` |
| stderr SHA-256 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| corrected inventory SHA-256 | `bd9bf9a1d4ecd869c34ac38ca07adc2805154c2fe42df1848b099b37984c8ae1` |
| overall | `FAIL` |

Predicate-group results: group 1 structural `PASS` with
`NOT_CHECKED_SEMANTICALLY`; group 2 inventory/manifest `FAIL`, with
`missing_from_inputs` equal to
`PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md`, `undeclared_inputs` equal to
`AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md`, and empty duplicate and unhashed
sets; group 3 paired manifests `PASS` with zero instantiated manifests; group 4
prohibited-language review `PASS` (39 classified hits); group 5
candidate-native leakage `PASS` (zero syntactic one-candidate hits); group 6
evidence-claim review `PASS` (45 classified hits); group 7 observation-level
provenance classification `PASS`; and group 8 status consistency `PASS`.
Overall status is `FAIL` because group 2 fails. These are documentary results
only and supply no candidate or Gate evidence.

## Boundary-package content binding

This is the completed binding available within the authorized artifact set.
`PACKAGE_WIDE_SAFE_CHECK_EXECUTION.md` is explicitly excluded to avoid a
self-hash. No other artifact is excluded. The aggregate is SHA-256 over sorted
UTF-8-without-BOM records encoded as relative name, U+0009, lowercase file
SHA-256, and U+000A, including the final U+000A.

Aggregate SHA-256: `b5bf8c1065fad00ead1e01650362f9d0854ea89661380e3d84313fa4b849a678`

| Bound artifact | SHA-256 |
|---|---|
| `CURRENT_FIXED_POINT_AUDIT.md` | `fac97bd4eb8535426297aa20f0f8323598ad3ecd3400b02b9b4730d7d9d47407` |
| `DECISION_DEPENDENCY_ORDER.md` | `03d715fbf101fb25186d3231a5bfb8a8b44a5fcc73bf2784b5d2f14a58380182` |
| `DECISION_PREREQUISITE_MATRIX.md` | `1444eb398fcec037a5354b573e70cc5e5ec857db0c41a43e46a1801fb95cb306` |
| `EVIDENCE_GAPS_AND_BLOCKERS.md` | `ac82bc95895f59eed51e9ddf113aafdece7af260934f10455cf11035991c9e52` |
| `FINAL_NONDECISIONAL_STATUS.md` | `e61123364641e0302f0cfc11bc4218f3ffd28e8c276af554a0b9027a6e9f9636` |
| `NEXT_SAFE_WORK_EXECUTION.md` | `ed469ed45011713b493cea194e3413e18b46ea789e7a0e9830f9afcb4e20c6bd` |
| `OWNER_APPROVAL_BOUNDARY.md` | `e23347c022306149201d13b7a72961443d83ab73a753ab5615ba8e49613c2c5c` |
| `PACKAGE_INVENTORY_PROVENANCE_DEFECT_RECORD.md` | `99d1f1e41f715474733a310b4f43e5e8f954e6e3270ec204e2658803a17f2ad1` |
| `PACKAGE_WIDE_SAFE_CHECK.ps1` | `0121c84c6d9d47aba09fa095ed71557ac297960ca2038c23cbf8bece5a611663` |
| `PROTECTED_DECISION_QUEUE.md` | `a410cbb7475ebe48c7cbf93de603035f075a3afd197943d861fb1a7f7b33e99f` |
| `README.md` | `eca7208c2b88679bbb85f2facb0b57b032d416fdc29ee9f7014b830bb1df76d3` |

This content binding detects changes to every non-receipt boundary artifact.
It is not an external durability mechanism. Git retention or another durable
anchor requires separate authorization and remains a prerequisite, not an
executed or implied operation.
