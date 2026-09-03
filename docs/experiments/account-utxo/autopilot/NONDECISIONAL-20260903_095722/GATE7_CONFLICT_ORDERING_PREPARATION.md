# Gate 7 — Conflict and Ordering Preparation

> **NON-NORMATIVE PREPARATION.** No conflict key, canonical winner, ordering,
> scheduler, lock, dependency graph mechanism, or parallel execution is selected.

## Frozen requirement and status

For fixed authoritative prior state, protocol version, candidate set, and
canonical ordering context, compliant implementations must agree on the required
authoritative result. Semantic conflict, canonical ordering, execution schedule,
and local parallelism are distinct; parallel execution is not itself a
state-model safety requirement. Source: matrix §§11.1–11.5 and
`CANONICAL_CONFLICT_ORDERING_SCHEDULING_EQUIVALENCE_REQUIREMENTS.md`. Process
requirements: **SATISFIED**. Candidate evidence: **PENDING**.

## Prepared common suite

Use an external semantic conflict graph independent of either candidate. Cover
disjoint effects; shared dependency; partial overlap; authorization/lifecycle
change versus spend; several value sources; dependency created/removed earlier;
multiple local schedules; batch; late failure; reorganization; and crypto-version
coexistence. For each frozen canonical-order context, enumerate schedule
permutations and compare only protocol-required outcomes, not internal traces.

## Oracles, blockers, and symmetry

Structural tests can verify that external conflict precedes candidate mapping,
independent/commutative cases remain visible, and ordering-sensitive validity is
separate from performance. Serial-reference, schedule-permutation,
failure-schedule, and independent-implementation results are **REQUIRES
IMPLEMENTATION**. Selecting canonical order/winner, predeclared access sets,
conflict keys, proposer influence, speculative execution, locks, commit behavior,
or scheduler is **REQUIRES OWNER DECISION**.

Account is not presumed globally conflicting; UTXO is not presumed always local.
Candidate-realized conflict amplification is an output, not a neutral score.
Dependencies: Gates 1, 2, 4, 5, 6, and Gate 9 resource semantics.

Scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**; Account **NO**;
UTXO **NO**; main merge **NOT DONE**.
