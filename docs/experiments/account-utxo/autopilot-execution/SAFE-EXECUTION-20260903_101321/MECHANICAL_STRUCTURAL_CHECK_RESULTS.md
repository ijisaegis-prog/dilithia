# Mechanical Structural Check Results — Invalidated Record

**Classification:** `STALE_EXECUTOR_REPORTED_OBSERVATION`; not a bound executed
check, candidate evidence, or Gate evidence.

## Why this record is invalid

| Input | Earlier reported SHA-256 | Current SHA-256 observed during correction |
|---|---|---|
| `NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md` | `ABC726BA8F950CE639B37768B96B09CA3F0441247C7FC126EB536E800976A962` | `4797EAE658877CB326013FE087D0BD766E1D23E824CDF4218F144F5A73BAC847` |
| `GATE1_PAIRED_CASE_RECORD_SKELETONS.md` | `29E75EEB5190BE29569287D5D5FA50F5775C741D11D548669099E967E05A5365` | `E6D310F4B3A2EF3EE4787FB93D3F10D050675F705EEAF11EAFABA6C739FEB37B` |
| `DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md` | `099FDFE7CEB3BE5F8AB31025BE8C6CBC0D5EFC86A22CB22B0CA40B5D602BE52E` | `099FDFE7CEB3BE5F8AB31025BE8C6CBC0D5EFC86A22CB22B0CA40B5D602BE52E` |

Two identities differ. Under the procedure's invalidation rule, every earlier
`PASS_STRUCTURAL` row is withdrawn.

## Missing execution provenance

The earlier execution did not durably record the exact executable command or
script bytes, stdout, stderr, and exit status. Its predicate descriptions
cannot reconstruct how the observations were evaluated. Therefore the earlier
rows support only an executor-reported historical observation even for the one
input whose identity still matches.

## Current disposition

| Item | Status |
|---|---|
| Earlier predicate results | `WITHDRAWN_STALE` |
| `S-15` | `PENDING_REEXECUTION` |
| Structural symmetry | `NOT_CURRENTLY_VERIFIED` |
| Substantive Account/UTXO symmetry | `NOT_CHECKED_SEMANTICALLY` |
| Package-wide lint and provenance validation | `PENDING_REEXECUTION` |

No replacement pass is claimed. A future receipt must satisfy
`PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md` and bind the then-current inventory.
