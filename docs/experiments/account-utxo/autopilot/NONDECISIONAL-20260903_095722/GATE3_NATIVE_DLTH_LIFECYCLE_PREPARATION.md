# Gate 3 — Native DLTH Lifecycle and Conservation Preparation

> **NON-NORMATIVE PREPARATION.** No issuance, burn, fee, reward, rent, migration,
> or state representation is chosen.

## Frozen requirement and process status

Every native-DLTH-affecting path must admit an auditable conservation argument
covering success, rejection, partial-failure attempts, protocol-authorized
creation/destruction, transfer, lifecycle, migration, reorganization, and
historical interpretation. No representation may introduce unstated issuance,
hidden loss, duplicate spendability, or ambiguity. Source: matrix §§7.1–7.5 and
`NATIVE_DLTH_LIFECYCLE_CONSERVATION_REQUIREMENTS.md`. Process requirements:
**SATISFIED**. Candidate comparison evidence: **PENDING**.

## Prepared inventory schema and cases

For each case record pre-value facts, authorized creation/destruction terms,
debits/consumed sources, credits/created results, post-value facts, arithmetic
domain, outcome class, canonical effect, rollback inverse, version, and history.
Prepare paired cases for ordinary transfer, several sources, several recipients,
exact balance, arithmetic failure, missing source, authorization failure,
late failure, conflict, reorganization, migration, and recreation attempt.

The neutral oracle is an equation over external native-value facts, with every
authorized supply-changing term named. It is not an Account balance equation or
UTXO record-count equation.

## Testable now, blockers, and dependencies

The current Gate-1 positive case permits only a scoped paper arithmetic check;
it is not direct Gate-3 evidence. Inventory completeness and equation templates
can be linted now. Complete transition enumeration, machine-checkable invariants,
failure execution, and rollback/reapplication are **REQUIRES IMPLEMENTATION**.
Supply-changing authorities and transaction/effect semantics are **REQUIRES
OWNER DECISION** before concrete execution.

Gate 3 depends on Gate 1 authorization, Gate 2 replay, Gate 4 outcome/effect
inventory, and Gate 5 lifecycle. Account and UTXO must receive identical
external value pre/postconditions; candidate-native mutations or record counts
remain descriptive only.

Comparison scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**;
Account **NO**; UTXO **NO**; main merge **NOT DONE**.
