# Dilithia Replay Exclusion and Canonical Identity Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records requirements, unresolved questions, comparison variables,
> adversarial scenarios, and evidence gates for replay exclusion and canonical
> identity only.
>
> It defines no consensus rule, resolves no Formal Specification TBD, selects no
> state model, transaction format, replay mechanism, sequence mechanism, output
> identity, transaction identifier, hash algorithm, state field, conflict rule,
> reorganization rule, or state commitment, and does not constitute protocol
> adoption.
>
> The Dilithia Constitution and Formal Specification remain authoritative.

## Status, Authority, Purpose, and Scope

Dilithia is Pre-Genesis. Transaction, state, consensus, Crypto Agility, ChainId,
canonical conflict, reorganization, and several domain-binding details remain
pending in the Formal Specification.

This document exists to clarify what replay exclusion and canonical identity
must mean at the requirement level before Minimal Account and Minimal UTXO can
be compared fairly or selected.

The authority order applied here is:

1. Dilithia Technical Constitution
2. Dilithia Formal Specification
3. Ratified HIP / Super HIP material, if any
4. Normatively adopted conformance vectors or tests
5. Implementation
6. `PROJECT_STATE.md`
7. Non-normative design, threat, resource, and benchmark documents
8. AI or other design discussion

Minimal Account and Minimal UTXO remain co-equal candidates.

This document does not select:

- Account nonce or sequence semantics;
- UTXO consumption semantics;
- transaction identity;
- output identity;
- effect identity encoding;
- identifier width;
- hash algorithm;
- output-position encoding;
- replay-state field;
- replay-state storage location;
- lifecycle tombstone;
- signed transaction format;
- exact signing bytes;
- `ChainId` representation;
- `NetworkId` discriminant values;
- the domain-tag registry;
- block ordering;
- fork choice;
- finality;
- reorganization depth;
- mempool replacement policy;
- authenticated-state commitment; or
- numeric resource limits.

## 1. Terminology

The following terms are separated to prevent an unresolved implementation
mechanism from being treated as an already-selected protocol rule.

| Concept | Decision-readiness meaning | Current status |
|---|---|---|
| Spendable effect | A consensus-relevant effect capable of changing control or disposition of native value when validly exercised | Exact transaction/state representation unresolved |
| Replay | Presentation or reconstruction of a candidate that would cause an already exercised spendable effect to be realized again while the earlier exercise remains authoritative in the current canonical history | Must be excluded; mechanism unresolved |
| Duplicate presentation | Re-presentation of candidate material that may be byte-identical or semantically related to material already observed | Its treatment must be deterministic; presentation alone does not define canonical replay semantics |
| Reapplication | Evaluation of a candidate after an authoritative prior application has been reverted from canonical history | Must be distinguished from replay; exact consensus behavior unresolved |
| Conflict | A relationship in which two candidate effects cannot both become canonical under the applicable state and rules | Canonical conflict semantics unresolved |
| Canonical identity | The consensus-relevant basis for deciding whether effects, references, or state transitions are the same, different, conflicting, already exercised, or eligible for reapplication | Required conceptually; representation unresolved |
| Transaction identity | A possible identity assigned to an entire future transaction | Not selected |
| Effect identity | A possible identity for one consensus-relevant effect independently of a whole transaction | Not selected |
| State-reference identity | A possible identity for state or value referenced by a transition | Not selected |
| Replay state | Any canonical state whose interpretation contributes to exclusion of a previously exercised effect | Whether such dedicated state exists is unresolved |
| Domain binding | Binding of authorization or identity interpretation to the protocol context in which it is valid | Partially established; exact complete context unresolved |
| Historical interpretation | Interpretation of old canonical data according to rules applicable to that history | Required at the property level |
| Current acceptance | Whether old candidate material, rules, identities, or authorization versions may authorize a new transition now | Unresolved |
| Reorganization | A future consensus event in which previously canonical effects cease to be canonical and another history becomes authoritative | Consensus mechanics unresolved |

Canonical identity in this document is not synonymous with a cryptographic hash.

A future design could use a sequence-like relation, explicit consumption,
transaction-derived identity, effect-derived identity, state version, or another
reviewed mechanism.

No such mechanism is selected here.

## 2. Existing Authoritative and Decision-Readiness Boundaries

Current authoritative material directly establishes or constrains the following
relevant boundaries:

- every consensus-critical operation must be deterministic across compliant
  implementations;
- all consensus-critical data must use canonical, versioned, and
  domain-separated serialization;
- previously valid canonical encodings must remain unambiguously decodable or
  safely migratable under the Formal Specification;
- address formats and state representations must remain permanently migratable
  without loss of ownership, and protocol upgrades must not render previously
  valid assets unreachable, unspendable, or un-migratable;
- `NetworkId` is included in every signed structure to prevent cross-network
  replay; and
- hashes and signatures over DCS-encoded bytes use purpose-specific domain
  separation.

Existing non-normative decision-readiness documents additionally record the
following requirements for analysis, without making them authoritative protocol
rules:

- historical and current protocol interpretation must eventually be
  deterministic and unambiguous across protocol versions;
- historical interpretation must be distinguished from current acceptance;
- replay and authorization behavior across protocol evolution must not depend on
  silent reinterpretation; and
- ownership and authorization analysis must state the assumptions required to
  preserve ownership and spendability through protocol evolution.

Those broader decision-readiness requirements remain subject to future
authoritative specification where consensus behavior depends on them.

The following remain unresolved:

- exact `NetworkId` discriminants;
- `ChainId` representation and complete semantics;
- domain-tag registry;
- transaction format;
- signing message;
- replay mechanism;
- transaction identity;
- output identity;
- effect identity;
- state lifecycle;
- conflict behavior;
- canonical ordering;
- reorganization behavior;
- finality; and
- version activation mechanics.

Existing NetworkId and purpose-specific domain-separation requirements do not by
themselves define complete replay exclusion.

## 3. Model-Independent Replay Safety Requirement

The required outcome is:

> An already exercised spendable effect must not be recreated merely by
> presenting the same or an equivalent authorization candidate again while the
> prior exercise remains authoritative in the current canonical history.

A future replay design must additionally ensure that:

- replay validity is deterministic for every canonical input;
- independent implementations agree on whether an effect is new, conflicting,
  already exercised, or eligible for reapplication;
- byte-level variation cannot silently create a second spendable effect when
  consensus semantics regard the underlying authority and effect as the same;
- lifecycle changes cannot accidentally restore previously exercised
  spendability;
- protocol-version changes cannot silently reinterpret old replay identity;
- failures cannot partially consume, advance, create, remove, or otherwise alter
  replay-relevant canonical state;
- local node history, caches, mempool contents, or observation order cannot
  determine consensus replay validity; and
- replay exclusion remains compatible with deterministic rollback and
  reapplication under the future consensus design.

These are outcome requirements, not a replay algorithm.

## 4. Canonical Identity Boundary

A future protocol must define enough canonical identity semantics to answer:

1. What exact consensus-relevant thing is being protected from repeated
   exercise?
2. What makes two candidate effects the same for replay purposes?
3. What makes them distinct?
4. What makes them conflicting even if their bytes or identifiers differ?
5. What canonical state proves that a prior effect has or has not already been
   exercised?
6. Which protocol domain applies to that identity?
7. Which protocol version determines its interpretation?
8. What happens when previously canonical history is reverted?
9. What lifecycle events may alter the relevant identity relation?
10. What information must remain available for independent historical
    verification?

The answers need not use one universal identifier.

Transaction identity, authorization identity, state-reference identity,
ownership identity, replay identity, and effect identity may be distinct.

No requirement in this document implies that they must collapse into one hash or
one field.

## 5. Byte Identity Is Not Automatically Semantic Replay Identity

Canonical encoding is necessary but is not alone sufficient to define replay
semantics.

A future design must explicitly consider whether different canonical byte
strings could authorize or express the same already-exercised spendable effect.

Examples of unresolved causes include:

- different valid authorization evidence for the same permitted effect;
- multiple credentials capable of satisfying one authority relation;
- randomized or otherwise variable cryptographic evidence under a future
  algorithm;
- authorization grouping differences;
- version transitions;
- transaction metadata that does not change the protected effect;
- reordered structures where ordering is not semantically meaningful; and
- future transaction composition rules.

Therefore:

> A future design must not assume that "different transaction bytes" alone prove
> "different spendable effect."

Likewise:

> A future design must not assume that "same bytes" alone determine current
> validity without evaluating the applicable canonical state and protocol
> context.

No transaction-hash or effect-hash construction is selected here.

## 6. Canonical Serialization and Malleability Boundary

Consensus-critical identity must not depend on alternative serialization of the
same canonical structure.

Future replay analysis must distinguish:

- malformed non-canonical bytes;
- alternate but canonical candidate structures;
- different authorization evidence;
- semantically equivalent effects;
- genuinely different effects; and
- conflicting effects.

Non-canonical DCS input must not become an alternate identity path.

A future identity construction must also state which consensus-relevant
mutations, if any, change identity and which do not.

No malleability rule, transaction digest, signature digest, or identifier
construction is defined here.

## 7. Network, Chain, Purpose, and Version Domains

Replay exclusion must account for the domain in which authorization and effects
are valid.

Current authoritative material establishes:

- `NetworkId` participation in every signed structure; and
- purpose-specific domain separation for hashes and signatures.

Future design must still determine the required relationship among:

- network;
- chain;
- protocol version;
- authorization version;
- transaction or effect purpose;
- referenced canonical state;
- ownership or authorization condition;
- state-model-specific identity, if any; and
- other validity-affecting context.

`ChainId` remains unresolved.

A purpose-specific domain tag prevents cross-purpose interpretation only to the
extent defined by the future consuming subsystem. It does not by itself define
same-purpose replay exclusion.

No exact signed message is defined.

## 8. Cross-Network Replay

A valid authorization from one network must not become valid on another network
merely because all other candidate material is identical.

The existing `NetworkId` requirement provides an authoritative boundary for
signed structures.

Future evidence must still demonstrate that:

- every relevant authorization path actually obeys the requirement;
- no alternate authorization path bypasses the network binding;
- unknown or malformed network discriminants fail deterministically; and
- historical interpretation remains unambiguous across protocol versions.

Exact NetworkId discriminants remain TBD.

## 9. Cross-Chain Replay

The future protocol must determine whether and how chain identity participates
in authorization and replay exclusion.

Questions include:

- Can more than one canonical chain domain exist under one NetworkId?
- What distinguishes a chain after Genesis or another authoritative chain
  creation event?
- Does authorization need direct ChainId binding?
- Does transaction or effect identity need ChainId binding independently of
  authorization?
- How are historical chain identifiers interpreted?
- How are malformed or unknown chain identifiers treated?

This document supplies no answer.

`ChainId` representation and semantics remain TBD.

## 10. Purpose Separation

Authorization valid for one protocol purpose must not silently become valid for
a different purpose.

Future analysis must determine distinct authorization contexts for any adopted
operations such as:

- ordinary native-value transfer;
- credential change;
- migration;
- ownership-condition change;
- issuance or destruction, if later authorized;
- protocol-system operations, if any; and
- other future consensus effects.

This list does not adopt any operation.

The exact domain-tag registry remains TBD.

## 11. Historical Interpretation Versus Current Acceptance

Historical interpretation and current acceptance are separate requirements.

Historical interpretation asks:

> Under which rules was this candidate or effect interpreted when that history
> was canonical?

Current acceptance asks:

> May this old identity, authorization version, or candidate material authorize a
> new transition under the current protocol rules?

Historical validity does not automatically imply current acceptability.

Future specification must prevent:

- silent reinterpretation of old identities;
- downgrade ambiguity;
- current replay through obsolete rules;
- disagreement among implementations about the applicable version; and
- version changes that accidentally recreate spendability.

No checkpoint, pruning, archival, or old-version acceptance policy is selected.

## 12. Reorganization and Canonical Reapplication

Replay and reapplication after an authoritative reorganization are not the same
concept.

A future consensus design must define, without ambiguity:

- when a formerly canonical effect is considered reverted;
- which replay-relevant state is restored;
- which created state ceases to exist;
- which previously consumed or otherwise unavailable state becomes current
  again, if applicable;
- whether and when the same candidate may be evaluated again;
- which protocol version applies during reapplication; and
- how independent implementations derive identical results.

If an earlier application has been authoritatively reverted, later reapplication
must not be rejected merely because a local node remembers having seen it.

Conversely, local removal from a mempool, cache, journal, or database must never
make a still-canonical exercised effect spendable again.

No fork-choice, rollback algorithm, journal format, or maximum reorg depth is
defined.

## 13. Lifecycle and Replay Reset Boundary

Entity lifecycle may affect replay safety and therefore cannot remain an
implicit implementation detail.

Future design must answer whether and how replay semantics interact with:

- absence;
- first creation;
- zero value;
- deletion;
- recreation;
- consumption;
- restoration after reorganization;
- credential or authorization-version change;
- protocol-version transition; and
- historical interpretation.

A lifecycle transition must not accidentally reset the only fact preventing a
duplicate spendable effect.

No tombstone, deletion marker, replay counter, consumed-record store, or
retention policy is selected.

## 14. Failure Atomicity

Replay-related canonical state must obey failure atomicity.

A failed candidate must not partially:

- advance replay state;
- consume replay state;
- reserve an identity permanently;
- create an identity permanently;
- delete an identity permanently;
- alter ownership or authorization state;
- consume value;
- create value; or
- leave any other canonical effect unless that effect is explicitly defined by
  a future authoritative rule.

Failure before authorization verification and failure after authorization
verification must both produce deterministic canonical outcomes.

Local attempted-work accounting and caches are separate from canonical rollback.

No validation order is selected.

## 15. Conflict and Ordering Boundary

Replay safety and conflict semantics are related but not identical.

Future design must distinguish:

- exact duplicate presentation;
- semantic replay;
- two different candidates attempting the same incompatible spendable effect;
- candidates sharing only some authority or state;
- independent disjoint candidates;
- candidate replacement or supersession, if ever adopted; and
- reapplication after authoritative history reversion.

The protocol must eventually define which relationships:

- make both candidates invalid;
- permit exactly one according to canonical order;
- make one invalid relative to the other;
- permit both;
- or require another deterministic result.

No canonical transaction ordering rule is selected here.

## 16. Serial and Parallel Equivalence

Parallel execution must not create a replay interpretation different from serial
execution.

For every future permitted concurrent schedule:

- validity must agree with the authoritative semantics;
- replay exclusion must agree;
- conflict detection must agree;
- post-state must agree; and
- deterministic resource results, where later defined, must agree.

Implementation scheduling is not a source of canonical identity.

No scheduler or parallel-execution design is selected.

## 17. Hostile Replay and Resource Exposure

Future evidence must cover adversarial candidate streams including:

- repeated exact duplicates;
- large numbers of semantically conflicting candidates;
- malformed identity material;
- malformed authorization evidence;
- unknown versions;
- cross-network candidates;
- cross-chain candidates, once ChainId is defined;
- cross-purpose evidence;
- candidates referencing missing state;
- candidates designed to fail only after expensive work;
- repeated old-version candidates; and
- structurally large candidate sets.

Replay rejection must eventually be bounded under the future deterministic
resource architecture.

This document defines no resource counter, weight, unit, fee, limit, cache, or
admission rule.

## 18. Minimal Account Candidate Questions

Minimal Account remains a candidate only.

Before an Account direction can be selected, replay analysis must answer:

- What consensus-relevant fact excludes repeated exercise?
- What is the scope of that fact?
- Is the scope related to a value holder, authorization relation, credential,
  transaction, effect, or another concept?
- If a sequence-like mechanism is considered, are gaps meaningful?
- If ordering of candidate sequence values matters, what exact semantics are
  required?
- Does failed validation alter any replay-related fact?
- Can account deletion remove replay protection?
- Can account recreation restore previously exercised spendability?
- Does zero value differ from absence for replay purposes?
- Does credential change affect replay scope?
- Does algorithm migration affect replay scope?
- Can several independent authorizers safely share or partition replay scope?
- Can disjoint effects involving one account execute concurrently?
- How are competing candidates deterministically recognized?
- What must be reverted and restored after a reorganization?
- What replay-related metadata persists?
- Can that metadata grow without bound?
- How is historical Account replay state interpreted after protocol upgrades?

No nonce, counter, sequence field, account identifier, lifecycle rule, or state
schema is defined here.

## 19. Minimal UTXO Candidate Questions

Minimal UTXO remains a candidate only.

Before a UTXO direction can be selected, replay analysis must answer:

- What makes a referenced value record uniquely distinguishable?
- Does output identity depend on transaction identity?
- If so, what assumptions must transaction identity satisfy?
- Does position or ordering participate in identity?
- Can semantically equivalent but byte-different parent transactions produce
  ambiguous descendant identity?
- How is relevant transaction or authorization malleability prevented from
  creating unintended distinct spendable references?
- What canonical fact demonstrates that a referenced value effect is currently
  available?
- What canonical fact distinguishes consumed from never-existing state?
- Can a previously consumed identity ever be recreated?
- How is creation-identity collision or aliasing handled?
- How are old-version identities interpreted?
- How does authorization migration affect dormant referenced value?
- Can one authorization cover several referenced value sources without changing
  replay semantics?
- How are competing candidates for the same referenced value recognized?
- What must be restored after a reorganization?
- How are dependency chains interpreted deterministically?
- How is hostile missing-reference work bounded?

No transaction identifier, output identifier, output index, consumption rule,
tombstone, state schema, or hash construction is defined here.

## 20. Account and UTXO Neutrality

Minimal Account and Minimal UTXO remain co-equal candidates.

No current evidence establishes that:

- a sequence-like replay model is simpler than explicit consumption;
- explicit consumption is more secure than a sequence-like model;
- Account has less replay state;
- UTXO has less replay state;
- Account has better concurrency;
- UTXO has better concurrency;
- one model has simpler reorganization behavior;
- one model has simpler historical verification;
- one model has smaller identity metadata;
- one model produces fewer conflicts; or
- one model is preferable for Genesis.

Those remain hypotheses until the relevant transaction, lifecycle, identity,
conflict, authorization, commitment, and resource assumptions are sufficiently
concrete.

## 21. Canonical Identity and State Commitment

Canonical identity semantics and authenticated-state commitment are distinct.

The protocol may eventually require proofs about:

- current membership;
- current absence;
- current availability;
- historical existence;
- historical consumption or transition;
- replay-related state; or
- another canonical property.

This document does not determine which proofs are necessary.

A commitment construction must not be used to invent replay semantics that are
absent from the logical state specification.

No hash tree, trie, accumulator, proof system, root encoding, or absence-proof
construction is selected.

## 22. Snapshot and Synchronization Boundary

A future node synchronizing from untrusted data must eventually be able to reach
the same authoritative replay-valid state as an independently replaying node.

Future analysis must answer:

- Which replay-relevant logical facts are part of current canonical state?
- Which facts are historical only?
- Which absence conditions matter?
- What must a snapshot prove?
- Can replay safety be reconstructed without full history?
- If some history is pruned, what current canonical fact replaces the need for
  that history?
- How are protocol-version boundaries represented?
- How does reorganization interact with a synchronized snapshot?

No snapshot format, pruning rule, archive requirement, or state commitment is
defined.

## 23. Local Mempool and Cache Boundary

Local policy must not define consensus replay semantics.

The following remain implementation or policy concerns unless later adopted
normatively:

- whether a node remembers rejected candidates;
- duplicate-packet filtering;
- mempool replacement;
- sender queues;
- local conflict indexes;
- seen-transaction caches;
- rate limits;
- peer scoring;
- storage indexes; and
- eviction policy.

A candidate rejected by one node's local policy may still need deterministic
consensus evaluation if it appears in an authoritative context.

Likewise, deleting local cache entries cannot restore canonical spendability.

## 24. Adversarial Scenario Matrix

The following scenarios define questions and evidence obligations only.

| Scenario | Required property | Unresolved common question | Account question | UTXO question | Evidence required |
|---|---|---|---|---|---|
| 1. Ordinary valid native transfer | Exactly the permitted spendable effect becomes canonical | What constitutes the protected effect? | What replay-relevant state is inspected or changed? | What availability or identity relation is inspected or changed? | Abstract effect and replay model |
| 2. Exact byte-for-byte presentation again | No duplicate spendable effect | Which canonical fact rejects repetition? | Which potential replay fact excludes it? | Which potential availability/consumption fact excludes it? | Deterministic replay oracle |
| 3. Same intended effect with different valid authorization evidence | Byte variation cannot bypass semantic replay exclusion | Can several evidence forms authorize the same effect? | Does replay scope survive credential/evidence variation? | Can differing authorization evidence create a distinct candidate identity? | Authorization-coverage and replay-equivalence model |
| 4. Semantically equivalent candidate with different irrelevant metadata | Non-semantic mutation cannot create unintended extra spendability | Which fields are identity-relevant? | Which potential Account context changes identity? | Which potential transaction/output context changes identity? | Future transaction semantic model |
| 5. Two incompatible spends of the same value | At most the canonically permitted incompatible effect becomes authoritative | What defines conflict? | Which replay/value state conflicts? | Which referenced value relation conflicts? | Conflict and ordering model |
| 6. Exact candidate on another NetworkId | Cross-network replay fails | How is NetworkId bound through every path? | Same | Same | Domain conformance vectors |
| 7. Exact candidate on another ChainId | Cross-chain behavior is explicit and deterministic | What ChainId semantics are required? | Same | Same | Chain-identity decision and conformance vectors |
| 8. Evidence reused for another protocol purpose | Cross-purpose authorization fails | Which purpose domains exist? | Same | Same | Domain-tag registry and signing semantics |
| 9. Old authorization version used for new transition | Historical validity does not imply current acceptance | Which version controls current validity? | How would potential replay state interact with authorization version? | How would potential ownership condition interact with version? | Cross-version conformance |
| 10. Unknown protocol or authorization version | Deterministic failure without authority escalation | How is applicability determined? | Same | Same | Version-selection rules |
| 11. Non-canonical serialization of otherwise equivalent content | No alternate identity path | Is malformed encoding rejected before identity use? | Same | Same | DCS negative vectors |
| 12. Candidate whose authorization evidence is malleable under a future algorithm | Malleability cannot recreate spendability | What equivalence relation matters? | Does replay bind to effect rather than evidence bytes where required? | Can identity dependency be changed by evidence variation? | Algorithm-specific security review |
| 13. Deletion or removal followed by old candidate | Lifecycle cannot accidentally reset replay exclusion | What lifecycle state remains authoritative? | Can deletion/recreation reset potential replay state? | Can absence recreate a consumed/reference identity? | Lifecycle model |
| 14. Recreation after zero or absence | Previously exercised effects remain excluded where required | Are zero, absence, deletion, and recreation distinct? | Account-specific lifecycle question | Reference existence question | Entity-lifecycle semantics |
| 15. Failure before replay validation completes | No partial canonical effect | What provisional work exists? | Does any potential replay state change? | Does any availability state change? | Failure-atomicity tests |
| 16. Failure after authorization succeeds | No partial replay or value effect | Which later checks can fail? | Which potential mutable facts must roll back? | Which potential consumed/created facts must be discarded? | Validation-stage model |
| 17. Formerly canonical transaction is reverted by reorganization | Reverted effects cease to control current canonical validity exactly as specified | What constitutes authoritative reversion? | Which replay facts restore? | Which referenced/created facts restore? | Reorg abstract model |
| 18. Same candidate is reapplied after authoritative reversion | Reapplication is distinguished from replay | When may it become valid again? | How is restored pre-state recognized? | How is restored referenced state recognized? | Reorg/reapplication vectors |
| 19. Local node restart loses seen-candidate cache | Consensus replay safety is unchanged | Which canonical state is sufficient? | Same | Same | Restart/state-reconstruction test |
| 20. Snapshot sync without full local history | Replay-valid current state remains verifiable | Which historical facts are still needed? | What potential replay state is committed? | What availability/absence facts are committed? | Snapshot/commitment requirements |
| 21. Concurrent disjoint candidates | Parallel and serial results agree | What proves disjointness? | Which Account effects are independent? | Which referenced/created effects are independent? | Differential serial/parallel test |
| 22. Concurrent conflicting candidates | Canonical conflict result is scheduling-independent | What relation makes them incompatible? | How is potential replay-state contention represented? | How is shared-reference contention represented? | Conflict/concurrency vectors |
| 23. Very large duplicate-candidate flood | Replay rejection work remains bounded | What can be rejected before expensive work? | Which state lookup may be needed? | Which reference lookup may be needed? | Hostile attempted-work evidence |
| 24. Long-dormant value crosses protocol versions | Spendability and replay interpretation follow explicit assumptions | Which historical/current identity rules survive? | What dormant replay metadata is required? | What old referenced-value identity is required? | Migration and dormant-state evidence |
| 25. Credential or algorithm migration occurs between two related effects | No version ambiguity recreates spendability | Does migration alter replay equivalence? | Does potential replay scope migrate? | Do old/new conditions alter reference identity? | Migration/replay cross-version analysis |
| 26. Identity collision or aliasing candidate | Two distinct protected effects cannot silently become one, and one effect cannot silently become several | What uniqueness property is actually required? | What Account-level referent could collide? | What transaction/output referent could collide? | Future identity-construction proof |
| 27. Proposer chooses order among conflicting candidates | Order cannot produce unspecified replay behavior | Which outcomes are permitted under canonical order? | Which Account replay/lifecycle effects are order-sensitive? | Which shared-reference effects are order-sensitive? | Canonical-order semantics |
| 28. Historical block is independently revalidated years later | Historical interpretation is reproducible | Which old rules and domains apply? | How is historical replay state interpreted? | How are historical reference identities interpreted? | Multi-version independent implementation vectors |

## 25. Evidence Required Before Candidate Comparison

Replay-related candidate comparison must eventually include:

- exact abstract replay property being tested;
- exact canonical domain assumptions;
- transaction/effect semantics sufficient to identify the protected effect;
- lifecycle semantics;
- protocol-version assumptions;
- historical-versus-current acceptance assumptions;
- conflict relation;
- reorganization/reapplication assumptions;
- authorization-coverage assumptions;
- candidate mapping;
- failure oracle;
- adversarial duplicate/replay cases;
- independent implementation or independent semantic review;
- resource-exposure classification; and
- explicit limitations on generalization.

Evidence from one frozen mapping must not automatically be generalized to the
entire Account or UTXO family.

## 26. Formal-Verification Obligations

Future replay design should support proofs or equivalent rigorous arguments for:

- no duplicate spendable effect while prior exercise remains canonical;
- correct domain binding;
- deterministic identity interpretation;
- deterministic conflict interpretation;
- lifecycle changes do not resurrect prohibited spendability;
- failure atomicity;
- reorganization and reapplication correctness;
- historical-version interpretation;
- serial/parallel equivalence; and
- independent-implementation equivalence.

Candidate-specific proof obligations may later include:

### Minimal Account

- replay-state invariant, if any replay state is selected;
- scope of any sequence-like relation;
- deletion/recreation safety;
- credential/version transition safety; and
- conflict/order invariants.

### Minimal UTXO

- unique current availability or equivalent;
- creation-identity uniqueness;
- non-duplication of previously exercised value;
- identity/malleability assumptions; and
- rollback/reapplication invariants.

These are proof topics, not selected mechanisms.

## 27. Premature-Commitment Matrix

| Classification | Items | Boundary |
|---|---|---|
| SAFE TO RECORD NOW | No duplicate spendable effect; deterministic replay interpretation; canonical DCS; NetworkId binding for signed structures; purpose-specific domain separation; failure atomicity; historical/current distinction; reorg/reapplication must be deterministic | Authority-derived or model-independent properties |
| ABSTRACT REQUIREMENT ONLY | Canonical identity sufficient to distinguish same/different/conflicting/already-exercised effects; lifecycle must not accidentally reset replay safety; replay behavior must survive protocol evolution | Exact representations and mechanisms unresolved |
| COMPARISON HYPOTHESIS ONLY | Account sequence-like replay is simpler; UTXO explicit consumption is simpler; Account has less replay state; UTXO has fewer replay edge cases; either candidate has superior concurrency or reorg behavior | Required semantics and evidence do not yet exist |
| KEEP AS CANDIDATE | Sequence-like exclusion; explicit-consumption exclusion; transaction-derived identity; effect-derived identity; state-reference identity; versioned identity interpretation | None is selected |
| BLOCKED | Account nonce field; exact sequence semantics; UTXO output-ID formula; transaction-ID formula; output index representation; hash algorithm; replay-field width; tombstone design; concrete signing bytes; ChainId representation; exact domain registry; canonical conflict order | Dependencies unresolved |
| DEFER | Mempool replacement; wallet duplicate tracking; local seen-candidate caches; physical replay indexes | Local policy cannot define consensus |
| DO NOT ADOPT | Local cache as sole replay protection; database key as implicit consensus identity; pointer/object identity; implementation-specific ordering; non-canonical byte encodings as alternate valid identities | Violates determinism or canonical interpretation |

## 28. Decision Gates

Replay/identity architecture selection and Account/UTXO ranking remain blocked
until sufficiently concrete abstract answers exist for:

1. Protected-effect definition
2. Same-versus-different-versus-conflicting effect semantics
3. Required canonical identity granularity
4. Network, chain, purpose, and version binding requirements
5. Byte-malleability and authorization-evidence variation treatment
6. Entity lifecycle and replay-reset semantics
7. Historical-versus-current interpretation
8. Reorganization and canonical-reapplication semantics
9. Conflict, ordering, and serial/parallel equivalence requirements
10. Failure atomicity and hostile replay-work bounds
11. Snapshot, synchronization, and retained-history requirements
12. Independent-implementation and formal-verification obligations

These gates require decision-ready abstract semantics and evidence.

They do not require selected fields, exact byte encodings, algorithms, or a
state-model decision.

## 29. Genesis Minimality

| Classification | Items |
|---|---|
| Genuinely necessary or near-unavoidable at the property level | No duplicate spendable effect; deterministic replay validity; domain binding sufficient for authorized native effects; lifecycle cannot accidentally recreate exercised spendability |
| Strong requirements or principles | Failure atomicity; independent-implementation reproducibility; deterministic historical interpretation; bounded hostile replay validation; deterministic reorg/reapplication behavior once consensus semantics exist |
| Premature | Nonce/sequence field; output identifier construction; transaction identifier construction; spent tombstones; explicit replay database; exact signed message; ChainId representation; mempool replacement rules |

Genesis simplicity is not a justification for leaving replay semantics
ambiguous.

Likewise, hypothetical extensibility is not a justification for expanding the
replay or identity surface before a requirement exists.

## 30. Formal Specification Boundary

Future authoritative specification will eventually need to define enough
consensus semantics for:

- protected native-value effects;
- transaction or effect boundaries;
- state entities and lifecycle;
- replay and double-spend exclusion;
- canonical identity where required;
- ownership and authorization binding;
- exact domain binding;
- NetworkId discriminants;
- ChainId, if required;
- protocol-version interpretation;
- signing semantics;
- conflict and ordering;
- failure atomicity;
- reorganization and reapplication;
- canonical serialization of adopted structures;
- authenticated-state interaction;
- validity-affecting resource rules; and
- conformance behavior.

This document defines none of those normative mechanisms.

It does not modify the Formal Specification.

## 31. Threat Model Boundary

This decision-readiness analysis does not by itself modify the Threat Model.

The current Threat Model already records generic threat classes and security
properties relevant to this analysis, including:

- nondeterminism;
- failure atomicity;
- invalid-candidate and duplicate-ingress resource pressure;
- persistent-state growth;
- cryptographic failure;
- version drift; and
- historical-interpretation risk.

Consensus-level replay and duplicate-spend exclusion are explicit
state-model decision requirements, but this document does not assume that the
current Threat Model already contains a dedicated replay or double-spend threat
class.

After independent review of this document, the Threat Model should be checked
separately to determine whether the replay analysis has exposed a genuinely new
generic threat class that should be recorded now, or whether existing generic
threat coverage is sufficient until a concrete state-model direction is
selected.

Any future Threat Model update must remain mechanism-neutral unless a replay or
identity architecture has actually been selected.

No Account-specific nonce threat or UTXO-specific output-identity threat becomes
project direction through this document.

## 32. Complete TBD Register

The following remain unresolved:

- final state model;
- protected-effect representation;
- transaction format;
- transaction identity;
- effect identity;
- state-reference identity;
- output identity;
- Account identity;
- Account replay mechanism;
- UTXO replay or availability mechanism;
- canonical replay state, if any;
- sequence-like semantics;
- explicit-consumption semantics;
- gap behavior;
- transaction malleability rules;
- authorization-evidence equivalence;
- exact signing bytes;
- ownership representation;
- authorization representation;
- credential format;
- cryptographic algorithms and parameters;
- protocol-version binding;
- authorization-version binding;
- ChainId;
- NetworkId discriminant values;
- domain-tag registry;
- entity existence semantics;
- zero semantics;
- deletion semantics;
- recreation semantics;
- consumed-state semantics;
- historical-retention requirements;
- conflict rules;
- canonical ordering;
- serial/parallel scheduling-equivalence contract;
- consensus algorithm;
- fork choice;
- finality;
- reorganization semantics;
- canonical reapplication rules;
- state commitment;
- proof format;
- snapshot semantics;
- synchronization semantics;
- pruning;
- archival obligations;
- deterministic resource accounting;
- numeric resource limits;
- fees or economic rules;
- activation mechanics;
- migration mechanics; and
- governance thresholds and HIP / Super HIP mechanics.

No item is resolved by its appearance in this document.

## 33. Current Project Impact

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

Replay mechanism selected:
NO

Account nonce or sequence selected:
NO

UTXO explicit-consumption mechanism selected:
NO

Transaction identity selected:
NO

Output identity selected:
NO

Hash algorithm selected:
NO

ChainId selected:
NO

Exact signing message selected:
NO

Canonical conflict rule selected:
NO

Reorganization mechanism selected:
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

## 34. Next Decision Boundary

Completion and independent review of this document may clarify the second
state-model decision gate:

Replay exclusion and canonical identity requirements.

It does not by itself satisfy the remaining state-model gates.

After this artifact is reviewed, the next model-neutral analytical area is:

Native DLTH lifecycle and conservation requirements.

That next analysis must not use replay conclusions to silently select Account,
UTXO, a transaction format, or a monetary mechanism.

## 35. Conclusion
Replay and double-spend exclusion is required.
No replay mechanism is selected.
Canonical identity is required only at the semantic level necessary to make
replay, conflict, lifecycle, domain, history, and reapplication deterministic.
Canonical identity is not assumed to be one cryptographic hash.
Different bytes do not automatically mean a different spendable effect.
Historical interpretation is distinct from current acceptance.
Replay is distinct from canonical reapplication after authoritative reversion.
NetworkId binding and purpose-specific domain separation remain authoritative
boundaries.
ChainId and the complete domain-binding model remain TBD.
Account nonce or sequence semantics remain blocked.
UTXO output identity and explicit-consumption semantics remain blocked.
Minimal Account and Minimal UTXO remain co-equal candidates.
Candidate ranking remains blocked.
This document defines no consensus rule.
