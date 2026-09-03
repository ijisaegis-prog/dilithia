# Cross-Gate Test and Evidence Matrix

> **NON-NORMATIVE PREPARATION.** “Prepare” below means define a neutral case,
> schema, oracle, or manifest. It does not mean evidence was executed or obtained.

## Scenario coverage

| Scenario family | G1 | G2 | G3 | G4 | G5 | G6 | G7 | G8 | G9 | Present disposition |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| ordinary authorized value effect | X | X | X | X | X | X |  | X | X | Gate-1 positive structural draft only; direct credit ineligible |
| malformed/invalid authorization | X |  | X | X |  | X | X |  | X | **PENDING / REQUIRES IMPLEMENTATION** |
| duplicate/replay/conflict | X | X | X | X | X |  | X | X | X | **PENDING** |
| deletion/consumption/recreation |  | X | X | X | X |  | X | X | X | **PENDING** |
| zero/empty/absence/invalid |  | X | X | X | X |  |  | X | X | **PENDING** |
| early/late failure and rollback | X | X | X | X | X | X | X | X | X | **PENDING / REQUIRES IMPLEMENTATION** |
| migration/deprecation/dormancy | X | X | X | X | X | X | X | X | X | **PENDING; profiles unresolved** |
| catastrophic crypto failure | X |  |  |  |  | X |  | X | X | **PENDING; no impossible guarantee** |
| reorganization/reapplication | X | X | X | X | X | X | X | X | X | **PENDING** |
| mixed/unsupported versions | X | X | X | X | X | X | X | X | X | **PENDING** |
| schedule permutations |  | X |  | X |  |  | X |  | X | **REQUIRES IMPLEMENTATION** |
| snapshot/sync/light-client attack |  | X |  |  | X |  |  | X | X | **BLOCKED on profile/implementation** |
| persistent growth/lifecycle churn | X |  |  | X | X | X |  | X | X | **PENDING** |
| producer/verifier or nested amplification | X |  |  | X |  | X | X | X | X | **PENDING** |

`X` means the gate consumes or constrains the scenario; it is not evidence credit.

## Evidence-class matrix

| Evidence class | Minimum qualifying artifact | What it may support | What it cannot silently become |
|---|---|---|---|
| Structural | Reviewed mapping, table, trace, invariant statement, bound source identities | Coherence under stated symbolic assumptions | Execution, performance, proof, or implementation fitness |
| Executable | Frozen harness/model, vectors, oracle, raw outcomes, commands and provenance | Behavior of that exact executable profile | Formal proof or production implementation |
| Formal | Defined semantics, stated theorem, assumptions, machine/check record and tool provenance | The proved proposition only | Empirical performance or unstated cases |
| Quantitative | Frozen neutral metric, case/profile, raw values and derivation | Profile-bound counts/measurements | Representation-independent superiority |
| Benchmark/performance | Benchmark-methodology-compliant manifest, implementation/hardware/toolchain, corpus, repetitions, raw and summarized results | Physical behavior of exact campaign | Consensus truth or other platforms |
| Independent implementation | Independently produced implementations/results with compatible frozen semantics and provenance | Reproduction/differential claims | Independence merely from separate processes or files |
| Historical | Versioned inputs, authoritative historical context, interpreter and expected meaning | Exact historical interpretation | Current acceptance or another version/profile |

## Evidence-reuse eligibility

Evidence may be reused only when the proposed claim matches all material fields:

1. common semantic-case identity and external pre/postconditions;
2. expected outcome class and authority relations;
3. replay, lifecycle, domain, protocol version, conflict and ordering context;
4. migration guarantee, participation, current acceptance, dormancy, historical
   validation, retrofit and independent-prior-information branches;
5. candidate mapping identity and maturity;
6. cryptographic, authenticated-state, resource/measurement, implementation and
   hardware profiles where material;
7. source, generator/harness, corpus, commands, outputs and review provenance;
8. evidence class and claim scope.

Any material mismatch yields **NOT ELIGIBLE**, a new evidence identity, or an
explicit sensitivity comparison. Similarity is insufficient. Results cannot be
upgraded across evidence classes. Old pilot evidence remains valid only for its
own frozen pilot identity and is not current Gate credit.

## Paired manifest minimum fields

- case ID/hash and governing source identities;
- external preconditions/postconditions and unchanged facts;
- success/rejection/accepted-unsuccessful classification;
- authority and authorization coverage;
- replay/lifecycle/domain/version/history context;
- semantic conflict and canonical-order context;
- migration/capability/unresolved branches;
- crypto, authenticated-state, resource and measurement profiles;
- both candidate mapping identities;
- oracle, exclusions, status, reproducibility and review provenance.

## Symmetry checks

For every paired case mechanically check:

- one shared external contract, not two candidate-shaped problems;
- the same capability availability where it belongs to the shared case;
- equivalent unresolved branches and failure/adversarial coverage;
- candidate-specific optimization allowed only under the reviewed symmetric policy;
- candidate-native counts reported separately and never treated as neutral scores;
- missing evidence shown, never imputed from the other candidate;
- no result interpreted as a winner, weight, rank, or protocol selection.

## Safe automation order

```text
freeze neutral case -> freeze material profiles -> review both mappings
-> freeze harness/oracle/provenance plan -> execute both or mark missing
-> retain raw outputs -> independent review -> classify evidence
-> compare only eligible like-for-like claims
```

The sequence ends before scoring. Any field requiring a protected choice is
marked **REQUIRES OWNER DECISION** and may be represented by multiple profiles.

## Fixed status

Comparison scoring: **NOT STARTED**. State-model ranking: **NONE**. State-model
decision: **NOT MADE**. Account selected: **NO**. UTXO selected: **NO**. Main
merge: **NOT DONE**.
