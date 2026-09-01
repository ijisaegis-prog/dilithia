# Dilithia Transaction Dependencies, State Effects, Atomicity, and Failure Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records model-neutral requirements, unresolved questions,
> comparison variables, adversarial scenarios, and evidence gates for abstract
> transaction dependencies, state effects, atomicity, and failure semantics.
>
> It defines no consensus rule, resolves no Formal Specification TBD, selects no
> state model, transaction format, execution model, conflict rule, ordering rule,
> fee rule, resource meter, rollback mechanism, or storage architecture.

## Status

This document is a Pre-Genesis decision-readiness artifact.

It exists to clarify the fourth state-model decision gate:

> Abstract transaction dependencies, state effects, atomicity, and failure.

The existence, completion, or review of this document does not by itself satisfy
that gate.

Minimal Account and Minimal UTXO remain co-equal candidates.

State-model selection remains **NOT MADE**.

Protocol authority and supporting evidence are distinguished as follows.

Authoritative protocol sources are:

1. the Dilithia Technical Constitution;
2. the Dilithia Formal Specification; and
3. HIP or Super HIP material only to the extent that it has been validly adopted,
   activated, and given protocol effect through the authoritative process
   permitted by the Constitution and Formal Specification.

Supporting material may provide decision-readiness, conformance, threat,
resource, experimental, implementation, or project-status evidence, but does not
independently create protocol authority.

Such supporting material includes:

- state-model decision requirements;
- ownership and authorization requirements;
- replay and canonical-identity requirements;
- native-DLTH lifecycle and conservation requirements;
- threat and resource-architecture documents;
- Account/UTXO workload and comparison artifacts;
- conformance vectors or tests unless their normative role is separately
  established through the authoritative protocol process;
- implementations;
- `PROJECT_STATE.md`; and
- AI or other design discussion.

Where supporting material conflicts with authoritative protocol sources, the
authoritative protocol sources control.

This document does not select:

- Account;
- UTXO;
- Hybrid;
- an account representation;
- a UTXO representation;
- a transaction byte format;
- transaction fields;
- transaction identifiers;
- input or output structures;
- balance fields;
- sequence or nonce semantics;
- explicit-consumption semantics;
- a dependency encoding;
- a read-set encoding;
- a write-set encoding;
- an execution virtual machine;
- a scripting system;
- a validation pipeline;
- a rollback mechanism;
- a journal or overlay architecture;
- a database transaction mechanism;
- a mempool policy;
- a conflict-resolution rule;
- canonical transaction ordering;
- block scheduling;
- parallel execution;
- fees;
- gas;
- resource units;
- resource numeric limits; or
- any state-model ranking.

## 1. Existing Authoritative Boundaries

The Constitution establishes model-independent boundaries relevant to this
analysis.

Consensus-critical behavior must be defined by authoritative specification, not
by accidental implementation behavior.

Consensus-critical operations must be deterministic across compliant
implementations.

Consensus correctness must not depend on:

- local clocks;
- floating-point arithmetic;
- undefined behavior;
- operating-system behavior;
- hardware architecture; or
- implementation-specific optimization.

Consensus-critical data that is eventually defined must respect the
Constitution's canonical and versioned serialization requirements.

Protocol evolution must preserve legitimate ownership and state accessibility in
accordance with the Constitution.

Economic-safety requirements constrain future transaction and state architecture,
but they do not by themselves select fees, gas, rent, refunds, or a resource
meter.

Detailed transaction and state semantics remain a future Formal Specification
responsibility where they become consensus-relevant.

This document does not amend the Formal Specification.

## 2. Purpose of the Fourth State-Model Gate

A state-model comparison is premature while transaction semantics remain an
unstated assumption.

Before Account and UTXO can be compared fairly, the project needs sufficiently
concrete abstract answers to questions such as:

- what logical state or prior facts a transaction may depend upon;
- how those dependencies become consensus-relevant;
- what conditions must hold before an effect is permitted;
- what logical effects successful acceptance may create;
- what unit or set of effects must be atomic;
- what happens when a candidate is rejected;
- how late failure avoids partial canonical state;
- how transaction semantics interact with authorization, replay, native-value
  conservation, resources, history, and protocol versions; and
- which questions belong to later conflict, entity-lifecycle, commitment, and
  ordering gates instead.

These are semantic requirements, not transaction-format requirements.

## 3. Abstract Transaction Terminology

For this document only, the following terms are analytical vocabulary.

**Transaction**

A candidate consensus-relevant request for one or more protocol-recognized
effects.

This term does not imply any byte format, field set, transaction identifier,
input/output structure, account structure, scripting language, or execution
engine.

**Dependency**

A protocol-relevant fact whose interpretation or value may affect whether a
candidate is accepted or what canonical effect it has.

**Precondition**

A consensus-relevant condition that must hold for a particular acceptance or
effect rule to apply.

**Logical observation**

A protocol-semantic fact examined while determining validity or effects.

It does not imply a physical database read.

**Logical effect**

A protocol-semantic change resulting from canonical acceptance.

It does not imply a database write, record format, mutation API, or storage
backend.

**Canonical state effect**

A state change recognized by authoritative consensus semantics.

**Attempted effect**

An effect requested or evaluated by a candidate before its canonical acceptance
has been established.

Attempted effect does not mean provisional canonical mutation.

**Rejection**

A determination that a candidate is not canonically accepted under the
applicable protocol rules.

**Atomicity boundary**

The protocol-semantic set of effects that must become canonical together or not
become canonical together under the applicable rule.

This does not select an implementation rollback mechanism.

**Failure**

A generic analytical term for an unsuccessful validation, condition,
authorization, arithmetic, replay, resource, or other consensus-relevant path.

This term alone does not determine whether a future protocol classifies a
particular event as rejection, accepted unsuccessful execution, or another
explicit outcome.

## 4. Transaction Format Remains Unselected

Abstract transaction semantics must not be mistaken for a transaction format.

This document defines no:

- header;
- version field;
- sender field;
- input field;
- output field;
- amount field;
- nonce field;
- sequence field;
- dependency list;
- read set;
- write set;
- authorization container;
- signature container;
- fee field;
- resource field;
- expiry field;
- status field;
- transaction hash construction; or
- canonical transaction encoding.

Future concrete structures require separate authoritative specification.

## 5. Dependency Determination

Every consensus-relevant dependency that can change validity or canonical effect
must eventually be determined under deterministic protocol semantics.

The project has not selected whether dependencies are:

- explicitly declared;
- discovered while evaluating the candidate;
- derived from canonical candidate content;
- derived from referenced state;
- determined through a combination of these approaches; or
- represented by another reviewed architecture.

A future architecture must prevent two compliant implementations from reaching
different validity or state results because they discovered or represented
dependencies differently.

This requirement does not imply that all dependencies must be enumerated before
validation begins.

## 6. Dependency Categories

Model-neutral dependency analysis may distinguish categories such as:

- authorization-related facts;
- replay-related facts;
- native-value facts;
- entity-existence facts;
- version or activation facts;
- state facts needed by a condition;
- validity-affecting resource facts; and
- other protocol facts introduced by a future authoritative rule.

These are analytical categories only.

They do not define transaction fields, storage namespaces, state objects, or
separate execution modules.

## 7. Logical Observation Boundary

Consensus semantics must distinguish logical protocol observations from physical
storage operations.

A logical observation must not be defined by:

- a database key lookup;
- a cache hit or miss;
- a database page;
- an index traversal;
- a journal read;
- a language object;
- a storage-engine cursor; or
- another implementation-specific event.

Whether a future architecture exposes explicit logical read sets remains TBD.

Repeated logical observations, deduplication, caching, and implementation-local
read optimization remain architecture or implementation questions unless a
future authoritative rule makes them consensus-relevant.

## 8. Preconditions

Future transaction semantics must make validity-affecting preconditions
deterministic.

A precondition may eventually depend on authorization, replay state, native
value, entity lifecycle, protocol version, resource limits, or other
authoritative facts.

This document does not define:

- the number of preconditions;
- their encoding;
- their evaluation order;
- whether they are declared or derived;
- short-circuit behavior;
- an execution language; or
- a generic predicate system.

A precondition that is not satisfied must not create an implementation-dependent
canonical result.

## 9. Logical Effects

A future accepted transaction must have an unambiguous canonical effect under
the applicable protocol version.

Logical effects may eventually include changes relevant to:

- native DLTH;
- ownership or authorization state;
- replay exclusion;
- entity lifecycle;
- consensus metadata;
- validity-affecting resource state; or
- other explicitly adopted protocol state.

This list does not state that any particular effect category exists in every
transaction.

The exact effect representation remains TBD.

## 10. Effect Determination

Two compliant implementations given the same canonical prior state, canonical
candidate, protocol version, and other authoritative consensus inputs must
derive the same:

- acceptance or rejection result;
- consensus-relevant dependency facts and dependency-derived outcomes;
- applicable preconditions;
- canonical effects; and
- consensus-relevant failure classification.

Agreement on consensus-relevant dependency facts does not require compliant
implementations to perform identical internal observation counts, observation
order, traces, caching, deduplication, physical accesses, or other
implementation-local work unless a future authoritative rule explicitly makes
such behavior consensus-relevant.

Implementation-local storage behavior must not alter those results.

## 11. Rejection Versus Accepted Unsuccessful Outcome

This document distinguishes rejection from any hypothetical future concept of a
canonically accepted transaction containing an unsuccessful internal operation.

For a rejected candidate:

- it is not canonically accepted; and
- rejection must not leave a partial canonical state effect.

This document does **not** decide whether Dilithia will ever support an accepted
transaction whose internal operation, script, sub-operation, or other component
has an unsuccessful outcome.

If such a concept is ever proposed, authoritative specification must explicitly
define:

- why the transaction itself is still accepted;
- which effects, if any, remain canonical;
- which effects do not occur;
- how native-value conservation is preserved;
- how replay semantics behave;
- how authorization applies;
- how resources are accounted;
- how the result is represented; and
- how independent implementations reproduce it.

No such mechanism is selected here.

## 12. Minimum Failure-Atomicity Requirement

A candidate that is rejected must not leave partial canonical state mutation.

This requirement applies regardless of how much validation work occurred before
rejection.

The project does not select a rollback implementation.

Conforming implementations may eventually use different internal techniques
provided they produce the same authoritative result.

Implementation possibilities such as:

- copy-on-write;
- state overlays;
- journals;
- transactional databases;
- immutable-state construction;
- deferred writes; or
- explicit rollback

are not protocol decisions merely because they can implement atomic semantics.

## 13. Atomicity Granularity Remains a Decision

This document does not automatically declare every conceivable future
multi-operation transaction to have one universal all-or-nothing execution
model.

Instead, future authoritative semantics must define the atomicity boundary for
every adopted transaction or operation class.

For the common semantic case of one ordinary native-DLTH transfer, all
consensus effects required for that transfer's successful canonical realization
must remain mutually consistent.

A future architecture must not permit a rejected transfer to leave only some of
its required authorization, replay, native-value, or lifecycle effects
canonical.

Whether future compound transactions, batches, modules, or other constructions
exist is TBD.

## 14. Coupled State Effects

Where multiple protocol facts jointly define one canonical semantic effect, the
future atomicity contract must prevent inconsistent partial realization.

Potential analytical examples include coupling between:

- authorization and value movement;
- replay exclusion and value movement;
- native-value debit-like and credit-like semantic effects;
- consumption-like and creation-like semantic effects;
- version migration and preserved ownership;
- entity lifecycle and native-value state; or
- validity-affecting resource state and acceptance.

These examples do not select Account balances, UTXO inputs or outputs, a replay
mechanism, migration mechanism, or resource model.

## 15. Failure Stage Independence

Canonical failure semantics must not depend on an implementation accidentally
performing one validation step earlier or later than another.

Different compliant implementations may optimize evaluation order only where
the authoritative semantics permit the same result.

The protocol must not acquire accidental meaning from:

- parser order;
- database lookup order;
- signature-check order;
- hash-check order;
- cache order;
- host thread scheduling;
- local transaction staging; or
- implementation-specific early exits.

This does not require one universal validation pipeline.

## 16. Authorization Failure Boundary

Authorization failure must not create a partial canonical transaction effect.

This document does not define:

- authorization evidence;
- credential format;
- signature algorithm;
- number of authorizers;
- multisig;
- delegation;
- recovery;
- authorization grouping; or
- verification order.

Ownership and authorization architecture remains governed by its separate
decision-readiness analysis until authoritative choices are made.

## 17. Replay Failure Boundary

Replay-related rejection must not accidentally recreate or partially realize a
spendable effect.

This document does not define:

- nonce semantics;
- sequence semantics;
- consumed-output semantics;
- transaction-hash replay identity;
- one-use capability representation;
- replay-state fields; or
- canonical identity construction.

Replay and canonical identity remain governed by their separate
decision-readiness analysis.

## 18. Native-Value Failure Boundary

Every value-affecting accepted transition must satisfy the authoritative
monetary and supply invariants applicable to that transition.

A rejected candidate must not leave a partial native-value effect.

This document does not select:

- amount representation;
- amount width;
- balance representation;
- consumed-value representation;
- created-value representation;
- issuance;
- destruction;
- rewards;
- fees;
- monetary denomination;
- monetary precision; or
- supply policy.

Representation creation is not automatically issuance.

Representation removal is not automatically destruction.

## 19. Arithmetic Failure Boundary

Consensus-relevant arithmetic must be deterministic and host-independent.

Overflow and underflow behavior must eventually be explicit.

An arithmetic-boundary failure must not produce:

- wraparound;
- silent saturation;
- partial canonical effects;
- host-width-dependent validity; or
- implementation-dependent results.

This document does not select numeric widths, signedness, saturation,
modular arithmetic, accumulator structure, or maximum supply.

## 20. Missing, Absent, or Conflicting State

Transaction semantics must eventually define how a required dependency that is
missing, absent, incompatible, stale, or otherwise unsatisfied affects validity.

This document deliberately does not define what:

- absence;
- zero;
- creation;
- deletion;
- recreation;
- consumption; or
- historical existence

means for a selected state model.

Those questions belong primarily to the next entity-lifecycle gate.

No Account-missing rule or UTXO-missing-reference rule is selected here.

## 21. Resource-Limit Failure

Hostile candidates, including candidates that fail late, must remain bounded
under future authoritative resource rules.

Resource exhaustion or limit failure must not create an
implementation-dependent canonical state result.

This document does not select:

- gas;
- fees;
- resource units;
- attempt meters;
- monotonic meters;
- reservation;
- refunds;
- no-refund behavior;
- counter widths;
- numeric resource limits; or
- validation-stage budgets.

Hard resource safety, monetary economics, and transaction state semantics remain
distinct decision surfaces.

## 22. Malformed Representation and Cryptographic Failure

Malformed serialization, unsupported versions, malformed cryptographic
artifacts, cryptographic verification failure, and unsatisfied semantic
conditions must not be silently conflated.

Their future exact classification belongs to authoritative transaction,
serialization, cryptographic, and version semantics.

This document defines no malformed transaction grammar and no cryptographic
artifact format.

Rejected malformed input must not leave a partial canonical state effect.

## 23. Local Policy Is Not Canonical Failure Semantics

Local node, wallet, peer, or mempool policy must not silently become consensus
transaction semantics.

Examples of local behavior that are not selected here include:

- refusing to relay a candidate;
- local duplicate suppression;
- wallet preflight checks;
- peer scoring;
- local resource admission;
- local caching;
- mempool replacement; and
- local transaction ordering.

A candidate rejected by local policy is not automatically protocol-invalid.

No mempool architecture is selected.

## 24. Conflict and Ordering Boundary

The fourth gate must expose dependencies and effects sufficiently for later
conflict analysis.

It must not prematurely select the seventh gate's canonical conflict, ordering,
or scheduling semantics.

This document therefore does not determine:

- which transactions conflict;
- conflict-key representation;
- first-seen behavior;
- proposer-selected order;
- canonical sorting;
- dependency-graph ordering;
- rejection of conflicting transactions;
- deterministic winner rules; or
- block scheduling.

Those questions require a separate later decision artifact.

## 25. Serial and Parallel Equivalence Boundary

Any future parallel implementation must preserve the same authoritative
validity, canonical state result, protocol-version interpretation, and every
consensus-visible or validity-affecting deterministic resource result required
by the applicable authoritative rules.

Parallelism must not create a different consensus result merely because
independent implementations schedule work differently.

This document does not require parallel execution.

It does not define:

- disjointness;
- dependency graphs;
- locks;
- optimistic execution;
- speculative execution;
- parallel batches;
- conflict detection; or
- commit ordering.

Exact scheduling-equivalence requirements belong to the later conflict and
ordering gate.

## 26. Reorganization and Canonical Reapplication

Transaction effect semantics must eventually be precise enough that canonical
state can be interpreted consistently when authoritative chain history changes.

This requirement does not select:

- fork choice;
- finality;
- reorganization depth;
- rollback storage;
- undo logs;
- historical snapshots;
- transaction journals; or
- state reconstruction architecture.

Replay of a candidate and canonical reapplication after authoritative history
change remain distinct concepts.

## 27. Historical Protocol Interpretation

Historical transactions and state effects must remain interpretable under the
protocol rules applicable to them.

A future protocol version must not cause implementations to reinterpret an old
canonical effect using accidental current-version behavior.

This document does not select:

- version-dispatch representation;
- activation mechanism;
- historical-state format;
- migration format; or
- archival requirements.

## 28. Upgrade and Migration Boundary

Protocol evolution may change future transaction semantics only through the
authoritative upgrade process.

Any future migration affecting transaction dependencies or state effects must
preserve applicable constitutional ownership and state-preservation guarantees.

Migration must not silently:

- create native value;
- destroy native value;
- reset replay protection;
- bypass authorization;
- reinterpret prior canonical effects inconsistently; or
- introduce implementation-dependent state.

The exact migration mechanism remains TBD.

## 29. Minimal Account Candidate Questions

For Minimal Account, later candidate mapping must answer questions such as:

- Which logical facts does an ordinary transfer depend upon?
- Which logical facts can change?
- Which dependency is replay-related, if any?
- Which authorization facts are inspected?
- Which native-value facts are inspected?
- What state conditions can cause rejection?
- Which effects must commit together?
- What happens when a required account-like entity is absent?
- How does arithmetic failure affect the candidate?
- Which dependency or effect concepts are required for conflict analysis?
- Which persistent metadata, if any, is necessary?
- How are historical versions interpreted?

These questions do not select:

- balances;
- account identifiers;
- nonces;
- sequence numbers;
- persistent authorization metadata; or
- account creation semantics.

## 30. Minimal UTXO Candidate Questions

For Minimal UTXO, later candidate mapping must answer questions such as:

- Which logical facts does an ordinary transfer depend upon?
- Which referenced facts must exist?
- Which ownership conditions are inspected?
- Which native-value facts are inspected?
- Which facts determine whether a reference remains exercisable?
- What conditions can cause rejection?
- Which effects must commit together?
- What happens when required referenced state is absent?
- How does arithmetic failure affect the candidate?
- Which dependency or effect concepts are required for conflict analysis?
- Which persistent metadata, if any, is necessary?
- How are historical versions interpreted?

These questions do not select:

- transaction inputs;
- outputs;
- output identifiers;
- explicit spent flags;
- consumption representation;
- change-output rules; or
- output creation semantics.

## 31. Account and UTXO Neutrality

Neither candidate currently receives a weaker atomicity requirement.

Neither candidate currently receives a stronger failure guarantee.

Neither candidate may rely on an unstated transaction shape.

Neither candidate may assume its preferred replay mechanism.

Neither candidate may assume its preferred entity-lifecycle semantics.

Neither candidate may redefine the external semantic case to simplify its own
mapping.

Candidate-specific dependencies, logical observations, effects, failure paths,
and persistent-state exposure are comparison outputs until the relevant
architecture is explicitly frozen for evidence.

No current evidence justifies ranking Account or UTXO on transaction atomicity or
failure semantics.

## 32. Adversarial Scenario Matrix

Future evidence should include equivalent semantic scenarios for both
candidates.

At minimum, analysis should cover:

| Scenario | Model-neutral property |
|---|---|
| Ordinary valid native-DLTH transfer | Deterministic acceptance and complete canonical effect |
| Authorization failure | No unauthorized or partial canonical effect |
| Replay-related failure | No duplicate spendable effect and no partial mutation |
| Native-value arithmetic boundary failure | No wrap, saturation, or partial monetary effect |
| Missing required state | Deterministic outcome without implementation-dependent mutation |
| Failure after authorization work | No partial canonical state |
| Failure after state observation | No partial canonical state |
| Failure after substantial cryptographic work | Bounded hostile work and no partial canonical effect |
| Late resource-limit failure | Deterministic failure semantics and bounded exposure |
| Duplicate presentation | Distinct from replay unless authoritative rules equate them |
| Two mutually incompatible candidates | Expose dependencies needed for later conflict analysis without selecting winner semantics |
| Two independent candidates | Expose dependencies needed for later scheduling-equivalence analysis |
| Reorganization and canonical reapplication | Deterministic authoritative state interpretation |
| Historical-version transaction | Interpretation under the applicable protocol version |
| Protocol-version transition | No accidental reinterpretation of prior effects |
| Ownership/cryptographic migration | Legitimate value remains protected without privileged authority |
| Large dependency population | Bounded logical validation exposure |
| Large effect population | Bounded logical mutation exposure |
| Malformed or non-canonical representation | Rejection without partial canonical effect |
| Unsupported cryptographic version | Deterministic rejection or future explicitly specified version behavior |
| Hostile candidate designed to fail late | Bounded attempt exposure before canonical commit |

Hypothetical future transaction classes may add scenarios only after their
external semantic contract is explicit.

## 33. Evidence Required Before Candidate Comparison

Account/UTXO transaction comparison requires evidence based on the same external
semantic cases.

Candidate mappings must make explicit:

- semantic dependencies;
- preconditions;
- authorization assumptions;
- replay assumptions;
- native-value assumptions;
- logical observations;
- logical effects;
- atomicity boundary;
- failure classes;
- historical-version assumptions;
- reorganization assumptions;
- resource assumptions; and
- unresolved lifecycle assumptions.

Evidence must distinguish:

- common external requirements;
- candidate-specific representation choices;
- candidate-specific optimization;
- architecture assumptions;
- measured implementation behavior; and
- authoritative protocol requirements.

Implementation measurements alone cannot establish consensus semantics.

Independent semantic review should check that neither mapping received hidden
advantages.

Conclusions must remain mapping-limited until broader evidence exists.

## 34. Formal-Verification Obligations

Future formal or executable evidence should be able to express properties such
as:

- deterministic validity;
- deterministic canonical effect;
- authorization correctness;
- replay exclusion;
- applicable native-value conservation;
- rejection without partial canonical state;
- arithmetic safety;
- version-correct historical interpretation;
- equivalence between permitted implementation strategies; and
- later, once defined, deterministic conflict and scheduling equivalence.

A useful abstract relation may eventually resemble:

`Apply(version, prior_state, candidate) -> result`

but this document does not standardize that function, its types, or its result
representation.

A proof model must not silently encode Account or UTXO assumptions into the
common specification.

## 35. Premature-Commitment Matrix

| Classification | Item |
|---|---|
| AUTHORITATIVE NOW | Constitution and currently adopted Formal Specification constraints |
| SAFE MODEL-INDEPENDENT REQUIREMENT | deterministic validity and effects |
| SAFE MODEL-INDEPENDENT REQUIREMENT | rejected validation leaves no partial canonical state |
| SAFE MODEL-INDEPENDENT REQUIREMENT | implementation-independent logical semantics |
| SAFE MODEL-INDEPENDENT REQUIREMENT | explicit overflow and underflow behavior for future consensus arithmetic |
| SAFE MODEL-INDEPENDENT REQUIREMENT | bounded hostile validation under future authoritative resource rules |
| ABSTRACT ONLY | transaction dependency |
| ABSTRACT ONLY | logical observation |
| ABSTRACT ONLY | logical effect |
| ABSTRACT ONLY | precondition |
| ABSTRACT ONLY | atomicity boundary |
| CONDITIONAL | accepted transaction containing an unsuccessful internal operation |
| CANDIDATE | Minimal Account |
| CANDIDATE | Minimal UTXO |
| BLOCKED | transaction fields and exact format |
| BLOCKED | Account nonce or sequence semantics |
| BLOCKED | UTXO input/output and consumption semantics |
| BLOCKED | read-set or write-set encoding |
| BLOCKED | conflict keys |
| BLOCKED | canonical transaction ordering |
| BLOCKED | parallel scheduling |
| BLOCKED | execution VM or scripting system |
| BLOCKED | rollback mechanism |
| BLOCKED | gas, fees, meters, refunds, and numeric resource limits |
| DEFERRED | wallet, mempool, and peer-local policy |
| DO NOT ADOPT | implementation-specific database behavior as consensus semantics |
| DO NOT ADOPT | partial canonical mutation from rejected validation |
| DO NOT ADOPT | host scheduling as consensus ordering |
| DO NOT ADOPT | hidden candidate-specific assumptions in common comparison cases |

## 36. Fourth-Gate Decision Gates

The fourth state-model gate remains blocked until sufficiently concrete,
reviewed abstract answers exist for:

1. the semantic scope of a transaction;
2. consensus-relevant dependency determination;
3. validity-affecting precondition semantics;
4. logical observation semantics;
5. canonical effect semantics;
6. the minimum atomicity boundary for ordinary native-DLTH transfer;
7. rejection and failure classification;
8. treatment of any future accepted-unsuccessful outcome, if such a concept is
   proposed;
9. authorization, replay, and native-value coupling requirements;
10. arithmetic-failure behavior;
11. historical-version and reapplication requirements;
12. hostile late-failure and validity-affecting resource boundaries; and
13. sufficient abstraction to expose later conflict and ordering requirements
    without selecting them.

These gates require decision-ready semantics and evidence.

They do not require a concrete transaction encoding or state-model selection.

The existence of this document alone satisfies none of them automatically.

## 37. Formal Specification Boundary

Future authoritative specification will need to define transaction semantics
where they become consensus-relevant for the architecture ultimately selected.

Depending on the selected architecture, this may include:

- canonical transaction representation;
- protocol-version interpretation;
- authorization scope and binding;
- replay-relevant binding;
- native-value effect semantics;
- transaction dependencies;
- preconditions;
- canonical state effects;
- atomicity boundaries;
- rejection and any other accepted outcome classifications;
- arithmetic failure behavior;
- entity-lifecycle interaction;
- canonical conflict and ordering behavior;
- reorganization and reapplication semantics;
- validity-affecting resource semantics; and
- canonical serialization of adopted structures.

Some items are architecture-contingent.

Their appearance here does not independently constitutionally require a
particular transaction, state, commitment, execution, conflict, or resource
construction.

This document defines none of those normative mechanisms.

## 38. Threat Model Boundary

Creation of this document alone does not justify a Threat Model change.

The current generic threat surface already includes concerns such as:

- invalid-candidate late failure;
- nondeterministic validation;
- hostile resource amplification;
- state workload and persistent growth;
- arithmetic failure;
- failure atomicity;
- version drift; and
- implementation-dependent rollback or accounting.

Future review should separately determine whether transaction analysis discovers
a genuinely new generic threat class.

Candidate-specific Account or UTXO threats should not be promoted into generic
Threat Model requirements merely because one candidate exhibits them.

A Threat Model update should be proposed only when supported by an actual new
generic threat or by a selected architecture requiring concrete threat treatment.

## 39. Complete TBD Register

The following remain unresolved:

- state model;
- transaction semantic type;
- transaction format;
- transaction fields;
- transaction identity;
- dependency representation;
- whether dependencies are declared, discovered, derived, or combined;
- read-set representation;
- write-set representation;
- precondition representation;
- effect representation;
- validation stages;
- validation order;
- atomicity granularity for any future compound transaction;
- whether accepted-unsuccessful execution exists;
- status or outcome representation;
- rollback implementation;
- overlay architecture;
- journal architecture;
- database transaction use;
- entity existence semantics;
- entity creation semantics;
- entity deletion semantics;
- entity recreation semantics;
- Account balance representation;
- Account nonce or sequence semantics;
- UTXO input representation;
- UTXO output representation;
- UTXO consumption representation;
- replay mechanism;
- canonical transaction identity;
- native-DLTH representation;
- amount width;
- monetary denomination;
- monetary precision;
- issuance;
- rewards;
- destruction or burn;
- fees;
- resource units;
- resource limits;
- resource counters;
- refunds;
- conflict definition;
- conflict representation;
- canonical transaction ordering;
- block scheduling;
- parallel execution;
- execution VM;
- scripting language;
- state commitment;
- snapshot format;
- fork choice;
- finality;
- reorganization depth;
- historical-storage architecture;
- activation mechanics; and
- migration mechanics.

No item is resolved by appearing in this register.

## 40. Current Project Impact

Creation of this document alone has the following effect:

```text
Account selected:
NO

UTXO selected:
NO

Hybrid selected:
NO

State-model decision:
NOT MADE

Transaction format selected:
NO

Transaction identity selected:
NO

Transaction fields selected:
NO

Dependency representation selected:
NO

Declared-dependency architecture selected:
NO

Discovered-dependency architecture selected:
NO

Read-set representation selected:
NO

Write-set representation selected:
NO

Execution model selected:
NO

Execution VM selected:
NO

Script system selected:
NO

Account nonce or sequence selected:
NO

UTXO input/output structure selected:
NO

Replay mechanism selected:
NO

Native DLTH representation selected:
NO

Failure-status representation selected:
NO

Accepted-unsuccessful transaction semantics selected:
NO

Rollback mechanism selected:
NO

Database transaction mechanism selected:
NO

Conflict rule selected:
NO

Canonical ordering selected:
NO

Parallel scheduling selected:
NO

Fee mechanism selected:
NO

Gas mechanism selected:
NO

Resource meter selected:
NO

State commitment selected:
NO

Formal Specification update:
NOT JUSTIFIED BY THIS DOCUMENT

PROJECT_STATE update:
NOT JUSTIFIED BY THIS DOCUMENT

Threat Model update:
NOT JUSTIFIED BY THIS DOCUMENT

Consensus implementation change:
NONE
```

## 41. Next Decision Boundary

Completion and independent review of this document may clarify the fourth
state-model decision gate:

> Abstract transaction dependencies, state effects, atomicity, and failure.

The existence, completion, or review of this document does not by itself satisfy
the fourth state-model decision gate.

It also does not satisfy the other remaining state-model gates.

The next model-neutral analytical area is:

> Entity existence, creation, deletion, recreation, and historical meaning.

That analysis must not use this document to silently select:

- Account;
- UTXO;
- account creation semantics;
- UTXO creation or consumption semantics;
- stable object identity;
- deletion semantics;
- recreation semantics;
- pruning;
- replay reset behavior; or
- a transaction format.

## 42. Conclusion

- This document is non-normative.
- The Formal Specification remains authoritative for protocol behavior.
- Transaction format remains unselected.
- Transaction identity remains unselected.
- Transaction fields remain unselected.
- Dependency representation remains unselected.
- Whether dependencies are declared, discovered, derived, or combined remains
  TBD.
- Logical observation does not mean physical database read.
- Logical effect does not mean physical database write.
- Rejected validation must not leave partial canonical state.
- No rollback implementation is selected.
- No database transaction mechanism is selected.
- No accepted-unsuccessful execution model is selected.
- No Account nonce or sequence mechanism is selected.
- No UTXO input/output or consumption mechanism is selected.
- Replay mechanism remains unselected.
- Entity lifecycle semantics remain unresolved.
- Conflict and ordering semantics remain unresolved.
- Parallel scheduling remains unresolved.
- Fees, gas, resource meters, and numeric resource limits remain unresolved.
- Minimal Account and Minimal UTXO remain co-equal candidates.
- Candidate ranking remains blocked.
- State-model decision remains NOT MADE.
- The fourth state-model decision gate is not satisfied merely by this document
  existing.
- This document defines no consensus rule.
