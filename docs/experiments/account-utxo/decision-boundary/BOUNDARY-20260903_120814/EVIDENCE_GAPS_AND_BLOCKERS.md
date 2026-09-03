# Evidence Gaps and Blockers

## Documentary integrity

| Gap | Exact blocker | Consequence |
|---|---|---|
| Execution-package inventory is not exact | after required receipt exclusion, the 24 inputs and 24 declarations differ: receipt wrongly declared; final dual review omitted | package-wide receipt predicate fails |
| Old structural PASS record is stale | current input hashes differ from hashes in the invalidated record | only this run's narrow rerun may be cited |
| No durable old-package receipt | checker identity/streams/status/timestamps/complete inventory were not durably bound | no package-wide PASS may be claimed |
| Existing package cannot be repaired here | this task's allowed writes exclude the older execution package | safe, non-decisional correction remains awaiting a separately authorized run; no protected protocol input is prerequisite |

## Candidate evidence

| Gate/work family | Missing evidence | Exact blocker |
|---|---|---|
| Gate 1 | executable malformed, invalid, wrong-scope, migration, catastrophe, historical, and reorg paired results | implementations/harnesses absent; several profiles protected; current evidence manifest requires provenance-complete rerun |
| Gate 2 | duplicate/alias/collision/replay/reorg/history results | semantic identity, replay/currentness, order and implementations absent |
| Gate 3 | complete conservation evidence across every supply-changing transition | transition/effect inventory and executable models absent |
| Gate 4 | dependency/effect/failure-atomicity/fault results | transaction semantics, implementations, fault oracle absent |
| Gate 5 | lifecycle/recreation/deletion/consumption/pruning/history results | lifecycle and retention meanings protected; implementations absent |
| Gate 6 | concrete PQ artifact/operation counts and hostile validation results | authorization/crypto architecture and implementations absent |
| Gate 7 | conflict/order/schedule permutation results | external conflict/order contexts and paired runners absent |
| Gate 8 | commitment/proof/snapshot/sync/light-client results | constructions, claim/trust profiles and implementations absent |
| Gate 9 | deterministic counts, growth, invalid-work, amplification and performance results | resource semantics, methods, corpora, implementations, environments and raw data absent |
| Cross-gate | substantive Account/UTXO symmetry | mirrored empty slots cannot establish semantic equivalence |

## Evidence classes explicitly absent

- Production or experimental candidate state-model implementation evidence
- Formal proofs for candidate transitions and invariants
- Decision-ready quantitative or performance evidence
- Independent candidate implementation and reproduction evidence
- Bound historical corpus/interpreter/results
- Registered benchmark campaign and raw samples
- Complete Gate 1–9 paired evidence matrix

## Non-evidence that must not fill the gaps

Templates, schemas, matrices, structural arithmetic, label counts, lint passes,
source searches, code-health checks, reviews of empty slots, and executor-reported
history do not substitute for candidate behavior or quantitative evidence.

## Blocker ownership

- Protected semantics: owner/protocol authority, enumerated in
  `PROTECTED_DECISION_QUEUE.md`.
- Implementation: separately authorized candidate-symmetric engineering work.
- Method: profile/corpus/oracle/measurement freeze and pre-result registration.
- Evidence: actual execution, retained raw artifacts, provenance, review, and
  where required independent reproduction.
- Documentary correction: safe, non-decisional work remains to repair the
  existing execution manifest and issue a fresh receipt. Its prerequisite is a
  run authorized to modify that package, not a protected semantic, candidate,
  method, or evidence decision.
