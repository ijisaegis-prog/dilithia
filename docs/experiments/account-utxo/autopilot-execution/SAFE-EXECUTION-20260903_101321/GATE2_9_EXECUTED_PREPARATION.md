# Gates 2-9 Preparation Summary

| Gate | Described structural obligation | Documentary observation | Artifact/evidence still absent |
|---|---|---|---|
| 2 Replay/identity | Vector schema binds prior authority, version/domain, semantic identity placeholder, candidate set, order, classification, effect, rollback, history | Byte identity is explicitly distinct from semantic identity; replay distinct from conflict and reapplication | executable alias/collision/reorg/history results |
| 3 Conservation | Inventory schema binds pre/post native value, named creation/destruction, debit/credit, outcome, canonical effect, inverse, version/history | Neutral equation uses external value, not Account balance or UTXO count; rejected path requires zero canonical delta | complete transition inventory and machine execution |
| 4 Effects/atomicity | Trace schema separates observations, absences, dependencies, local work, canonical effects, outcome, commit placeholder, rollback | Rejection-zero-effect obligation and full effect inventory are explicit | failure/fault/commit execution |
| 5 Lifecycle | State-label template distinguishes never/current/absent/previous/deleted/consumed/recreated/reverted/migrated/historical-only | No pruning=deletion or absence=never-existed conflation; replay history retained as dependency | executable lifecycle/history/pruning suite |
| 6 PQ authorization | Symbolic schema separates authority graph, coverage, credentials, artifacts, operations, reuse/cache, versions, batch/aggregation | No one-account/one-signature or one-input/one-signature stereotype; common capabilities symmetric | concrete architecture, artifacts, counts, timings |
| 7 Conflict/order | External semantic conflict graph precedes candidate mapping; order context distinct from schedule | Commutativity, protocol outcome, schedule, and performance remain separate | frozen order context and schedule execution |
| 8 Authenticated state | Claim template separates membership/absence/update/history/snapshot/trust/sync/light-client claims | Zero/empty/absent/removed/consumed/invalid distinctions preserved | construction, proofs, hostile snapshots, sync results |
| 9 Resources | Logical taxonomy covers accepted/rejected work, access/absence/mutation, gross/net effects, persistence, composition and producer/verifier amplification | Candidate-native counters remain descriptive; formulas require finite symbolic inputs and rejected paths require zero canonical effect | schemas, meters, implementations, corpus, hardware, raw measurements |

These rows remain summaries, not schemas, vectors, results, or evidence. The
separate `NEUTRAL_PAIRED_EVIDENCE_RECORD_SCHEMA.md` and
`DETERMINISTIC_ABSTRACT_VECTOR_TEMPLATES.md` instantiate the neutral templates;
`MECHANICAL_STRUCTURAL_CHECK_RESULTS.md` preserves a withdrawn historical
check report; no replacement documentary-shape check exists. No candidate
behavior or substantive symmetry was tested.
