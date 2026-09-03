# Package Inventory and Provenance Defect Record

## Corrected inventory result

The inspected directory physically contains 25 Markdown files. Under
`PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md`, that schema document is the receipt
and is explicitly excluded from its own input inventory. No other file is
excluded. The resulting governed inventory has 24 inputs.

`EXECUTION_MANIFEST.md` also declares 24 names, but the sets are unequal:

- `PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md` is declared even though it is the
  excluded receipt.
- `AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md` is an actual governed input but is
  not declared.

The defect is therefore a two-sided set mismatch, not a 25-versus-24 count
defect and not merely one undeclared extra file.

## Sorted governed input inventory

Inventory SHA-256: `bd9bf9a1d4ecd869c34ac38ca07adc2805154c2fe42df1848b099b37984c8ae1`

The corrected digest is SHA-256 over UTF-8 (without BOM) sorted records encoded
as the relative name, one U+0009 tab byte, lowercase file SHA-256, and U+000A,
including the final U+000A. The earlier
`c2f6ac436a63d8c1e6ddb15f9f2130126a42cb67ce4999bff2fea2d16823391e`
used the two literal characters backtick and `t`; that digest and its claimed
tab serialization are retracted.

| Relative input | SHA-256 |
|---|---|
| `AUTOPILOT_EXECUTION_BOUNDARY.md` | `69cb88bd36865c16016b998ffbd7d5d18d28598e8a394f5dd37a15a3a1dbb262` |
| `AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md` | `17b07d3cd056014ff3ebabf77435f3d54ba38f82250d56fc908871687e5e5f40` |
| `BLOCKED_WORK_REGISTRY.md` | `97d258696197d245c38c3b06010a6b81884e84bcb7f0b53bdf3dcfff7c186b0b` |
| `CROSS_GATE_EXECUTED_CHECKS.md` | `323659ca5aa629c2066595937a4429d1ac8b1d50c081dbb6e1946fd5f51a39eb` |
| `DECISION_QUEUE_UNCHANGED.md` | `8a8049a935bbcced5ea3a665f2230dc62dbf57b4283f791e076dac367f4f108e` |
| `DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md` | `21828397c4332cec31ff1cd3c9197f0e7361b86645584537e171df2356e4d8df` |
| `EXECUTION_MANIFEST.md` | `322d49787a7faee5b0a9cc18b09bbea50628cd02ae68f69e00448393ea049307` |
| `FIXED_POINT_REPORT.md` | `b740c5154260060e9c97afe8fd2f24a4ac2fa650610c2e2c14e2220453505f25` |
| `GATE1_DEFERRED_CASE_MANIFESTS.md` | `739fecaba360608ed9a09f897fcd289119718070237cc8356109c5d0449dbfba` |
| `GATE1_EXECUTED_STRUCTURAL_CASES.md` | `4e19c26fbcc164903bfe61bd733a7e6e2d77e6298a9dd447f49caba62de66100` |
| `GATE1_NEUTRAL_PROFILE_MATRIX.md` | `cc7ba847cad0310c622a963fa9aabea54d565189963cca8cb3a262e1e0842b8f` |
| `GATE1_PAIRED_CASE_RECORD_SKELETONS.md` | `511101325163561b45aaabf9e0ab5a997b9450f26efb978f6c963c5f1a356cb2` |
| `GATE1_RERUN_PRE_RESULT_MANIFEST_TEMPLATE.md` | `e36531fce5f06769aa243815a8fb15e6985bca04fab7bf62b8a13ce5eea88982` |
| `GATE2_9_EXECUTED_PREPARATION.md` | `16165c56316b34ed681aafd86f862e294cbdd79192b4b0c2953150eaf78c1f00` |
| `MECHANICAL_STRUCTURAL_CHECK_RESULTS.md` | `59ac104618283d236b7e880ae02485f5f7b74624a690d030d1c9ca897f3309b0` |
| `MECHANICAL_STRUCTURAL_CHECK_SPECIFICATION.md` | `7bcd140624501798028ea37df7fad207438c0839929aa2d72bdcf98d2f9111aa` |
| `NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md` | `4797eae658877cb326013fe087d0bd766e1d23e824cdf4218f144f5a73bac847` |
| `NEXT_SAFE_AUTOMATION_QUEUE.md` | `2750e97fa3c3844e24104845d8ba3416a9f2e551a9edc9c0fe834acfdc1b68c4` |
| `PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md` | `6d7164872e8aafb43bd23b853975e3ac8aed68e53d0a148cd0f4a4a18387c8be` |
| `PROVENANCE_AND_REUSE_AUDIT.md` | `d8367200ea184312931f4ba574a5d20b7aa3d1f2ae1a2d64224ac3fd2a2d626f` |
| `README.md` | `a735c088d0fa64c79b85144acfa10a5d988c96d8b9c88ff88add097cd6efc01e` |
| `REPOSITORY_VERIFICATION_RESULTS.md` | `dfadd07ad7a46cd65e08fff5f582447b3e3612e57b6542ff105211f650f76d46` |
| `RESUME_CODE_HEALTH_RECEIPT.md` | `22fde0d23b1a48318f1e4e6bc6d5f142c79c7e24df12ad57545bd4ec2efe5d5b` |
| `SAFE_TASK_CLASSIFICATION.md` | `327a0b01c2146df17d318a6407be2171ef091d72d312a4f60386ef988751a9da` |

The excluded receipt's file hash is
`0d8c7861fa95de81ad6d0f5e7d5e3d4050d04feb9d8f55db41350ce74ffce2fb`;
it is recorded for clarity but is not part of the inventory digest.

## Claim boundary

The inspected package has no complete package-wide receipt. This outer record
does not repair its manifest or provide durable binding for that package. It
records documentary integrity only and produces no candidate evidence.
