# Dilithia Canonical Conflict, Ordering, and Scheduling-Equivalence Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records model-neutral requirements, unresolved questions,
> comparison variables, adversarial scenarios, and evidence gates for canonical
> conflict, ordering, and scheduling-equivalence analysis.
>
> It defines no consensus rule, resolves no Formal Specification TBD, selects no
> state model, transaction format, conflict-key representation, read-set or
> write-set encoding, access-list format, first-seen rule, proposer-order rule,
> canonical sorting rule, deterministic winner mechanism, scheduler, dependency
> graph, lock scheme, optimistic or speculative execution architecture, parallel
> batch mechanism, rollback mechanism, resource meter, fee rule, state
> commitment, consensus algorithm, fork choice, finality mechanism, or
> state-model ranking, and it does not constitute protocol adoption.

## 1. Status, Authority, Purpose, and Scope

Dilithia is Pre-Genesis. Minimal Account and Minimal UTXO remain co-equal
candidates. The state-model decision remains **NOT MADE**.

The authoritative protocol sources applicable to this document are:

1. Dilithia Technical Constitution
2. Dilithia Formal Specification
3. Validly adopted and activated HIP / Super HIP material, where applicable

Conformance vectors or tests have normative force only where their normative
role is established by an authoritative protocol source.

Implementation, `PROJECT_STATE.md`, non-normative decision-readiness,
comparison, threat, resource, and benchmark documents, and AI or other design
discussion are supporting evidence only. They do not independently create
protocol authority.

Where supporting material conflicts with an authoritative protocol source, the
authoritative protocol source controls.

The exact seventh state-model decision gate is:

> **Canonical conflict, ordering, and scheduling-equivalence requirements.**

This document exists to make that gate precise enough for fair Account/UTXO
comparison without silently choosing a state model or an execution architecture.

## 2. Existing Authoritative Determinism Boundary

The Constitution requires consensus-critical operations to produce identical
results across compliant implementations and forbids consensus correctness from
depending on implementation accidents such as local clocks, floating-point
behavior, undefined behavior, operating-system behavior, hardware architecture,
or implementation-specific optimization.

The current Formal Specification applies the same determinism direction to
consensus-critical protocol behavior and canonical serialization.

This document does not convert those general determinism requirements into a
specific transaction-ordering mechanism. It only derives decision-readiness
questions that a future authoritative design must answer consistently with them.

## 3. Model-Neutral Safety Properties

Any future conflict and ordering design must be compatible with the following
model-neutral properties:

- the same authoritative inputs produce the same validity result;
- the same authoritative inputs produce the same canonical state result;
- replay or double-spend handling does not depend on local arrival accidents;
- rejected validation leaves no partial canonical state effect;
- authorization, native-value conservation, lifecycle, and historical-version
  rules remain mutually consistent;
- implementation scheduling does not change a consensus-visible result;
- validity-affecting or consensus-visible deterministic resource results do not
  change merely because work is scheduled differently; and
- local storage, caching, thread count, batching speedup, or database behavior
  does not become hidden consensus semantics.

These are decision requirements, not adopted transition mechanisms.

## 4. Terminology

For this document:

| Term | Decision-readiness meaning |
|---|---|
| **Candidate** | A canonical or potentially canonical protocol input being evaluated under a declared protocol version and prior state |
| **Semantic effect** | An abstract protocol-relevant effect independent of Account/UTXO representation |
| **External semantic conflict relation** | A model-neutral relation declared outside candidate mappings stating that two semantic effects cannot coexist, or that a declared ordering relation is material, under the same versioned semantic case |
| **Realized conflict** | A conflict, overlap, contention, or scheduling constraint produced by a concrete candidate mapping under a frozen external semantic case |
| **Mutual-exclusion conflict** | A symmetric semantic incompatibility in which the relevant effects cannot coexist under the declared case |
| **Order-sensitive relation** | A relation in which a declared sequence or partial order is part of the semantic case or comparison profile |
| **Independent relation** | A relation in which the semantic case does not require one effect to exclude or order another |
| **Canonical outcome** | The protocol-visible validity and state consequence required by the applicable authoritative semantics |
| **Schedule** | An implementation execution ordering of evaluation work; not automatically a consensus transaction order |
| **Scheduling-equivalence** | Equality of all required consensus-visible or validity-affecting results across permitted serial or parallel schedules |
| **Local policy** | Wallet, peer, mempool, cache, ingress, replacement, or other implementation-local behavior that is not protocol semantics unless expressly adopted |

These terms do not define transaction fields, state fields, conflict keys, or a
scheduler.

## 5. External Semantic Conflict Comes Before Candidate Mapping

Fair Account/UTXO comparison must freeze the same external semantic conflict
relation before candidate mapping.

A candidate mapping must not redefine an external semantic conflict merely
because one representation makes the relation narrower, broader, cheaper, or
easier to detect.

Candidate-representation conflict breadth, overlap, contention, or amplification
is a candidate output.

This preserves the distinction between:

- common semantic workload;
- candidate mapping;
- candidate-specific realized conflict; and
- implementation measurement.

## 6. No State-Model Conflict Stereotypes

This document rejects label-derived assumptions such as:

- "UTXO conflicts are inherently local";
- "Account conflicts are inherently global";
- "one Account means one conflict domain";
- "one UTXO input means one independent conflict";
- "shared ownership metadata necessarily implies shared scheduling"; or
- "different physical records necessarily imply independent semantic effects."

Such claims require explicit candidate mappings and evidence under frozen
external semantics.

## 7. External Conflict Density and Internal Conflict Are Different Variables

An externally defined semantic conflict density may be used as a common workload
parameter.

Internal source count, realized logical conflict breadth, conflict-set
cardinality, record or unit overlap, schema-induced conflict amplification, and
candidate-specific scheduling constraints remain candidate outputs unless a
future authoritative rule explicitly makes one of them part of protocol
semantics.

Fairness does not require Account and UTXO to produce equal internal conflict
counts.

## 8. Conflict Is Not Mere Physical Overlap

Physical database overlap is not by itself a protocol conflict definition.

Two operations may touch:

- the same physical page;
- the same cache line;
- the same database table;
- the same implementation lock;
- the same allocator;
- different physical records representing one logical dependency; or
- one physical record containing multiple logically independent facts.

Conflict analysis must therefore be expressed first in logical protocol terms.

## 9. Required Conflict Distinctions

Future design and evidence must distinguish at least:

- exact duplicate presentation;
- semantic replay;
- two different candidates attempting the same incompatible spendable effect;
- candidates sharing only some authority relation;
- candidates sharing only some logical state dependency;
- candidates with overlapping logical effects;
- candidates whose effects are order-sensitive but not mutually exclusive;
- independent disjoint candidates;
- malformed or non-canonical candidates;
- protocol-invalid candidates unrelated to another candidate; and
- local-policy conflicts that are not protocol conflicts.

These categories may overlap only where a future authoritative specification
defines the relationship unambiguously.

## 10. Mutual Exclusion and Order Sensitivity Are Separate

A pure mutual-exclusion relation is not automatically an ordering relation.

An order-sensitive relation is not automatically a mutual-exclusion relation.

Future analysis must state whether a case means:

- both effects may coexist;
- both effects cannot coexist;
- one effect may be valid only relative to the other;
- a sequence is semantically material;
- only a partial order is semantically material; or
- another deterministic relation is required.

No total-order requirement is inferred merely from the existence of conflict.

## 11. Canonical Conflict Outcome Must Be Deterministic

For every adopted conflict relationship, the authoritative protocol must
eventually determine the canonical result.

Possible future result classes may include, without selection here:

- both candidates invalid;
- exactly one permitted according to a canonical rule;
- one invalid relative to another;
- both permitted;
- both permitted only under a declared order;
- a deterministic combined result; or
- another explicitly specified deterministic result.

This document selects none of those result classes as a default.

## 12. No Canonical Winner Mechanism Is Selected

This document does not select:

- first-seen wins;
- last-seen wins;
- proposer-selected winner;
- transaction-identifier sorting;
- fee-priority ordering;
- lexical ordering;
- timestamp ordering;
- account sequence ordering;
- UTXO-input ordering;
- randomized ordering;
- auction ordering; or
- any other winner mechanism.

If a future design makes an ordering input authoritative, that choice must be
explicitly specified and versioned. Network arrival timing or implementation
accident must not silently become the rule.

## 13. Transaction Order and Execution Schedule Are Distinct

A protocol-defined transaction order, if one is eventually adopted, is a
consensus concept.

An implementation execution schedule is an implementation concept unless the
authoritative protocol expressly makes some scheduling relation observable.

A compliant implementation may evaluate work in a different internal order only
when all required consensus-visible and validity-affecting outcomes remain
equivalent.

This document selects neither a canonical transaction order nor an execution
schedule.

## 14. Dependency Determination Remains Model-Neutral

Every consensus-relevant dependency that can change validity or canonical effect
must eventually be determined under deterministic protocol semantics.

This document does not select whether dependencies are:

- explicitly declared;
- discovered while evaluating the candidate;
- derived from canonical state;
- derived from transaction semantics;
- derived from versioned metadata; or
- determined by another reviewed mechanism.

Two compliant implementations must not reach different validity or state results
because they discover or represent dependencies differently.

## 15. No Mandatory Predeclared Access Set

Deterministic dependency semantics do not imply that every dependency must be
enumerated before validation begins.

This document does not select:

- read-set encoding;
- write-set encoding;
- access-list encoding;
- conflict-key encoding;
- state-key locking;
- declared object lists;
- declared account lists;
- declared UTXO lists beyond any future transaction semantics; or
- a pre-execution dependency graph.

Any such structure would require separate design and review.

## 16. Logical Observation and Logical Effect Boundary

Conflict analysis must use logical protocol observations and logical effects, not
physical storage operations.

A candidate's relevant logical dependency or effect may include future,
still-unselected facts related to:

- authorization;
- replay exclusion;
- native value;
- entity existence or lifecycle;
- version interpretation;
- transaction preconditions;
- transaction state effects; or
- another adopted consensus-relevant condition.

This section does not define any concrete state schema.

## 17. Atomicity Remains Binding During Conflict Resolution

Conflict handling must remain consistent with the transaction atomicity and
failure requirements.

A rejected candidate must not leave a partial canonical state effect merely
because some work was scheduled, speculated, locked, cached, or tentatively
applied before conflict resolution completed.

This document does not select rollback, journaling, copy-on-write, shadow state,
or any other implementation mechanism.

## 18. Replay and Conflict Are Distinct

Replay exclusion and conflict handling are related but distinct.

A replay is not defined merely by two candidates conflicting.

Two conflicting candidates are not necessarily replays.

Canonical reapplication after authoritative reversion is distinct from replay
while an earlier exercise remains authoritative.

Gate 7 must therefore preserve the replay and historical-interpretation
boundaries established by earlier decision-readiness work without inventing a
new replay mechanism.

## 19. Lifecycle and Conflict Are Distinct

Entity absence, existence, creation, deletion, recreation, consumption, and
historical interpretation may affect whether two effects conflict, but lifecycle
semantics are not selected by this document.

A lifecycle transition must not be reinterpreted solely to produce a convenient
conflict rule.

Conflict analysis must consume the lifecycle assumptions of the declared case or
profile rather than silently invent them.

## 20. Authorization and Conflict Are Distinct

Shared authorizers, credentials, keys, proofs, signatures, or authorization
objects do not automatically imply a semantic state conflict.

Likewise, different authorization evidence does not automatically imply
independent effects.

Conflict analysis must distinguish:

- semantic authority relations;
- authorization scope;
- evidence multiplicity;
- verification-operation count; and
- logical state/effect dependency.

No authorization architecture is selected here.

## 21. PQ Artifact Assumptions Must Not Define Conflict by Accident

PQ artifact size, verification count, grouping, batching, aggregation,
credential reuse, evidence reuse, version coexistence, and cryptographic
algorithm profile may affect implementation cost or candidate outputs.

They must not silently redefine the external semantic conflict relation.

If a cryptographic feature changes the externally intended semantic contract,
that change belongs in a distinct declared case or profile and must be applied
symmetrically to both candidates.

## 22. Local Policy Is Not Canonical Conflict Semantics

Local node, wallet, peer, or mempool policy must not silently become consensus
conflict or ordering semantics.

Examples of local behavior include:

- duplicate suppression;
- wallet preflight;
- peer scoring;
- local resource admission;
- local caching;
- mempool replacement;
- speculative prevalidation; and
- local transaction ordering.

A candidate rejected, delayed, replaced, or deprioritized by local policy is not
automatically protocol-invalid.

No mempool architecture is selected.

## 23. Proposer Influence Must Be Explicit or Irrelevant

Proposer behavior must not accidentally determine consensus semantics.

Future protocol design must make one of the following kinds of relationship
explicit:

- proposer-supplied order is an authoritative input under a specified rule;
- proposer-supplied order is constrained by a canonical relation;
- proposer-supplied order is semantically irrelevant within an equivalence
  class; or
- another deterministic relationship is adopted.

This document selects none of these.

Observed proposal order, network arrival order, or implementation iteration order
must not acquire consensus meaning by precedent.

## 24. Ordering Profiles for Evidence Are Not Protocol Rules

Non-normative experiments may use declared ordering profiles such as:

- no order assertion;
- declared sequence; or
- declared partial order.

Those are experimental assumptions only unless separately adopted through the
authoritative protocol process.

A serial experimental oracle does not imply serial consensus execution, block
ordering, or a consensus ordering protocol.

## 25. Serial and Parallel Scheduling-Equivalence Boundary

Permitted serial and parallel implementations must preserve the same
authoritative:

- validity result;
- canonical state result;
- replay/conflict interpretation required by the case;
- protocol-version and historical interpretation; and
- every consensus-visible or validity-affecting deterministic resource result
  required by applicable authoritative rules.

Parallelism must not create a different consensus result merely because
independent implementations schedule work differently.

This document does not require parallel execution.

## 26. Internal Trace Equality Is Not Required

Scheduling-equivalence does not require identical:

- thread interleavings;
- worker counts;
- lock acquisition order;
- speculative paths;
- cache hits or misses;
- allocator activity;
- database reads at the physical layer;
- batching speedup;
- SIMD behavior;
- temporary object layout; or
- wall-clock timing.

These may differ while authoritative outcomes remain identical.

If a future rule makes any currently internal property consensus-visible, that
property must then be defined deterministically and versioned.

## 27. Resource Equivalence Is Limited to Protocol-Relevant Results

Resource semantics must not depend on threads, host scheduling, batching speedup,
SIMD, cache state, allocator behavior, database backend, compiler optimization,
CPU architecture, or measured timing.

Gate 7 requires equivalence only for deterministic resource results that are
consensus-visible or validity-affecting under applicable authoritative rules.

This document does not select:

- resource units;
- resource dimensions;
- counter widths;
- block budgets;
- transaction budgets;
- fees;
- refunds;
- rent;
- pricing; or
- numeric limits.

Those remain within the later resource-bound decision area or other future
authoritative design.

## 28. Failure Results Must Be Schedule-Independent

For the same authoritative inputs, a failure or rejection result must not depend
on implementation scheduling.

In particular, scheduling must not cause one compliant implementation to:

- accept while another rejects;
- leave a partial canonical effect while another leaves none;
- classify the same authoritative conflict differently;
- consume replay-related canonical state differently;
- alter native-value conservation; or
- produce a different validity-affecting deterministic resource result.

No rollback implementation is selected.

## 29. Optimistic and Speculative Execution Remain Unselected

This document does not select:

- optimistic concurrency control;
- speculative execution;
- speculative state mutation;
- transaction abort/retry;
- validation replay;
- software transactional memory;
- lock-free structures;
- deterministic concurrency control; or
- speculative parallel batches.

Any future proposal must prove scheduling-equivalence against the authoritative
semantic contract rather than redefine that contract around the optimization.

## 30. Locks and Dependency Graphs Remain Implementation Candidates Only

No lock, dependency-graph, or scheduling topology is protocol-selected.

Future implementations may explore:

- coarse or fine logical locks;
- dependency graphs;
- topological execution;
- worker pools;
- static partitioning;
- dynamic work stealing;
- optimistic detection;
- serial fallback; or
- other techniques,

provided none changes authoritative results.

A future protocol may eventually specify a canonical dependency relation without
requiring any one validator data structure.

## 31. Deterministic Commit Behavior Must Be Defined Before It Is Relied Upon

If future parallel or speculative execution requires a distinction between
evaluation order and canonical commit order, the authoritative semantics must
make the consensus consequence unambiguous.

This document does not select:

- commit queues;
- commit barriers;
- deterministic commit sorting;
- transactional journals;
- retry priority;
- conflict epochs; or
- block-internal commit protocols.

Implementation convenience cannot substitute for an authoritative rule.

## 32. Same Semantic Case, Different Candidate Realization

For a paired Account/UTXO comparison, both candidates must receive the same:

- intended semantic effects;
- external semantic conflict relation;
- applicable external ordering assumptions;
- applicable authorization workload assumptions;
- protocol-version assumptions;
- lifecycle assumptions; and
- relevant cryptographic profile assumptions.

Candidate mappings may legitimately differ in:

- logical source count;
- logical reads and writes;
- realized conflict breadth;
- overlap cardinality;
- representation-induced contention;
- validation-stage position;
- scheduling constraints;
- artifact counts;
- bytes; and
- implementation performance.

Those differences are evidence, not unfairness, when the external contract is
frozen symmetrically.

## 33. Minimal Account Candidate Questions

Before Minimal Account can be compared under a conflict/order workload, the
mapping must answer, without assuming an Account winner:

- Which logical facts are read or changed?
- Which replay-related facts, if any, participate?
- Which authority or ownership facts participate?
- Which lifecycle facts can affect coexistence or order sensitivity?
- Can multiple credits to one recipient commute under every applicable
  arithmetic, lifecycle, and failure condition?
- Which candidate-specific overlaps arise from the chosen Account mapping?
- Does a broad record representation create conflict amplification?
- Can independent semantic effects be evaluated without changing authoritative
  results?
- Which ordering assumptions come from the external case rather than the Account
  representation?
- Which invalid or late-failure cases expose order sensitivity?
- Which realized conflicts are mapping outputs rather than semantic inputs?

This section selects no account field, nonce, sequence, account lock, account
creation rule, or account-wide conflict domain.

## 34. Minimal UTXO Candidate Questions

Before Minimal UTXO can be compared under a conflict/order workload, the mapping
must answer, without assuming a UTXO winner:

- Which referenced logical facts are read or changed?
- Which replay or one-use facts, if any, participate?
- Which authority or ownership facts participate?
- Which lifecycle facts can affect coexistence or order sensitivity?
- Which candidate-specific overlaps arise from the chosen UTXO mapping?
- Are multiple semantic effects represented through shared or independent
  references?
- Which dependency chains affect validation or scheduling?
- Which ordering assumptions come from the external case rather than the UTXO
  representation?
- Which missing, conflicting, or invalid references expose late failure?
- Which realized conflicts are mapping outputs rather than semantic inputs?

This section selects no outpoint format, input/output identity, consumption rule,
UTXO lock, input-conflict rule, or output ordering rule.

## 35. Independent and Commutative Cases Must Remain Visible

Evidence must include cases where semantic effects are independent or
potentially commutative, not only conflict-heavy cases.

Internal overlap must not redefine the external semantic conflict relation or be
treated as a correctness failure merely because it differs between candidate
mappings.

Genuine representation-induced contention, serialization, conflict
amplification, and performance effects remain reportable candidate outputs under
frozen external semantics.

Conversely, an implementation must not claim parallel safety merely because a
benchmark case happened not to expose an order-sensitive edge.

Commutativity claims require explicit semantic conditions and evidence.

## 36. Conflict Amplification Is a Candidate Output

Candidate representation may transform one external semantic conflict relation
into a broader realized contention surface.

Evidence may report:

- conflict-set cardinality;
- logical overlap;
- representation-induced serialization;
- false or conservative conflict detection;
- schema-induced coupling;
- dependency fan-in or fan-out; and
- scheduling constraints.

Such evidence does not by itself select or rank a state model.

## 37. Conflict Detection Correctness and Conflict Detection Cost Are Separate

A future conflict-detection method must be judged separately on:

1. semantic correctness; and
2. resource/performance cost.

A faster detector is not correct merely because it is fast.

A conservative detector that serializes independent work may preserve consensus
correctness while producing different performance evidence.

An unsound detector that permits different authoritative outcomes is not an
acceptable optimization.

No detector is selected here.

## 38. Adversarial Conflict Scenarios

Future Gate-7 evidence should include model-neutral adversarial families such as:

- many candidates targeting one incompatible semantic effect;
- many candidates sharing only one of several dependencies;
- many semantically independent candidates that collide only because of a
  candidate representation;
- deep dependency chains;
- broad dependency fan-out;
- malformed candidates mixed with valid conflicting candidates;
- late-failing candidates mixed with valid candidates;
- repeated duplicate presentation;
- replay-like material that must remain distinct from ordinary conflict;
- mixed protocol-version or cryptographic-profile cases where applicable; and
- schedules intentionally varied to search for consensus divergence.

Exact numeric sizes and resource bounds remain outside this gate unless supplied
by an already authoritative rule.

## 39. Serial Reference Execution

A simple serial implementation may serve as a non-normative audit oracle when
its semantic contract is explicitly frozen.

Such an oracle may help detect:

- schedule-dependent validity;
- schedule-dependent state;
- schedule-dependent replay handling;
- schedule-dependent conflict outcome; or
- schedule-dependent deterministic resource results.

The oracle does not become consensus authority merely because it is simple or
serial.

## 40. Parallel Evidence

A parallel implementation used for evidence must declare enough information to
reproduce the semantic comparison without turning implementation choices into
protocol assumptions.

Relevant evidence may include:

- realized parallelism;
- conflict rate;
- retry or abort count, if the implementation has such concepts;
- scheduling constraints;
- temporary memory;
- logical work;
- wall-clock performance; and
- divergence tests against the semantic oracle.

Timing and host-specific speedups remain implementation evidence.

## 41. Evidence Freeze Requirements

Before a formal paired conflict/order campaign, freeze at least:

- semantic case identity;
- protocol/version assumptions;
- external semantic conflict relation;
- any intrinsic or profile-level ordering relation;
- candidate mappings;
- authorization profile;
- lifecycle assumptions;
- cryptographic profile where material;
- logical-access definitions needed by the claim;
- candidate conflict model used to report realized conflict outputs;
- correctness oracle or conformance procedure;
- measurement procedure; and
- result grammar and provenance requirements where applicable.

Candidate mappings must not redefine the external conflict relation after
results are observed.

## 42. Evidence Claims Must Bind Their Scope

A conflict or ordering result must state whether it is:

- semantic;
- mapping-derived;
- implementation-derived;
- resource-related;
- performance-related;
- version-specific;
- profile-specific; or
- exploratory.

Statements such as "Account has more conflicts" or "UTXO parallelizes better"
are incomplete unless the external workload, candidate mappings, conflict model,
ordering profile, and implementation conditions are identified.

## 43. Historical Protocol Versions

Historical transactions and state effects must remain interpretable under the
protocol rules applicable to them.

A future change to conflict, ordering, or scheduling-equivalence semantics must
not silently reinterpret earlier canonical history.

This document does not define activation, migration, reorganization, or
historical replay mechanics.

## 44. Reorganization Boundary

Canonical reapplication after authoritative history change is distinct from:

- duplicate presentation;
- replay while the earlier effect remains authoritative;
- ordinary conflict between contemporaneous candidates; and
- local mempool replacement.

Conflict/order analysis must not invent reorganization semantics.

Any future reorganization mechanism must preserve the applicable authoritative
validity, state, replay, lifecycle, and historical-interpretation rules.

## 45. Authenticated-State Boundary

Gate 7 may identify logical membership, absence, or update facts that future
conflict analysis needs.

It does not select:

- state commitment construction;
- authenticated dictionary;
- Merkle structure;
- proof format;
- absence-proof format;
- snapshot format;
- synchronization protocol; or
- light-client protocol.

Those belong to the next state-model decision area:

> **Authenticated-state membership, absence, snapshot, and light-client requirements.**

## 46. Resource-Bound Boundary

Gate 7 may expose:

- logical access patterns;
- logical mutation patterns;
- hostile conflict populations;
- late-failure exposure;
- conflict amplification; and
- schedule-sensitive implementation cost.

It does not select:

- numeric access limits;
- numeric mutation limits;
- persistent-growth limits;
- invalid-candidate work limits;
- resource meters;
- fees; or
- pricing.

Those remain within the later resource-bound decision area unless already
authoritatively defined elsewhere.

## 47. Threat Model Boundary

The existing Threat Model already covers generic consensus divergence through
non-determinism, including implementation scheduling or host behavior changing
consensus-visible validity or resource results.

Gate 7 must therefore test whether its findings are already covered by that
generic class.

A dedicated conflict/order threat class is not presumed to already exist.

After independent review of this document, the Threat Model should be checked
separately to determine whether Gate 7 exposes a genuinely new generic threat
class, such as shared-state order dependence not adequately represented by the
existing Threat Model coverage for nondeterminism, failure atomicity, state
workload, persistent growth, and resource risks, together with replay and
state-integrity requirements recorded in other decision-readiness artifacts.

Any future Threat Model addition must remain state-model-neutral unless a state
model has actually been selected.

## 48. Formal Specification Boundary

A future authoritative Formal Specification may eventually need to define,
depending on the adopted design:

- canonical conflict semantics;
- which relations are mutually exclusive;
- which relations are order-sensitive;
- any consensus-visible ordering relation;
- any authoritative conflict outcome rule;
- dependency semantics that affect validity or canonical effect;
- schedule-independent validity and state requirements;
- protocol-version interpretation of conflict/order rules;
- any consensus-visible deterministic resource consequences;
- rejection/failure interaction; and
- activation and historical interpretation of changed rules.

This document does not fill those Formal Specification sections.

## 49. Explicit Non-Selection Register

This document does **not** select:

- Minimal Account;
- Minimal UTXO;
- an active hybrid value model;
- a transaction format;
- transaction identifiers;
- conflict keys;
- read-set encoding;
- write-set encoding;
- access lists;
- predeclared dependency lists;
- a dependency-graph format;
- first-seen behavior;
- proposer-selected order;
- fee-priority order;
- canonical transaction sorting;
- deterministic winner mechanism;
- block scheduling;
- parallel execution as a protocol requirement;
- a scheduler;
- locks;
- optimistic execution;
- speculative execution;
- parallel batches;
- a parallel VM;
- commit-order mechanism;
- retry/abort semantics;
- rollback or journal mechanism;
- Account nonce or sequence ordering;
- Account-wide locking;
- UTXO consumption conflict semantics;
- UTXO input ordering;
- mempool replacement policy;
- replay mechanism;
- reorganization mechanism;
- consensus algorithm;
- fork choice;
- finality;
- state commitment;
- proof system;
- resource units;
- resource meter;
- numeric resource limits;
- fee, refund, rent, or pricing rules;
- a state-model ranking; or
- state-model adoption.

## 50. Seventh-Gate Decision-Readiness Checklist

The seventh gate is not decision-ready merely because this document exists.

Before the gate may be treated as substantively clarified, reviewed abstract
answers or explicit deferrals must exist for at least:

1. the meaning of the external semantic conflict relation;
2. the distinction between mutual exclusion and order sensitivity;
3. the distinction among duplicate presentation, replay, conflict, and
   independence;
4. deterministic dependency determination;
5. the boundary between logical protocol conflict and physical implementation
   overlap;
6. the relationship between external semantic conflict and realized
   candidate-specific conflict;
7. the treatment of candidate-specific conflict amplification;
8. which adopted relationships require a canonical conflict outcome;
9. how an authoritative ordering input, if any, is distinguished from local or
   accidental order;
10. the proposer/local-policy boundary;
11. the distinction between consensus transaction order and implementation
    execution schedule;
12. the distinction between authoritative outcome equivalence and internal
    execution-trace equality;
13. serial/parallel validity equivalence;
14. serial/parallel canonical-state equivalence;
15. replay/conflict interpretation equivalence across schedules;
16. protocol-version and historical-interpretation equivalence across schedules;
17. deterministic resource-result equivalence where such results are
    consensus-visible or validity-affecting;
18. rejection and failure atomicity under varying schedules;
19. the relationship between evaluation order and canonical commit order if a
    future architecture distinguishes them;
20. the distinction between canonical reapplication after authoritative history
    change, replay, and ordinary conflict;
21. the role, if any, of a serial non-normative audit oracle;
22. evidence treatment of independent or commutative semantic cases;
23. evidence treatment of order-sensitive cases;
24. evidence treatment of adversarial conflict populations;
25. evidence freeze requirements before paired candidate mapping;
26. Minimal Account mapping questions;
27. Minimal UTXO mapping questions;
28. Threat Model review boundary;
29. Formal Specification handoff boundary;
30. authenticated-state handoff boundary; and
31. resource-bound handoff boundary.

A reviewed answer may remain "deferred" where the question belongs to a later
gate, provided the deferral is explicit and does not hide a candidate-specific
default.

## 51. Project Impact

State-model decision:
**NOT MADE**

Minimal Account selected:
**NO**

Minimal UTXO selected:
**NO**

State-model ranking justified:
**NO**

Canonical conflict rule selected:
**NO**

Canonical transaction ordering selected:
**NO**

Conflict-key representation selected:
**NO**

First-seen behavior selected:
**NO**

Proposer-selected winner selected:
**NO**

Read-set or write-set format selected:
**NO**

Access-list format selected:
**NO**

Dependency-graph format selected:
**NO**

Parallel scheduling architecture selected:
**NO**

Parallel execution required:
**NO**

Optimistic or speculative execution selected:
**NO**

Rollback or journal mechanism selected:
**NO**

Account nonce or sequence ordering selected:
**NO**

UTXO consumption conflict mechanism selected:
**NO**

Mempool replacement rule selected:
**NO**

Resource meter or numeric limit selected:
**NO**

Fee or pricing rule selected:
**NO**

State commitment selected:
**NO**

Consensus algorithm selected:
**NO**

Threat Model update selected:
**NONE**

Formal Specification consensus rule created by this document:
**NO**

## 52. Next Model-Neutral Analytical Area

The next state-model decision area is:

> **Authenticated-state membership, absence, snapshot, and light-client requirements.**

That next analysis must not use this document to silently select:

- Account or UTXO;
- a state commitment construction;
- a proof system;
- a snapshot trust model;
- a synchronization protocol;
- a light-client protocol;
- a conflict-key representation;
- a resource meter; or
- a state-model ranking.

## 53. Conclusion

Canonical conflict behavior must eventually be deterministic under authoritative
protocol semantics.

Fair Account/UTXO comparison begins with the same external semantic conflict and
ordering assumptions, while realized candidate conflict breadth and
representation-induced scheduling constraints remain candidate outputs.

Dependency semantics must be deterministic, but this document does not require
dependencies to be predeclared.

Local arrival order, mempool policy, proposer accidents, host scheduling,
threads, caching, batching speedup, database behavior, and other implementation
details must not silently become consensus semantics.

Permitted serial and parallel schedules may differ internally, but they must
preserve every consensus-visible or validity-affecting result required by the
applicable authoritative semantics.

No conflict mechanism, ordering mechanism, scheduler, state model, or ranking is
selected.

Completion or independent review of this document does not by itself satisfy the
seventh state-model gate or constitute protocol adoption.