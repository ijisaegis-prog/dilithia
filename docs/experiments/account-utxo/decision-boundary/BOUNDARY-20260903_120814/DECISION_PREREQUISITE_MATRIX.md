# Decision Prerequisite Matrix

This matrix states prerequisites for responsible decisions. It neither ranks
questions nor endorses answers.

| Decision IDs | Semantic prerequisites | Implementation/evidence prerequisites | Authority prerequisite | Current disposition |
|---|---|---|---|---|
| PDQ-01, PDQ-15 | shared external contract; complete candidate mappings; explicit missingness and reversible branches | paired executable/formal evidence across applicable Gates; independent review; reproducibility | owner declares decision method and adoption authority | blocked across all three classes |
| PDQ-02 | transaction/effect vocabulary, dependency/effect inventory, failure outcomes, versions/domains | canonical vectors, parsers/validators, transition models, cross-implementation tests | protocol design and later normative adoption | protected; implementation absent |
| PDQ-03 | duplicate, currentness, replay, conflict, reverted-history and reapplication distinctions | alias/collision/replay/reorg corpora and paired results | protocol design | protected; profiles and implementation absent |
| PDQ-04 | authority graph, ownership continuity, credential lifecycle, signing coverage, recovery claims | hostile authorization corpus, migration cases, formal coverage properties | owner/protocol plus constitutional compliance | protected; Gate-1 direct evidence incomplete |
| PDQ-05 | algorithm/version/artifact/operation taxonomy and failure behavior | standardized primitive evidence, portable implementations, malformed-input and batching tests | crypto/protocol authority | protected; concrete profile absent |
| PDQ-06 | logical state subjects, membership/absence/update/history claim semantics | independent commitment/proof implementations and conformance vectors | protocol design | protected; construction absent |
| PDQ-07 | node/light-client roles, claim classes, anchors, trust and availability model | hostile snapshots, sync/recovery tests, distributed evidence | consensus/bootstrap authority | protected; consensus dependency absent |
| PDQ-08 | semantic conflict relation, canonical order context, serial outcome oracle | paired serial/parallel implementations, permutation and fault results | transaction/consensus design | protected and method-blocked |
| PDQ-09 | deterministic logical units, checked arithmetic, reset/composition/failure semantics | registered benchmark campaign, adversarial corpus, raw data, multiple environments/implementations, uncertainty analysis | protocol/economic authority; numeric adoption process | protected and method/evidence-blocked |
| PDQ-10 | adversary model, validator/participant roles, chain/history identity | simulation, formal safety/liveness work, network evidence, independent implementations | consensus authority | protected; foundational dependency absent |
| PDQ-11 | governance participants, proposal types, constitutional classification | security/formal review and deterministic activation tests | authorized HIP/Super-HIP process | protected by definition |
| PDQ-12 | version coexistence, participation, acceptance, dormancy, retrofit, recovery and history profiles | migration/recovery/catastrophic-case corpora and evidence | owner/protocol; constitutional Articles 1–13 as applicable | protected; profile not frozen |
| PDQ-13 | operational roles separated from protocol authority | reproducible-build provenance, signing/verification procedure, network adversarial testing | owner/operations and any applicable protocol process | protected production policy |
| PDQ-14 | exact normative gap and proposed semantics | impact, threat, compatibility, formal and independent review | applicable HIP or Super HIP | protected by authority hierarchy |

## Cross-cutting readiness rules

- A template, schema, checklist, review plan, code-health check, or structural
  derivation is not candidate evidence.
- Pre-result identities must bind all material inputs before decision-relevant
  results are generated; later metadata cannot retroactively create that
  provenance.
- Account and UTXO must receive the same external cases, profile branches,
  evidence classes, optimization opportunities, and missingness treatment.
- Candidate-native observations may remain descriptive; they may not silently
  become a shared scalar or comparison score.
- Unknowns capable of reversing a conclusion must be resolved or reported as
  explicit sensitivity branches before ranking.
