# Dilithia Account / UTXO Gate-1-Through-9 Comparison Evidence Matrix

**Status:** NON-NORMATIVE — REVISION 2 PROVENANCE-CLARIFIED NEUTRAL COMPARISON-MATRIX DRAFT — FOCUSED REVIEW PENDING
**State-model decision:** NOT MADE
**Account selected:** NO
**UTXO selected:** NO
**State-model ranking:** NONE
**Comparison scoring:** NOT STARTED
**Protocol adoption effect:** NONE

> This document translates the satisfied State-Model Decision Gates into one
> common evidence framework for Minimal Account and Minimal UTXO.
>
> It does not select, rank, recommend, or reject either candidate.
>
> The purpose of this document is to ensure that both candidates receive the
> same external semantic, security, historical, and resource requirements before
> new Gate-1-through-9 comparison evidence is collected under this matrix.
>
> Limited mapping-qualified deterministic pilot evidence already tracked in the
> repository predates this completed matrix framework. That pilot evidence is
> retained as provenance and potential evidence input, but it is not treated as
> completed Gate-1-through-9 matrix evidence, does not start scoring, and does not
> rank either candidate.
>
> A candidate-specific mechanism must not be treated as a neutral comparison
> unit merely because that mechanism is natural to one candidate.

---

## 1. Authority and Scope

This matrix is subordinate to:

1. `docs/CONSTITUTION.md`;
2. `docs/SPECIFICATION.md`;
3. normative protocol requirements already established by the repository; and
4. the satisfied Gate-1-through-9 requirement records.

The matrix is a comparison artifact only.

It does not create a new protocol rule.

Architecture notes, benchmark documents, implementation habits, AI reviews, and
candidate hypotheses are evidence inputs only where consistent with superior
authority.

---

## 2. Candidates in Scope

The two co-equal candidates are:

- **Minimal Account**
- **Minimal UTXO**

The following remain outside this comparison decision unless separately
authorized later:

- Generalized Object;
- an active hybrid native-value model;
- a second native-value representation; and
- another state model introduced only to avoid choosing between the two current
  candidates.

This matrix does not assume either candidate has already-defined:

- transaction fields;
- replay fields;
- account identifiers;
- output identifiers;
- credentials;
- authorization descriptors;
- state keys;
- proof layouts;
- commitment structures;
- cryptographic primitives;
- resource counters; or
- fee semantics.

---

## 3. Comparison Rule

Every material comparison must follow this order:

1. state the **external protocol requirement**;
2. state the **common scenario**;
3. map that scenario into Minimal Account without granting it favorable hidden
   assumptions;
4. map the same scenario into Minimal UTXO without granting it favorable hidden
   assumptions;
5. state all material assumptions;
6. identify the evidence required;
7. classify unresolved dependencies explicitly;
8. only then assess whether the candidate satisfies the requirement.

No candidate receives credit merely because a property is commonly associated
with its name.

Every paired Account/UTXO evidence claim, whether qualitative or quantitative,
must bind to one content-identified common semantic case.

The common case must freeze every material external comparison condition,
including, as applicable:

- external preconditions;
- externally required effects and postconditions;
- expected success, rejection, or accepted-unsuccessful classification;
- required authority relations;
- replay and lifecycle context;
- domain and protocol-version context;
- conflict relations;
- canonical ordering context;
- migration guarantee branch;
- cryptographic/profile branch;
- authenticated-state/profile branch;
- resource-relevant branch; and
- unresolved capability branches that could change the result.

Candidate-specific mapping may use different internal representation, state,
dependencies, artifacts, or implementation strategy.

Those internal differences are candidate outputs.

They must not silently change the common external semantic case.

Both candidate mappings must reference the same common-case identity and the
same applicable shared manifest/profile selections.

If a material external condition cannot be held equivalent, the result must be
marked `ASSUMPTION DEPENDENT`, `BLOCKED`, or otherwise unavailable for direct
ranking rather than treated as an ordinary paired comparison.

Examples of forbidden shortcuts include:

- Account means one authorization;
- UTXO means one authorization per input;
- Account means one lookup;
- UTXO means one database lookup per input;
- Account conflict is inherently global;
- UTXO conflict is inherently local;
- Account is inherently more compact;
- UTXO is inherently easier to prove;
- Account is inherently better for PQ signatures;
- UTXO is inherently better for parallel execution.

---

## 4. Evidence Status Vocabulary

Each comparison item may later receive one evidence status:

| Status | Meaning |
|---|---|
| `REQUIREMENT MAPPED` | The common external requirement has been mapped into both candidates. |
| `EVIDENCE PENDING` | The candidate-specific claim is identified but supporting evidence is not yet sufficient. |
| `ASSUMPTION DEPENDENT` | The result depends materially on an unresolved profile or design assumption. |
| `SUPPORTED` | Reviewed evidence supports the claim under the recorded assumptions. |
| `NOT SUPPORTED` | Reviewed evidence fails to support the claim under the recorded assumptions. |
| `BLOCKED` | A prerequisite decision is genuinely required before useful evidence can be produced. |
| `DEFERRED` | The issue belongs to a later design stage and is not necessary for the state-model decision. |

`SUPPORTED` does not mean that a candidate has won the state-model decision.

---

# 5. Gate 1 — Ownership and Authorization

## 5.1 Common external requirement

Both candidates must demonstrate a deterministic relationship between:

- control of native DLTH;
- accepted authorization evidence;
- the exact effect being authorized;
- applicable protocol and cryptographic versions; and
- the canonical state and history relevant to that authorization.

Protocol evolution must not itself cause protocol-created loss of ownership or
spendability where the applicable assumptions permit migration.

No privileged administrator, founder, foundation, recovery master key, or
equivalent authority may override ordinary ownership.

Failure of authorization must not create a partial canonical state effect.

Migration evidence must state the guarantee actually being claimed and the
assumptions necessary for that guarantee.

Where authorization evidence can become forgeable, the comparison must
distinguish:

- an ordinary planned deprecation or migration;
- compromise where legitimate authority remains distinguishable by other
  accepted information; and
- catastrophic loss of distinguishing information.

If forged and legitimate authorization are indistinguishable under all
available authoritative evidence, the comparison must not award either
candidate an impossible recovery guarantee.

Any claimed recovery or migration advantage must identify the independent
pre-existing information, participation, or authority relation that makes the
claim possible.

This requirement selects no recovery mechanism, alternate credential, identity
system, migration mechanism, or privileged authority.

## 5.2 Common scenarios

Both candidates must eventually be evaluated under at least:

1. ordinary native-DLTH authorization;
2. several logical funding sources controlled by one party;
3. several independent required authorizers, if such authorization is later
   supported;
4. credential or authorization-version migration;
5. competing authorization-related effects;
6. compromised credential;
7. lost credential;
8. obsolete cryptographic profile;
9. historical authorization after deprecation;
10. malformed authorization evidence;
11. many expensive invalid authorization artifacts;
12. authorization metadata growth;
13. dormant native value across a planned cryptographic deprecation;
14. migration with owner participation where participation is required by the
    evaluated guarantee;
15. migration where no owner participation is assumed;
16. historical validation after a cryptographic profile is retired; and
17. catastrophic authorization-evidence forgeability where prior distinguishing
    information may or may not exist.

## 5.3 Minimal Account mapping questions

The Account candidate must explain, without assuming an answer:

- which logical facts determine control;
- which state is consulted for authorization;
- whether one logical account effect may require one or several independent
  authorities;
- whether authorization evidence can cover several effects;
- whether authorization-related state is mutable;
- what deletion or recreation would mean for authorization history;
- how migration changes authorization interpretation;
- how dormant Account-controlled value or state is treated across cryptographic
  deprecation and migration;
- what migration guarantee is being claimed;
- what owner-participation assumption applies;
- what current-acceptance policy is assumed during migration;
- what prior distinguishing information, if any, remains available after a
  cryptographic failure;
- which authorization facts survive reorganization; and
- which metadata persists.

## 5.4 Minimal UTXO mapping questions

The UTXO candidate must explain, without assuming an answer:

- which logical facts determine control of referenced native value;
- whether authorization conditions are directly attached, indirectly referenced,
  or represented another way;
- whether one authorization may cover several referenced value sources;
- whether several independent authorization groups may participate;
- which authorization metadata may be duplicated;
- how dormant referenced value is treated across cryptographic deprecation and
  migration;
- what migration guarantee is being claimed;
- what owner-participation assumption applies;
- what current-acceptance policy is assumed during migration;
- what prior distinguishing information, if any, remains available after a
  cryptographic failure;
- which authorization facts survive reorganization; and
- which metadata persists.

## 5.5 Evidence required

Evidence should eventually include:

- authorization-coverage tables;
- scenario traces;
- version-transition traces;
- failure traces;
- adversarial authorization cases;
- explicit reuse/grouping assumptions;
- migration-guarantee level;
- owner-participation assumption;
- current-acceptance policy;
- dormant-value/state assumption;
- historical-validation assumption;
- retrofit assumption;
- independent prior distinguishing-information assumption where relevant; and
- independently reproducible authorization outcomes.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 6. Gate 2 — Replay Exclusion and Canonical Identity

## 6.1 Common external requirement

An already exercised spendable effect must not become spendable again merely
because the same or an equivalent candidate is presented while the earlier
exercise remains authoritative.

Both candidates must distinguish:

- replay;
- duplicate presentation;
- conflict;
- reapplication after reverted history;
- current validity;
- historical validity; and
- canonical identity relevant to those relations.

Canonical identity is a semantic requirement and is not assumed to equal a
cryptographic hash.

## 6.2 Common scenarios

Both candidates must eventually be evaluated under:

1. exact duplicate presentation;
2. semantically equivalent duplicate presentation;
3. two conflicting spends/effects;
4. missing referenced state;
5. deletion or consumption;
6. recreation;
7. zero/empty/absence distinctions;
8. authorization-version change;
9. protocol-version change;
10. cross-domain replay;
11. reorganization followed by reapplication;
12. historical replay analysis; and
13. synchronization from untrusted state.

## 6.3 Minimal Account mapping questions

The Account candidate must determine:

- what canonical relation prevents an exercised effect from being exercised
  again;
- whether any sequence-like state is needed;
- what deletion or recreation means for replay protection;
- whether zero value and absence are distinct;
- how authorization changes interact with replay scope;
- how disjoint effects involving one logical account are distinguished;
- which replay facts are restored after reorganization; and
- which replay facts must remain historically interpretable.

No nonce, sequence, counter, or account identifier is selected by this matrix.

## 6.4 Minimal UTXO mapping questions

The UTXO candidate must determine:

- what canonical relation proves that referenced native value is current or
  already exercised;
- what makes created value references unambiguous;
- whether reference identity depends on a wider transaction identity;
- how relevant malleability or aliasing is excluded;
- whether a previously consumed identity can ever become current again;
- how grouping several referenced values affects replay interpretation;
- which facts are restored after reorganization; and
- which facts must remain historically interpretable.

No transaction ID, output ID, position rule, or consumption encoding is selected
by this matrix.

## 6.5 Evidence required

Evidence should eventually include:

- replay-state invariants;
- identity collision/alias analysis;
- lifecycle replay traces;
- reorganization/reapplication traces;
- protocol-version traces; and
- independent historical interpretation tests.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 7. Gate 3 — Native DLTH Lifecycle and Conservation

## 7.1 Common external requirement

Every native-DLTH-affecting path must admit an auditable conservation argument.

The argument must cover:

- successful transitions;
- rejected transitions;
- partial-failure attempts;
- creation where protocol-authorized;
- destruction where protocol-authorized;
- transfers;
- lifecycle transitions;
- migration;
- reorganization; and
- historical interpretation.

No state-model representation may create an unstated issuance path, hidden value
loss, duplicate spendability, or protocol-caused value ambiguity.

## 7.2 Common scenarios

Both candidates must eventually be evaluated under:

1. ordinary transfer;
2. several value sources;
3. several recipients/effects;
4. exact-balance transfer;
5. arithmetic boundary failure;
6. missing source;
7. authorization failure;
8. later-stage failure;
9. conflicting transition;
10. reorganization;
11. version migration; and
12. lifecycle recreation attempt.

## 7.3 Minimal Account mapping questions

Account evidence must eventually identify:

- all logical debit concepts;
- all logical credit concepts;
- every native-value-affecting mutation;
- conservation across several affected logical entities;
- arithmetic and failure boundaries;
- any relationship between absence, zero, deletion, and value; and
- conservation under rollback and reapplication.

## 7.4 Minimal UTXO mapping questions

UTXO evidence must eventually identify:

- every logically consumed value source;
- every logically created value result;
- conservation across the consumed/created set;
- arithmetic and failure boundaries;
- treatment of missing or already-consumed value;
- lifecycle meaning of created and consumed value; and
- conservation under rollback and reapplication.

## 7.5 Evidence required

Evidence should eventually include:

- a complete native-value transition inventory;
- conservation equations or equivalent formal invariants once transaction
  semantics exist;
- success and failure traces;
- reorganization traces; and
- machine-checkable conservation tests where practical.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 8. Gate 4 — Transaction Dependencies, State Effects, Atomicity, and Failure

## 8.1 Common external requirement

Both candidates must make every validity-affecting logical dependency and
canonical effect sufficiently explicit to reason about:

- successful evaluation;
- rejection;
- dependency failure;
- authorization failure;
- arithmetic failure;
- resource failure;
- version failure;
- conflicting state; and
- reorganization.

A rejected candidate must not leave a partial canonical transition.

## 8.2 Common scenarios

Both candidates must eventually be evaluated under:

1. all dependencies present;
2. one dependency missing;
3. one dependency stale;
4. one dependency conflicting;
5. authorization failure before mutation;
6. authorization failure after earlier validation work;
7. arithmetic failure;
8. malformed input;
9. unsupported version;
10. resource-bound failure;
11. several logically dependent effects;
12. later-stage failure after temporary local work; and
13. rollback/reapplication.

## 8.3 Minimal Account mapping questions

Account evidence must eventually identify:

- the logical dependencies of an effect;
- every logical entity read;
- every logical entity mutated;
- preconditions on current state;
- cross-entity atomicity requirements;
- whether an effect depends on absence as well as presence; and
- which candidate-visible intermediate concepts must never become partial
  canonical state.

## 8.4 Minimal UTXO mapping questions

UTXO evidence must eventually identify:

- the logical dependencies of an effect;
- every referenced value/state entity;
- every created logical entity;
- required current membership or absence conditions;
- atomicity across consumption and creation;
- any auxiliary canonical facts required by the transition; and
- which candidate-visible intermediate concepts must never become partial
  canonical state.

## 8.5 Evidence required

Evidence should eventually include:

- abstract transition tables;
- dependency sets;
- success/failure traces;
- atomicity invariants;
- malformed and unsupported-version cases; and
- differential outcome tests.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 9. Gate 5 — Entity Lifecycle and Historical Meaning

## 9.1 Common external requirement

Both candidates must distinguish protocol-semantic lifecycle from physical data
retention.

Where relevant, the design must define the meaning of:

- never existed;
- currently exists;
- currently absent;
- previously existed;
- deleted;
- consumed;
- recreated;
- reverted;
- migrated; and
- historical-only.

Current absence must not automatically mean that an entity never existed.

Pruning must not silently redefine lifecycle truth.

## 9.2 Common scenarios

Both candidates must eventually be evaluated under:

1. first creation;
2. repeated creation attempt;
3. ordinary current existence;
4. transition to non-current state;
5. deletion or consumption;
6. recreation attempt;
7. zero/empty/absence distinction;
8. migration;
9. reorganization;
10. pruning;
11. snapshot restoration; and
12. historical query/verification.

## 9.3 Minimal Account mapping questions

The Account candidate must determine:

- when a logical account-related entity exists;
- whether zero-value state is current existence or absence;
- what deletion means;
- whether recreation is permitted;
- whether recreation inherits any replay or authorization history;
- how migration changes lifecycle interpretation; and
- what historical facts must remain interpretable.

## 9.4 Minimal UTXO mapping questions

The UTXO candidate must determine:

- when a logical value entity exists;
- what creation means;
- what consumption means;
- whether an identical or equivalent reference may ever be recreated;
- how dormant and consumed value are distinguished;
- how migration changes lifecycle interpretation; and
- what historical facts must remain interpretable.

## 9.5 Evidence required

Evidence should eventually include:

- lifecycle state diagrams;
- creation/deletion/consumption/recreation traces;
- current-versus-historical truth tables;
- pruning invariants;
- migration traces; and
- reorganization tests.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 10. Gate 6 — PQ Authorization Count and Artifact Assumptions

## 10.1 Common external requirement

The comparison must not assume:

- Account equals one signature;
- UTXO equals one signature per input;
- one owner equals one credential;
- one authorization object equals one verification; or
- one candidate always uses one cryptographic version.

Cryptographic workload must be derived from explicit authorization architecture
and transaction/effect coverage assumptions.

## 10.2 Common profile variables

Both candidates must eventually be compared under explicit profiles describing,
as applicable:

- independent authorizer count;
- authorization grouping;
- credential multiplicity;
- authorization-artifact count;
- artifact byte size;
- cryptographic verification-operation count;
- public-key reuse;
- credential reuse;
- ownership-condition reuse;
- authorization-condition reuse;
- authorization-evidence reuse;
- signature or proof reuse;
- verification-result reuse;
- implementation-cache reuse;
- invalidation conditions for every material reuse class;
- whether each claimed reuse is protocol-visible or implementation-local;
- cryptographic-version diversity;
- batch or aggregation assumptions where later permitted; and
- invalid-artifact failure behavior.

Reuse is neither required nor prohibited by this matrix.

A local cache hit must not be credited as a protocol-visible reduction in
required work.

For a paired claim, both candidates must receive the same availability
assumption for any reuse capability that belongs to the common semantic case.

Where equivalent reuse genuinely depends on unresolved candidate architecture,
the comparison must use explicit symmetric branches or remain
`ASSUMPTION DEPENDENT`.

## 10.3 Minimal Account mapping questions

Account evidence must determine, rather than assume:

- how many independent authorities are required for each scenario;
- what one authorization can cover;
- whether several effects can share evidence;
- whether any verification result is reusable under protocol rules;
- how authorization-version coexistence changes artifact counts; and
- whether account-related metadata changes verification exposure.

## 10.4 Minimal UTXO mapping questions

UTXO evidence must determine, rather than assume:

- how many independent authorities are required for each scenario;
- whether one authorization can cover several referenced value sources;
- whether several references share an authorization condition;
- whether any verification result is reusable under protocol rules;
- how authorization-version coexistence changes artifact counts; and
- whether duplicated or indirect ownership metadata changes verification
  exposure.

## 10.5 Evidence required

Evidence should eventually include:

- frozen authorization profiles;
- artifact inventories;
- byte counts under the same cryptographic profile;
- verification-operation counts;
- success and invalid-artifact workloads;
- reuse assumptions;
- coexistence workloads; and
- migration workloads.

No PQ primitive is selected here.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 11. Gate 7 — Canonical Conflict, Ordering, and Scheduling Equivalence

## 11.1 Common external requirement

For a fixed authoritative prior state, protocol version, candidate set, and
canonical ordering context, compliant implementations must agree on the
authoritative result required by the protocol.

The comparison must distinguish:

- semantic conflict;
- execution scheduling;
- local implementation parallelism; and
- canonical ordering.

Parallel execution is not itself a state-model safety requirement.

## 11.2 Common scenarios

Both candidates must eventually be evaluated under:

1. disjoint effects;
2. two effects sharing one logical dependency;
3. several partially overlapping dependencies;
4. authorization change versus spend;
5. lifecycle change versus spend;
6. several value sources;
7. dependency created by an earlier candidate;
8. dependency removed by an earlier candidate;
9. different local evaluation schedules;
10. batch evaluation;
11. late failure;
12. reorganization; and
13. cryptographic-version coexistence.

## 11.3 Minimal Account mapping questions

Account evidence must determine:

- which logical facts make two effects conflict;
- whether disjoint effects involving the same broader account concept can remain
  semantically independent;
- which authorization/replay/lifecycle facts create hidden dependencies;
- what ordering affects validity;
- what ordering affects only implementation performance; and
- whether independent schedules reproduce the authoritative result.

The matrix does not assume Account conflicts are global.

## 11.4 Minimal UTXO mapping questions

UTXO evidence must determine:

- which logical facts make two effects conflict;
- whether apparently disjoint value references share authorization, lifecycle,
  migration, or other canonical dependencies;
- what ordering affects validity;
- what ordering affects only implementation performance; and
- whether independent schedules reproduce the authoritative result.

The matrix does not assume UTXO conflicts are always local.

## 11.5 Evidence required

Evidence should eventually include:

- conflict-set derivations;
- dependency graphs;
- schedule permutations;
- serial-equivalence or other applicable deterministic-result evidence;
- conflicting/failing workload tests; and
- independent implementation comparisons.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 12. Gate 8 — Authenticated State, Membership, Absence, Snapshot, and Light Client

## 12.1 Common external requirement

Both candidates receive the same external authenticated-state requirements.

Where protocol validity depends on them, the future authenticated-state design
must support deterministic interpretation of:

- required current membership;
- required current absence;
- ownership-relevant facts;
- replay-relevant facts;
- lifecycle-relevant facts;
- native-value facts;
- cryptographic-version facts;
- validity-affecting resource facts;
- historical claims where required; and
- canonical-history/version binding.

Zero, empty, absent, removed, consumed, and invalid must not be conflated unless
the authoritative protocol semantics intentionally make them equivalent.

## 12.2 Common scenarios

Both candidates must eventually be evaluated under:

1. valid current membership;
2. required current absence;
3. false membership claim;
4. false absence claim;
5. zero-versus-absence ambiguity;
6. current-versus-historical ambiguity;
7. state update followed by failure;
8. corrupted snapshot;
9. omitted snapshot component;
10. duplicated snapshot component;
11. mixed-version snapshot;
12. snapshot from non-canonical history;
13. reorganization after snapshot;
14. light-client head substitution;
15. cryptographic migration;
16. cryptographic deprecation; and
17. catastrophic cryptographic-assumption analysis.

## 12.3 Minimal Account mapping questions

Account evidence must identify:

- which current account-related facts require authenticated membership;
- which account-related absences affect validity;
- how zero/empty/absent meanings differ;
- which ownership/replay/lifecycle/version facts are authoritative;
- what a snapshot must reconstruct or prove; and
- what a light client must verify for the selected claim profile.

## 12.4 Minimal UTXO mapping questions

UTXO evidence must identify:

- which current value-reference facts require authenticated membership;
- which absences affect validity;
- how never-created, current, consumed, and absent meanings differ;
- which ownership/replay/lifecycle/version facts are authoritative;
- what a snapshot must reconstruct or prove; and
- what a light client must verify for the selected claim profile.

## 12.5 Evidence required

Commitment-dependent evidence must use an explicitly frozen common profile.

Evidence should eventually include:

- logical fact inventory;
- membership/absence requirements;
- proof-generation logical accesses;
- proof-verification logical accesses;
- proof sizes under a common frozen construction;
- update workloads;
- snapshot contents and verification;
- synchronization workloads;
- light-client claim profiles;
- history-binding evidence;
- trust assumptions; and
- Crypto Agility assumptions.

No commitment, proof system, snapshot protocol, or light-client protocol is
selected here.

**Current matrix status:** `REQUIREMENT MAPPED — COMMITMENT-DEPENDENT EVIDENCE PENDING`

---

# 13. Gate 9 — Logical Access, Mutation, Persistent Growth, and Invalid-Candidate Resources

## 13.1 Common external requirement

Both candidates must be evaluated against the same externally meaningful
resource workload and security requirement.

Resource analysis must cover:

- accepted work;
- rejected work;
- malformed input;
- logical access;
- existence and absence evaluation;
- logical mutation;
- gross mutation;
- net state effect;
- persistent growth;
- cryptographic work;
- proof/snapshot/light-client work;
- nested composition;
- cross-candidate composition;
- containing-object composition;
- version coexistence;
- historical interpretation; and
- local-versus-consensus resource boundaries.

## 13.2 Core safety requirements

For both candidates:

- invalid-candidate work must admit a deterministic finite upper bound;
- validation must support bounded staging;
- logical-access exposure must be upper-boundable before unbounded work;
- per-access adversarial exposure must be bounded;
- existence/absence evaluation must not require attacker-controlled unbounded
  global search;
- logical mutation exposure must be bounded;
- gross and net mutation must not be conflated;
- protocol-required persistent growth must be accounted for;
- otherwise-unbounded state growth, free spam, or permanent storage without
  cost must retain the Constitution Article 11 economic-protection requirement
  without selecting the economic mechanism;
- economic payment must not substitute for a required hard resource-safety
  bound;
- rejected candidates must preserve canonical atomicity;
- a fully rejected candidate must be distinguished from a canonically accepted
  object or accepted-unsuccessful outcome;
- fully rejected candidates must have no canonical state, economic, or other
  canonical effect;
- any future canonically accepted-unsuccessful outcome with an economic or
  resource consequence must satisfy the applicable authorization,
  conservation, replay, lifecycle, atomicity, determinism, and resource rules;
- nested and cross-candidate amplification must be bounded;
- producer-versus-verifier amplification must be included in hostile workload
  analysis;
- bounded P2P framing and pre-candidate processing must be included where work
  occurs before a fully decoded candidate exists;
- cryptographic work and material reuse assumptions must be explicit;
- unknown or unsupported versions must fail deterministically and boundedly;
- if consensus-visible resource arithmetic is later introduced, its type,
  arithmetic interpretation, overflow, and underflow behavior must be
  deterministic and explicit;
- if refunds, credits, rebates, or negative resource deltas are later
  introduced, they must not erase already-required work, reset a safety budget,
  underflow accounting, or create an exploitable accounting cycle;
- local admission policy must not redefine canonical validity;
- local OOM, timeout, cache state, storage-engine behavior, or thread scheduling
  must not silently redefine protocol validity;
- proof, snapshot, and light-client verification exposure must be bounded; and
- independent implementations must reproduce consensus-visible resource
  semantics where such semantics exist.

## 13.3 Minimal Account mapping questions

Account evidence must determine:

- logical accesses required by each scenario;
- existence and absence checks;
- logical mutations;
- gross versus net mutation;
- replay/authorization/lifecycle metadata growth;
- invalid-candidate late-failure exposure;
- cryptographic verification exposure;
- cross-candidate amplification;
- lifecycle churn; and
- proof/snapshot resource exposure under common frozen profiles.

No account-native counter is automatically a neutral resource unit.

## 13.4 Minimal UTXO mapping questions

UTXO evidence must determine:

- logical accesses required by each scenario;
- existence and absence checks;
- logical mutations;
- gross versus net mutation;
- ownership/replay/lifecycle metadata growth;
- invalid-candidate late-failure exposure;
- cryptographic verification exposure;
- cross-candidate amplification;
- lifecycle churn; and
- proof/snapshot resource exposure under common frozen profiles.

No UTXO-native counter is automatically a neutral resource unit.

## 13.5 Evidence required

Evidence should eventually include:

- logical-access inventories;
- logical-mutation inventories;
- gross and net mutation profiles;
- persistent-growth profiles;
- invalid-candidate worst-case workloads;
- cryptographic hostile-work profiles;
- batch/fallback workloads;
- lifecycle-churn workloads;
- cross-candidate composition workloads;
- proof/snapshot workloads;
- producer/verifier amplification workloads;
- P2P framing and pre-candidate workloads;
- fully rejected versus accepted-unsuccessful outcome traces where the future
  protocol distinguishes them;
- conditional resource-accounting overflow/underflow workloads if such
  accounting is introduced;
- conditional refund/credit/negative-delta abuse workloads if such mechanisms
  are introduced;
- persistent-growth workloads that distinguish economic compensation from hard
  safety;
- historical-version workloads; and
- independent implementation results.

No resource meter, scalar gas unit, resource vector, fee mechanism, rent,
hardware target, or numeric resource limit is selected here.

**Current matrix status:** `REQUIREMENT MAPPED — EVIDENCE PENDING`

---

# 14. Shared Comparison Scenario Families

The following scenario families should be reused across gates wherever
applicable instead of inventing candidate-specific favorable workloads.

| Scenario family | Common purpose |
|---|---|
| Ordinary single-authority transfer | Establish baseline authorization, conservation, dependencies, and resources |
| Several logical funding sources | Test grouping, value-source semantics, authorization coverage, and resource composition |
| Several independent authorities | Test multi-authority coverage without assuming one signature model |
| Conflicting effects | Test replay, conflict, ordering, and atomicity |
| Missing dependency | Test membership/absence, failure, and bounded rejection |
| Malformed candidate | Test decoding, early rejection, and implementation independence |
| Cryptographically invalid candidate | Test hostile verification exposure and failure atomicity |
| Late-invalid candidate | Test bounded late-failure work |
| Lifecycle deletion/consumption | Test historical meaning and replay |
| Lifecycle recreation | Test identity reuse, replay reset, and persistent metadata |
| Protocol-version migration | Test deterministic historical/current interpretation |
| Cryptographic-version migration | Test Crypto Agility, artifact profile, and dormant value |
| Multiple crypto versions coexist | Test downgrade confusion and adversarial expensive-profile selection |
| Reorganization | Test rollback, reapplication, lifecycle, replay, and resource effects |
| Snapshot restoration | Test completeness, trust, membership/absence, and historical dependencies |
| Light-client claim | Test claim scope and canonical-history binding |
| Batch with one invalid component | Test fallback amplification and failure semantics |
| Adversarial prior state | Test whether resource and validity claims survive attacker-influenced canonical state |
| Persistent metadata churn | Test long-term storage burden and lifecycle/accounting bypass |
| Dormant value across crypto deprecation | Test migration assumptions and current/historical authorization interpretation symmetrically |
| Catastrophic authorization-evidence forgeability | Test information-theoretic recovery limits and any independent prior distinguishing basis |
| Producer/verifier amplification | Test whether bounded producer effort can force disproportionate mandatory verifier work |
| P2P/pre-candidate framing | Test resource exposure before a fully decoded protocol candidate exists |
| Rejected versus accepted-unsuccessful outcome | Test canonical-effect taxonomy and authorization of any economic/resource consequence |
| Resource-accounting overflow/refund abuse | Conditionally test arithmetic and negative-delta safety if such accounting exists |
| Uncompensated persistent growth | Test Article-11 economic protection separately from hard safety bounds |

These scenarios do not define a transaction format.

---

# 15. Candidate-Neutral Evidence Dimensions

Future evidence may compare candidates only after the compared quantity is
defined independently enough to avoid candidate-native unit bias.

Candidate-neutral dimensions may include, under explicitly frozen assumptions:

- required authorization coverage;
- accepted authorization-artifact bytes;
- invalid authorization-artifact bytes;
- cryptographic verification operations under the same crypto profile;
- logical access classes;
- logical mutation classes;
- gross mutation exposure;
- net canonical state effect;
- protocol-required persistent bytes or logical facts;
- proof bytes under the same commitment profile;
- proof-generation work;
- proof-verification work;
- snapshot bytes under the same snapshot completeness profile;
- synchronization verification work;
- number and structure of semantic conflicts;
- hostile rejection work;
- lifecycle-churn exposure;
- historical-support obligations; and
- implementation-independent proof obligations.

This list does not select a protocol resource meter.

Before any listed dimension contributes to a paired comparative claim, its
measurement meaning must be frozen sufficiently to prevent candidate-native
vocabulary from becoming the comparison standard.

At minimum:

- logical-access evidence must state the shared semantic access taxonomy and
  separately report candidate-realized accesses;
- logical-mutation evidence must state the shared semantic mutation taxonomy and
  separately report candidate-realized mutation structure;
- persistent-growth evidence must distinguish logical protocol-required facts
  from exact encoded bytes;
- exact-byte claims require a frozen experimental schema/encoding/profile and
  must not be presented as representation-independent truth;
- persistent-growth claims must state lifecycle, retention, role, and historical
  support assumptions where material;
- conflict evidence must distinguish the shared external semantic conflict
  relation from the candidate-realized conflict footprint;
- cryptographic verification-operation evidence must state the frozen
  cryptographic and reuse profile;
- proof-generation, proof-verification, and synchronization work must be labeled
  as either logical protocol evidence or physical implementation measurement;
- physical implementation measurements require the applicable implementation
  and hardware profile; and
- candidate-native reads, writes, records, slots, outputs, accounts, or other
  internal counters remain descriptive outputs until mapped through a reviewed
  shared metric definition.

# 16. Common Case Manifest and Evidence Profiles

A paired result is not portable across materially different assumptions.

The common semantic case and every material evidence profile must therefore be
content-identified before the paired claim is treated as comparison evidence.

This requirement applies to qualitative as well as quantitative paired claims
where changing the assumption could change the comparison result.

## 16.0 Common paired-evidence manifest

Every paired evidence record must identify one shared comparison manifest.

The manifest must record, where material:

- common semantic-case identity;
- external preconditions;
- expected external postconditions;
- expected success, rejection, or accepted-unsuccessful classification;
- authority relations;
- replay and lifecycle context;
- domain and protocol-version context;
- semantic conflict relation;
- canonical ordering context;
- migration-guarantee branch;
- capability branches;
- unresolved branches;
- cryptographic-profile identity;
- authenticated-state-profile identity;
- resource/measurement-profile identity; and
- the candidate-specific mapping artifacts being compared.

The Account and UTXO mappings may differ internally.

A candidate mapping must not alter the shared external contract while retaining
the same common-case identity.

If a material manifest field differs, the evidence identity must differ and the
results must not be presented as a direct paired comparison unless the
difference is itself the explicitly evaluated sensitivity branch.

Before a quantitative comparison is treated as evidence, also record at least
the material parts of the applicable profiles:

## 16.1 Transaction/effect profile

- semantic scenario;
- number of logical value sources;
- number of logical recipients/effects;
- required independent authorities;
- authorization coverage relation;
- relevant lifecycle state;
- relevant replay state;
- expected success/rejection/accepted-unsuccessful outcome;
- domain context;
- semantic conflict relation where material;
- canonical ordering context where material;
- migration guarantee and participation assumptions where material;
- current-acceptance and dormant-state assumptions where material; and
- protocol-version context.

## 16.2 Cryptographic profile

- algorithm/profile identity for the experiment only;
- artifact sizes;
- verification operation assumptions;
- grouping assumptions;
- batch/aggregation assumptions;
- public-key reuse assumptions;
- credential reuse assumptions;
- ownership-condition reuse assumptions;
- authorization-condition reuse assumptions;
- authorization-evidence reuse assumptions;
- signature/proof reuse assumptions;
- verification-result reuse assumptions;
- implementation-cache assumptions;
- invalidation conditions for material reuse;
- protocol-visible versus implementation-local reuse classification;
- coexistence assumptions;
- migration-guarantee and owner-participation assumptions where material;
- current-acceptance and dormant-state assumptions where material;
- independent prior distinguishing-information assumptions where material; and
- historical-validation assumptions.

The experimental profile does not select the Genesis primitive.

## 16.3 Authenticated-state profile

- commitment construction used for the experiment;
- construction/version identity;
- logical state being committed;
- candidate population/scale assumptions where material;
- membership capability;
- absence capability;
- update capability;
- proof target;
- hostile-proof/input profile;
- snapshot capability;
- snapshot completeness profile;
- snapshot trust/bootstrap assumptions;
- synchronization capability;
- retained-current-fact and historical-support assumptions;
- history and reorganization profile;
- light-client claim class; and
- cryptographic assumptions.

If any material authenticated-state profile dimension changes, the evidence
identity must change.

The experimental profile does not select the protocol commitment, snapshot
mechanism, bootstrap trust model, or light-client protocol.

## 16.4 Implementation/hardware profile

Where physical benchmarks are used:

- implementation identity;
- implementation version;
- compiler/toolchain;
- hardware;
- operating system;
- storage environment;
- benchmark configuration; and
- reproducibility information.

Physical benchmark results must not be presented as semantic protocol truth.

---

# 17. Scoring Boundary

No weighted score is authorized by this first matrix draft.

Before scoring is introduced, a later reviewed record must define:

- which dimensions are decision-critical;
- which are pass/fail requirements;
- which are comparative advantages;
- how assumption-dependent results are handled;
- how incomparable evidence is treated;
- whether any weights are justified;
- how security requirements override convenience or performance advantages; and
- how uncertainty is represented.

A candidate must not compensate for failure of a mandatory safety requirement
by receiving a higher performance score elsewhere.

---

# 18. Wallet and Product Complexity Boundary

Wallet/user-product complexity is real evidence but must remain distinct from
consensus-semantic complexity.

For example:

- UTXO coin selection;
- change handling;
- consolidation; and
- balance presentation

may create wallet complexity.

Likewise, simple account balance presentation does not prove that Account has
simpler:

- authorization;
- replay;
- lifecycle;
- migration;
- conflict;
- historical interpretation; or
- resource semantics.

Wallet complexity may later be a comparison dimension, but it must not be
silently substituted for protocol-security evidence.

---

# 19. Formal and Executable Evidence Boundary

Where practical, final decision evidence should move beyond prose.

Potential evidence classes include:

- explicit invariants;
- abstract transition models;
- adversarial scenario vectors;
- conformance vectors;
- property-based tests;
- differential tests between independent implementations;
- historical replay tests;
- migration tests;
- malformed-input tests;
- fuzzing;
- executable models; and
- formal proof artifacts where justified.

This section selects no formal-verification language or tool.

---

# 20. Current Matrix Classification

| Gate | Common requirement mapped? | Account evidence | UTXO evidence | Ranking effect now |
|---|---|---|---|---|
| Gate 1 — Ownership / authorization | YES | PENDING | PENDING | NONE |
| Gate 2 — Replay / canonical identity | YES | PENDING | PENDING | NONE |
| Gate 3 — DLTH lifecycle / conservation | YES | PENDING | PENDING | NONE |
| Gate 4 — Dependencies / effects / atomicity / failure | YES | PENDING | PENDING | NONE |
| Gate 5 — Entity lifecycle / history | YES | PENDING | PENDING | NONE |
| Gate 6 — PQ authorization/artifact assumptions | YES | PENDING | PENDING | NONE |
| Gate 7 — Conflict / ordering / scheduling equivalence | YES | PENDING | PENDING | NONE |
| Gate 8 — Authenticated state / snapshot / light client | YES | PENDING | PENDING | NONE |
| Gate 9 — Logical access / mutation / growth / invalid resources | YES | PENDING | PENDING | NONE |

The table is intentionally not a scorecard yet.

---

# 21. Claims Explicitly Not Supported Yet

The current evidence does not justify recording any of the following as project
direction:

- Account currently leads;
- UTXO currently leads;
- Account is inherently more secure;
- UTXO is inherently more secure;
- Account is inherently more PQ-efficient;
- UTXO is inherently more PQ-efficient;
- Account inherently requires fewer authorizations;
- UTXO inherently requires one authorization per input;
- Account is inherently more compact;
- UTXO is inherently more proof-friendly;
- Account inherently has worse parallelism;
- UTXO inherently has better parallelism;
- Account inherently has lower state growth;
- UTXO inherently has lower state growth; or
- either candidate should be selected before comparable evidence exists.

---

# 22. Explicit Non-Selections

This matrix does not select:

- Account;
- UTXO;
- a state-model ranking;
- stable logical identity;
- direct credential-bound ownership;
- authorization descriptor structure;
- recovery;
- delegation;
- multisig;
- nonce or sequence semantics;
- UTXO consumption encoding;
- transaction identity;
- output identity;
- effect identity;
- transaction format;
- block format;
- native state schema;
- state key layout;
- state commitment;
- proof system;
- membership-proof format;
- absence-proof construction;
- snapshot format;
- snapshot trust model;
- synchronization protocol;
- light-client protocol;
- cryptographic primitive;
- PQ primitive;
- signing message;
- authorization grouping rule;
- batching algorithm;
- aggregation algorithm;
- resource meter;
- scalar gas;
- resource vector;
- fee mechanism;
- storage rent;
- refund mechanism;
- numeric resource limits;
- hardware target;
- consensus algorithm;
- fork-choice rule;
- finality rule;
- reorganization depth;
- scheduler;
- parallel-execution mechanism; or
- formal-verification tool.

---

# 23. First-Draft Review Questions

Before candidate evidence collection begins, review this matrix for:

1. Does every Gate 1-through-9 requirement have a candidate-neutral comparison
   home?
2. Is any mandatory inherited requirement missing?
3. Does any row silently favor Account vocabulary?
4. Does any row silently favor UTXO vocabulary?
5. Does the matrix accidentally assume Account means one account object?
6. Does it accidentally assume UTXO means one signature per input?
7. Does it accidentally select an Account nonce?
8. Does it accidentally select UTXO consumption identity?
9. Does it distinguish replay from reapplication?
10. Does it preserve failure atomicity?
11. Does it preserve native-value conservation?
12. Does it distinguish current absence from historical nonexistence?
13. Does it preserve Crypto Agility?
14. Are catastrophic cryptographic limits stated without promising impossible
    recovery?
15. Does it keep commitment-dependent claims profile-dependent?
16. Does it prevent candidate-specific resource units from becoming neutral
    metrics?
17. Does it keep local performance behavior separate from consensus validity?
18. Does it prevent wallet complexity from being confused with consensus
    complexity?
19. Does it require the same adversarial scenarios for both candidates?
20. Does any sentence rank Account or UTXO before evidence exists?
21. Does any quantitative comparison lack a frozen material profile?
22. Does the matrix accidentally select a transaction, crypto, commitment,
    resource, or consensus mechanism?
23. Is a required scenario missing for reorganization, migration, lifecycle
    churn, invalid cryptography, proof/snapshot failure, or historical
    interpretation?
24. Are evidence requirements strong enough to support a later auditable
    state-model decision?
25. Does every paired claim bind to one content-identified common semantic case
    and one shared applicable manifest?
26. Are migration guarantee, participation, current-acceptance, dormant-state,
    historical-validation, retrofit, and prior-information assumptions explicit
    where material?
27. Are protocol-visible reuse and implementation-local caching explicitly
    distinguished?
28. Does the authenticated-state profile freeze completeness, trust/bootstrap,
    history/reorganization, hostile-input, and population assumptions where
    material?
29. Are producer/verifier amplification, P2P/pre-candidate work,
    rejected-versus-accepted-unsuccessful outcomes, and conditional resource
    arithmetic represented?
30. Are logical, encoded-byte, semantic-conflict, and physical-work metrics kept
    distinct until their shared comparison meaning is frozen?
31. Is there any reason candidate evidence collection must remain blocked after
    this matrix is corrected and reviewed?

---

# 24. Satisfaction State

Gate 1 through Gate 9 requirements:

**SATISFIED AT THEIR RESPECTIVE PROJECT-PROCESS GATES**

Comparison source scan:

**COMPLETE**

Neutral comparison matrix:

**REVISION 2 CREATED — AU-CM-007 PROVENANCE CLARIFICATION — FOCUSED REVIEW PENDING**

Candidate-specific Gate-1-through-9 evidence collected under this matrix:

**NOT YET COLLECTED**

Limited mapping-qualified deterministic pilot evidence already exists in the
tracked pilot evidence package.

That pilot evidence remains non-ranking and does not by itself satisfy any
Gate-1-through-9 comparison row.

Comparison scoring:

**NOT STARTED**

State-model ranking:

**NONE**

State-model decision:

**NOT MADE**

Minimal Account selected:

**NO**

Minimal UTXO selected:

**NO**

---

# 25. Next Step

The next step is not to score the two candidates.

The next step is:

> adversarially review this exact neutral matrix for inherited-requirement
> omissions, hidden Account/UTXO bias, premature mechanism selection, and
> evidence asymmetry.

Only after material review findings are resolved should candidate-specific
evidence collection begin.

**STATE MODEL DECISION: NOT MADE**