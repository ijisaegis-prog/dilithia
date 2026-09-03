# Decision Dependency Order

This orders decision **questions** by logical dependency. It does not order,
score, or recommend their answer choices. Parallel questions on the same level
remain co-equal.

## Level 0 — Authority and comparison governance

- PDQ-14: determine whether a normative change is proposed and the applicable
  authority process.
- PDQ-15: declare the future evidence-to-decision method and adoption authority.
- PDQ-01: confirm the candidate scope to which that method will apply.

These questions define process and scope. They do not choose a candidate.

## Level 1 — Shared semantic foundations

- PDQ-02: transaction, identity, effects, dependencies, and outcomes.
- PDQ-04: ownership, authority, credentials, signing coverage, and recovery.
- PDQ-10: consensus history, fork choice, finality, and reorganization context.
- PDQ-11: governance and activation framework.

Without these, later mechanisms lack stable meaning or authority.

## Level 2 — Dependent protocol semantics

- PDQ-03 depends on PDQ-02, PDQ-04, and the history context in PDQ-10.
- PDQ-05 depends on authorization/signing requirements in PDQ-04 and versioning
  contexts from PDQ-02/PDQ-11.
- PDQ-08 depends on transaction/effect semantics in PDQ-02 and canonical block
  context in PDQ-10.
- PDQ-12 depends on PDQ-03 through PDQ-06, PDQ-10, and PDQ-11 to state what is
  migrated, retained, recovered, and activated.

## Level 3 — Authenticated state and verification roles

- PDQ-06 depends on the logical state/effect subjects in PDQ-02, ownership facts
  in PDQ-04, and cryptographic/version context in PDQ-05.
- PDQ-07 depends on PDQ-06 for claims and on PDQ-10 for canonical-history/head
  meaning; migration aspects also depend on PDQ-12.

## Level 4 — Resource and production architecture

- PDQ-09 depends on the formats/workflows in PDQ-02, crypto work in PDQ-05,
  state operations in PDQ-06, ordering/composition in PDQ-08, and block/liveness
  context in PDQ-10.
- PDQ-13 depends on PDQ-05 for artifact verification, PDQ-10 for network roles,
  and PDQ-11 for any protocol-affecting activation distinction.

## Level 5 — Evidence collection and final state-model question

Once applicable upstream questions are frozen as explicit experimental profiles,
candidate-symmetric implementations and evidence campaigns may run. Only after
their prerequisites, provenance, reviews, and sensitivity branches exist can
PDQ-01/PDQ-15 proceed from scope/method questions to an authorized state-model
decision and any later main integration.
