# Dilithia State-Model Decision Requirements

> **NON-NORMATIVE DECISION-REQUIREMENTS DOCUMENT**
>
> This document records requirements, unresolved dependencies, comparison
> questions, and evidence gates for a future state-model decision. It does not
> define consensus rules, select or rank a state model, resolve any Formal
> Specification TBD, or constitute protocol adoption. The Dilithia Constitution
> and Formal Specification remain authoritative. Existing architecture,
> benchmark, threat, project-status, implementation, and discussion materials
> provide non-normative evidence or rationale only unless adopted through the
> authoritative protocol process.

## Status, Authority, and Scope

Dilithia is Pre-Genesis, and the Formal Specification's transaction, state,
consensus, and Crypto Agility sections remain pending. This document exists only
to identify what must be understood before a state model can be selected safely.
It neither fills those pending sections nor predicts their contents.

The authority order applied here is:

1. Dilithia Technical Constitution
2. Dilithia Formal Specification
3. Ratified HIP / Super HIP material, if any
4. Normatively adopted conformance vectors
5. Implementation
6. `PROJECT_STATE.md`
7. Non-normative architecture, threat, resource, and benchmark documents
8. AI or other design discussion

Minimal Account and minimal UTXO are co-equal candidates in this document.
Neither currently leads. The currently demonstrated differences between them
are less decision-relevant than the unresolved ownership, authorization, replay,
transaction, lifecycle, commitment, and resource dependencies described below.

This document does not define account fields, replay fields, UTXO fields, output
identifiers, object identifiers, transaction or block formats, state encoding,
state commitment, address formats, cryptographic algorithms, fee mechanics,
numeric resource limits, or activation and governance mechanics.

## 1. Model-Independent Safety Properties

Any future state-model proposal must be evaluated against these
model-independent properties without this document defining their mechanisms:

- State transitions are deterministic for every canonical input and applicable
  protocol version.
- Logical state semantics are reproducible by independent implementations.
- Consensus-critical encodings are canonical and versioned where the Formal
  Specification eventually defines them.
- Consensus-critical arithmetic is checked and cannot wrap or silently
  saturate.
- Floating-point arithmetic does not participate in consensus-critical state
  logic.
- Native DLTH value is conserved except where an authoritative specification
  explicitly permits value creation or destruction.
- State changes occur only under correct ownership and authorization rules.
- Replays and double spends cannot recreate a previously exercised spendable
  effect.
- Failed validation leaves no partial canonical state effect.
- Logical state access and mutation remain bounded under valid and hostile
  inputs.
- Persistent-state exposure remains bounded and subject to the Constitution's
  economic-safety requirements.
- Consensus semantics do not depend on a database engine, physical key layout,
  cache, allocator, storage device, or compaction strategy.
- Historical data remains interpretable under the protocol version that governs
  it.
- Ownership and authorization representations remain migratable without loss
  of legitimate spendability.
- Current consensus state is distinguished from retained history and local
  archival data.
- Conflict behavior is canonical and independent of proposer or implementation
  accidents not expressly made part of the future protocol.
- Serial and parallel implementations produce identical validity, state, and
  deterministic resource results.

These are decision requirements, not adopted transition mechanisms.

## 2. Candidate Status

### Minimal Account

Minimal Account is a candidate only. No account representation, field set,
replay mechanism, lifecycle, or authorization structure is selected here.

### Minimal UTXO

Minimal UTXO is a candidate only. No output representation, identifier,
ownership condition, transaction dependency, or consumption mechanism is
selected here.

The candidates remain co-equal. Claims about PQ bandwidth, state compactness,
proof efficiency, synchronization, wallet usability, or formal simplicity are
comparison hypotheses until the dependencies needed to evaluate them are
defined.

## 3. Deferred Models

### Generalized Object State

Generalized object state is deferred because no current authoritative
requirement requires generic object lifecycle, generic object identity, shared
mutable objects, or object-runtime semantics. This deferral does not prohibit
later reconsideration if a concrete protocol requirement emerges.

### Active Hybrid Value Model

An active hybrid value model is deferred because multiple live native-DLTH or
value representations would introduce cross-model conservation, replay,
atomicity, commitment, migration, and resource-accounting questions. No current
requirement justifies that additional decision surface.

This is distinct from namespace-separated, non-value system or extension state.
Namespace separation may remain a candidate concept without creating two native
value models. No extension-state entity or format is defined here.

## 4. Unresolved Decision Register

| Area | Questions that remain unresolved | Why selection is blocked |
|---|---|---|
| A. Authorization coverage and ownership model | What entity is authorized? How many independent authorizers may be involved? How are credentials and versions migrated? What relationship, if any, exists between ownership and a cryptographic credential? | Authorization shape affects transaction structure, PQ cost, migration, and state requirements. |
| B. Replay and canonical identity model | Is replay excluded through a sequence-like possibility, explicit-consumption possibility, or another reviewed mechanism? What transaction or output identity dependencies exist? How are network and chain domains bound? | Replay safety cannot be compared without knowing what is made unique and what state changes after acceptance. |
| C. Native DLTH lifecycle and conservation boundary | How is ownership represented? Which value-creation or value-destruction operations, if any, will be legal? How is conservation demonstrated across every value-affecting path? | A state model cannot be compared without the native value invariant it must preserve. |
| D. Transaction shape and failure semantics | What state may be affected? What dependencies must be declared or discovered? What is atomic? What is the canonical effect of failure? | Access sets, conflicts, rollback, and resource exposure depend on transaction semantics. |
| E. Entity existence and lifecycle | What do absence, zero, creation, deletion, recreation, and historical interpretation mean? | Lifecycle rules affect replay, proof, storage, and migration safety. |
| F. PQ authorization assumptions | How many independent authorizations are required? What artifact-size implications matter? Are grouping, aggregation, batching, or key reuse possible under a future algorithm? | PQ cost is determined by authorization architecture and transaction shape, not by a state-model label alone. |
| G. Conflict and ordering requirements | Which conflicts must be explicit? What deterministic ordering is required? Which concurrency is permissible without changing results? | Parallelism and proposer-order effects cannot be evaluated without canonical conflict semantics. |
| H. Authenticated-state requirements | Which membership, absence, update, snapshot, synchronization, and light-client capabilities are required? | Proof and synchronization comparisons depend on the unchosen commitment requirements and construction. |
| I. Resource-bound requirements | How are logical access, mutation, persistent growth, and hostile invalid-candidate work bounded? | Candidate safety depends on deterministic resource exposure and pre-action bounding. |

This register asks questions only. It supplies no implied answer or default.

## 5. Ownership and Crypto-Agility Boundary

The safe requirement is limited to the following:

> Ownership and authorization representations must eventually be versioned and
> migratable without loss of legitimate spendability.

The future design may need to evaluate, without this document preferring any
choice:

- direct public-key-derived identity;
- indirect logical identity;
- versioned ownership conditions;
- authorization descriptors;
- credential or algorithm rotation structures;
- multisignature or multi-credential semantics; and
- recovery semantics, if any can exist without violating the Constitution.

No stable logical owner identity is selected. No privileged recovery mechanism
may be inferred from the migration requirement.

## 6. Account Comparison Questions

The following questions must be answered before Minimal Account can be selected:

- How is replay prevented?
- What is the canonical result of a replay-state mismatch?
- Are gaps meaningful, and if so, how are they interpreted?
- Does failure consume or otherwise change replay-related state?
- How do deletion and recreation affect replay exclusion?
- What distinguishes zero, absent, deleted, and newly created accounts?
- Does recipient creation change native-transfer semantics or resource effects?
- Can multiple credits to one recipient commute safely under every applicable
  arithmetic, creation, deletion, and failure condition?
- How are overflow and proposer-controlled ordering handled deterministically?
- What authorization metadata, if any, is persistent?
- How is credential or algorithm rotation represented and interpreted across
  versions?
- How are account spam and long-lived state growth bounded?
- What exact logical state must a native transfer read and mutate?
- Can the native transfer path remain narrow without introducing account
  abstraction or unrelated dispatch?

These are comparison questions, not an Account state machine or field proposal.

## 7. UTXO Comparison Questions

The following questions must be answered before Minimal UTXO can be selected:

- How are outputs uniquely identified?
- What transaction identity assumptions would an output-identity design require?
- How is relevant transaction or authorization malleability avoided?
- How are multiple inputs governed by the same ownership condition authorized?
- Is authorization count necessarily equal to input count under any candidate
  transaction design?
- How are ownership conditions versioned?
- How do old outputs remain legitimately spendable after a cryptographic
  migration?
- How are dust and fragmentation risks bounded?
- How are input and output structural bounds enforced before expensive work?
- How are dependency chains treated during validation and local admission?
- How are output ordering and uniqueness specified without implementation
  dependence?
- What authenticated-state properties are required for unspent-state evidence?
- After wallet coin selection and change handling are classified as product
  concerns, what consensus complexity remains?

These are comparison questions, not a UTXO state machine, field proposal, or
identifier definition.

## 8. Common Adversarial Scenario Matrix

The same scenarios must be applied to both candidates. The questions below do
not supply candidate-specific answers.

| Scenario | Security property that must hold | Account questions | UTXO questions | Evidence required before comparison |
|---|---|---|---|---|
| 1. One ordinary valid DLTH transfer | Correct authorization, conservation, atomicity, and deterministic result | Which logical entities and metadata are inspected or changed? | Which referenced and created entities are inspected or changed? | Abstract transaction dependencies, authorization coverage, and conservation obligations |
| 2. Replay of an accepted transfer or effect | No duplicate spendable effect | Which persistent facts prevent replay across lifecycle changes? | Which uniqueness and consumption facts prevent replay? | Replay specification, canonical identity requirements, and historical-version rules |
| 3. Two conflicting spends | At most one incompatible effect can become canonical | How is a conflict recognized and ordered? | How is a shared-consumption conflict recognized and ordered? | Conflict definitions and deterministic block-order semantics |
| 4. Multiple independent transfers to one recipient | Deterministic validity and final state | Do credits commute across overflow, existence, and lifecycle cases? | Do independently created recipient effects interact or conflict? | Arithmetic, entity lifecycle, and ordering semantics |
| 5. Failure after authorization has been checked | No partial canonical effect; hostile work remains bounded | Which provisional effects require rollback? | Which provisional canonical effects, if any, must be discarded on failure? | Validation stages, atomicity boundary, and attempt-work requirements |
| 6. Failure after some state has been inspected | No mutation or implementation-dependent result | Can later failure expose state-dependent ordering behavior? | Can missing or conflicting referenced state fail only after costly inspection? | Logical access contract and failure-path resource analysis |
| 7. Deletion or consumption followed by replay | Removed spendability cannot be recreated accidentally | Can deletion or recreation reset replay protection? | Can an old consumed reference become valid again? | Lifecycle, uniqueness, and reorg interpretation |
| 8. Reorg and canonical reapplication | Reversion and reapplication produce the authoritative state | Which mutable records and lifecycle effects must be restored? | Which consumed and created effects must be restored? | Abstract reversible effects and versioned replay rules |
| 9. Ownership cryptographic-algorithm migration | Existing legitimate value remains spendable without privileged authority | What state or historical rules carry authorization versions? | What ownership conditions remain interpretable for old value? | Crypto Agility requirements and migration proof obligations |
| 10. Multiple independent authorizers | Every required owner authorizes exactly the permitted effect | How are multiple authorization domains represented without account abstraction? | How are authorizations grouped across referenced value? | Authorization coverage model and PQ artifact assumptions |
| 11. Persistent-state growth attack | Long-lived state exposure remains bounded | Which creations or metadata changes persist? | Which created unspent effects persist and fragment state? | State population model, lifecycle rules, and persistent-growth accounting requirements |
| 12. Many small transfers | Aggregate validation and storage exposure remain bounded | Do repeated recipients, replay updates, or creations amplify work? | Do small outputs, dependencies, or fragmentation amplify work? | Aggregate workload model and structural bounds |
| 13. Large structurally complex transfer | Complexity is deterministically bounded before unsafe work | Which authorization or state dependencies can grow? | Which input, output, or ownership dependencies can grow? | Candidate transaction-shape bounds and validation stages |
| 14. Invalid candidate that fails late | Attempted work is bounded before canonical commit | Which state-dependent checks may fail late? | Which missing, conflicting, or unauthorized references may fail late? | Conservative attempt-envelope analysis under the resource architecture |
| 15. Protocol-version transition | Historical and new semantics remain unambiguous | How are earlier lifecycle and replay rules interpreted? | How are earlier identity, ownership, and consumption rules interpreted? | Version-selection and historical-replay requirements |
| 16. Snapshot or state synchronization | Untrusted state can be verified against authoritative commitments | Which logical records and absence conditions require verification? | Which unspent records and absence conditions require verification? | Commitment requirements, population fixtures, and snapshot trust model |
| 17. Cross-network or cross-chain replay attempt | An effect valid in one domain is not reusable in another | What signed domain information binds authorization? | What signed domain information binds authorization and identity? | NetworkId, ChainId, and domain-separation decisions |
| 18. Arithmetic boundary failure | No wrap, saturation, partial effect, or order-dependent ambiguity | Which balance-affecting operations can encounter the boundary? | Which consumed or created value totals can encounter the boundary? | Value domain and checked-arithmetic semantics |
| 19. Concurrent disjoint transactions | Parallel and serial validation agree exactly | How is disjointness demonstrated for all logical effects? | How is disjointness demonstrated for all referenced and created effects? | Explicit conflict contract and differential serial/parallel evidence |
| 20. Conflicting transactions with proposer-controlled ordering | Canonical ordering cannot cause unspecified behavior | Which replay, recipient, lifecycle, or overflow effects are order-sensitive? | Which shared references or dependent effects are order-sensitive? | Canonical order, conflict rejection, and resource-composition rules |

## 9. Value-Conservation Evidence

Before selection, both candidates need an auditable conservation argument that
covers every native-value-affecting path and every failure path.

Minimal Account evidence would eventually need to address:

- debit and credit conservation concepts;
- every path that can affect a balance or equivalent value record;
- atomic mutation across every affected logical entity; and
- creation, deletion, recreation, migration, and version interactions.

Minimal UTXO evidence would eventually need to address:

- unique consumption concepts;
- conservation across consumed and created value;
- creation uniqueness; and
- ownership-condition interpretation across versions.

Issuance, destruction, fees, and other monetary exceptions remain TBD. This
document defines no conservation equation or transaction structure.

## 10. Replay and Double-Spend Evidence

Both candidates must eventually demonstrate:

- exclusion of an already exercised spendable effect;
- correct network, chain, and purpose binding;
- consistent behavior across reorg and canonical reapplication;
- unambiguous interpretation at protocol-version boundaries; and
- absence of any lifecycle reset that incorrectly recreates spendability.

The evidence must cover valid, conflicting, missing-state, failure, historical,
and migration scenarios. This document chooses neither a sequence mechanism nor
an explicit-consumption mechanism.

## 11. PQ and Cryptographic Evidence

The assumptions "Account equals one signature" and "UTXO equals one signature
per input" are not established. Future comparison must instead evaluate:

- number of independent authorizers;
- authorization grouping permitted by the transaction and ownership design;
- key, signature, proof, and other artifact sizes;
- key and ownership-condition representation;
- aggregation or batching capability, if available under a future algorithm;
- ownership-condition reuse; and
- representative and adversarial transaction shapes.

No cryptographic algorithm, parameter, registry, grouping rule, or batch rule is
selected here.

## 12. State-Size and Synchronization Evidence

No current evidence establishes that Account is smaller or UTXO is harder to
synchronize. Any such ranking is **BLOCKED** until the comparison has:

- candidate logical state schemas;
- a justified state-population model;
- replay and authorization metadata requirements;
- dust and fragmentation behavior;
- creation, deletion, recreation, and retention semantics;
- authenticated-state requirements and a reviewed construction candidate;
- snapshot and restoration requirements; and
- a history-retention or pruning policy.

Physical storage measurements may later provide implementation evidence but
cannot define logical consensus state.

## 13. State-Commitment Neutrality

No tree, trie, accumulator, hash algorithm, or proof system is selected. Future
authenticated-state work must eventually address these abstract requirements:

- deterministic commitment to canonical logical state;
- canonical interpretation of logical keys, values, existence, and absence;
- membership and absence capability where the protocol requires them;
- reproducibility across independent implementations;
- explicit versioning and domain separation;
- migration compatibility; and
- snapshot and synchronization verification capability.

Proof size, update cost, snapshot size, and synchronization rankings are
commitment-dependent and therefore cannot currently rank Account and UTXO.

## 14. Resource and Invalid-Candidate Evidence

Candidate comparison must ask:

- How much required state is named before validation begins?
- How much state is discovered only during validation?
- Which structural upper bounds can be checked before expensive work?
- How does missing state fail, and after how much attempted work?
- How does authorization failure remain bounded?
- How is arithmetic failure detected before partial canonical effect?
- When and how are conflicts detected?
- Which hostile inputs can cause late failure?
- Which operations create persistent growth or high gross mutation?

These questions relate to the non-normative resource architecture and the
Threat Model's current invalid-candidate, state-workload, persistent-growth, and
failure-atomicity properties. This document defines no resource counter, unit,
weight, budget, or numeric limit.

## 15. Formal-Verification Evidence

Future verification evidence should address common obligations:

- authorization correctness;
- native-value conservation;
- replay and double-spend exclusion;
- failure atomicity;
- deterministic conflicts and ordering;
- checked arithmetic; and
- protocol-version and historical interpretation.

Minimal Account comparison questions include how to state and prove
replay-related invariants, logical-map lifecycle, and deletion or recreation
safety.

Minimal UTXO comparison questions include how to state and prove unique
consumption, output-identity requirements, and transaction-local conservation.

Neither candidate is declared formally superior.

## 16. Wallet Versus Protocol Complexity

Wallet and product complexity must be assessed separately from consensus
protocol complexity.

UTXO coin selection, change handling, consolidation, and user-facing balance
presentation are real wallet costs. They must not be counted as equivalent to
consensus-safety complexity without identifying the actual protocol rule they
affect.

Likewise, simple Account balance presentation does not establish that Account
authorization, replay, lifecycle, or migration semantics are simpler at the
consensus layer.

Both categories matter to an eventual decision, but they are separate evidence
dimensions.

## 17. Namespace Separation

The following remains a candidate principle only:

- one stable native DLTH value model; and
- separately versioned, non-value system or extension state.

This possibility is not approval of a hybrid value model and does not imply a
second native-value representation. No extension-state entity, lifecycle,
serialization, commitment namespace, or transaction interaction is defined.

## 18. Premature-Commitment Matrix

| Classification | Items | Boundary |
|---|---|---|
| SAFE TO RECORD NOW | Deterministic, failure-atomic, bounded state transitions; logical semantics independent of database layout; current state distinguished from history; migratable ownership and authorization representation; value conservation; checked arithmetic | These are model-independent requirements, not mechanisms. |
| COMPARISON HYPOTHESIS ONLY | Account PQ bandwidth advantage; Account state compactness; UTXO proof friendliness; snapshot or synchronization rankings | Evidence depends on unresolved transaction, authorization, state, commitment, and population assumptions. |
| KEEP AS CANDIDATE | Minimal Account; minimal UTXO; namespace-separated non-value state | No candidate is selected or ranked. |
| BLOCKED | Stable logical ownership identity architecture; Account replay or nonce semantics; UTXO output identity; concrete transition tables; state-commitment structure | Required dependencies are unresolved. |
| DO NOT RECORD AS PROJECT DIRECTION | Account currently leads; UTXO currently leads; Account is inherently best for PQ; active multiple-native-value-model hybrid without a concrete requirement | These claims are unsupported or would prematurely direct protocol design. |

## 19. Decision Gates

Candidate ranking remains blocked until sufficiently concrete abstract answers
exist for all of these gates:

1. Ownership and authorization coverage, versioning, and migration requirements
2. Replay exclusion and canonical identity requirements
3. Native DLTH lifecycle and conservation requirements
4. Abstract transaction dependencies, state effects, atomicity, and failure
5. Entity existence, creation, deletion, recreation, and historical meaning
6. PQ authorization-count and artifact assumptions sufficient for comparison
7. Canonical conflict, ordering, and scheduling-equivalence requirements
8. Authenticated-state membership, absence, snapshot, and light-client
   requirements
9. Logical access, mutation, persistent-growth, and invalid-candidate resource
   requirements

These gates do not require complete implementation designs. They require enough
reviewed abstract semantics to prevent candidate comparison from depending on
unstated assumptions.

## 20. Formal Specification Boundary

Eventual authoritative specification is required for consensus-relevant matters
including:

- state entities and their semantics;
- ownership and authorization;
- replay and double-spend exclusion;
- native-value conservation;
- entity existence and lifecycle;
- transaction preconditions and effects;
- canonical conflict and ordering behavior;
- failure atomicity;
- canonical state encoding;
- authenticated-state commitment semantics;
- protocol-version and historical interpretation; and
- resource rules that affect validity or canonical state.

The following should remain outside normative protocol semantics unless a future
authoritative design demonstrates otherwise:

- database backend;
- cache strategy;
- physical key layout;
- implementation scheduler;
- wallet coin selection;
- local indexes; and
- benchmark-machine characteristics.

This document does not modify or supplement the Formal Specification.

## 21. Threat Model Boundary

The current Threat Model already records generic resource exhaustion,
invalid-candidate late failure, state workload, persistent growth,
failure-atomicity, nondeterminism, and version-drift threat classes.

No Threat Model change is proposed merely to record candidate-specific Account
or UTXO questions. Model-specific threat additions should be considered after a
model direction is selected, or earlier only if analysis discovers a genuinely
new threat class not already represented by the generic properties.

## 22. Complete TBD Register

The following remain unresolved and are not answered by this document:

- final state model;
- transaction format;
- block integration;
- native DLTH value representation;
- ownership identity;
- authorization representation;
- replay semantics;
- Account lifecycle;
- output identity;
- state-entity representation;
- creation, deletion, recreation, and historical rules;
- cryptographic algorithms and parameters;
- PQ authorization structure;
- multisignature and multi-credential behavior;
- `ChainId` representation;
- `NetworkId` discriminant values;
- domain-tag registry;
- authenticated-state commitment structure;
- proof format;
- snapshot and synchronization semantics;
- conflict and order rules;
- reorg and canonical-reapplication behavior;
- deterministic resource units and accounting semantics;
- numeric resource limits;
- fees, rent, refunds, cleanup, and pruning;
- consensus algorithm and finality;
- protocol activation and migration mechanics; and
- governance thresholds and HIP / Super HIP mechanics.

## 23. Recommended Decision Sequence

This non-normative, model-neutral sequence is proposed for future review:

1. Record model-independent requirements and open questions.
2. Clarify abstract ownership and authorization requirements without selecting
   an identity architecture.
3. Clarify replay-exclusion and canonical-identity requirements.
4. Clarify native DLTH conservation and lifecycle requirements.
5. Clarify abstract transaction dependencies, state effects, and failure
   requirements.
6. Re-run Account and UTXO comparison using identical scenarios and evidence
   standards.
7. Permit candidate ranking only after the decision gates are satisfied.
8. Select a state model through an explicit, independently reviewed design
   decision and the applicable authoritative process.
9. Define concrete state and transaction semantics only after selection.
10. Define authenticated-state commitment integration against those semantics.
11. Produce conformance vectors and independent prototypes.
12. Benchmark before selecting numeric limits or economic parameters.

This sequence does not imply that either candidate will be selected.

## 24. Conclusion

- No state model is selected.
- No candidate currently leads.
- Minimal Account and minimal UTXO remain co-equal candidates.
- Generalized Object and active hybrid value models remain deferred.
- Namespace-separated non-value state remains only a candidate concept.
- Candidate ranking is blocked by unresolved ownership, replay, conservation,
  transaction, lifecycle, PQ, conflict, commitment, and resource semantics.
- This document defines no consensus rule.
