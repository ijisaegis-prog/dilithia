# Mechanical Structural Check Specification

> **STRUCTURAL PROCEDURE ONLY.** This procedure checks documentary shape. It
> does not compare candidate semantics, validate field truth, run candidate
> behavior, or produce Gate evidence.

## Inputs

- `NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md`
- `GATE1_PAIRED_CASE_RECORD_SKELETONS.md`
- `DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md`

The procedure records SHA-256 for the exact input bytes in its result. A later
edit invalidates that result until rerun.

This specification does not itself supply an executable implementation. A
future execution must use `PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md`, which
requires the exact checker identity and invocation plus captured stdout,
stderr, exit status, timestamps, and a bound package inventory. Predicate
descriptions alone are not execution provenance.

## Predicates

1. Schema mirror: extract backtick keys from the `Account slot` and `UTXO slot`
   tables, excluding `candidate_slot`; require identical ordered key sets.
2. Gate-1 case coverage: require exactly the 16 planned case headings and, in
   each section, one shared-contract marker, one Account slot, one UTXO slot,
   one profile-set marker, one provenance marker, and `EVIDENCE:NONE`.
3. Vector mirror: for every `VECTOR_ID:` line, require exactly one Account slot
   and one UTXO slot plus shared input, expected external relation, profile set,
   and provenance.
4. Rejection structure: every vector whose outcome is `REJECTION` must contain
   `ZERO_CANONICAL_EFFECT` in the same vector line.
5. Reorganization neutrality: every vector ID containing `REORG` must contain
   `REORG_PROFILE_BRANCH:REQUIRED_EXPLICIT_COMPATIBLE_PROFILE_UNRESOLVED`,
   `ROLLBACK_PRIOR:UNRESOLVED_PROTECTED_DECISION`, and
   `EXPECTED_EXTERNAL_RELATION:UNRESOLVED_PROTECTED_DECISION` in the same line.
   The predicate must reject an unconditional restoration or reapplication
   relation outside a later, explicitly bound compatible profile branch.
6. Gate coverage: vector records must cover every Gate from 2 through 9.
7. Template boundary: candidate slots in the vector input must be `NO_RESULT`;
   Gate-1 skeletons must contain no result other than `EVIDENCE:NONE`.

## Result vocabulary

- `PASS_STRUCTURAL`: the exact syntactic predicate passed.
- `FAIL_STRUCTURAL`: the exact syntactic predicate failed.
- `NOT_CHECKED_SEMANTICALLY`: mandatory overall boundary.

If durably executed on current inputs, passing all predicates supports only
this statement: the bound templates have
mirrored required key shapes, candidate slots, planned Gate-1 case headings,
Gate 2–9 abstract-vector coverage, and the specified rejection/reorganization
markers. It does not support substantive candidate symmetry.
