# Account-vs-UTXO Decision Boundary Audit

**Run:** `BOUNDARY-20260903_120814`
**Source supplied by owner:** branch `analysis/account-utxo-state-model-comparison`, commit `fe094d20d9e6f13903a2033c74a289ab18605d40`
**Prior reviewed baseline Git identity:** `d7af9f06464983695bae6e7b18749dc445fa17b1`
**Exact reviewed package Git identity:** `84d617835493e0ee5e8da699263b2e1286ffd460`
**Reviewed branch tip:** `origin/automation/decision-boundary-20260903_120814`
**Current-byte binding:** the reviewed package bytes are committed at the exact identity above; a receipt-excluded SHA-256 binding is also recorded in `PACKAGE_WIDE_SAFE_CHECK_EXECUTION.md`
**Classification:** non-normative audit and documentary execution record
**Protocol adoption effect:** none

## Outcome

The prior fixed-point claim was correctly retracted by its own final integrated
package. This audit records bounded structural and documentary observations; it
does not claim a complete durable execution of every package-wide predicate.
Corrective safe work ran eight checker-labelled groups, recorded the governed
inventory digest, and applied bounded lexical and documentary heuristics. The
checker is not a complete implementation of all governing predicates: groups
3, 5, 6, and 8 have the limitations stated in the execution record. Its
overall result remains `FAIL` because group 2's
inventory check fails set equality: after the required
receipt exclusion, the 24 governed inputs and 24 declarations differ because
the receipt is wrongly declared while
`AUTOPILOT_EXECUTION_FINAL_DUAL_REVIEW.md` is omitted.

That defect does not select or disadvantage either candidate and does not
invalidate the narrow final dual-review verdict. It does mean the package does
not have the complete, current, content-bound package-wide receipt required by
its own schema. This run cannot repair the old manifest because the mission
forbids modifying existing tracked files.

The corrected package bytes are committed at exact reviewed clean branch tip
`84d617835493e0ee5e8da699263b2e1286ffd460`, matching
`origin/automation/decision-boundary-20260903_120814`. The receipt-excluded
content-addressed binding is a separate artifact-level integrity record. This
task does not repair the older execution package because that package is outside
the allowed write set; its manifest and receipt repair remains non-decisional
documentary work awaiting separate authorization. Further material state-model
comparison progress from the current frozen inputs requires protected inputs,
candidate-symmetric implementations, or missing evidence after applicable
method prerequisites are frozen.

## Documents

- `CURRENT_FIXED_POINT_AUDIT.md` — re-evaluation and current stopping boundary
- `NEXT_SAFE_WORK_EXECUTION.md` — exact task dispositions and executed checks
- `PROTECTED_DECISION_QUEUE.md` — questions reserved for owner/protocol authority
- `DECISION_PREREQUISITE_MATRIX.md` — evidence needed before each decision
- `DECISION_DEPENDENCY_ORDER.md` — dependency order among questions, not answers
- `OWNER_APPROVAL_BOUNDARY.md` — precise authority handoff
- `EVIDENCE_GAPS_AND_BLOCKERS.md` — absent evidence and exact blockers
- `PACKAGE_INVENTORY_PROVENANCE_DEFECT_RECORD.md` — current-run inventory and
  non-normative superseding defect record
- `FINAL_NONDECISIONAL_STATUS.md` — final locked process status

## Status lock

- Comparison scoring: **NOT STARTED**
- State-model ranking: **NONE**
- State-model decision: **NOT MADE**
- Account selected: **NO**
- UTXO selected: **NO**
- Main merge: **NOT DONE**
