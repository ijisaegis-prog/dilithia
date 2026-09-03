# Gate 2 — Replay and Canonical Identity Preparation

> **NON-NORMATIVE PREPARATION.** No identity, nonce, sequence, reference, or
> replay mechanism is selected.

## Frozen requirement and process status

An exercised spendable effect must not become spendable again merely through
presentation of the same or an equivalent candidate while the earlier exercise
remains authoritative. Both candidates must distinguish replay, duplicate
presentation, conflict, reapplication after reverted history, current validity,
historical validity, and the canonical identity relevant to each. Semantic
identity is not assumed to be a hash. Source: matrix §§6.1–6.5 and
`REPLAY_CANONICAL_IDENTITY_REQUIREMENTS.md`. Requirement/process level:
**SATISFIED**. Candidate evidence: **PENDING**.

## Prepared common vectors

- exact duplicate; semantically equivalent but byte-distinct duplicate;
- conflicting effects; missing state; deletion/consumption; recreation;
- zero/empty/absence distinctions; authorization or protocol-version change;
- cross-domain presentation; rollback and reapplication; historical analysis;
- bootstrap from untrusted state.

Each vector template records prior authoritative state, version/domain context,
external effect identity placeholder, candidate set, ordering context, expected
classification, expected canonical effect, rollback state, and historical
interpretation. Alternative identity profiles receive different identifiers.

## Testable now versus blocked

Structural tests can verify that every vector distinguishes byte identity from
semantic identity and replay from conflict/reapplication, and that both mappings
receive identical external facts. Collision/alias behavior, lifecycle replay,
historical interpretation, and executable reorganization tests are **REQUIRES
IMPLEMENTATION**. Choosing transaction/output/effect identity, nonce/sequence,
consumption encoding, domain representation, or canonical ordering is
**REQUIRES OWNER DECISION**.

## Dependencies and symmetry

Consumes Gate-1 authority scope, Gate-4 effects/outcomes, Gate-5 lifecycle,
Gate-7 ordering, and Gate-8 history/state claims. Account must disclose any
sequence-like or deletion/recreation facts; UTXO must disclose reference
unambiguity, aliasing, consumption/currentness, and grouping. Neither disclosure
is a neutral cost or preferred mechanism.

Required future evidence: replay invariants, alias/collision analysis, lifecycle
and reorganization traces, version traces, and independently reproduced
historical interpretation. All remain **PENDING**.

Comparison scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**;
Account **NO**; UTXO **NO**; main merge **NOT DONE**.
