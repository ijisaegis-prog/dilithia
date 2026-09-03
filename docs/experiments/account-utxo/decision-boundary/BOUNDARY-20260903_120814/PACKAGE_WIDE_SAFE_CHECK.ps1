param([Parameter(Mandatory=$true)][string]$PackagePath)
$ErrorActionPreference='Stop'
$package=(Resolve-Path -LiteralPath $PackagePath).Path
$receiptName='PACKAGE_WIDE_CHECK_RECEIPT.md'
function HashText([string]$s){$b=[Text.UTF8Encoding]::new($false).GetBytes($s);$h=[Security.Cryptography.SHA256]::Create();try{([BitConverter]::ToString($h.ComputeHash($b))).Replace('-','').ToLowerInvariant()}finally{$h.Dispose()}}
function Raw([string]$n){Get-Content -LiteralPath (Join-Path $package $n) -Raw}
function Hits([regex]$pattern){$r=[Collections.Generic.List[object]]::new();foreach($f in $inputs){$n=0;foreach($l in Get-Content -LiteralPath $f.FullName){$n++;if([string]$l -match $pattern){$r.Add([ordered]@{id="$($f.Name):$n";file=$f.Name;line=$n;text=[string]$l})}}};@($r)}
function InList([string]$id,[string[]]$items){$items -contains $id}
function SectionText([string]$file,[int]$line){$ls=@(Get-Content -LiteralPath (Join-Path $package $file));$start=0;for($i=0;$i-lt[Math]::Min($line,$ls.Count);$i++){if($ls[$i]-match'^#{1,6}\s'){$start=$i}};$end=$ls.Count;for($i=$line;$i-lt$ls.Count;$i++){if($ls[$i]-match'^#{1,6}\s'){$end=$i;break}};($ls[$start..($end-1)]-join"`n")}

$all=@(Get-ChildItem -LiteralPath $package -File -Filter '*.md'|Sort-Object Name)
$inputs=@($all|Where-Object Name -ne $receiptName)
$inv=@($inputs|ForEach-Object{"{0}`t{1}" -f $_.Name,(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()})
$invText=($inv -join "`n")+"`n"
$manifest=Raw 'EXECUTION_MANIFEST.md'
$declared=@([regex]::Matches($manifest,'(?m)^\| `([^`]+\.md)` \|$')|ForEach-Object{$_.Groups[1].Value})
$dupes=@($declared|Group-Object|Where-Object Count -gt 1|ForEach-Object Name|Sort-Object)
$group2MissingFromInputs=@($declared|Where-Object{$_ -notin $inputs.Name}|Sort-Object -Unique)
$extra=@($inputs.Name|Where-Object{$_ -notin $declared}|Sort-Object -Unique)
$unhashed=@($declared|Where-Object{$_ -in $inputs.Name -and -not (Get-FileHash -LiteralPath (Join-Path $package $_) -Algorithm SHA256).Hash})
$p2=$group2MissingFromInputs.Count -eq 0 -and $extra.Count -eq 0 -and $dupes.Count -eq 0 -and $unhashed.Count -eq 0

# Group 1: all seven predicates from the structural specification.
$schema=Raw 'NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md'
$ab=[regex]::Match($schema,'(?s)### Account slot(.*?)(?=### UTXO slot)').Groups[1].Value
$ub=[regex]::Match($schema,'(?s)### UTXO slot(.*?)(?=##|\z)').Groups[1].Value
$ak=@([regex]::Matches($ab,'(?m)^\| `([^`]+)` \|')|ForEach-Object{$_.Groups[1].Value}|Where-Object{$_ -ne 'candidate_slot'})
$uk=@([regex]::Matches($ub,'(?m)^\| `([^`]+)` \|')|ForEach-Object{$_.Groups[1].Value}|Where-Object{$_ -ne 'candidate_slot'})
$s1=(($ak -join "`n") -ceq ($uk -join "`n"))-and $ak.Count -gt 0
$sk=Raw 'GATE1_PAIRED_CASE_RECORD_SKELETONS.md'
$parts=@([regex]::Matches($sk,'(?ms)^###\s+G1-[^\r\n]+\r?\n(.*?)(?=^###\s+|^##\s+|\z)'))
$s2=$parts.Count -eq 16
foreach($x in $parts){$b=$x.Groups[1].Value;$s2=$s2-and([regex]::Matches($b,'SHARED_CONTRACT:').Count-eq 1)-and([regex]::Matches($b,'ACCOUNT_SLOT:').Count-eq 1)-and([regex]::Matches($b,'UTXO_SLOT:').Count-eq 1)-and([regex]::Matches($b,'PROFILE_SET:').Count-eq 1)-and([regex]::Matches($b,'PROVENANCE:').Count-eq 1)-and($b-match'EVIDENCE:NONE')}
$vec=@((Get-Content -LiteralPath (Join-Path $package 'DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md'))|Where-Object{$_ -match'VECTOR_ID:'})
$s3=$vec.Count-gt 0
foreach($v in $vec){$s3=$s3-and([regex]::Matches($v,'ACCOUNT_SLOT:').Count-eq 1)-and([regex]::Matches($v,'UTXO_SLOT:').Count-eq 1)-and($v-match'SHARED_INPUT:')-and($v-match'EXPECTED_EXTERNAL_RELATION:')-and($v-match'PROFILE_SET:')-and($v-match'PROVENANCE:')}
$s4=@($vec|Where-Object{$_-match'REJECTION'-and$_-notmatch'ZERO_CANONICAL_EFFECT'}).Count-eq 0
$s5=@($vec|Where-Object{$_-match'VECTOR_ID:[^|]*REORG'-and($_-notmatch'REORG_PROFILE_BRANCH:REQUIRED_EXPLICIT_COMPATIBLE_PROFILE_UNRESOLVED'-or$_-notmatch'ROLLBACK_PRIOR:UNRESOLVED_PROTECTED_DECISION'-or$_-notmatch'EXPECTED_EXTERNAL_RELATION:UNRESOLVED_PROTECTED_DECISION')}).Count-eq 0
$gf=@($vec|ForEach-Object{if($_-match'GATE:([2-9])'){[int]$Matches[1]}}|Sort-Object -Unique)
$s6=($gf-join',')-eq'2,3,4,5,6,7,8,9'
$s7=@($vec|Where-Object{$_-notmatch'ACCOUNT_SLOT:NO_RESULT'-or$_-notmatch'UTXO_SLOT:NO_RESULT'}).Count-eq 0 -and [regex]::Matches($sk,'EVIDENCE:(?!NONE)').Count-eq 0
$struct=[ordered]@{schema_mirror=$s1;gate1_case_coverage=$s2;vector_mirror=$s3;rejection_structure=$s4;reorganization_neutrality=$s5;gate_2_through_9_coverage=$s6;template_boundary=$s7;semantic_boundary='NOT_CHECKED_SEMANTICALLY'}
$p1=@($s1,$s2,$s3,$s4,$s5,$s6,$s7)-notcontains $false

# Group 3: exempt only an explicitly marked uninstantiated template. Parse
# every instance fail-closed against the template's complete field set.
$mf=@($all|Where-Object { $_.Name -match 'PAIRED_CONTENT_ADDRESSABLE_MANIFEST' })
$instances=@($mf|Where-Object{-not($_.Name-eq'PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md'-and(Raw $_.Name)-match'UNINSTANTIATED NEUTRAL SCHEMA')})
$requiredEnvelope=@('manifest schema version','manifest identity algorithm','manifest identity','canonicalization rule','source commit','governing artifact path and SHA-256, one row each','shared case path and SHA-256','frozen profile-set path and SHA-256','shared oracle path and SHA-256','evidence class and claim boundary','registration status and receipt identity')
$requiredBindings=@('mapping path','mapping SHA-256','implementation or model path','implementation or model SHA-256','harness path','harness SHA-256','oracle binding identity','input/corpus identity','execution status','result/output identity','review identity')
$requiredProvenance=@('working directory','executable identity','command/argument vector','environment allowlist','toolchain identities','start/end timestamps','stdout identity','stderr identity','exit status','raw-output locations and identities','reviewer identity','exclusions')
function TableRows([string]$text,[string]$heading){$m=[regex]::Match($text,"(?ms)^## $([regex]::Escape($heading))\s*\r?\n(.*?)(?=^##\s|\z)");if(-not$m.Success){return @()};@([regex]::Matches($m.Groups[1].Value,'(?m)^\|\s*(?!-)(.*?)\s*\|\s*(.*?)\s*\|(?:\s*(.*?)\s*\|)?$')|ForEach-Object{,@($_.Groups[1].Value.Trim(' ','`'),$_.Groups[2].Value.Trim(' ','`'),$_.Groups[3].Value.Trim(' ','`'))}|Where-Object{$_[0]-notin@('Field','Required binding')})}
function ResolveBoundPath([string]$path){if([IO.Path]::IsPathRooted($path)){return $path};$local=Join-Path $package $path;if(Test-Path -LiteralPath $local -PathType Leaf){return $local};Join-Path (Get-Location).Path $path}
$ir=@();foreach($i in $instances){
 $t=Raw $i.Name;$en=@(TableRows $t 'Manifest envelope');$bn=@(TableRows $t 'Co-equal candidate bindings')
 $enNames=@($en|ForEach-Object{$_[0]});$bnNames=@($bn|ForEach-Object{$_[0]})
 $missingEnvelope=@($requiredEnvelope|Where-Object{$_-notin$enNames});$duplicateEnvelope=@($enNames|Group-Object|Where-Object{$_.Count-gt 1-and$_.Name-ne'governing artifact path and SHA-256, one row each'}|ForEach-Object Name)
 $missingBindings=@($requiredBindings|Where-Object{$_-notin$bnNames});$duplicateBindings=@($bnNames|Group-Object|Where-Object Count -gt 1|ForEach-Object Name)
 $missingProvenance=@($requiredProvenance|Where-Object{$t-notmatch("(?i)\b"+[regex]::Escape($_)+"\b")})
 $placeholderRows=@($en+$bn|Where-Object{@($_[1],$_[2]) -match '^(?:UNRESOLVED|NONE|BLOCKED_MISSING_INPUT|PENDING)$'}|ForEach-Object{$_[0]}|Sort-Object -Unique)
 $digestChecks=@();$pathRows=@($en|Where-Object{$_[0]-match'path and SHA-256'});foreach($row in $pathRows){$matches=@([regex]::Matches($row[1],'`?([^`;|]+?)`?\s*(?:;|,|\s)\s*`?([0-9A-Fa-f]{64})`?(?=\s|$)'));if($matches.Count-eq 0){$digestChecks+=[ordered]@{field=$row[0];path='';digest_syntax=$false;resolvable=$false;hash_matches=$false}};foreach($match in $matches){$rp=$match.Groups[1].Value.Trim(' ','`');$dg=$match.Groups[2].Value.ToLowerInvariant();$q=ResolveBoundPath $rp;$exists=Test-Path -LiteralPath $q -PathType Leaf;$digestChecks+=[ordered]@{field=$row[0];path=$rp;digest_syntax=$true;resolvable=$exists;hash_matches=($exists-and(Get-FileHash -LiteralPath $q -Algorithm SHA256).Hash.ToLowerInvariant()-eq$dg)}}}
 foreach($candidateColumn in 1,2){foreach($prefix in @('mapping','implementation or model','harness')){$pr=@($bn|Where-Object{$_[0]-eq"$prefix path"});$dr=@($bn|Where-Object{$_[0]-eq"$prefix SHA-256"});$pv=if($pr.Count){$pr[0][$candidateColumn]}else{''};$dv=if($dr.Count){$dr[0][$candidateColumn]}else{''};$syntax=$dv-match'^[0-9A-Fa-f]{64}$';$q=if($pv){ResolveBoundPath $pv}else{''};$exists=$pv-and(Test-Path -LiteralPath $q -PathType Leaf);$digestChecks+=[ordered]@{field="$(if($candidateColumn-eq1){'Account'}else{'UTXO'}) $prefix";path=$pv;digest_syntax=$syntax;resolvable=[bool]$exists;hash_matches=($syntax-and$exists-and(Get-FileHash -LiteralPath $q -Algorithm SHA256).Hash.ToLowerInvariant()-eq$dv.ToLowerInvariant())}}}
 $coequalColumns=$t-match'(?m)^\| Required binding \| Account \| UTXO \|\s*$'
 $statusChecks=@();foreach($candidateColumn in 1,2){$statusRow=@($bn|Where-Object{$_[0]-eq'execution status'});$resultRow=@($bn|Where-Object{$_[0]-eq'result/output identity'});$status=if($statusRow.Count){$statusRow[0][$candidateColumn]}else{''};$result=if($resultRow.Count){$resultRow[0][$candidateColumn]}else{''};$hasResult=$result-and$result-notmatch'^(?:NONE|UNRESOLVED|PENDING|BLOCKED_MISSING_INPUT)$';$expectsResult=$status-match'^(?:COMPLETE|COMPLETED|EXECUTED|PASS|PASSED|SUCCESS)';$forbidsResult=$status-match'^(?:BLOCKED|PENDING|NOT_EXECUTED|FAILED|FAIL)';$consistent=($expectsResult-and$hasResult)-or($forbidsResult-and-not$hasResult);$statusChecks+=[ordered]@{candidate=$(if($candidateColumn-eq1){'Account'}else{'UTXO'});status=$status;result=$result;consistent=$consistent}}
 $identityRow=@($en|Where-Object{$_[0]-eq'manifest identity'});$algorithmRow=@($en|Where-Object{$_[0]-eq'manifest identity algorithm'});$canonicalRow=@($en|Where-Object{$_[0]-eq'canonicalization rule'});$identity=if($identityRow.Count){$identityRow[0][1]}else{''};$identitySyntax=$identity-match'^[0-9A-Fa-f]{64}$';$canonicalSupported=$algorithmRow.Count-and$algorithmRow[0][1]-match'(?i)^SHA-256$'-and$canonicalRow.Count-and$canonicalRow[0][1]-match'(?i)replace only the manifest identity value with 64 zero(?:es|s).+UTF-8 without BOM'
 $canonicalText=if($identitySyntax){$t.Replace($identity,'0'*64)}else{$t};$identityMatches=$identitySyntax-and$canonicalSupported-and(HashText $canonicalText)-eq$identity.ToLowerInvariant()
 $ok=$missingEnvelope.Count-eq0-and$duplicateEnvelope.Count-eq0-and$missingBindings.Count-eq0-and$duplicateBindings.Count-eq0-and$missingProvenance.Count-eq0-and$placeholderRows.Count-eq0-and$coequalColumns-and@($digestChecks|Where-Object{-not$_.digest_syntax-or-not$_.resolvable-or-not$_.hash_matches}).Count-eq0-and@($statusChecks|Where-Object{-not$_.consistent}).Count-eq0-and$identityMatches
 $ir+=[ordered]@{file=$i.Name;missing_envelope_fields=$missingEnvelope;duplicate_envelope_fields=$duplicateEnvelope;missing_binding_fields=$missingBindings;duplicate_binding_fields=$duplicateBindings;missing_provenance_fields=$missingProvenance;unresolved_required_rows=$placeholderRows;coequal_columns=$coequalColumns;digest_and_path_checks=$digestChecks;status_result_checks=$statusChecks;manifest_identity_syntax=$identitySyntax;canonicalization_supported=$canonicalSupported;manifest_identity_matches=$identityMatches;pass=$ok}
}
$p3=$instances.Count-eq 0-or@($ir|Where-Object{-not$_.pass}).Count-eq 0

# Group 4: every lexical hit must have an explicit reviewed disposition.
# Entries are exact file/line identities for this inventory; edits move or add
# hits and therefore fail closed until the allowlist is reviewed again.
$sh=@(Hits ([regex]'(?i)\b(score|scored|scoring|weight|weighted|weighting|rank|ranked|ranking|prefer|preferred|preference|recommend|recommended|recommendation|adopt|adopted|adoption|select|selected|selection|winner)\b'))
$selectionNegated=@('AUTOPILOT_EXECUTION_BOUNDARY.md:14','AUTOPILOT_EXECUTION_BOUNDARY.md:35','AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md:7','AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md:8','AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md:10','AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md:11','DECISION_QUEUE_UNCHANGED.md:3','DECISION_QUEUE_UNCHANGED.md:4','DECISION_QUEUE_UNCHANGED.md:8','DECISION_QUEUE_UNCHANGED.md:22','EXECUTION_MANIFEST.md:83','FIXED_POINT_REPORT.md:42','FIXED_POINT_REPORT.md:53','FIXED_POINT_REPORT.md:54','GATE1_RERUN_PRE_RESULT_MANIFEST_TEMPLATE.md:99','GATE1_RERUN_PRE_RESULT_MANIFEST_TEMPLATE.md:100','NEXT_SAFE_AUTOMATION_QUEUE.md:27','PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md:5','README.md:4','README.md:39','README.md:40','README.md:42','README.md:43','RESUME_CODE_HEALTH_RECEIPT.md:257','RESUME_CODE_HEALTH_RECEIPT.md:258','RESUME_CODE_HEALTH_RECEIPT.md:260','RESUME_CODE_HEALTH_RECEIPT.md:261')
$selectionTasks=@('BLOCKED_WORK_REGISTRY.md:18','FIXED_POINT_REPORT.md:35','PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md:28','PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md:29','PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md:36','SAFE_TASK_CLASSIFICATION.md:17','SAFE_TASK_CLASSIFICATION.md:37')
$selectionOutOfScope=@('GATE1_DEFERRED_CASE_MANIFESTS.md:4','GATE1_NEUTRAL_PROFILE_MATRIX.md:4','NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:23','NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:26')
$selectionReview=@($sh|ForEach-Object{$d=if(InList $_.id $selectionNegated){'ALLOWLIST_NEGATED_BOUNDARY'}elseif(InList $_.id $selectionTasks){'ALLOWLIST_PROHIBITIVE_TASK'}elseif(InList $_.id $selectionOutOfScope){'OUT_OF_SCOPE_HOMONYM_NOT_ALLOWLISTED'}else{'UNREVIEWED'};[ordered]@{id=$_.id;text=$_.text;disposition=$d;pass=$d-ne'UNREVIEWED'}})
$p4=@($selectionReview|Where-Object{-not$_.pass}).Count-eq 0

# Group 5: every metric-bearing candidate block receives a disposition. Merely
# naming both candidates is insufficient: candidate-specific values require a
# value for each candidate and explicit provenance in the same block.
$metricWords='(?i)\b(metric|threshold|workload|criterion|criteria|latency|throughput|size|bytes|count|performance)\b'
$measurement='(?i)(?:\b\d+(?:\.\d+)?\s*(?:ns|us|ms|s|bytes?|kib|mib|gib|ops(?:/s)?|tx(?:/s)?|%|percent)\b|\b(?:metric|threshold|latency|throughput|size|bytes|count|performance)\s*(?:=|:|is)\s*`?\d)'
$leakReview=@();foreach($f in $inputs){$blocks=[regex]::Split((Get-Content -LiteralPath $f.FullName -Raw),'(?:\r?\n){2,}');for($bi=0;$bi-lt$blocks.Count;$bi++){$b=$blocks[$bi];if($b-notmatch$metricWords){continue};$hasA=$b-match'(?i)\bAccount\b';$hasU=$b-match'(?i)\bUTXO\b';if(-not($hasA-or$hasU)){continue};$aMeasured=$b-match("(?is)(?:\bAccount\b.{0,160}"+$measurement+"|"+$measurement+".{0,160}\bAccount\b)");$uMeasured=$b-match("(?is)(?:\bUTXO\b.{0,160}"+$measurement+"|"+$measurement+".{0,160}\bUTXO\b)");$anyMeasured=$aMeasured-or$uMeasured;$pairedScope=$hasA-and$hasU;$pairedMeasurements=$aMeasured-and$uMeasured;$provenance=$b-match'(?i)\b(provenance|receipt|source (?:identity|path)|command(?:/argument vector)?|stdout identity|raw-output)\b';$mirroredOrShared=$b-match'(?i)\b(shared|external dimension|mirrored|co-equal|symmetric|candidate slots?|labels?|labelcounts)\b';$pass=if($anyMeasured){$pairedScope-and$pairedMeasurements-and$provenance}else{$pairedScope-and$mirroredOrShared};$disposition=if($anyMeasured-and-not$pairedMeasurements){'FAIL_UNPAIRED_CANDIDATE_MEASUREMENT'}elseif($anyMeasured-and-not$provenance){'FAIL_PAIRED_MEASUREMENT_MISSING_PROVENANCE'}elseif($anyMeasured){'PAIRED_MEASUREMENT_WITH_PROVENANCE'}elseif($pass){'SHARED_OR_MIRRORED_DIMENSION_NO_MEASUREMENT'}else{'FAIL_UNREVIEWED_CANDIDATE_METRIC_SCOPE'};$leakReview+=[ordered]@{id=("{0}:block-{1}"-f$f.Name,($bi+1));has_account=$hasA;has_utxo=$hasU;account_measurement=$aMeasured;utxo_measurement=$uMeasured;paired_scope=$pairedScope;paired_measurements=$pairedMeasurements;provenance=$provenance;shared_or_mirrored=$mirroredOrShared;disposition=$disposition;pass=$pass;text=($b-replace'\r?\n',' ')}}}
$p5=@($leakReview|Where-Object{-not$_.pass}).Count-eq 0

# Groups 6 and 7: apply the reviewed claim classification at observation
# granularity. Negations/retractions and template requirements are permitted;
# historical transcripts and explicitly pending executor reports are not
# upgraded to evidence. Any new lexical hit fails until reviewed.
$eh=@(Hits ([regex]'(?i)\b(proves?|verified|verifies|establish(?:es|ed)?|demonstrat(?:es|ed)?|passed|executed)\b'))
$evidenceNegated=@('AUTOPILOT_EXECUTION_BOUNDARY.md:32','CROSS_GATE_EXECUTED_CHECKS.md:5','CROSS_GATE_EXECUTED_CHECKS.md:9','CROSS_GATE_EXECUTED_CHECKS.md:20','DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md:3','DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md:70','EXECUTION_MANIFEST.md:20','EXECUTION_MANIFEST.md:21','GATE1_DEFERRED_CASE_MANIFESTS.md:8','GATE1_EXECUTED_STRUCTURAL_CASES.md:25','GATE1_EXECUTED_STRUCTURAL_CASES.md:31','GATE1_PAIRED_CASE_RECORD_SKELETONS.md:89','MECHANICAL_STRUCTURAL_CHECK_RESULTS.md:3','MECHANICAL_STRUCTURAL_CHECK_RESULTS.md:33','NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:5','NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:122','README.md:13','REPOSITORY_VERIFICATION_RESULTS.md:24','SAFE_TASK_CLASSIFICATION.md:39','SAFE_TASK_CLASSIFICATION.md:51')
$evidenceTemplate=@('EXECUTION_MANIFEST.md:15','GATE1_EXECUTED_STRUCTURAL_CASES.md:1','GATE1_EXECUTED_STRUCTURAL_CASES.md:18','MECHANICAL_STRUCTURAL_CHECK_SPECIFICATION.md:46','MECHANICAL_STRUCTURAL_CHECK_SPECIFICATION.md:50','NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md:119','PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md:37','PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md:38','PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md:52','PAIRED_CONTENT_ADDRESSABLE_MANIFEST_TEMPLATE.md:53','PROVENANCE_AND_REUSE_AUDIT.md:52')
$evidenceHistorical=@('REPOSITORY_VERIFICATION_RESULTS.md:17','RESUME_CODE_HEALTH_RECEIPT.md:82','RESUME_CODE_HEALTH_RECEIPT.md:88','RESUME_CODE_HEALTH_RECEIPT.md:94','RESUME_CODE_HEALTH_RECEIPT.md:100','RESUME_CODE_HEALTH_RECEIPT.md:105','RESUME_CODE_HEALTH_RECEIPT.md:111','RESUME_CODE_HEALTH_RECEIPT.md:161','RESUME_CODE_HEALTH_RECEIPT.md:166','RESUME_CODE_HEALTH_RECEIPT.md:171','RESUME_CODE_HEALTH_RECEIPT.md:176','RESUME_CODE_HEALTH_RECEIPT.md:181','RESUME_CODE_HEALTH_RECEIPT.md:186','RESUME_CODE_HEALTH_RECEIPT.md:191')
$evidencePending=@('SAFE_TASK_CLASSIFICATION.md:13')
$evidenceSupported=@('FIXED_POINT_REPORT.md:13')
$evidenceReview=@($eh|ForEach-Object{$ef=$_.id-replace':\d+$','';$eln=[int]($_.id-replace'^.*:','');$sec=SectionText $ef $eln;$sameSectionReceipt=($sec-match'PACKAGE_WIDE_CHECK_RECEIPT\.md')-and($sec-match'(?i)predicate|output|stream') -and ($sec-match'(?i)inventory|identity|bound');$d=if(InList $_.id $evidenceNegated){'NEGATED_OR_RETRACTED'}elseif(InList $_.id $evidenceTemplate){'TEMPLATE_REQUIREMENT_OR_HEADING'}elseif(InList $_.id $evidenceHistorical){'EXECUTOR_REPORTED_OBSERVATION'}elseif(InList $_.id $evidencePending){'PENDING'}elseif((InList $_.id $evidenceSupported)-and$sameSectionReceipt){'SUPPORTED_BY_SAME_SECTION_RECEIPT'}else{'UNREVIEWED_EVIDENCE_CLAIM'};[ordered]@{id=$_.id;text=$_.text;disposition=$d;same_section_receipt=$sameSectionReceipt;pass=$d-ne'UNREVIEWED_EVIDENCE_CLAIM'}})
$p6=@($evidenceReview|Where-Object{-not$_.pass}).Count-eq 0
$receiptText=Raw $receiptName
$receiptHasCompleteFields=$receiptText-notmatch'<the invocation recorded below>'
foreach($field in @('exact executable command and argument vector','working directory','input inventory path and SHA-256','stdout path and SHA-256','stderr path and SHA-256','exit status','start and end timestamps with timezone')){$receiptHasCompleteFields=$receiptHasCompleteFields-and$receiptText.Contains($field)}
$pr=@($evidenceReview|Where-Object{$_.disposition-in @('EXECUTOR_REPORTED_OBSERVATION','PENDING','SUPPORTED_BY_SAME_SECTION_RECEIPT')}|ForEach-Object{$hit=$_;$ef=$hit.id-replace':\d+$','';$eln=[int]($hit.id-replace'^.*:','');$sec=SectionText $ef $eln;$group7MissingProvenanceFields=@();foreach($field in @('command','working directory','input','stdout','stderr','exit status','start','end')){if($sec-notmatch[regex]::Escape($field)){$group7MissingProvenanceFields+=$field}};$complete=$group7MissingProvenanceFields.Count-eq 0;$class=if($complete){'COMPLETE_PROVENANCE'}elseif($hit.disposition-in@('EXECUTOR_REPORTED_OBSERVATION','PENDING')){$hit.disposition}elseif($hit.disposition-eq'SUPPORTED_BY_SAME_SECTION_RECEIPT'-and$receiptHasCompleteFields){'COMPLETE_LINKED_RECEIPT_PROVENANCE'}else{'UNCLASSIFIED_INCOMPLETE_PROVENANCE'};[ordered]@{id=$hit.id;classification=$class;missing_fields=$group7MissingProvenanceFields;complete_provenance=($complete-or$class-eq'COMPLETE_LINKED_RECEIPT_PROVENANCE');pass=$class-ne'UNCLASSIFIED_INCOMPLETE_PROVENANCE'}})
$p7=(@($evidenceReview|Where-Object{$_.disposition-eq'UNREVIEWED_EVIDENCE_CLAIM'}).Count-eq 0)-and(@($pr|Where-Object{-not$_.pass}).Count-eq 0)

# Group 8: enforce the exact intended distinction across the five governing
# records: the old result/fixed-point overclaim stays retracted, while only the
# separate receipt makes S-15 current documentary work.
$sf=@('README.md','EXECUTION_MANIFEST.md','SAFE_TASK_CLASSIFICATION.md','FIXED_POINT_REPORT.md','MECHANICAL_STRUCTURAL_CHECK_RESULTS.md')
$statusTests=[ordered]@{
 'README.md'={param($t) $t-match'current documentary package check is\s+bound by `PACKAGE_WIDE_CHECK_RECEIPT\.md`'-and$t-match'Comparison scoring: \*\*NOT STARTED\*\*'-and$t-match'State-model decision: \*\*NOT MADE\*\*'}
 'EXECUTION_MANIFEST.md'={param($t) $t-match'current execution receipt'-and$t-match'Comparison scoring \*\*NOT STARTED\*\*; ranking \*\*NONE\*\*; decision \*\*NOT MADE\*\*'}
 'SAFE_TASK_CLASSIFICATION.md'={param($t)
   $t-match'(?m)^\| S-07 \|.*\| EXECUTED_SAFE_NOW \|.*PACKAGE_WIDE_CHECK_RECEIPT\.md' -and
   $t-match'(?m)^\| S-11 \|.*\| EXECUTED_SAFE_NOW \|.*PACKAGE_WIDE_CHECK_RECEIPT\.md' -and
   $t-match'(?m)^\| S-15 \|.*\| EXECUTED_SAFE_NOW \|.*PACKAGE_WIDE_CHECK_RECEIPT\.md' -and
   $t-match'(?m)^\| S-18 \|.*\| EXECUTED_SAFE_NOW \|.*PACKAGE_WIDE_CHECK_RECEIPT\.md'
 }
 'FIXED_POINT_REPORT.md'={param($t) $t-match'earlier documentary fixed-point assertion remains \*\*RETRACTED\*\*'-and$t-match'`S-15` is `EXECUTED_SAFE_NOW` only as documentary validation'-and$t-match'`SAFE_NOW_MEANINGFUL_TASKS_REMAIN: NO`'}
 'MECHANICAL_STRUCTURAL_CHECK_RESULTS.md'={param($t) $t-match'STALE_EXECUTOR_REPORTED_OBSERVATION'-and$t-match'\| `S-15` \| `EXECUTED_SAFE_NOW` only under the separate current package-wide receipt'}
}
$sr=@($sf|ForEach-Object{$t=Raw $_;$ok=&$statusTests[$_] $t;[ordered]@{file=$_;expected_status_consistent=[bool]$ok}})
$p8=@($sr|Where-Object{-not$_.expected_status_consistent}).Count-eq 0

$groups=[ordered]@{
 group_1_structural=[ordered]@{pass=$p1;output=$struct}
 group_2_inventory_manifest=[ordered]@{pass=$p2;missing_from_inputs=$group2MissingFromInputs;undeclared_inputs=$extra;duplicate_declarations=$dupes;unhashed_entries=$unhashed}
 group_3_paired_manifests=[ordered]@{pass=$p3;template_count=($mf.Count-$instances.Count);instance_count=$instances.Count;instances=$ir}
 group_4_selection_language=[ordered]@{pass=$p4;reviewed_hits=$selectionReview}
 group_5_candidate_native_leakage=[ordered]@{pass=$p5;reviewed_hits=$leakReview}
 group_6_evidence_verbs=[ordered]@{pass=$p6;reviewed_hits=$evidenceReview}
 group_7_provenance=[ordered]@{pass=$p7;classified_observations=$pr;rule='Claims lacking complete observation provenance remain executor-reported or pending and receive no evidence credit.'}
 group_8_status_consistency=[ordered]@{pass=$p8;records=$sr}
}
$ok=@($groups.Values|ForEach-Object{$_.pass})-notcontains $false
$result=[ordered]@{check_scope=$package;timestamp=[DateTimeOffset]::Now.ToString('o');inventory=[ordered]@{serialization='UTF-8 without BOM: relative-name + U+0009 + lowercase SHA-256 + U+000A; final U+000A included';explicit_excluded_receipt=$receiptName;other_exclusions=@();input_inventory_sha256=HashText $invText;input_entries=$inv};predicate_groups=$groups;overall_pass=$ok;overall_status=$(if($ok){'PASS_DOCUMENTARY_ONLY'}else{'FAIL'})}
$result|ConvertTo-Json -Depth 9 -Compress
if(-not$ok){exit 1}
