# Gate 1 Deferred Case Manifests

All cases share one external contract: the same prior semantic facts, authority
relations, proposed effect, outcome vocabulary, unchanged facts, and selected
profile branch are supplied to both candidates. Candidate mappings must disclose
internal dependencies/effects separately. `UNRESOLVED` never means a default.

Every row is a case-summary stub, not an instantiated manifest and not executed
evidence. The rows do not bind shared semantic inputs, content identities, or
separate Account and UTXO mapping slots.

| Case | Input variation | External oracle / required observation | Required profile fields | Execution state |
|---|---|---|---|---|
| G1-N01 | Malformed authorization evidence | Deterministic rejection; zero canonical effect; bounded staging obligation | artifact profile unresolved | PENDING / REQUIRES IMPLEMENTATION |
| G1-N02 | Well-formed invalid evidence | Exact external failure class; zero partial effect | validator/crypto profile unresolved | PENDING / REQUIRES IMPLEMENTATION |
| G1-N03 | Valid evidence for other effect, purpose, domain, or version | Scope mismatch; no proposed effect | purpose/domain/version branches | PENDING / REQUIRES IMPLEMENTATION |
| G1-N04 | Missing, stale, or conflicting authorization state | Dependency named; success not inferred; zero effect on rejection | lifecycle/order profile unresolved | PENDING / REQUIRES IMPLEMENTATION |
| G1-N05 | Many expensive invalid artifacts | Attempted-work and staging trace; finite symbolic bound required | workload/artifact profile unresolved | PENDING / REQUIRES IMPLEMENTATION |
| G1-M01 | Planned deprecation, owner participation required | Eligibility, handoff, acceptance, history stated | participation=required; other axes unresolved | PENDING |
| G1-M02 | Planned deprecation, owner participation not assumed | Achievable guarantee or explicit impossibility; no confiscation inference | participation=not-assumed | PENDING |
| G1-M03 | Legacy currently accepted | Current and historical interpretations remain distinct | acceptance=legacy-accepted | PENDING |
| G1-M04 | Legacy currently rejected | Dormant value/state consequence recorded; no recovery or confiscation claim | acceptance=legacy-rejected | PENDING |
| G1-M05 | Retrofit information pre-exists failure | Exact independent prior distinguishing information must be identified | retrofit=available, identity unresolved | PENDING |
| G1-M06 | No retrofit basis or independent prior information | No recovery advantage claim; limitation symmetric | retrofit=unavailable | PENDING |
| G1-C01 | Distinguishable credential compromise | Competing legitimate/forged effects evaluated only under declared authority facts | distinguishability profile unresolved | PENDING |
| G1-C02 | Catastrophic forgeability with distinguishability | Only profile-supported guarantee; no automatic recovery | distinguishable catastrophe | PENDING |
| G1-C03 | Catastrophic forgeability without distinguishability | Information-theoretic limitation recorded identically | indistinguishable catastrophe | PENDING |
| G1-H01 | Historical validation after retirement | Version-relative historical interpretation; no current-acceptance inference | historical policy/corpus unresolved | PENDING |
| G1-R01 | Rollback then reapplication | Restore prior authorization/replay facts before reevaluation | canonical history/order unresolved | PENDING |

## Per-case record schema

An instantiated manifest must bind: case hash; governing blobs; external P0/E/P1
and unchanged facts; success/rejection/explicitly-defined accepted-unsuccessful
class; authority coverage; guarantee; owner participation; current acceptance;
dormancy; historical validation; retrofit availability and exact independent
prior information; protocol/crypto version; replay/lifecycle/domain/order
context; both mapping identities; harness/oracle/commands/raw outputs/review;
and evidence class. A material profile change creates a new evidence identity.
This schema is a requirement, not evidence that any row above satisfies it.
