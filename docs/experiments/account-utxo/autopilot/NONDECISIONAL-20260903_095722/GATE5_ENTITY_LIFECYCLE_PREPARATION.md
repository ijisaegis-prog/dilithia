# Gate 5 — Entity Lifecycle Preparation

> **NON-NORMATIVE PREPARATION.** Logical lifecycle is kept separate from physical
> retention; no entity representation, deletion, recreation, or pruning rule is chosen.

## Frozen requirement and status

Both candidates must distinguish protocol-semantic lifecycle from physical data
retention and define, where relevant: never existed, currently exists, currently
absent, previously existed, deleted, consumed, recreated, reverted, migrated,
and historical-only. Current absence is not automatically nonexistence; pruning
must not redefine lifecycle truth. Source: matrix §§9.1–9.5 and
`ENTITY_LIFECYCLE_HISTORICAL_MEANING_REQUIREMENTS.md`. Process requirements:
**SATISFIED**. Candidate evidence: **PENDING**.

## Prepared lifecycle suite

Create one external state-machine template with version-relative state labels,
transition preconditions, replay consequences, authority requirements,
historical claim, retained-current facts, optional support data, and rollback
edge. Instantiate paired cases for first/repeated creation, current existence,
transition to non-current, deletion/consumption, recreation, zero/empty/absence,
migration, reorganization, pruning, snapshot restoration, and historical query.

## Testable now and blockers

Tables can be checked now for distinct lifecycle labels, no pruning/deletion
conflation, no accidental replay reset, and identical external cases. Executable
creation/recreation, migration, pruning/snapshot, historical query, and rollback
tests are **REQUIRES IMPLEMENTATION**. Choosing identity persistence, recreation
permission, tombstones, retention, pruning, archive duties, or migration
continuity is **REQUIRES OWNER DECISION**.

Account must disclose zero/absence/deletion and inherited replay/authorization
history. UTXO must disclose creation, currentness, consumption, possible
reference recreation, dormancy, and history. Neither vocabulary is the neutral
standard. Dependencies: Gates 1–4, Gate 8 history/snapshot, Gate 9 growth.

Scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**; Account **NO**;
UTXO **NO**; main merge **NOT DONE**.
