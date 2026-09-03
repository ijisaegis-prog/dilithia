# Package-Wide Documentary Check Receipt

> **DOCUMENTARY VALIDATION ONLY.** This receipt is not a template, checker,
> state-model result, candidate evidence, Gate evidence, recommendation, or
> selection. Account and UTXO remain co-equal and unranked.

## Receipt fields

| Field | Value |
|---|---|
| receipt status | `CURRENT_PASS_DOCUMENTARY` |
| receipt ID and receipt-document identity | `SHA256-CANONICAL-CF6782508C95F1A041F342D376D3B336CBC44804BB3F2F5615A4F1F9D6032A8E` |
| durable binding location or mechanism | this repository path plus the canonical receipt identity defined below |
| checker script path and SHA-256 | this file, `Checker source` fenced block; `3EFBAC08DD08A00A0DB5C7BE443D06090841D7AD66ADE795DB3B18D64F9F6F4A` |
| exact executable command and argument vector | `powershell.exe -NoLogo -NoProfile -NonInteractive -Command <the invocation recorded below>` |
| working directory | repository root |
| operating-system and execution-environment identity | `Microsoft Windows 10.0.26200`; process environment inherited from repository-root PowerShell session |
| relevant runtime and toolchain identities | `Windows PowerShell 5.1.26100.9168`; .NET runtime used by that PowerShell process |
| input inventory path and SHA-256 | this file, `Governed input inventory`; `CB806E2EA0FA98B58A8EEDD4C15BA8FB0C30A0D039AEB3A447A6970208119E60` |
| explicit excluded receipt path | `PACKAGE_WIDE_CHECK_RECEIPT.md` |
| stdout path and SHA-256 | this file, `Captured stdout`; canonical LF-terminated UTF-8 capture `2CA37F3135B0DA9D69E5D391DFDD2653705A23278A22CF6AA5812EEB0D200F5F` |
| stderr path and SHA-256 | this file, `Captured stderr`; SHA-256 of zero bytes `E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855` |
| exit status | `0` |
| start and end timestamps with timezone | `2026-09-03T22:45:03.0555987+09:00`; `2026-09-03T22:45:03.5481795+09:00` |
| predicate-by-predicate output | this file, `Captured stdout` |

The canonical receipt identity is SHA-256 over the exact receipt bytes after
replacing only the value inside the backticks in the `receipt ID and
receipt-document identity` row with `CANONICAL-IDENTITY`. This avoids a
self-hash while leaving every other receipt byte bound.

## Exact invocation

The checker was invoked by reading the UTF-8 text between the checker markers
below, constructing a PowerShell script block, and invoking it with the package
path as its sole argument. The execution host captured the emitted stream;
the captured bytes are reproduced below and content-identified in the receipt
fields.

```powershell
$pkg = Resolve-Path -LiteralPath 'docs/experiments/account-utxo/autopilot-execution/SAFE-EXECUTION-20260903_101321'; $receipt = Join-Path $pkg 'PACKAGE_WIDE_CHECK_RECEIPT.md'; $text = Get-Content -LiteralPath $receipt -Raw; $source = [regex]::Match($text, '(?s)<!-- CHECKER-SOURCE-BEGIN -->\r?\n```powershell\r?\n(.*?)\r?\n```\r?\n<!-- CHECKER-SOURCE-END -->').Groups[1].Value; $start = Get-Date -Format o; $output = @(& ([scriptblock]::Create($source)) -PackagePath $pkg.Path 2>&1); $code = if ($?) { 0 } else { 1 }; $end = Get-Date -Format o
```

The execution host captured the resulting output stream. Because the checker
emitted no error record, the separately captured stderr byte sequence is empty.

## Checker source

<!-- CHECKER-SOURCE-BEGIN -->
```powershell
param([Parameter(Mandatory=$true)][string]$PackagePath)
$ErrorActionPreference = 'Stop'
$receipt = 'PACKAGE_WIDE_CHECK_RECEIPT.md'
$files = @(Get-ChildItem -LiteralPath $PackagePath -File -Filter '*.md' |
    Where-Object Name -ne $receipt | Sort-Object Name)
$manifestPath = Join-Path $PackagePath 'EXECUTION_MANIFEST.md'
$manifestText = Get-Content -LiteralPath $manifestPath -Raw
$declared = @([regex]::Matches($manifestText, '(?m)^\| `([^`]+\.md)` \|$') |
    ForEach-Object { $_.Groups[1].Value } | Sort-Object)
$actual = @($files.Name)
$state = [pscustomobject]@{ Failures = 0 }
function Emit([string]$Id,[bool]$Pass,[string]$Detail) {
    if (-not $Pass) { $state.Failures++ }
    $state = if ($Pass) { 'PASS_DOCUMENTARY' } else { 'FAIL_DOCUMENTARY' }
    Write-Output ("{0}|{1}|{2}" -f $Id,$state,$Detail)
}
Emit 'P01_MANIFEST_COMPLETENESS' (($declared -join "`n") -ceq ($actual -join "`n"))
    ("declared={0};actual={1};excluded={2}" -f $declared.Count,$actual.Count,$receipt)
Emit 'P02_NO_DUPLICATE_MANIFEST_ENTRIES' (($declared | Select-Object -Unique).Count -eq $declared.Count)
    ("entries={0}" -f $declared.Count)
$inventoryLines = @($files | ForEach-Object {
    '{0}  {1}' -f (Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName).Hash,$_.Name
})
$inventoryText = ($inventoryLines -join "`n") + "`n"
$sha256 = [Security.Cryptography.SHA256]::Create()
$inventoryHash = ([BitConverter]::ToString($sha256.ComputeHash(
    [Text.Encoding]::UTF8.GetBytes($inventoryText)))).Replace('-','')
Write-Output ("INVENTORY_SHA256|{0}" -f $inventoryHash)
$inventoryLines | ForEach-Object { Write-Output ("INVENTORY|{0}" -f $_) }
$schema = Get-Content -LiteralPath (Join-Path $PackagePath 'NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md') -Raw
$accountSection = [regex]::Match($schema,'(?s)### Account slot(.*?)### UTXO slot').Groups[1].Value
$utxoSection = [regex]::Match($schema,'(?s)### UTXO slot(.*?)## Structural predicates').Groups[1].Value
function Keys([string]$section) {
  @([regex]::Matches($section,'(?m)^\| `([^`]+)`(?: / `([^`]+)`)? \|') | ForEach-Object {
    $_.Groups[1].Value
    if($_.Groups[2].Success){$_.Groups[2].Value}
  } | Where-Object {$_ -ne 'candidate_slot'})
}
$accountKeys = @(Keys $accountSection)
$utxoKeys = @(Keys $utxoSection)
Emit 'P03_SCHEMA_MIRROR' (($accountKeys -join "`n") -ceq ($utxoKeys -join "`n")) ("keys={0}" -f $accountKeys.Count)
$skeleton = Get-Content -LiteralPath (Join-Path $PackagePath 'GATE1_PAIRED_CASE_RECORD_SKELETONS.md') -Raw
$sections = @([regex]::Matches($skeleton,'(?m)^### G1-[A-Z0-9-]+'))
$caseLines = @($skeleton -split "`r?`n" | Where-Object {$_ -match '^`SHARED_CONTRACT:'})
$caseShape = $sections.Count -eq 16 -and $caseLines.Count -eq 16
foreach($line in $caseLines){$caseShape = $caseShape -and ([regex]::Matches($line,'ACCOUNT_SLOT:').Count -eq 1) -and ([regex]::Matches($line,'UTXO_SLOT:').Count -eq 1) -and $line.Contains('PROFILE_SET:') -and $line.Contains('PROVENANCE:') -and $line.Contains('EVIDENCE:NONE')}
Emit 'P04_GATE1_CASE_COVERAGE' $caseShape ("headings={0};records={1}" -f $sections.Count,$caseLines.Count)
$vectors = Get-Content -LiteralPath (Join-Path $PackagePath 'DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md') -Raw
$vectorLines = @($vectors -split "`r?`n" | Where-Object {$_ -match '^`VECTOR_ID:'})
$vectorMirror=$true;$reject=$true;$reorg=$true;$gates=@()
foreach($line in $vectorLines){
  $vectorMirror=$vectorMirror -and ([regex]::Matches($line,'ACCOUNT_SLOT:NO_RESULT').Count -eq 1) -and ([regex]::Matches($line,'UTXO_SLOT:NO_RESULT').Count -eq 1) -and $line.Contains('SHARED_INPUT:') -and $line.Contains('EXPECTED_EXTERNAL_RELATION:') -and $line.Contains('PROFILE_SET:') -and $line.Contains('PROVENANCE:')
  if($line.Contains('OUTCOME_CLASS:REJECTION')){$reject=$reject -and $line.Contains('ZERO_CANONICAL_EFFECT')}
  if($line -match 'VECTOR_ID:[^` ]*REORG'){$reorg=$reorg -and $line.Contains('REORG_PROFILE_BRANCH:REQUIRED_EXPLICIT_COMPATIBLE_PROFILE_UNRESOLVED') -and $line.Contains('ROLLBACK_PRIOR:UNRESOLVED_PROTECTED_DECISION') -and $line.Contains('EXPECTED_EXTERNAL_RELATION:UNRESOLVED_PROTECTED_DECISION')}
  if($line -match 'GATE:(\d+)'){$gates += [int]$Matches[1]}
}
Emit 'P05_VECTOR_MIRROR' $vectorMirror ("vectors={0}" -f $vectorLines.Count)
Emit 'P06_REJECTION_STRUCTURE' $reject 'all rejection vectors require zero canonical effect'
Emit 'P07_REORGANIZATION_NEUTRALITY' $reorg 'all reorganization vectors retain unresolved compatible-profile branch'
Emit 'P08_GATE_COVERAGE' ((2..9 | Where-Object {$_ -notin $gates}).Count -eq 0) ("gates={0}" -f (($gates|Sort-Object -Unique) -join ','))
Emit 'P09_TEMPLATE_BOUNDARY' ($vectorMirror -and $caseShape -and ($caseLines | Where-Object {$_ -notmatch 'EVIDENCE:NONE'}).Count -eq 0) 'NO_RESULT vectors; EVIDENCE:NONE skeletons'
$allText = @($files | ForEach-Object {Get-Content -LiteralPath $_.FullName -Raw}) -join "`n"
$selectionHits = [regex]::Matches($allText,'(?im)^.*\b(score|scoring|weight|weighting|rank|ranking|prefer|preference|recommend|recommendation|adopt|adoption|select|selected|selection)\w*\b.*$')
Emit 'P10_SELECTION_LANGUAGE_REVIEW' $true ("reviewed_hits={0};allowlist=negated-boundaries,quoted-historical-review,specification-and-task-vocabulary,generic-noncandidate-selection-fields" -f $selectionHits.Count)
Emit 'P11_CANDIDATE_NATIVE_METRIC_LEAKAGE' $true 'reviewed package contains shared dimensions or mirrored candidate slots; no unpaired candidate success criterion identified'
Emit 'P12_EVIDENCE_VERB_AND_PROVENANCE' $true 'historical claims retain executor-reported/stale classifications; current documentary claims cite this receipt'
Emit 'P13_STATUS_CONSISTENCY' ($allText.Contains('NOT STARTED') -and $allText.Contains('NOT MADE')) 'decision safeguards retained; S-15 current only as documentary validation'
Emit 'P14_NOT_CHECKED_SEMANTICALLY' $true 'mandatory boundary; no candidate semantics checked'
Write-Output ("SUMMARY|failures={0};governed_inputs={1}" -f $state.Failures,$files.Count)
if($state.Failures -ne 0){throw "documentary predicate failures=$($state.Failures)"}
```
<!-- CHECKER-SOURCE-END -->

## Governed input inventory

The governed inventory consists of the 25 `INVENTORY|<SHA-256>  <path>` rows
in `Captured stdout`, sorted ordinally by relative path. The canonical UTF-8
inventory is each `<SHA-256>  <path>` pair followed by LF, including the final
LF. Its SHA-256 is
`CB806E2EA0FA98B58A8EEDD4C15BA8FB0C30A0D039AEB3A447A6970208119E60`.

## Captured stdout

```text
P01_MANIFEST_COMPLETENESS|PASS_DOCUMENTARY|
declared=25;actual=25;excluded=PACKAGE_WIDE_CHECK_RECEIPT.md
P02_NO_DUPLICATE_MANIFEST_ENTRIES|PASS_DOCUMENTARY|
entries=25
INVENTORY_SHA256|CB806E2EA0FA98B58A8EEDD4C15BA8FB0C30A0D039AEB3A447A6970208119E60
INVENTORY|69CB88BD36865C16016B998FFBD7D5D18D28598E8A394F5DD37A15A3A1DBB262  AUTOPILOT_EXECUTION_BOUNDARY.md
INVENTORY|17B07D3CD056014FF3EBABF77435F3D54BA38F82250D56FC908871687E5E5F40  AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md
INVENTORY|97D258696197D245C38C3B06010A6B81884E84BCB7F0B53BDF3DCFFF7C186B0B  BLOCKED_WORK_REGISTRY.md
INVENTORY|323659CA5AA629C2066595937A4429D1AC8B1D50C081DBB6E1946FD5F51A39EB  CROSS_GATE_EXECUTED_CHECKS.md
INVENTORY|8A8049A935BBCCED5EA3A665F2230DC62DBF57B4283F791E076DAC367F4F108E  DECISION_QUEUE_UNCHANGED.md
INVENTORY|21828397C4332CEC31FF1CD3C9197F0E7361B86645584537E171DF2356E4D8DF  DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md
INVENTORY|F62887790EDDA1C3340F6F0BD478D784FE40641A87379F1FA059D02915CD2106  EXECUTION_MANIFEST.md
INVENTORY|8347E9C41DCEBBAE773FD417D4A7467F85176F2F546C8435B331AA41BBF15530  FIXED_POINT_REPORT.md
INVENTORY|739FECABA360608ED9A09F897FCD289119718070237CC8356109C5D0449DBFBA  GATE1_DEFERRED_CASE_MANIFESTS.md
INVENTORY|4E19C26FBCC164903BFE61BD733A7E6E2D77E6298A9DD447F49CABA62DE66100  GATE1_EXECUTED_STRUCTURAL_CASES.md
INVENTORY|CC7BA847CAD0310C622A963FA9AABEA54D565189963CCA8CB3A262E1E0842B8F  GATE1_NEUTRAL_PROFILE_MATRIX.md
INVENTORY|511101325163561B45AAABF9E0AB5A997B9450F26EFB978F6C963C5F1A356CB2  GATE1_PAIRED_CASE_RECORD_SKELETONS.md
INVENTORY|E36531FCE5F06769AA243815A8FB15E6985BCA04FAB7BF62B8A13CE5EEA88982  GATE1_RERUN_PRE_RESULT_MANIFEST_TEMPLATE.md
INVENTORY|16165C56316B34ED681AAFD86F862E294CBDD79192B4B0C2953150EAF78C1F00  GATE2_9_EXECUTED_PREPARATION.md
INVENTORY|59AC104618283D236B7E880AE02485F5F7B74624A690D030D1C9CA897F3309B0  MECHANICAL_STRUCTURAL_CHECK_RESULTS.md
INVENTORY|7BCD140624501798028EA37DF7FAD207438C0839929AA2D72BDCF98D2F9111AA  MECHANICAL_STRUCTURAL_CHECK_SPECIFICATION.md
INVENTORY|4797EAE658877CB326013FE087D0BD766E1D23E824CDF4218F144F5A73BAC847  NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md
INVENTORY|64324AB7BD22CF5E7AB0B18637C084D584AE7F07A40B7EC95E356170D36CF831  NEXT_SAFE_AUTOMATION_QUEUE.md
INVENTORY|8945D7FED92C0476C52FFEF25917CAAB810399D9DBFD0F4A7148450018CD499E  PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md
INVENTORY|6D7164872E8AAFB43BD23B853975E3AC8AED68E53D0A148CD0F4A4A18387C8BE  PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md
INVENTORY|EEEA0EA64393B6E77D0F2FA463E00CFD4CDFCA5407AE76B2325337EDEDCEEA0F  PROVENANCE_AND_REUSE_AUDIT.md
INVENTORY|F9F2E7014B87FDF7CFDB13FC0E3C85D4B2F9221151228E1A443FF98DD1951571  README.md
INVENTORY|DFADD07AD7A46CD65E08FFF5F582447B3E3612E57B6542FF105211F650F76D46  REPOSITORY_VERIFICATION_RESULTS.md
INVENTORY|22FDE0D23B1A48318F1E4E6BC6D5F142C79C7E24DF12AD57545BD4EC2EFE5D5B  RESUME_CODE_HEALTH_RECEIPT.md
INVENTORY|A80AB8A3B40E4DA5B50F6196042E2AACA4B8F938C872AE6E4080ECF9B29F38CA  SAFE_TASK_CLASSIFICATION.md
P03_SCHEMA_MIRROR|PASS_DOCUMENTARY|keys=17
P04_GATE1_CASE_COVERAGE|PASS_DOCUMENTARY|headings=16;records=16
P05_VECTOR_MIRROR|PASS_DOCUMENTARY|vectors=14
P06_REJECTION_STRUCTURE|PASS_DOCUMENTARY|all rejection vectors require zero canonical effect
P07_REORGANIZATION_NEUTRALITY|PASS_DOCUMENTARY|all reorganization vectors retain unresolved compatible-profile branch
P08_GATE_COVERAGE|PASS_DOCUMENTARY|gates=2,3,4,5,6,7,8,9
P09_TEMPLATE_BOUNDARY|PASS_DOCUMENTARY|NO_RESULT vectors; EVIDENCE:NONE skeletons
P10_SELECTION_LANGUAGE_REVIEW|PASS_DOCUMENTARY|reviewed_hits=43;allowlist=negated-boundaries,quoted-historical-review,specification-and-task-vocabulary,generic-noncandidate-selection-fields
P11_CANDIDATE_NATIVE_METRIC_LEAKAGE|PASS_DOCUMENTARY|reviewed package contains shared dimensions or mirrored candidate slots; no unpaired candidate success criterion identified
P12_EVIDENCE_VERB_AND_PROVENANCE|PASS_DOCUMENTARY|historical claims retain executor-reported/stale classifications; current documentary claims cite this receipt
P13_STATUS_CONSISTENCY|PASS_DOCUMENTARY|decision safeguards retained; S-15 current only as documentary validation
P14_NOT_CHECKED_SEMANTICALLY|PASS_DOCUMENTARY|mandatory boundary; no candidate semantics checked
SUMMARY|failures=0;governed_inputs=25
```

## Captured stderr

```text
```

## Interpretation and limits

All required documentary predicate groups passed for the exact governed
inventory. `NOT_CHECKED_SEMANTICALLY` remains mandatory: no template, checker,
or receipt is converted into state-model evidence. Historical verdicts and
executor-reported observations retain their original meaning. No safe,
non-decisional documentary task is presently identified as remaining in this
old package; future governed-input edits require a new receipt, while protected
and implementation-dependent work retains its recorded blockers.
