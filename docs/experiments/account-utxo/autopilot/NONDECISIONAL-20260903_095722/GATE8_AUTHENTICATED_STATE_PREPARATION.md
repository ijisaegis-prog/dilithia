# Gate 8 — Authenticated State Preparation

> **NON-NORMATIVE PREPARATION.** Gate 8 is process-satisfied. No commitment,
> hash, proof, snapshot, bootstrap, synchronization, or light-client mechanism is selected.

## Frozen requirement and status

Where validity depends on them, both candidates must support deterministic
interpretation of current membership and absence; ownership, replay, lifecycle,
native-value, crypto-version and resource facts; required historical claims;
and canonical-history/version binding. Zero, empty, absent, removed, consumed,
and invalid are not accidentally conflated. Commitment-dependent evidence uses
an explicitly frozen common profile. Source: matrix §§12.1–12.5,
`AUTHENTICATED_STATE_MEMBERSHIP_ABSENCE_SNAPSHOT_LIGHT_CLIENT_REQUIREMENTS.md`,
and `GATE8_SATISFACTION_DECISION.md`. Gate 8: **SATISFIED** at project-process
level. Candidate evidence: **PENDING**.

## Prepared claim/profile templates

Record logical state subject, membership/absence claim, construction/version
identity placeholder, scale/population, update capability, proof target,
hostile-input profile, snapshot completeness, bootstrap/trust assumptions,
sync success claim, current/history retention, reorganization context,
light-client claim class, and cryptographic assumptions.

Prepare cases for valid membership/required absence; false membership/absence;
zero/current/history ambiguity; failed update; corrupted/omitted/duplicated/
mixed-version/noncanonical snapshot; post-snapshot reorganization; head
substitution; migration/deprecation; and catastrophic assumption failure.

## Testability and blockers

Logical fact inventories and claim completeness can be reviewed now. Proof
generation/verification, exact sizes, update workloads, snapshot reconstruction,
sync, history binding, and light-client validation are **BLOCKED / REQUIRES
IMPLEMENTATION** until a separately authorized experimental construction and
claim profile exist. Construction, trust/bootstrap, completeness policy,
history retention, and light-client purpose are **REQUIRES OWNER DECISION**.

Both candidates receive identical external claims and scale/trust/history
profiles. Candidate state subjects may differ internally; their record/proof
counts are not neutral scores. Gate 8 consumes Gates 1–7 semantics and feeds
Gate 9 resource exposure.

Scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**; Account **NO**;
UTXO **NO**; main merge **NOT DONE**.
