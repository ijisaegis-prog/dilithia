# Account/UTXO Non-Decisional Preparation Pack

> **NON-NORMATIVE PREPARATION.** This directory records research, test planning,
> evidence prerequisites, and decision boundaries only. It creates no protocol
> rule, project-owner decision, candidate preference, score, or implementation.

## Run identity

- Source freeze: `9bc137eff2723773dcbabc9088599c5007e68c19`
- Preparation run: `NONDECISIONAL-20260903_095722`
- Candidates: Minimal Account and Minimal UTXO, co-equal
- Generalized Object and active Hybrid: deferred; not selected
- Evidence rule: a source is credited only for the exact case, profiles,
  mappings, provenance, and evidence class it actually supports.

## Fixed process status

- Comparison scoring: **NOT STARTED**
- State-model ranking: **NONE**
- State-model decision: **NOT MADE**
- Account selected: **NO**
- UTXO selected: **NO**
- Main merge: **NOT DONE**

## Contents

- `CURRENT_STATUS_AND_DEPENDENCY_MAP.md`: authority, status, and cross-gate dependency map.
- `GATE1_REMAINING_EVIDENCE_PREPARATION.md`: remaining authorization evidence branches and package prerequisites.
- `GATE2_REPLAY_CANONICAL_IDENTITY_PREPARATION.md` through
  `GATE9_RESOURCE_PREPARATION.md`: frozen requirements, safe preparations,
  blockers, symmetry rules, and owner stop points.
- `CROSS_GATE_TEST_AND_EVIDENCE_MATRIX.md`: shared scenario/evidence plan and reuse rules.
- `DECISION_QUEUE_DO_NOT_AUTOSELECT.md`: protected decisions, expressed as unresolved branches.
- `AUTOMATION_BOUNDARY_REPORT.md`: completed safe automation, prohibited actions, and future backlog.

## Reading rule

“Prepared” means a neutral template, case family, oracle requirement, or
reproducibility checklist can be written now. It does **not** mean the case has
been executed or that evidence exists. Status labels used here are:

- **FROZEN REQUIREMENT**: quoted or faithfully restated from an identified reviewed source.
- **PROCESS SATISFIED**: the project-process requirements gate is satisfied.
- **PREPARED**: non-executable planning material exists in this directory.
- **PENDING**: qualifying evidence has not been located.
- **REQUIRES IMPLEMENTATION**: execution needs a candidate realization or harness not present.
- **REQUIRES OWNER DECISION**: work must stop before selecting a protected mechanism/profile.
- **BLOCKED**: a named prerequisite prevents useful execution.

## Authority and provenance

Protocol authority remains with `docs/CONSTITUTION.md`, the applicable portions
of `docs/SPECIFICATION.md`, and valid later authority under their hierarchy.
The principal comparison sources are the reviewed Gate 1–9 requirements,
`docs/ACCOUNT_UTXO_GATE1_9_COMPARISON_EVIDENCE_MATRIX.md`,
`docs/ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md`,
`docs/ACCOUNT_UTXO_WORKLOAD_MODEL.md`,
`docs/ACCOUNT_UTXO_CANDIDATE_MAPPING_REQUIREMENTS.md`, and
`docs/BENCHMARK_METHODOLOGY.md`. Gate 8 and Gate 9 satisfaction are established
by their separate satisfaction-decision records. Embedded older status wording
is historical where a later reviewed record supersedes it.

Nothing in this pack amends any source named above.
