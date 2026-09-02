# Dilithia G1-AUTH-001 Paired Manifest Focused Re-Review Record

**Status:** FINAL FOCUSED RE-REVIEW — PASS
**Gate:** 1 — Ownership and Authorization
**Manifest alias:** `G1-AUTH-001-PAIR-001`
**Review target:** Paired Manifest Revision 1
**Review type:** Focused read-only adversarial re-review
**Protocol adoption authority:** NONE
**State-model decision:** NOT MADE
**Account selected:** NO
**UTXO selected:** NO
**State-model ranking:** NONE
**Comparison scoring:** NOT STARTED
**Candidate evidence generated:** NO

---

## 1. Exact Reviewed Manifest

File:

`docs/experiments/account-utxo/gate1/G1_AUTH_001_PAIRED_MANIFEST.md`

Exact SHA-256:

`3456D4AB164DA7C6B4CB05282E2A14EE884187C374DA0996805AA44940E4555C`

Exact raw Git blob:

`6bd8a59e640839d3a4402f6f67a5aa5cf45c6409`

Bytes:

`25064`

Logical lines:

`917`

Review repository state:

- branch: `analysis/account-utxo-state-model-comparison`
- parent HEAD: `bf1a84d29539bc02e2f3b2987d56c614c066564b`
- worktree contained exactly the untracked paired manifest
- trailing whitespace: NONE
- Markdown structure: VALID
- repository mutations made by review: ZERO

This PASS applies only to the exact manifest identity above.

Any byte change creates a different manifest and is not covered by this review.

---

## 2. Prior Review Result

The first-draft focused review returned:

**REVISION REQUIRED**

with exactly three material findings:

- `G1-AUTH-001-H-01` — HIGH
- `G1-AUTH-001-H-02` — HIGH
- `G1-AUTH-001-M-01` — MEDIUM

Prior BLOCKER findings:

**NONE**

Prior LOW / EDITORIAL findings:

**NONE**

---

## 3. G1-AUTH-001-H-01

Classification:

**FIXED**

Prior defect:

The first draft allowed later candidate mapping evidence to weaken the frozen
unchanged-external-facts requirement.

Revision 1 now requires:

- candidate inability does not modify a frozen component;
- inability remains visible as a mapping limitation, unsupported mapping, or
  failure;
- the unchanged-facts requirement is frozen before mapping;
- a candidate may not weaken, remove, reinterpret, or retroactively narrow it;
- inability to preserve the requirement remains visible;
- changing the requirement creates a new content-identified paired case;
- candidate-specific internal housekeeping must be disclosed and justified as
  having no independently meaningful external effect.

Focused re-review result:

Escape hatch removed:

**YES**

Candidate inability remains visible:

**YES**

Shared case cannot be weakened after mapping:

**YES**

Remaining material issue:

**NONE**

---

## 4. G1-AUTH-001-H-02

Classification:

**FIXED**

Prior defect:

The first draft intended to insert later Account and UTXO mapping SHA/blob
identities into the reviewed manifest itself.

That would have changed the manifest identity and created a circular provenance
dependency.

Revision 1 now requires:

- no mutable mapping-identity slots exist in the manifest;
- after PASS, manifest bytes, SHA-256, and raw Git blob remain immutable for
  evidence under this identity;
- later Account and UTXO mappings each reference this frozen manifest identity;
- mapping artifacts are not written back into this manifest;
- later mapping identities are recorded in a separate binding, index, or
  evidence-layer artifact;
- that later artifact is process/evidence provenance only;
- no protocol identity mechanism is selected.

Focused re-review result:

Manifest remains byte-immutable:

**YES**

Mutable mapping slots removed:

**YES**

Separate binding/index layer resolves later identities:

**YES**

Circular dependency removed:

**YES**

Remaining material issue:

**NONE**

The dependency is:

frozen manifest -> candidate mappings -> separate binding/index artifact

This dependency is acyclic.

---

## 5. G1-AUTH-001-M-01

Classification:

**FIXED**

Prior defect:

The first draft did not explicitly state the property-level treatment of
consensus-visible fee/resource-economic side effects.

Revision 1 adds:

`G1-AUTH-ECON-EXCLUDED-001`

The focused re-review confirmed:

- the paired case does not evaluate or compare consensus-visible
  fee/resource-economic side effects;
- no zero-fee assumption exists;
- no assumption that economic effects are absent exists;
- no assumption that such effects are unchanged exists;
- excluded economic effects cannot alter a frozen `C1-PAIR-001` field while
  preserving success;
- mappings must disclose required consensus-visible economic effects;
- mappings must disclose whether such effects touch a frozen field;
- if a required economic effect changes a frozen field, the candidate cannot
  satisfy this exact case by reinterpretation;
- another content-identified economic-bearing paired case remains possible;
- Gate-9 and Article-11 requirements are not globally waived.

Economic-side-effect disposition explicit:

**YES**

No zero-fee assumption:

**YES**

No fee mechanism selected:

**YES**

Frozen C1 components protected:

**YES**

Candidate disclosure required:

**YES**

Remaining material issue:

**NONE**

---

## 6. M1 Completeness

Complete for this paired case:

**YES**

Missing material parameter:

**NONE**

`M1-001` includes:

- authorization profile;
- domain/version profile;
- lifecycle profile;
- replay/history profile;
- conflict/order profile;
- supply/conservation disposition;
- economic-side-effect disposition;
- unchanged-facts disposition;
- negative-case deferrals;
- scoring disabled; and
- ranking none.

---

## 7. Regression Check

X/Y neutrality:

**UNCHANGED**

Value granularity / source multiplicity:

**UNCHANGED**

E1/A1 neutrality:

**UNCHANGED**

Authorization profile:

**UNCHANGED**

Domain/version:

**UNCHANGED**

Lifecycle/history/conflict:

**UNCHANGED**

Supply/conservation:

**UNCHANGED**

P1:

**UNCHANGED**

Candidate symmetry:

**UNCHANGED**

Metric neutrality:

**UNCHANGED**

Material regression:

**NONE**

---

## 8. Accidental Selection Audit

The focused re-review found no selection of:

- Account;
- UTXO;
- hybrid;
- ownership representation;
- stable logical identity;
- owner identity;
- address role;
- sender role;
- recipient role;
- transfer transaction grammar;
- transaction format;
- transaction identifier;
- effect identifier;
- input structure;
- output structure;
- Account nonce;
- sequence counter;
- UTXO one-use encoding;
- replay mechanism;
- domain encoding;
- credential format;
- key format;
- signature format;
- proof format;
- cryptographic primitive;
- PQ primitive;
- authorization descriptor;
- authorization grouping;
- multisignature;
- threshold authorization;
- delegation;
- recovery;
- alternate authority;
- key rotation;
- migration mechanism;
- state schema;
- state key;
- state commitment;
- proof system;
- snapshot trust model;
- light-client protocol;
- resource meter;
- gas;
- fee mechanism;
- fee payer;
- fee recipient;
- producer compensation rule;
- storage rent;
- storage charge;
- refund mechanism;
- pricing function;
- numeric resource limit;
- consensus algorithm;
- fork-choice rule;
- finality rule;
- scheduler;
- parallel execution; or
- state-model ranking.

Every item above:

**NO**

---

## 9. New Findings

BLOCKER:

**NONE**

HIGH:

**NONE**

MEDIUM:

**NONE**

LOW / EDITORIAL:

**NONE**

---

## 10. Freeze / Mapping Readiness

Ready to freeze exact Revision-1 manifest:

**YES**

Manifest may remain byte-immutable after freeze:

**YES**

Ready to create Account mapping after freeze:

**YES**

Ready to create UTXO mapping after freeze:

**YES**

Separate mapping-identity binding artifact needed later:

**YES**

Candidate evidence generated:

**NO**

Comparison scoring started:

**NO**

State-model ranking exists:

**NO**

State-model decision made:

**NO**

Account selected:

**NO**

UTXO selected:

**NO**

---

## 11. Overall Verdict

**PASS**

`G1-AUTH-001-H-01` is fixed.

`G1-AUTH-001-H-02` is fixed.

`G1-AUTH-001-M-01` is fixed.

No material regression exists.

The exact Revision-1 manifest may now be frozen.

Account and UTXO mapping creation may begin only after that freeze.

---

## 12. Process Status

For project-process purposes, this separate review record establishes:

G1-AUTH-001 PAIRED MANIFEST: REVIEWED — PASS

G1-AUTH-001 PAIRED MANIFEST FREEZE: AUTHORIZED

ACCOUNT MAPPING CREATION AFTER FREEZE: AUTHORIZED

UTXO MAPPING CREATION AFTER FREEZE: AUTHORIZED

CANDIDATE EVIDENCE GENERATED: NO

COMPARISON SCORING: NOT STARTED

STATE-MODEL RANKING: NONE

STATE MODEL DECISION: NOT MADE

ACCOUNT SELECTED: NO

UTXO SELECTED: NO

The reviewed manifest itself must not be modified merely to update its embedded
review-status wording.

Its exact reviewed byte identity remains the evidence anchor.

**STATE MODEL DECISION: NOT MADE**