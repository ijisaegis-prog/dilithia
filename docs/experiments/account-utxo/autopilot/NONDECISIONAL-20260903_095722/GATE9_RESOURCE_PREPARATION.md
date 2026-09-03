# Gate 9 — Resource Preparation

> **NON-NORMATIVE PREPARATION.** Gate 9 is process-satisfied. No meter, gas,
> vector, fee, rent, refund, limit, hardware target, or production policy is selected.

## Frozen requirement and status

Both candidates face the same external workloads and security requirements.
Analysis covers accepted/rejected/malformed work; logical access, absence,
mutation, gross/net effects and persistent growth; crypto and authenticated-state
work; nested/cross-candidate composition; versions/history; and local versus
consensus boundaries. The 30 named R1–R30 requirements in
`LOGICAL_ACCESS_MUTATION_PERSISTENT_GROWTH_INVALID_CANDIDATE_RESOURCE_REQUIREMENTS.md`
require bounded invalid work and staging, upper-boundable access/mutation,
bounded amplification/composition, atomic rejection, deterministic conditional
resource arithmetic, nonnegative accounting safety, economic/hard-bound
separation, versioning, local-policy separation, bounded proof/snapshot work,
independent implementation, shared semantics, metric neutrality, optional
bounded dynamic access, and no parallel-execution presumption. The matrix
§§13.1–13.5 and `GATE9_SATISFACTION_DECISION.md` confirm Gate 9 **SATISFIED** at
requirements level. Candidate evidence: **PENDING**.

## Prepared workload families

- accepted baseline and fully rejected versus conditionally defined
  accepted-unsuccessful branches;
- malformed/unknown version, early/late invalid cryptography, batch fallback;
- existence/absence, dynamic access, gross/net mutation, lifecycle churn;
- persistent metadata/state growth with role/retention/history separated;
- nesting, aggregation, cross-candidate and containing-object composition;
- producer/verifier amplification, duplicate invalid candidates, P2P framing;
- proof/snapshot/light-client hostile work and crypto coexistence;
- conditional arithmetic overflow/underflow and refund/credit abuse only if such
  semantics are later introduced.

## Evidence layers and present testability

Now: review shared logical taxonomy, require finite symbolic bounds, check
composition formulas, verify rejected-case zero canonical effect, and lint that
candidate-native counters are descriptive. Later: exact counts need frozen
schemas; physical CPU/memory/bandwidth/storage/timing needs implementations,
hardware/toolchains, corpora, repeated trials, raw results, statistics and
provenance under `BENCHMARK_METHODOLOGY.md`. Worst-case and independent results
are **REQUIRES IMPLEMENTATION**. No benchmark is claimed here.

## Symmetry and owner stop points

Both candidates receive identical external cases, adversarial budgets expressed
symbolically, crypto/authenticated-state profiles, outcome classes, and
measurement definitions. Account reads/writes and UTXO sources/outputs are not
interchangeable neutral units. Selecting any meter, unit, economic mechanism,
refund rule, numeric value, hardware target, admission/P2P policy, or consensus-
visible resource semantics is **REQUIRES OWNER DECISION**.

Scoring **NOT STARTED**; ranking **NONE**; decision **NOT MADE**; Account **NO**;
UTXO **NO**; main merge **NOT DONE**.
