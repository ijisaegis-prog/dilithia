# Gate 4 — Transaction Effect and Atomicity Preparation

> **NON-NORMATIVE PREPARATION.** No transaction grammar, identity, effect
> identity, atomicity mechanism, or accepted-unsuccessful outcome is introduced.

## Frozen requirement and process status

Every validity-affecting logical dependency and canonical effect must be
explicit enough to reason about success, rejection, dependency, authorization,
arithmetic, resource, version and conflict failures, and reorganization. A
rejected candidate must leave no partial canonical transition. Source: matrix
§§8.1–8.5 and `TRANSACTION_STATE_EFFECTS_ATOMICITY_FAILURE_REQUIREMENTS.md`.
Process requirements: **SATISFIED**. Candidate evidence: **PENDING**.

## Prepared trace schema

For both candidates use one external case record containing: logical observations
and required absences; preconditions; authority relations; ordered validation
stages without prescribing implementation order; temporary local work;
candidate-visible effects; external postcondition; outcome class; canonical
commit boundary placeholder; and rollback/reapplication expectation.

Prepare cases for all dependencies present; one missing/stale/conflicting;
authorization failure early/late; arithmetic failure; malformed representation;
unsupported version; resource failure; several coupled effects; late failure
after temporary work; and rollback/reapplication.

## Oracles and blockers

Structural checks can require identical external dependencies and outcomes,
complete effect inventories, and zero canonical effects for full rejection.
Executable atomicity, differential outcomes, crash/fault injection, and commit
behavior are **REQUIRES IMPLEMENTATION**. Defining transaction/effect format,
atomicity granularity, resource-failure semantics, or any accepted-unsuccessful
canonical consequence is **REQUIRES OWNER DECISION**.

Account discloses reads, mutations, absence dependencies and cross-entity
atomicity. UTXO discloses referenced and created entities, membership/absence,
auxiliary facts and consumption/creation atomicity. The different internal
taxonomies are not scores.

Comparison scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**;
Account **NO**; UTXO **NO**; main merge **NOT DONE**.
