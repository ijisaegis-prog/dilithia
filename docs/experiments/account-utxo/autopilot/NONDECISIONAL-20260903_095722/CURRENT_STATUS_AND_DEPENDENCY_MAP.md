# Current Status and Dependency Map

> **NON-NORMATIVE PREPARATION.** Facts below are attributed to the frozen source
> tree; dependency arrows are planning aids, not architecture selections.

## Current state

| Layer | Frozen status | Evidence implication |
|---|---|---|
| Constitution | Pre-Genesis Ratified Draft (Frozen) | Superior constraints remain binding. |
| Formal Specification | Early normative baseline; transactions, state, and consensus remain high-level | No missing concrete rule may be invented here. |
| Threat Model | Draft threat boundary exists | It is not amended by this pack. |
| Gates 1–9 requirements | **SATISFIED at their respective project-process gates** | Requirement readiness is not candidate evidence. |
| Neutral comparison matrix | Revision 2; focused review pending in its embedded status | Candidate evidence remains uncollected under the matrix. |
| Gate 1 common case, paired manifest, mappings, bindings | Reviewed/frozen for `G1-AUTH-001` | Supports scoped structural preparation only. |
| Gate 1 structural package | Documentary positive baseline generated; durable complete-package provenance pending | **NOT ELIGIBLE** for direct Gate-1 credit; re-execution required. |
| Older value-effect pilot | Genuine evidence for its own identity | **NOT ELIGIBLE** for direct `G1-AUTH-001` credit. |
| Gates 2–9 candidate evidence | **PENDING** | No executable, formal, quantitative, benchmark, or independent credit inferred. |

## Dependency graph

```text
Constitution + applicable Formal Specification + Threat Model
                         |
                         v
          shared semantic cases and authority relations
                         |
          +--------------+--------------+
          v                             v
 G1 authorization                 G2 replay/identity
          |                             |
          +--------------+--------------+
                         v
             G3 DLTH conservation
                         |
                         v
        G4 dependencies/effects/atomicity
                         |
                         v
              G5 lifecycle/history
                         |
          +--------------+--------------+
          v                             v
 G6 authorization/PQ profiles     G7 conflict/order equivalence
          +--------------+--------------+
                         v
        G8 authenticated-state claim profiles
                         |
                         v
       G9 resource exposure and benchmarks
                         |
                         v
     future comparable evidence (not scoring or ranking)
```

The arrows mean “consumes semantic facts from,” not “must be implemented in
this order.” Reorganization, migration, versioning, invalidity, and historical
interpretation cut across every gate.

## Critical dependency contracts

1. A Gate 2 identity experiment cannot silently choose signing bytes,
   transaction identity, or a nonce mechanism.
2. Gate 3 conservation requires a complete inventory of Gate 4 effects and
   outcome classes; rejected and accepted-unsuccessful outcomes must differ.
3. Gate 5 lifecycle meanings feed replay, authorization, commitment, and growth.
4. Gate 6 counts require explicit authorization architecture and crypto profiles;
   state-model labels do not supply them.
5. Gate 7 requires an external semantic conflict relation and canonical ordering
   context before comparing candidate-realized footprints.
6. Gate 8 proof/byte/work claims require one frozen authenticated-state profile.
7. Gate 9 physical measurements require frozen cases, mappings, implementations,
   hardware, toolchain, and benchmark provenance.

## Evidence maturity ladder

Structural prose or tables do not become executable evidence. Executable model
results do not become formal proofs. A single implementation does not become
independent-implementation evidence. Exact bytes are profile-specific, and
physical timings are implementation/hardware-specific. Quantitative comparison
requires a previously frozen neutral measurement meaning.

## Global stop condition

Preparation stops before any choice of model, transaction/identity/replay
mechanism, authorization architecture, cryptography, commitment/proof/snapshot
construction, ordering/scheduling mechanism, resource economics or constants,
consensus/finality, governance/emergency rules, migration/recovery, release
signing, production P2P policy, normative edit, runtime code, or merge.

## Process status

Comparison scoring: **NOT STARTED**. State-model ranking: **NONE**. State-model
decision: **NOT MADE**. Account selected: **NO**. UTXO selected: **NO**. Main
merge: **NOT DONE**.
