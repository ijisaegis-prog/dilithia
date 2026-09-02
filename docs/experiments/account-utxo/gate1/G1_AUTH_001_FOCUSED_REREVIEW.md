# Dilithia G1-AUTH-001 Focused Re-Review Record

**Status:** FINAL FOCUSED RE-REVIEW RECORD — PASS
**Gate:** 1 — Ownership and Authorization
**Common case:** `G1-AUTH-001-ORDINARY-NATIVE-DLTH-AUTHORIZATION`
**Review type:** Focused read-only Revision-1 re-review
**Protocol adoption authority:** NONE
**State-model decision:** NOT MADE
**Account selected:** NO
**UTXO selected:** NO
**State-model ranking:** NONE
**Comparison scoring:** NOT STARTED
**Candidate evidence generated:** NO
**Paired manifest created:** NO

---

## 1. Exact Reviewed Artifact

File:

`docs/experiments/account-utxo/gate1/G1_AUTH_001_ORDINARY_NATIVE_DLTH_AUTHORIZATION_COMMON_CASE.md`

Exact SHA-256:

`F52F3B81F47B49370F913F812829ACB1DAA8D1010E4E17AD7488B7E0BCC90AB3`

Exact raw Git blob:

`27795f874ed13164eee6e56617c8a0c025c90682`

Bytes:

`21035`

Logical lines:

`756`

Review repository state:

- branch: `analysis/account-utxo-state-model-comparison`
- parent HEAD: `386a4d2e3a1da3e3db13f95158b78ba1391c9adb`
- common case was the only untracked file at review time
- Markdown fences balanced: YES
- headings structurally restored: YES
- trailing whitespace: NONE
- repository mutations made by review: ZERO

This record applies only to the exact common-case identity above.

A content change creates a different artifact and is not covered by this PASS.

---

## 2. Prior Findings

The prior focused review returned:

**REVISION REQUIRED**

with two material findings.

### G1-AUTH-001-001

Severity:

**HIGH**

The original `V` and `E1` definitions did not sufficiently freeze one common
candidate-independent external value/effect contract.

Account and UTXO could therefore have mapped materially different external
semantic problems.

### G1-AUTH-001-002

Severity:

**MEDIUM**

An unclosed Markdown fence caused material sections after the `A1` example to
render as literal code instead of structured Markdown.

---

## 3. G1-AUTH-001-001 Resolution

Classification:

**FIXED**

Revision 1 introduces the candidate-independent external semantic contract:

`C1 = (P0, V, E1, A1, P1, M1)`

The focused re-review confirmed:

- `P0` defines a shared representation-neutral external precondition boundary
- `V` is tied to the shared `P0` and `P1` relation
- `E1` is the candidate-independent external effect
- `P1` supplies the required shared observable postcondition
- `A1` remains one neutral independently required consensus authority relation
- `M1` is restricted to external semantic or profile context
- the same content-identified `C1` must be supplied to both candidates
- a material change to `C1` creates a different paired evidence case
- material external preconditions must be instantiated once and identically
- Account and UTXO may differ internally but may not redefine the external problem

Remaining material issue:

**NONE**

Account / UTXO symmetry preserved:

**YES**

---

## 4. G1-AUTH-001-002 Resolution

Classification:

**FIXED**

The focused re-review confirmed that all common-case Markdown fences are
properly bounded.

Reviewed bounded blocks include:

- `E1 : P0 -> P1`
- `N = 1 independent required consensus authority relation`
- `C1 = (P0, V, E1, A1, P1, M1)`
- the required external authorization outcome
- exact-effect scope
- the repeated common-contract expression

Sections 3.6 through 19 are restored as actual Markdown structure.

Markdown fence structure valid:

**YES**

Section structure restored:

**YES**

Remaining material issue:

**NONE**

---

## 5. Regression Check

Results:

- N=1 neutrality: PASS — unchanged
- A1/E1 property neutrality: PASS — unchanged
- positive/negative boundary: PASS — strengthened without mechanism selection
- failure/atomicity boundary: PASS — unchanged
- replay/domain boundary: PASS — shared profile binding without mechanism selection
- cryptographic neutrality: PASS — unchanged
- candidate-output symmetry: PASS — strengthened and symmetric

Material regression:

**NONE**

---

## 6. Common-Case / Manifest Boundary

Common case sufficiently frozen:

**YES**

Paired manifest may instantiate remaining parameters:

**YES**

Common-case under-specification remains:

**NO**

Common-case over-specification introduced:

**NO**

Both candidates guaranteed the same external semantic problem:

**YES**

The common case defines semantic variables, property relations, parameter
classes, equality requirements, and candidate-neutral invariants.

The paired manifest may instantiate one shared concrete case.

It may not create different external semantics for Account and UTXO.

---

## 7. Hidden-Bias Review

Account hidden assumptions:

**NO**

UTXO hidden assumptions:

**NO**

Candidate-native metric leakage:

**NO**

The focused re-review found no requirement for Account balance semantics,
Account nonce, stable Account identity, one persistent Account record, UTXO
input/output grammar, UTXO one-use semantics, mandatory UTXO consumption, one
signature per input, one source, one recipient, one credential, one
authorization artifact, or one state object.

---

## 8. Accidental Selection Audit

No effective selection was found for:

- Account
- UTXO
- hybrid state model
- ownership representation
- stable logical identity
- owner identity
- address role
- credential format
- key format
- signature format
- proof format
- cryptographic primitive
- PQ primitive
- authorization descriptor
- authorization grouping
- multisignature
- threshold authorization
- delegation
- recovery
- alternate authority
- key rotation
- migration mechanism
- transaction format
- transaction identifier
- effect identifier
- input structure
- output structure
- Account nonce
- sequence counter
- UTXO one-use encoding
- state schema
- state key
- state commitment
- proof system
- snapshot trust model
- light-client protocol
- resource meter
- gas
- fee mechanism
- storage rent
- refund mechanism
- numeric resource limit
- consensus algorithm
- fork-choice rule
- finality rule
- scheduler
- parallel-execution mechanism
- state-model ranking

Every item above:

**NO**

---

## 9. Overconstraints

Material overconstraints:

**NONE**

Revision 1 does not require:

- `P0` or `P1` to be candidate state
- `V` to be one candidate-native object
- `E1` to equal one internal state mutation
- `A1` to equal one credential
- one authorization artifact
- one cryptographic verification
- one state access
- identical candidate-native structure
- identical candidate-native counts
- identical storage layout
- a particular replay mechanism
- a particular cryptographic profile
- a particular transition grammar

---

## 10. Remaining Parameters

Common-case defects:

**NONE**

Legitimate paired-manifest parameters remain:

- concrete case-local `P0`
- concrete external `V` semantics
- concrete external `E1`
- external `A1` scope
- required external `P1`
- effect magnitude/value relation where material
- domain and protocol-version profile
- authorization/cryptographic profile
- lifecycle/admissibility disposition
- replay/history disposition
- conflict/order disposition where material
- supply/conservation disposition
- unchanged external facts
- excluded and deferred negative scenarios
- eventual Account and UTXO mapping artifact identities

These parameters must be instantiated identically for both candidates.

---

## 11. Paired-Manifest Requirement Sufficiency

Verdict:

**SUFFICIENT**

The reviewed common case requires the paired manifest to content-identify the
material shared case fields needed to prevent semantic drift between Account and
UTXO.

The manifest may instantiate parameters.

It may not redefine the common-case semantics.

---

## 12. New Findings

BLOCKER:

**NONE**

HIGH:

**NONE**

MEDIUM:

**NONE**

LOW / EDITORIAL:

**NONE**

---

## 13. Manifest-Creation Readiness

Ready to create paired manifest for G1-AUTH-001:

**YES**

Ready to create Account mapping now:

**NO**

Ready to create UTXO mapping now:

**NO**

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

## 14. Overall Verdict

**PASS**

G1-AUTH-001-001 is fixed.

G1-AUTH-001-002 is fixed.

No material regression exists.

The exact reviewed common case is ready for paired-manifest creation.

---

## 15. Process Status

This review record is evidence tied to the exact reviewed common-case SHA-256
and raw Git blob.

The common-case file itself is not modified after review merely to update its
embedded review-status wording, because doing so would create a new artifact
identity requiring another review.

For project-process purposes this record establishes:

G1-AUTH-001 COMMON CASE: REVIEWED — PASS

PAIRED-MANIFEST CREATION: AUTHORIZED

ACCOUNT MAPPING: NOT AUTHORIZED YET

UTXO MAPPING: NOT AUTHORIZED YET

COMPARISON SCORING: NOT STARTED

STATE MODEL DECISION: NOT MADE

ACCOUNT SELECTED: NO

UTXO SELECTED: NO

STATE-MODEL RANKING: NONE

**STATE MODEL DECISION: NOT MADE**