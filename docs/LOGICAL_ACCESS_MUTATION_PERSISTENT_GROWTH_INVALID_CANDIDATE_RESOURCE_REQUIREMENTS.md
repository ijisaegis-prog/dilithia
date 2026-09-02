# Dilithia — Logical Access, Mutation, Persistent-Growth, and Invalid-Candidate Resource Requirements

**Status:** REVISION 1 WORKING DRAFT — NOT YET RE-REVIEWED
**Project stage:** Pre-Genesis
**State-model decision:** NOT MADE
**Gate:** 9 of the state-model decision-readiness sequence
**Base reviewed main:** `17f6b626f54762a2111165792bf84cd1e862b694`

> This document defines model-neutral resource-safety requirements needed before
> Minimal Account and Minimal UTXO can be compared fairly.
>
> It does **not** select Account, UTXO, a state commitment, a resource meter,
> gas, a fee mechanism, storage rent, refund rules, numeric limits, a PQ
> primitive, transaction/block formats, a consensus algorithm, or a node
> hardware target.

---

## 1. Purpose

The exact ninth state-model decision gate is:

> **Logical access, mutation, persistent-growth, and invalid-candidate resource requirements.**

The purpose of Gate 9 is to determine what must be true about protocol resource
exposure before a fair Account/UTXO comparison can occur.

Gate 9 is not a price-setting exercise.

It does not answer:

- how much a transaction pays;
- how many bytes a block may contain;
- how many cryptographic verifications are numerically permitted;
- how much CPU, memory, bandwidth, or storage a node must provide;
- whether resources are represented by one scalar or several dimensions;
- whether a resource envelope is declared explicitly;
- whether state access is pre-enumerated;
- whether storage rent exists;
- whether rejected candidates are economically charged; or
- which state model should ultimately be selected.

Gate 9 instead defines the safety properties that any later mechanism must
satisfy.

---

## 2. Authority Boundary

This document is a state-model decision-readiness artifact.

It is subordinate to:

1. the Dilithia Constitution;
2. the Formal Specification where already normative; and
3. any later protocol rule validly adopted under those authorities.

Non-normative architecture documents, benchmark results, implementation code,
test harnesses, AI reviews, repositories, release infrastructure, nodes,
founders, foundations, reviewers, snapshot providers, and archive providers do
not become protocol authority merely by being referenced here.

If a resource rule later affects:

- canonical validity;
- canonical state;
- consensus-visible resource accounting;
- canonical transaction or block acceptance; or
- historical protocol interpretation,

that rule must be represented in the applicable authoritative, versioned
protocol specification before it can govern consensus.

---

## 3. Existing Inherited Constraints

Gate 9 begins with requirements already inherited from prior project material.

### 3.1 Determinism

Consensus-visible validity and resource consequences must not depend on:

- wall-clock timing;
- CPU speed;
- allocator behavior;
- cache state;
- database implementation;
- operating system;
- thread scheduling;
- SIMD availability;
- batching speedup;
- compiler optimization;
- host pointer width;
- `usize`;
- floating-point behavior; or
- another implementation accident.

Physical performance may differ between implementations.

Consensus-visible results may not.

### 3.2 Failure atomicity

A rejected candidate must not leave a partial canonical state transition merely
because evaluation performed some work before rejection.

### 3.3 Bounded decoding

Consensus-critical decoding must reject structurally excessive inputs before an
unbounded allocation or expansion occurs.

Exact numeric limits remain outside this Gate-9 draft.

### 3.4 Persistent-growth protection

Unbounded state growth, free spam, and uncompensated permanent storage are not
acceptable protocol outcomes.

Gate 9 must preserve that requirement without prematurely selecting:

- storage rent;
- one-time fees;
- expiry;
- dust rules;
- cleanup;
- pruning;
- deposits;
- burns; or
- another economic mechanism.

### 3.5 Local-policy separation

Peer, ingress, mempool, cache, queue, admission, and replacement policies may
protect an individual implementation.

They do not become canonical validity merely because one implementation relies
on them.

### 3.6 State-model neutrality

Minimal Account and Minimal UTXO remain co-equal candidates.

No statement in this document may assume that:

- one Account implies one lookup;
- one Account implies one authorization;
- one UTXO input implies one authorization;
- UTXO conflict is inherently local;
- Account conflict is inherently global;
- Account access is inherently constant-cost;
- UTXO access is inherently linear-cost; or
- either model has a particular commitment or database structure.

---

## 4. Terms

### 4.1 Candidate

A canonical or potentially canonical protocol input being evaluated under an
applicable protocol version and authoritative prior-state/history context.

### 4.2 Logical access

A protocol-semantic read, existence test, absence test, lookup, dependency
inspection, or other authoritative information dependency required to determine
candidate validity or effect.

Logical access does not mean one physical database read.

### 4.3 Logical mutation

A protocol-semantic creation, update, deletion, removal, consumption,
replacement, metadata change, or other canonical state consequence.

Logical mutation does not imply one database write.

### 4.4 Gross mutation exposure

The total logical mutation work caused during candidate evaluation or successful
transition, before cancellations or netting are used to summarize the final
state difference.

### 4.5 Net state effect

The final authoritative difference between the prior canonical state and the
accepted resulting canonical state.

A small net state effect does not prove that gross mutation exposure was small.

### 4.6 Persistent growth

An increase in protocol-relevant information that future compliant roles may be
required to retain, reconstruct, authenticate, interpret, or otherwise account
for beyond the immediate transition.

### 4.7 Attempted work

Resource-relevant evaluation performed on a candidate regardless of whether the
candidate is ultimately accepted or rejected.

### 4.8 Invalid candidate

A candidate that cannot be accepted under the applicable authoritative rules.

This can include, without conflating the categories:

- malformed encoding;
- non-canonical encoding;
- unsupported or unknown version;
- structurally excessive input;
- cryptographically invalid evidence;
- semantically unauthorized action;
- wrong domain, network, subject, purpose, or version;
- missing or conflicting state dependency;
- stale or reverted context;
- replay-invalid use;
- resource-bound failure; or
- another deterministic rejection class.

### 4.9 Resource exposure bound

A finite, deterministic upper bound on a defined class of protocol-relevant
work.

Gate 9 does not require a particular representation of that bound.

### 4.10 Consensus-visible resource semantics

Any resource-related value, rule, classification, counter, acceptance
condition, or effect that can change canonical validity, canonical state, or
another consensus-visible result.

### 4.11 Local resource policy

Implementation-local protection that may affect whether a node chooses to
receive, relay, cache, queue, pre-screen, or retain an object but does not
redefine canonical validity.

### 4.12 Resource composition

The combined resource exposure of:

- nested structures;
- repeated components;
- multiple authorizations;
- multiple logical accesses;
- multiple mutations;
- batches or aggregations;
- multiple candidates; or
- a containing protocol object such as a future block.

### 4.13 Cryptographic profile

A comparison-level description of cryptographic or authorization assumptions
used to reason about resource exposure.

Use of a profile does not select a protocol cryptographic primitive.

### 4.14 Historical resource semantics

The resource-related protocol rules applicable to historical data under the
protocol version that governed that data.

---

## 5. Adversary Model for Gate 9

A resource-safety requirement is not meaningful unless the hostile party is
defined.

For Gate-9 analysis, assume an adversary may:

- generate arbitrary byte strings;
- generate arbitrarily many distinct candidate attempts over time;
- repeatedly send identical or near-identical invalid candidates;
- understand implementation behavior;
- choose adversarial candidate ordering;
- choose semantic subjects, identifiers, references, and keys where protocol
  rules give the sender that freedom;
- choose among simultaneously supported protocol or cryptographic profiles
  where the protocol permits that choice;
- construct malformed, non-canonical, unsupported, stale, conflicting,
  unauthorized, or cryptographically invalid candidates;
- create valid candidates for authority the adversary legitimately controls;
- attempt to maximize CPU, memory, bandwidth, I/O, state access, mutation,
  proof, decoding, and cryptographic-verification work;
- exploit nested or repeated structures;
- exploit failure and fallback paths;
- exploit cross-candidate composition;
- act as a peer flooding ingress traffic; and
- where the future consensus design permits, occupy a block-producing or
  proposing role and construct an adversarially expensive otherwise-valid
  containing object.

Gate 9 must not rely on an assumption that an attacker is deterred merely by a
future fee.

Gate 9 also does not assume that an attacker can forge authorization accepted
by a still-secure cryptographic primitive without the required authority.

A future cryptographic break is governed by the separate Crypto Agility and
catastrophic-failure boundaries.

---

## 6. Requirement R1 — Invalid-Candidate Work Must Be Bounded

The protocol architecture must permit a deterministic finite upper bound on
hostile attempted work for rejected candidates.

This requirement applies separately from limits eventually chosen for accepted
candidates.

A protocol is not resource-safe merely because valid transactions or blocks are
bounded.

The rejection path itself must be bounded.

This includes candidates that fail:

- immediately;
- after structural parsing;
- after state inspection;
- during authorization evaluation;
- during cryptographic verification;
- during conflict or replay evaluation;
- during resource evaluation; or
- at another late validation stage.

This requirement does not assert that every rejection must be cheaper than
every acceptance.

It requires that rejection cannot expose unbounded adversarial work.

---

## 7. Requirement R2 — Validation Must Support Bounded Staging

Validation must expose deterministic structural or semantic preconditions that
are sufficient to establish the applicable worst-case resource bound before
evaluation can enter a stage whose hostile exposure would otherwise exceed that
established bound.

Applicable preconditions can include:

- framing and encoded-size checks;
- canonical-encoding checks;
- structural counts;
- version and domain interpretation;
- cryptographic-artifact structural parsing;
- state-independent semantic rejection; and
- other deterministic preconditions.

Exact stage ordering is not selected by this document.

This document also does not require every structural check to be `O(1)`.

A check may scale with bounded input length or another deterministic quantity.

The requirement is that candidate evaluation cannot obtain access to unbounded
later work before the protocol has established a deterministic bound covering
that exposure.

Relative physical cost, measured wall-clock performance, and implementation
optimization order are non-normative evidence unless a later authoritative
logical resource taxonomy explicitly defines otherwise.

Implementations may optimize evaluation strategy only within the authoritative
resource-safety boundary and without changing consensus-visible results.

---

## 8. Requirement R3 — Logical-Access Exposure Must Be Upper-Boundable Before Unbounded Work

Before candidate evaluation can cause unbounded state inspection, the maximum
logical-access exposure must be deterministically upper-boundable from
information already validated under the protocol rules.

This does not require that every access identifier be explicitly enumerated in
the candidate.

A future design may use:

- explicit references;
- declared bounds;
- protocol-derived bounds;
- bounded semantic expansion; or
- another reviewed mechanism.

Gate 9 selects none of them.

The requirement is model-neutral:

> an attacker must not be able to cause an implementation to discover an
> unbounded set of authoritative state dependencies merely by beginning
> candidate execution.

---

## 9. Requirement R4 — Per-Access Worst-Case Exposure Must Also Be Bounded

Bounding only the number of logical accesses is insufficient if one permitted
access can itself require unbounded work.

For every authoritative logical-access class required by a future design, there
must be a finite adversarial worst-case upper bound under the applicable
protocol assumptions.

This requirement includes attacker-chosen semantic identifiers or subjects
where those choices can affect access cost.

It does not require:

- one physical read per logical access;
- equal physical latency for all keys;
- a specific tree depth;
- a trie;
- a hash table;
- an accumulator;
- a Merkle structure; or
- a particular commitment scheme.

Physical cache misses or storage-engine behavior are evidence.

They are not themselves canonical resource semantics unless explicitly adopted
by the authoritative specification.

---

## 10. Requirement R5 — Existence and Absence Must Not Require Unbounded Global Search

If validity depends on whether an authoritative subject exists or is absent,
the protocol design must permit that determination without requiring
attacker-controlled unbounded traversal of the entire logical state.

This requirement applies equally to Account and UTXO candidates.

It does not select an authenticated-state commitment or absence-proof system.

---

## 11. Requirement R6 — Mutation Exposure Must Be Bounded

The protocol must permit a deterministic finite upper bound on the logical
mutation exposure attributable to one candidate and to any containing
consensus object.

Relevant semantic effects can include:

- create;
- update;
- delete;
- remove;
- consume;
- replace;
- ownership or authorization metadata change;
- lifecycle-state change;
- version/migration metadata change; and
- other canonical mutations.

Gate 9 does not define the final resource unit or weighting of those effects.

---

## 12. Requirement R7 — Gross and Net Mutation Must Not Be Conflated

A candidate whose final net state delta is small can still cause substantial:

- creation;
- deletion;
- rewriting;
- metadata churn;
- commitment-update work; or
- historical-support work.

Resource analysis must therefore preserve enough information to distinguish
gross mutation exposure from net state effect where the distinction is
security-relevant.

This does not require a particular number of resource dimensions.

---

## 13. Requirement R8 — Persistent-Growth Coverage Must Be Complete

Future resource analysis must account for every class of protocol-required
persistent growth that can materially affect long-term node or network burden.

Depending on future protocol choices, relevant classes may include:

- current live canonical facts;
- ownership or authorization-related metadata;
- replay-exclusion or lifecycle information;
- cryptographic-version or migration information;
- other consensus-validity system facts;
- historical support information that a required role cannot replace with a
  sufficient authenticated current fact; and
- proof, snapshot, or synchronization support data where retention is actually
  required by the future architecture.

This requirement does not claim that all historical data must remain in the
current state.

It also does not claim that every validating node must retain all historical
bytes forever.

Local caches, indexes, database metadata, and implementation accelerators are
not automatically protocol persistent state.

---

## 14. Requirement R9 — Unbounded Uncompensated Persistent Growth Is Forbidden

Where protocol rules would otherwise permit unbounded state growth, free spam,
or permanent storage without cost, the Formal Specification must contain
explicit economic rules that compensate for that burden, consistent with
Constitution Article 11.

That economic protection does not by itself authorize any candidate to exceed
a hard protocol resource-safety bound where such a bound is required.

Gate 9 does not select how the economic protection is achieved.

In particular, this requirement does not itself select:

- storage rent;
- expiry;
- one-time storage fees;
- burns;
- deposits;
- refunds;
- dust rules;
- cleanup;
- pruning; or
- another mechanism.

A later economic mechanism also does not replace a hard safety bound where a
hard bound is required to prevent resource exhaustion.

---

## 15. Requirement R10 — Failure Must Preserve Canonical Atomicity

A rejected candidate must not leave a partial canonical state effect.

If evaluation temporarily computes, buffers, caches, stages, or speculates on
intermediate results, those implementation details must not become partial
authoritative state merely because the candidate later fails.

The canonical outcome of rejection must be deterministic.

This requirement does not prohibit implementation-local cache changes that do
not change authoritative protocol meaning.

---

## 16. Requirement R11 — Fully Rejected Candidates Have No Canonical Effect

Gate 9 preserves the inherited distinction between:

1. a fully rejected candidate; and
2. a canonically accepted object or accepted outcome whose internal operation
   may be unsuccessful under explicitly specified protocol semantics.

A fully rejected candidate produces no consensus-visible:

- canonical state effect;
- economic charge, debit, burn, lock, credit, or refund;
- successful-resource effect; or
- other canonical side effect.

A future protocol may separately define a canonically accepted object or
accepted-unsuccessful outcome that has explicitly specified resource or
economic consequences.

Such an outcome is not a fully rejected candidate for purposes of this
requirement.

Any such future accepted outcome must independently satisfy all applicable:

- authorization;
- ownership;
- conservation;
- replay;
- lifecycle;
- atomicity;
- determinism; and
- resource-safety requirements.

A candidate must not impose a canonical economic effect on an unauthenticated or
unrelated party merely by naming, referencing, or claiming that party.

Required authority must be established under the applicable protocol rules
before a canonical debit, charge, burn, lock, or comparable economic effect can
be attributed to that party.

Physical work performed locally while determining that a candidate is rejected
does not itself create a canonical charge or canonical state effect.

Gate 9 does not select:

- rejected-candidate charging;
- accepted-unsuccessful transaction semantics;
- fees;
- refunds;
- prepaid execution; or
- another economic mechanism.

---

## 17. Requirement R12 — Consensus-Visible Resource Arithmetic Must Be Deterministic

If a future protocol rule introduces consensus-visible resource accounting, its
arithmetic must be:

- explicitly typed;
- deterministic;
- reproducible by independent implementations;
- independent of host integer width;
- independent of floating-point behavior; and
- explicit about overflow and underflow.

Silent wraparound, implementation-dependent overflow, or host-dependent
conversion cannot determine canonical validity.

This document does not select:

- a counter width;
- scalar versus vector accounting;
- saturating versus rejecting semantics;
- refund arithmetic; or
- a concrete resource unit.

Those choices require later review before adoption.

---

## 18. Requirement R13 — Resource Consumption Must Not Be Made Negative by Accounting Tricks

If later resource accounting supports:

- refunds;
- credits;
- cleanup adjustments;
- negative deltas; or
- another compensating mechanism,

the mechanism must not permit an attacker to:

- make already-consumed validation work disappear;
- reuse the same exhausted budget;
- produce underflow;
- obtain unbounded execution capacity through cycles;
- transform rollback into new resource capacity; or
- cause implementation-dependent accounting.

Gate 9 does not require any refund or credit mechanism to exist.

---

## 19. Requirement R14 — Economic Payment Is Not a Hard Safety Bound

A fee, price, bid, deposit, rent payment, or other economic mechanism must not be
treated as sufficient proof that a candidate is safe to evaluate.

A party willing or able to pay must still be unable to exceed applicable hard
protocol resource-safety bounds.

This remains true where a future block producer or proposer may receive some or
all of the economic payment associated with its own containing object.

The exact economic mechanism is deferred.

---

## 20. Requirement R15 — Resource Amplification Must Be Bounded

A small or cheap-to-construct input must not permit unbounded verifier work,
memory expansion, logical state inspection, persistent growth, proof work, or
other protocol-relevant amplification.

Relevant amplification surfaces include:

- nested encodings;
- length-prefixed structures;
- repeated references;
- recursive or indirectly expanding structures;
- cryptographic artifacts;
- batches;
- aggregations;
- fallback paths;
- proofs;
- snapshots;
- decompression or expansion;
- repeated mutation; and
- cross-candidate interaction.

Gate 9 does not define a numeric amplification ratio.

It requires that the adversarial worst case be finite and evidence-accessible.

---

## 21. Requirement R16 — Nested and Aggregated Work Must Compose Safely

A future limit on individual elements is insufficient if combining many
individually permitted elements can bypass the containing resource bound.

Resource-safe composition must prevent:

- nested limits from resetting an outer budget;
- repeated substructures from multiplying without a containing bound;
- aggregation from hiding expensive fallback behavior;
- batch failure from creating unbounded re-verification;
- component-local accounting from bypassing transaction-level safety; and
- transaction-local accounting from bypassing containing-object safety.

This document does not require all resource composition to be mathematically
additive.

Nonlinear composition may be valid if its worst-case behavior remains
deterministically bounded.

---

## 22. Requirement R17 — Cross-Candidate Composition Must Be Bounded

A candidate-specific bound does not automatically imply that a future block or
other containing consensus object is safe.

Resource analysis must consider adversarial composition in which earlier
candidates affect the state or context used to validate later candidates.

The containing-object safety argument must therefore account for:

- adversarial candidate order;
- adversarially produced intermediate canonical states;
- shared dependencies;
- repeated access to expensive semantic regions;
- repeated cryptographic profiles;
- mutation churn; and
- other cross-candidate amplification.

No block format, block limit, ordering rule, or consensus algorithm is selected
here.

---

## 23. Requirement R18 — Cryptographic Work Must Be Explicitly Exposed to Resource Analysis

Cryptographic and authorization work must not be hidden behind state-model
labels.

Resource analysis must be able to represent hostile exposure associated with:

- authorization-artifact parsing;
- cryptographic verification;
- invalid cryptographic evidence;
- multiple authorization requirements;
- grouping or aggregation where later supported;
- batch success and batch failure paths;
- algorithm/version coexistence; and
- migration between cryptographic eras.

Every comparison-level resource profile must also state each material reuse
assumption that can change resource exposure.

Relevant reuse assumptions can include:

- authorization evidence reuse;
- verification-result reuse;
- key or credential reuse;
- ownership or authorization-condition reuse;
- reuse invalidation conditions; and
- other protocol-visible reuse that changes required work.

The analysis must distinguish protocol-visible reuse from implementation-local
optimization such as caching.

An implementation-local cache hit must not silently reduce the authoritative
resource requirement assigned to one state-model candidate while the other
candidate is evaluated as though equivalent reuse were unavailable.

Gate 9 neither requires nor prohibits reuse.

It requires the assumptions and their resource consequences to be explicit and
symmetrically applied during comparison.

This document does not select a cryptographic resource unit.

---

## 24. Requirement R19 — Crypto Agility Requires Resource Re-Evaluation

A cryptographic algorithm or profile change may alter:

- encoded size;
- public-key size;
- signature or proof size;
- verification work;
- parsing work;
- memory requirements;
- bandwidth requirements;
- batching behavior;
- failure behavior; and
- persistent metadata.

Therefore, a Crypto Agility change that affects resource exposure requires a
reviewed resource-safety analysis before the new profile can become
consensus-active.

Coexisting profiles must be analyzed under adversarial profile selection.

An attacker may choose the most expensive profile that the protocol still
permits.

---

## 25. Requirement R20 — Unsupported and Unknown Versions Must Fail in a Bounded, Deterministic Way

Unknown or unsupported:

- protocol versions;
- cryptographic versions;
- authorization versions;
- resource-accounting versions; or
- other consensus-critical version tags

must not cause:

- permissive reinterpretation;
- implementation-specific fallback;
- unbounded best-effort parsing;
- unbounded cryptographic probing; or
- implementation-dependent validity.

The exact version registry and upgrade mechanism remain outside Gate 9.

---

## 26. Requirement R21 — Resource Semantics Must Be Versioned Where They Affect Validity

If resource rules affect canonical validity or canonical state, historical data
must be interpreted under the resource semantics applicable to its authoritative
protocol era.

A later resource rule must not silently reinterpret historical validity.

Historical replay by a newer implementation must therefore preserve the
applicable historical protocol meaning.

When previously authoritative history is reverted by a permitted
reorganization, consensus-visible resource validity consequences and any
resource-related canonical effects attributable solely to that reverted history
must cease to be authoritative with that history.

If a candidate is later canonically reapplied, its consensus-visible
resource-related result must be derived again from the newly authoritative prior
state and history under the applicable versioned protocol rules.

All compliant implementations must reproduce the same authoritative result for
that reapplication.

This requirement applies conditionally to any future canonical:

- resource allowance;
- resource counter;
- economic charge;
- refund;
- resource-related state; or
- other resource consequence

that the protocol eventually defines.

This requirement does not select:

- a replay implementation;
- archive format;
- checkpoint mechanism;
- bootstrap trust model;
- rollback storage strategy;
- reconstruction mechanism;
- fork-choice rule;
- finality rule;
- reorganization depth; or
- consensus algorithm.

---

## 27. Requirement R22 — Local Admission Must Not Redefine Canonical Validity

A compliant implementation may use local defenses such as:

- peer scoring;
- connection limits;
- ingress quotas;
- duplicate suppression;
- caches;
- queues;
- mempool policies;
- local rate limits; or
- local storage limits.

Such policies may be stricter than protocol acceptance.

But a candidate that appears in authoritative consensus context must receive the
same canonical validity interpretation from compliant implementations,
regardless of whether one node would have admitted that candidate to its local
mempool or cache.

Consensus safety must not depend on every node enabling the same local defense.

---

## 28. Requirement R23 — Local Resource Failure Is Not Automatically Protocol Invalidity

Wall-clock timeout, local out-of-memory, local disk exhaustion, cache miss,
thread starvation, storage-engine failure, and other host conditions do not
automatically define canonical invalidity.

If a protocol input exceeds an authoritative protocol resource bound, rejection
may be canonical according to the applicable specification.

If only one machine cannot process an otherwise protocol-valid object because
of its local environment, that local failure does not by itself redefine the
object's protocol meaning.

Operational liveness and minimum hardware policy require later analysis.

---

## 29. Requirement R24 — Proof, Snapshot, and Light-Client Verification Must Be Resource-Bounded

Any future protocol role that is required to validate:

- membership;
- absence;
- authenticated state;
- snapshot contents;
- snapshot relations;
- light-client claims; or
- canonical-history binding

must have a finite adversarial worst-case validation exposure under its declared
role assumptions.

A small requested claim must not require unbounded proof expansion or global
state traversal.

This requirement does not select:

- a proof system;
- a commitment system;
- a snapshot protocol;
- a light-client protocol;
- a bootstrap mechanism; or
- numeric proof limits.

---

## 30. Requirement R25 — Duplicate and Repeated Invalid Inputs Must Not Require a Consensus Cache

Attackers may resend:

- identical invalid candidates;
- slightly modified invalid candidates; or
- semantically equivalent invalid candidates.

An implementation may use caches to avoid repeated work.

But protocol safety must not require a globally identical rejected-candidate
cache.

Two compliant nodes may perform different amounts of physical work due to cache
state while still reaching exactly the same canonical validity result.

---

## 31. Requirement R26 — Resource-Safety Rules Must Survive Implementation Independence

A resource-safety claim is insufficient if it is only true for one:

- database;
- cache layout;
- programming language;
- runtime;
- allocator;
- compiler;
- thread model;
- machine architecture; or
- implementation-specific optimization.

Where resource semantics affect consensus, independent compliant
implementations must be able to reproduce the same authoritative result from
the same authoritative inputs.

Implementation measurements remain evidence for later parameter selection.

They do not become protocol authority.

---

## 32. Requirement R27 — State-Model Comparison Must Use Shared External Semantics

Minimal Account and Minimal UTXO must be evaluated against the same external
semantic workload and the same security requirement.

Fairness does not require their internal structures to be identical.

The comparison must not assume:

- equal internal record count;
- equal lookup count;
- equal signature count;
- equal mutation count;
- equal proof structure;
- equal conflict structure;
- equal physical I/O;
- equal encoded representation; or
- equal implementation optimization.

Those are candidate-specific consequences to expose, not prerequisites to
assume.

---

## 33. Requirement R28 — Resource Units Must Not Be Smuggled In Through One Candidate's Native Vocabulary

A future comparison metric must not silently treat a candidate-specific internal
counter as a universally neutral resource unit without a separate neutrality
argument.

Examples of potentially biased assumptions include:

- "one account record";
- "one UTXO";
- "one input";
- "one account access";
- "one signature per account";
- "one signature per input";
- "one state slot";
- "one conflict domain"; or
- "one native database record".

Gate 9 requires externally meaningful resource exposure to be mapped into each
candidate rather than defining the external requirement from one candidate's
native representation.

---

## 34. Requirement R29 — Dynamic Access Is Not Automatically Forbidden

Gate 9 does not require all state dependencies to be statically enumerated.

It requires that any dynamic dependency discovery remain within a deterministic
finite resource bound that can be established before unbounded work occurs.

Therefore neither of the following is selected:

- mandatory pre-enumeration of all logical state references; or
- unrestricted dynamic traversal.

A future candidate must demonstrate how its chosen semantics satisfy the bound.

---

## 35. Requirement R30 — Parallel Execution Is Not a Gate-9 Safety Requirement

Gate 9 requires deterministic canonical results and bounded resource exposure.

It does not require:

- static conflict sets;
- parallel execution;
- speculative execution;
- locks;
- dependency graphs;
- a scheduler; or
- a particular execution engine.

A future implementation may exploit parallelism only if every required
consensus-visible result remains equivalent.

---

## 36. Resource-Safety Treatment of Important Candidate Classes

The following candidate classes must eventually have explicit bounded treatment.

| Candidate class | Gate-9 requirement |
|---|---|
| Malformed bytes | Bounded structural rejection |
| Non-canonical encoding | Bounded deterministic rejection |
| Unsupported version | Bounded fail-closed rejection |
| Structurally oversized input | Rejected before unbounded allocation/work |
| Cryptographically invalid evidence | Bounded hostile verification exposure |
| Unauthorized semantic action | Bounded rejection with no partial canonical effect |
| Missing state dependency | Bounded existence/absence evaluation |
| Conflicting dependency | Bounded conflict evaluation |
| Replay-invalid candidate | Bounded replay evaluation |
| Wrong domain/network/subject/purpose | Bounded deterministic rejection |
| Stale or reverted context | Current validity rejected deterministically; historical meaning remains separate |
| Valid high-resource candidate | Subject to future hard safety bounds independent of willingness to pay |
| Nested/aggregated candidate | Composition cannot bypass containing bound |
| Batch with one invalid component | Failure/fallback path included in worst-case exposure |
| Candidate during crypto coexistence | Adversarially expensive permitted profile considered |
| Snapshot/proof input | Bounded verification before unbounded expansion |
| Repeated duplicate invalid input | Cache optional; canonical validity unchanged |

---

## 37. Persistent-Growth Distinctions

Gate 9 must keep the following concepts separate.

### 37.1 Logical integrity

Whether required authoritative information remains correctly interpretable.

### 37.2 Physical availability

Whether some party currently stores particular historical bytes.

### 37.3 Mandatory role retention

Whether a declared protocol role is required to retain particular information.

### 37.4 Current canonical state

Facts required to interpret the current authoritative state.

### 37.5 Historical support information

Information required for a supported historical claim where no sufficient
authenticated current replacement exists.

### 37.6 Local implementation data

Indexes, caches, derived tables, database metadata, and accelerators not
themselves part of protocol truth.

Gate 9 must not count implementation-local storage as canonical persistent
growth merely because one reference implementation uses it.

It also must not omit protocol-required persistent metadata merely because it
does not hold native DLTH value.

---

## 38. Lifecycle Churn

Create-delete-recreate, consume-recreate, version migration, authorization
rotation, and similar lifecycle churn can create substantial resource exposure
even where the final visible value or object population changes little.

Future candidate analysis must therefore test whether lifecycle churn can:

- amplify writes;
- amplify authenticated-state updates;
- grow persistent metadata;
- bypass resource accounting;
- create refund or negative-delta abuse if such mechanisms later exist;
- alter replay or lifecycle interpretation;
- resurrect stale metadata improperly; or
- create unbounded historical-support burden.

Gate 9 does not decide whether identity reuse or recreation is permitted.

---

## 39. Production-versus-Validation Amplification

A protocol must not permit an attacker to create or transmit a bounded input
whose mandatory validation cost becomes unbounded for compliant verifiers.

The exact ratio between producer effort and verifier effort is not selected.

Later evidence should expose the adversarial amplification ratio for relevant
classes.

This requirement is particularly important for:

- invalid cryptographic artifacts;
- proofs;
- nested structures;
- batch fallback;
- state-access patterns; and
- version coexistence.

---

## 40. Network Ingress Boundary

Resource exhaustion can occur before an object becomes a fully decoded
consensus candidate.

Future P2P framing and message formats must therefore support bounded:

- frame parsing;
- declared-length handling;
- memory allocation;
- decompression or expansion;
- nested decoding; and
- pre-candidate structural processing.

The exact network protocol and local rate limits remain outside Gate 9.

Ingress protections must not silently redefine canonical transaction validity.

---

## 41. Crypto-Agility Coexistence Window

A migration period in which multiple cryptographic profiles are accepted can be
more resource-demanding than either steady-state era.

Future Crypto Agility analysis must account for:

- the most expensive simultaneously permitted valid profile;
- malformed artifacts for each supported profile;
- cross-version authorization combinations;
- version-selection attacks;
- persistent metadata from coexistence;
- migration-specific state access; and
- historical validation under retired profiles where still required.

No exact coexistence mechanism or cryptographic primitive is selected here.

---

## 42. Historical Replay Boundary

Resource rules can evolve.

Protocol history must not become dependent on whichever resource meter happens
to exist in the latest implementation.

Where historical validity depended on resource semantics, compliant
implementations must have enough version information to reproduce the
historically applicable rule.

This is separate from:

- archive retention policy;
- weak subjectivity;
- checkpoints;
- snapshot trust;
- consensus finality; and
- historical data availability.

Those remain separate design surfaces.

---

## 43. Evidence Required Before Numeric Limits

Gate 9 does not select numeric limits.

Before a later numeric resource parameter is adopted, evidence should be
available for the parameter's actual protocol context.

Relevant evidence can include:

1. worst-case accepted-candidate cost by validation stage;
2. worst-case rejected-candidate cost by failure stage;
3. malformed-input corpus results;
4. decoding expansion ratios;
5. logical-access exposure;
6. adversarial key/reference distributions;
7. gross versus net mutation exposure;
8. persistent-growth rates;
9. authorization-metadata growth;
10. cryptographic profile cost distributions;
11. cryptographic invalid-input behavior;
12. batch-success and batch-failure fallback behavior;
13. cross-candidate and containing-object composition;
14. producer-versus-verifier amplification;
15. proof and snapshot verification exposure;
16. historical replay under versioned rules;
17. independent-implementation differential results;
18. deterministic conformance vectors;
19. failure-atomicity fault injection;
20. local-policy leakage tests;
21. cold versus warm implementation measurements where relevant as
    non-normative physical evidence; and
22. evidence from hardware profiles selected by a later decentralization and
    deployment policy.

Measurement alone does not define consensus semantics.

A numeric parameter must be justified against the already-defined protocol
resource boundary it is intended to protect.

---

## 44. Benchmarking Neutrality

Future Account/UTXO resource evidence must:

- use the same external semantic case;
- use the same authority requirement;
- use the same value-transfer requirement;
- use the same replay requirement;
- use the same lifecycle requirement;
- use the same cryptographic profile assumptions;
- use the same protocol-version assumptions;
- expose missingness rather than invent defaults;
- distinguish logical protocol metrics from physical implementation metrics; and
- preserve candidate-specific consequences instead of normalizing them away.

A result is not fair evidence if one candidate is measured with a warm cache and
the other with an adversarial cold cache unless that asymmetry is itself the
declared experimental question.

---

## 45. Candidate-Neutral Workload Families for Later Evidence

The following names may be used as non-normative workload families.

They are not transaction formats and do not select protocol semantics.

- W1 — one external authority requirement, one simple value effect;
- W2 — multiple external authority requirements;
- W3 — one authority relation affecting multiple logical subjects;
- W4 — high creation pressure;
- W5 — high delete/consume pressure;
- W6 — small net state delta with high gross mutation;
- W7 — many missing/absence dependencies;
- W8 — expensive authorization artifacts ending in rejection;
- W9 — nested, repeated, batched, or aggregated structures;
- W10 — authorization/version metadata change with little or no value change;
- W11 — coexistence of old and new cryptographic profiles;
- W12 — repeated duplicate invalid candidate;
- W13 — adversarial cross-candidate composition;
- W14 — snapshot/proof verification under hostile input;
- W15 — lifecycle churn and recreation.

A candidate may implement the same external workload differently.

That difference is evidence.

It is not automatically bias.

---

## 46. Explicit Non-Selections

This document does **not** select:

- Account;
- UTXO;
- a state-model ranking;
- a transaction format;
- a block format;
- a state schema;
- a commitment tree;
- a trie;
- a Merkle structure;
- a Verkle structure;
- an accumulator;
- a vector commitment;
- a proof system;
- a snapshot protocol;
- a light-client protocol;
- a bootstrap trust model;
- a consensus algorithm;
- a fork-choice rule;
- a finality rule;
- a scheduler;
- parallel execution;
- static access lists;
- dynamic unrestricted access;
- a resource meter;
- scalar gas;
- multidimensional resource vectors;
- a gas unit;
- a fee mechanism;
- a fee amount;
- storage rent;
- expiry;
- dust policy;
- refund rules;
- prepaid execution;
- a declared resource-envelope field;
- a numeric CPU limit;
- a numeric memory limit;
- a numeric bandwidth limit;
- a numeric storage limit;
- a numeric state-growth limit;
- a transaction byte limit;
- a block byte limit;
- a proof-size limit;
- a signature-count limit;
- a cryptographic-verification-count limit;
- a PQ primitive;
- an authorization algorithm;
- a batching algorithm;
- an aggregation algorithm;
- a node hardware target;
- a mempool policy;
- a peer-scoring policy;
- a local ingress limit;
- archive incentives;
- archive retention duration; or
- governance thresholds.

---

## 47. Explicitly Deferred Design Questions

The following require later evidence or prerequisite protocol choices.

| Question | Status |
|---|---|
| Exact resource units | DEFERRED |
| Scalar vs vector accounting | DEFERRED |
| Numeric limits | DEFERRED |
| Transaction/block aggregate numbers | DEFERRED |
| Fee/pricing design | DEFERRED |
| Rejected-candidate charging | DEFERRED |
| Storage economics | DEFERRED |
| Rent/expiry/dust/cleanup mechanism | DEFERRED |
| Refund existence and formula | DEFERRED |
| Exact counter types | DEFERRED |
| Exact overflow result | DEFERRED pending accounting design |
| Access declaration mechanism | DEFERRED |
| Static vs dynamic access architecture | DEFERRED |
| Commitment-dependent access costs | DEFERRED |
| Proof system and proof limits | DEFERRED |
| Snapshot protocol and limits | DEFERRED |
| Light-client protocol and limits | DEFERRED |
| PQ primitive and parameters | DEFERRED |
| Crypto batching/aggregation | DEFERRED |
| Node hardware target | DEFERRED |
| Local ingress/mempool parameters | DEFERRED |
| Archival roles and incentives | DEFERRED |
| Consensus algorithm | DEFERRED |
| State-model choice | DEFERRED until comparison evidence |

---

## 48. Gate-9 Review Questions

A Gate-9 review must be able to answer the following without assuming an
Account or UTXO winner.

1. What qualifies as protocol-relevant logical access?
2. How is maximum logical-access exposure bounded before unbounded work?
3. Can one logical access itself require unbounded attacker-controlled work?
4. Can existence or absence require whole-state scanning?
5. What qualifies as logical mutation?
6. Is gross mutation exposure distinguishable from net final state effect?
7. Can create/delete/rewrite churn hide large resource use behind a small net
   delta?
8. Which protocol-required facts can grow persistently?
9. Are authorization, replay, lifecycle, migration, and version metadata
   included where applicable?
10. Are local caches and indexes excluded from canonical persistent growth?
11. Can invalid candidates require unbounded work?
12. Can a candidate intentionally fail after expensive state inspection?
13. Can a candidate intentionally fail after expensive cryptographic work?
14. Can a batch or aggregate failure trigger unbounded fallback work?
15. Can a small encoding expand into unbounded memory or work?
16. Can nested limits reset and bypass a containing limit?
17. Can individually bounded candidates compose into an unbounded containing
   object?
18. Can earlier candidates make later candidates arbitrarily more expensive?
19. Does economic payment remain separate from hard safety?
20. Can rejected-candidate accounting charge an unauthenticated or unrelated
   victim?
21. If consensus-visible resource arithmetic later exists, is it deterministic?
22. Can overflow or underflow silently alter validity?
23. Can refunds or negative deltas create new execution capacity?
24. Can local OOM, timeout, cache state, or storage behavior change canonical
   validity?
25. Can local mempool or ingress policy redefine protocol truth?
26. Can duplicate-invalid caching become a hidden consensus requirement?
27. Can an attacker choose a more expensive permitted crypto profile?
28. Does Crypto Agility force resource re-evaluation before activation?
29. Are unknown versions rejected deterministically and with bounded work?
30. Are historical objects interpreted under their applicable historical
   resource semantics?
31. Can proof or snapshot validation expose unbounded hostile work?
32. Can absence proofs expose unbounded hostile work?
33. Are Account and UTXO given the same external semantic workload?
34. Does any requirement secretly assume one signature, one lookup, one record,
   or one conflict domain?
35. Is any candidate-native counter being treated as a universal metric without
   a neutrality argument?
36. Does Gate 9 accidentally require static access enumeration?
37. Does Gate 9 accidentally require dynamic access?
38. Does Gate 9 accidentally require parallel execution?
39. Does Gate 9 accidentally select scalar or vector resource accounting?
40. Does Gate 9 accidentally select a fee, rent, refund, or prepaid mechanism?
41. Does Gate 9 accidentally select a cryptographic primitive?
42. Does Gate 9 accidentally select a commitment or proof mechanism?
43. Does Gate 9 accidentally select numeric limits?
44. Can independent implementations reproduce every consensus-visible
   resource-related result?
45. Is physical benchmark evidence kept separate from normative protocol
   semantics?
46. Can a producer cheaply construct an object that causes unbounded verifier
   work?
47. Can framing or pre-candidate decoding cause unbounded work before consensus
   validation begins?
48. Are cross-version and crypto-coexistence resource exposures represented?
49. Is lifecycle churn covered?
50. Is persistent growth bounded without prematurely selecting its economic
   mechanism?
51. Is a fully rejected candidate clearly distinguished from a canonically
   accepted unsuccessful outcome, with zero canonical effect for the former?
52. Are all material authorization/evidence/verification reuse assumptions
   explicit, and is protocol-visible reuse distinguished from local caching?
53. Do all consensus-visible resource effects follow authoritative history
   correctly across reorganization and canonical reapplication?

---

## 49. Gate-9 Satisfaction Standard

Gate 9 may be considered decision-ready only when:

1. logical-access requirements are explicit;
2. mutation requirements are explicit;
3. persistent-growth requirements are explicit;
4. invalid-candidate requirements are explicit;
5. cryptographic hostile-work requirements are explicit;
6. cross-candidate composition requirements are explicit;
7. failure atomicity remains intact;
8. consensus-visible resource determinism is preserved;
9. local policy remains separate from protocol validity;
10. historical resource semantics are version-aware;
11. snapshot/proof inherited resource requirements are addressed at the
    requirement level;
12. Account and UTXO remain symmetrically constrained;
13. no resource meter has been silently selected;
14. no fee or storage-economics mechanism has been silently selected;
15. no numeric limit has been silently selected;
16. no commitment, proof, crypto, transaction, block, or consensus design has
    been silently selected;
17. unresolved mechanism choices are explicitly deferred;
18. evidence prerequisites for later numeric limits are recorded;
19. adversarial review finds no material unresolved requirement gap;
20. the inherited rejected-versus-accepted-unsuccessful taxonomy is preserved;
21. material authorization and verification reuse assumptions are explicit and
    model-neutral;
22. consensus-visible resource effects follow authoritative history across
    reorganization and canonical reapplication; and
23. the state-model decision remains **NOT MADE**.

Gate-9 satisfaction is a project-process decision-readiness judgment.

It is not protocol activation.

It does not make this document the Formal Specification.

---

## 50. Current Gate-9 Working-Draft Result

At creation of this working draft:

State-model decision:

> **NOT MADE**

Account selected:

> **NO**

UTXO selected:

> **NO**

State-model ranking:

> **NONE**

Resource meter selected:

> **NO**

Scalar gas selected:

> **NO**

Resource vector selected:

> **NO**

Fee mechanism selected:

> **NO**

Storage-rent mechanism selected:

> **NO**

Refund mechanism selected:

> **NO**

Numeric resource limits selected:

> **NONE**

PQ primitive selected:

> **NO**

State commitment selected:

> **NO**

Proof system selected:

> **NO**

Snapshot protocol selected:

> **NO**

Light-client protocol selected:

> **NO**

Consensus algorithm selected:

> **NO**

Formal Specification rule created by this working draft:

> **NO**

Gate-9 satisfaction decision:

> **NOT YET MADE**

Next required action:

> **Focused adversarial review of this exact Gate-9 working draft before any
> satisfaction decision or state-model comparison.**

---

## 51. Summary

Gate 9 freezes the following principle:

> A state-model candidate is not ready for fair comparison unless its logical
> access, logical mutation, persistent growth, accepted and rejected validation
> work, cryptographic exposure, and cross-candidate composition can all be
> bounded under deterministic, implementation-independent, model-neutral
> requirements.

Fees do not replace safety bounds.

Local policy does not redefine consensus.

Invalid candidates do not receive an unlimited work budget.

Persistent growth cannot be ignored merely because it is metadata.

Crypto Agility does not receive an exemption from resource safety.

Account and UTXO receive the same external requirements without assuming the
same internal representation.

The exact mechanisms and numbers remain deliberately unresolved.