# Dilithia Native DLTH Lifecycle and Conservation Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records requirements, unresolved questions, comparison variables,
> adversarial scenarios, and evidence gates for the lifecycle and monetary
> conservation of native DLTH only.
>
> It defines no consensus rule, resolves no Formal Specification TBD, selects no
> state model, transaction format, native-value representation, issuance
> mechanism, reward mechanism, destruction mechanism, fee mechanism, supply
> schedule, monetary formula, balance representation, output representation,
> lifecycle encoding, or economic policy, and does not constitute protocol
> adoption.
>
> The Dilithia Constitution and Formal Specification remain authoritative.

## Status, Authority, Purpose, and Scope

Dilithia is Pre-Genesis.

No state model has been selected.

Minimal Account and Minimal UTXO remain co-equal candidates.

Transactions, State, Consensus, exact monetary mechanics, native DLTH value
representation, issuance behavior, destruction behavior, fees, rewards, resource
pricing, and several lifecycle semantics remain unresolved.

This document exists to clarify the model-independent requirements that native
DLTH accounting and lifecycle must eventually satisfy before Account and UTXO
can be compared or ranked fairly.

Protocol authority and supporting evidence are distinguished as follows.

Authoritative protocol sources are:

1. the Dilithia Technical Constitution;
2. the Dilithia Formal Specification; and
3. HIP or Super HIP material only to the extent that it has been validly adopted,
   activated, and given protocol effect through the authoritative process
   permitted by the Constitution and Formal Specification.

Supporting material may provide decision-readiness, conformance, implementation,
or project-status evidence, but does not independently create protocol authority.

Such supporting material includes:

- conformance vectors or tests, unless and only to the extent that their
  normative role is explicitly established through the authoritative protocol
  process;
- implementations;
- `PROJECT_STATE.md`;
- non-normative design, threat, resource, benchmark, and decision-readiness
  documents; and
- AI or other design discussion.

Where supporting material conflicts with authoritative protocol sources, the
authoritative protocol sources control.

This document does not select:

- initial circulating supply;
- Genesis issuance mechanics;
- ongoing issuance;
- issuance schedule;
- issuance recipient classes;
- block rewards;
- validator rewards;
- proposer rewards;
- staking rewards;
- treasury allocation;
- foundation allocation;
- team allocation;
- reserved allocation;
- inflation;
- deflation;
- supply cap;
- perpetual issuance;
- emission curve;
- monetary epochs;
- halving;
- burn;
- destruction;
- fee burning;
- fee redistribution;
- fees;
- rent;
- refunds;
- subsidies;
- storage pricing;
- dust policy;
- cleanup policy;
- pruning economics;
- Account balances;
- UTXO amounts;
- transaction inputs or outputs;
- value-record identity;
- quantity width;
- arithmetic representation;
- transaction-value equation;
- supply accumulator;
- authenticated-state structure;
- snapshot format;
- state commitment;
- consensus algorithm; or
- governance thresholds.

## 1. Existing Authoritative Monetary Boundaries

The Constitution directly establishes the following monetary boundaries:

- zero pre-mine;
- zero foundation allocation;
- zero team allocation;
- zero reserved coins;
- no retroactive issuance;
- no privileged allocation;
- no hidden issuance;
- no preferential monetary treatment;
- detailed monetary mechanics must be defined by the Formal Specification;
- protocol upgrades remain subordinate to the Constitution;
- no privileged cryptographic authority may exist;
- state and address evolution must satisfy the Constitution's ownership-
  preservation requirements; and
- consensus-critical monetary behavior must remain deterministic across
  compliant implementations.

These boundaries do not currently establish:

- the exact Genesis monetary transition;
- whether post-Genesis issuance exists;
- a reward schedule;
- a supply cap;
- a burn rule;
- a fee rule;
- a rent rule;
- a subsidy rule;
- a destruction rule;
- a monetary epoch;
- a value width;
- a balance model;
- an output model; or
- a state-model-specific conservation equation.

Those matters remain pending unless and until adopted authoritatively.

## 2. Conservation Does Not Mean Universal Supply Constancy

For this decision-readiness document, conservation does not mean:

> Native DLTH supply can never change.

That statement would prematurely decide issuance and destruction policy.

Instead, the model-independent requirement is:

> Every consensus-valid transition that affects native DLTH must satisfy the
> authoritative monetary and supply invariants applicable to that transition.

A future transition may be:

- supply-neutral;
- supply-increasing under an explicitly authorized monetary rule;
- supply-decreasing under an explicitly authorized monetary rule; or
- another explicitly specified monetary class.

This document selects none of those classes as an actual protocol feature.

For every adopted class, compliant implementations must derive the same
deterministic monetary result.

## 3. Native DLTH Terminology

The following terms are analytical only.

| Term | Decision-readiness meaning | Current status |
|---|---|---|
| Native DLTH value | Consensus-recognized quantity belonging to the protocol's native monetary domain | Representation unresolved |
| Recognized native supply | The quantity of DLTH that authoritative protocol semantics recognize as existing at a canonical point | Exact derivation unresolved |
| Value-bearing state | Any future canonical logical state whose interpretation contributes to recognized native DLTH value | Representation unresolved |
| Supply-neutral transition | A future transition whose applicable authoritative monetary rule permits no change in recognized native supply | Conditional concept only |
| Supply-changing transition | A future transition for which the authoritative monetary rules explicitly permit a non-zero supply change | Mechanisms and policies unresolved |
| Monetary delta | The authoritative change, if any, in recognized native supply caused by a canonical transition | Representation unresolved |
| Issuance | A future authoritative monetary effect that increases recognized native supply | Not selected |
| Destruction | A future authoritative monetary effect that decreases recognized native supply | Not selected |
| Representation creation | Creation of a logical state representation that may hold or refer to existing value | Must not automatically imply issuance |
| Representation removal | Removal or retirement of logical state representation | Must not automatically imply monetary destruction |
| Transfer | Reassignment or movement of already-recognized value under future authoritative rules | Transaction representation unresolved |
| Split | Representation of existing value in more resulting value-bearing relations or records | Not a monetary policy decision |
| Merge | Representation of existing value in fewer resulting value-bearing relations or records | Not a monetary policy decision |
| Migration | Protocol-authorized change in representation or interpretation across versions | Mechanics unresolved |
| Monetary reconciliation | Demonstration that a transition's native-value effect exactly matches the monetary rule applicable to that transition | Required conceptually |

These terms do not imply Account, UTXO, object, hybrid, or any other concrete
state representation.

## 4. Representation Change Is Not Automatically Supply Change

A future protocol must distinguish changes in representation from changes in
native supply.

Examples include:

- creating a new logical value-bearing record;
- removing an old logical value-bearing record;
- splitting one logical value relation into several;
- merging several logical value relations into one;
- moving value between authorities;
- re-encoding state;
- migrating to a new protocol version; or
- restoring state after authoritative history reversion.

None of those events may be treated as issuance or destruction merely because
the number, shape, identity, or encoding of logical records changes.

Likewise, a future monetary issuance or destruction rule must not be hidden
inside an apparently ordinary representation operation.

The future authoritative specification must make the distinction explicit.

## 5. Supply-Neutral Transition Requirement

If a future transition is authoritatively classified as supply-neutral, its
canonical post-state must represent exactly the same total native DLTH quantity
as its canonical pre-state because the applicable rule permits no monetary
supply change.

The proof of that property must not depend on:

- database layout;
- implementation-specific indexes;
- local caches;
- wallet accounting;
- presentation-layer balances;
- floating-point arithmetic;
- host integer overflow;
- execution scheduling; or
- implementation-specific iteration order.

This document does not define how total native DLTH is represented or computed.

## 6. Supply-Changing Transition Requirement

If future authoritative rules permit a supply-changing transition, then:

- the transition class must be explicitly defined;
- the permitted monetary delta must be deterministic;
- every condition affecting that delta must be consensus-defined;
- the monetary effect must be independently reproducible;
- no implementation may create an additional unaccounted delta;
- failure must not leave a partial monetary delta;
- reorganization and reapplication must not double-count the effect; and
- historical verification must interpret the effect under the rules applicable
  to that history.

This document does not determine whether any such transition will exist.

## 7. Issuance Boundary

The Constitution does not permit:

- pre-mine;
- foundation allocation;
- team allocation;
- reserved coins;
- retroactive issuance;
- privileged allocation;
- hidden issuance; or
- preferential monetary treatment.

Any future issuance design must remain within those constitutional boundaries.

This document does not determine:

- whether issuance exists;
- when issuance may occur;
- who may receive it;
- how eligibility is determined;
- how much may be issued;
- whether issuance changes over time;
- whether issuance terminates;
- whether rewards are issuance;
- whether fees affect issuance;
- whether issuance is tied to consensus participation; or
- how issuance is represented in state.

No privileged key or privileged discretionary authority may be introduced as a
shortcut for monetary control.

## 8. Genesis Monetary Boundary

Genesis is constitutionally immutable once the network launches.

The Constitution also permanently prohibits pre-mine and reserved or privileged
allocations.

The future Formal Specification must therefore eventually make the Genesis
monetary state and any launch-time monetary transition unambiguous.

This document does not define:

- exact Genesis native supply;
- exact first circulating supply;
- a Genesis issuance event;
- a first-block issuance event;
- launch rewards;
- allocation tables;
- recipient sets; or
- monetary activation timing.

No ambiguity in those TBDs may be resolved by implementation convention.

## 9. Destruction and Burn Boundary

This document does not select destruction or burn.

If a future authoritative monetary rule permits destruction, it must eventually
define:

- the exact event that constitutes destruction;
- the exact quantity destroyed;
- whether destruction is voluntary, rule-driven, or another deterministic
  category;
- what authorization, if any, is required;
- the distinction between destroyed value and merely inaccessible value;
- the distinction between destroyed value and removed representation;
- failure behavior;
- historical interpretation;
- reorganization behavior; and
- independently verifiable supply impact.

A lost credential, inaccessible asset, deleted local wallet, missing local
database record, or abandoned value must not automatically be interpreted as
protocol-level destruction.

## 10. Inaccessibility Is Not Automatically Monetary Destruction

Spendability, authorization, ownership, accessibility, and recognized native
supply are distinct concepts.

A value quantity may become practically inaccessible because suitable
authorization evidence cannot be produced.

That fact alone does not establish that the protocol has destroyed the value.

Conversely, if a future authoritative rule explicitly destroys value, the
protocol must not continue to count that value as live merely because old
historical representations remain observable.

This document defines no lost-key policy, recovery mechanism, confiscation rule,
expiry rule, dormant-value rule, or automatic destruction rule.

## 11. Fees, Rewards, Rent, Refunds, and Subsidies

All of the following remain TBD:

- transaction fees;
- block fees;
- fee recipients;
- fee burning;
- fee redistribution;
- validator rewards;
- proposer rewards;
- staking rewards;
- subsidies;
- storage rent;
- refunds;
- rebates;
- cleanup incentives;
- dust economics; and
- any scalar pricing formula.

A future design must classify every adopted monetary effect explicitly.

For example, an adopted fee mechanism would eventually need to state whether
native value is:

- transferred;
- destroyed;
- redistributed;
- retained by another protocol-defined entity;
- combined with an issuance rule; or
- handled through another explicitly specified monetary effect.

This list does not select any option.

## 12. Economic Safety Versus Monetary Conservation

Economic pricing and monetary accounting are related but distinct.

A fee or rent rule cannot be assumed merely because resource safety requires
bounded behavior.

Likewise, conservation correctness cannot substitute for resource or economic
safety.

Future protocol work must distinguish:

- hard validity and resource-safety limits;
- canonical native-value accounting;
- monetary supply rules;
- economic pricing;
- local admission policy; and
- wallet behavior.

No fee can purchase permission to violate a future hard safety rule.

No local policy may change canonical native supply.

## 13. Arithmetic Safety

Consensus-relevant native-value arithmetic must eventually have:

- deterministic semantics;
- host-independent behavior;
- explicit overflow behavior;
- explicit underflow behavior;
- no floating-point dependence;
- no undefined behavior; and
- identical interpretation across compliant implementations.

Future authoritative native-value semantics must also make the semantic monetary
unit or denomination unambiguous, including its precision and any conversion or
rounding behavior that can affect consensus interpretation across protocol
versions.

This document does not select:

- native monetary denomination or base unit;
- monetary precision;
- conversion ratio or scaling rule; or
- rounding rule.
- amount width;
- signed or unsigned amount representation;
- accumulator width;
- intermediate width;
- maximum supply;
- maximum transfer amount;
- saturation;
- modular arithmetic; or
- numeric limits.

An arithmetic boundary failure must not create or destroy DLTH through wraparound,
partial mutation, or implementation disagreement.

## 14. Failure Atomicity

A failed candidate must not leave a partial canonical monetary effect.

Failure must not partially:

- debit value;
- credit value;
- issue value;
- destroy value;
- transfer fees;
- apply rewards;
- apply refunds;
- create a value-bearing representation;
- remove a value-bearing representation;
- change recognized supply;
- migrate only part of a value relation; or
- alter monetary-accounting metadata that would affect later validity.

The future authoritative transaction model must define the exact failure boundary.

This document selects no validation order or transaction mechanism.

## 15. Creation, Deletion, Zero, and Absence

Future state semantics must distinguish where necessary among:

- logical absence;
- first creation;
- zero quantity;
- non-zero quantity;
- deletion;
- retirement;
- replacement;
- recreation;
- historical existence; and
- current existence.

These lifecycle states must not accidentally change recognized native supply.

Questions that remain unresolved include:

- Is a zero-valued logical relation present or absent?
- Can value-bearing state be deleted?
- Does deletion require zero value?
- Can a deleted logical entity later be recreated?
- Does recreation restore any old monetary meaning?
- Does absence prove non-existence, zero, consumption, or something else?
- Which lifecycle facts must remain verifiable historically?

This document defines no answers.

## 16. Transfer, Split, and Merge

The future native-value model must demonstrate monetary reconciliation for
ordinary value movement.

Analytical cases may include:

- one authority relation to another;
- one value-bearing relation split into several;
- several value-bearing relations merged into fewer;
- fan-out;
- fan-in;
- self-directed value movement;
- zero-value edge cases, if such values are later permitted; and
- boundary-value cases.

These are semantic scenarios only.

They do not define transaction inputs, outputs, accounts, balances, change
outputs, record identities, or authorization structures.

## 17. Migration and Protocol Evolution

Protocol evolution must not create an unexplained native-supply delta.

A future migration must eventually specify:

- which old value-bearing semantics remain historically valid;
- which current representation replaces or interprets them;
- whether migration itself is supply-neutral;
- whether any monetary rule is activated simultaneously;
- how duplicate migration is excluded;
- how partially failed migration behaves;
- how dormant value is treated;
- how independent implementations verify the migration result; and
- how ownership-preservation requirements remain satisfied.

Re-encoding value is not automatically issuance.

Retiring an old representation is not automatically destruction.

No migration mechanism is selected here.

## 18. Historical Interpretation Versus Current Monetary Acceptance

Historical monetary interpretation and current acceptance are separate.

Historical interpretation asks:

> What native-value and supply effect did this canonical transition have under
> the authoritative rules applicable to that history?

Current acceptance asks:

> Which old monetary forms, rules, representations, or authorization conditions
> remain usable for a new current transition?

A protocol upgrade must not silently reinterpret an old supply-neutral transition
as supply-changing or vice versa.

No old-version acceptance period, deprecation schedule, checkpoint, archival
policy, or migration schedule is selected.

## 19. Reorganization and Monetary Reapplication

A future consensus design must ensure that authoritative history reversion and
reapplication produce deterministic native-value accounting.

Future specification must eventually define enough semantics to ensure that:

- reverted value transfers no longer affect current canonical state;
- reverted supply-changing effects no longer contribute to current canonical
  supply exactly as specified;
- created value-bearing state is reverted correctly;
- removed or replaced value-bearing state is restored correctly where
  applicable;
- reapplying a previously reverted transition applies its monetary effect exactly
  once in the new canonical history; and
- local memory of prior application cannot cause monetary disagreement.

No fork-choice rule, reorganization depth, finality mechanism, journal format, or
rollback implementation is selected.

## 20. Replay and Monetary Conservation

Replay exclusion and monetary conservation are distinct but interacting
requirements.

A replay defect may create an unintended second monetary realization even when
each isolated application appears locally balanced.

A conservation proof must therefore eventually account for:

- already-exercised effects;
- conflicting effects;
- authoritative reversion;
- canonical reapplication;
- version changes;
- lifecycle reset risks; and
- supply-changing effects, if any are later adopted.

This document does not reopen or replace the replay-identity requirements.

It selects no replay mechanism.

## 21. Authorization and Monetary Authority

Authorization to spend existing value and authority to change recognized native
supply are distinct concepts.

A future monetary design must not silently infer supply-changing authority from:

- possession of an ordinary spending credential;
- implementation administrator status;
- node operator status;
- repository ownership;
- foundation status;
- developer status;
- local configuration;
- database access; or
- any prohibited privileged key.

If any future supply-changing operation exists, its authority must arise from
explicit deterministic protocol rules consistent with the Constitution.

This document selects no monetary authorizer or governance mechanism.

## 22. Minimal Account Candidate Questions

Minimal Account remains a candidate only.

For lifecycle and conservation comparison, future Account analysis must answer:

- What logical relation carries native value?
- How is the quantity interpreted?
- What constitutes a value-affecting transition?
- How is an ordinary supply-neutral transition reconciled?
- What happens when a logical value relation first appears?
- Is zero distinct from absence?
- Can a logical value relation be removed?
- Can it later be recreated?
- Can deletion or recreation accidentally alter recognized supply?
- How are arithmetic boundaries handled?
- How are multiple affected logical relations updated atomically?
- How are failed transitions prevented from leaving partial value changes?
- How are historical quantities interpreted across versions?
- How does migration preserve monetary reconciliation?
- How does reorganization restore prior monetary state?
- If future supply-changing rules exist, where and how are they distinguished
  from ordinary quantity changes?

No balance field, Account identifier, nonce, lifecycle rule, monetary field, or
state schema is selected here.

## 23. Minimal UTXO Candidate Questions

Minimal UTXO remains a candidate only.

For lifecycle and conservation comparison, future UTXO analysis must answer:

- What logical records carry native value?
- How is each quantity interpreted?
- What replacement relation constitutes an ordinary native-value transition?
- How is a supply-neutral replacement reconciled?
- What distinguishes creation of a representation from issuance?
- What distinguishes retirement or consumption of a representation from
  monetary destruction?
- How are multiple replaced and resulting value quantities reconciled?
- How are arithmetic boundaries handled?
- How does failure prevent partial replacement?
- Can a retired value identity or equivalent monetary relation be recreated?
- How are historical value records interpreted across versions?
- How does migration preserve monetary reconciliation?
- How does reorganization restore prior monetary state?
- If future supply-changing rules exist, how are they distinguished from
  ordinary replacement?

No transaction identifier, output identifier, input structure, output structure,
consumption rule, amount field, index, or state schema is selected here.

## 24. Account and UTXO Neutrality

Minimal Account and Minimal UTXO remain co-equal candidates.

No current evidence establishes that:

- Account has simpler conservation proofs;
- UTXO has simpler conservation proofs;
- Account has safer arithmetic;
- UTXO has safer arithmetic;
- Account has fewer lifecycle hazards;
- UTXO has fewer lifecycle hazards;
- one candidate makes issuance safer;
- one candidate makes destruction safer;
- one candidate has better monetary auditability;
- one candidate has smaller monetary metadata;
- one candidate has better migration behavior; or
- one candidate is preferable for Genesis monetary integrity.

Every future comparison must attribute observed properties to a specific frozen
candidate mapping and assumption profile rather than to the family label alone.

## 25. Monetary Auditability

Future authoritative semantics must make native-value accounting independently
verifiable.

At minimum, compliant independent implementations must eventually be able to
agree on:

- the monetary interpretation of a canonical transition;
- whether that transition is supply-neutral or supply-changing under the
  applicable rules;
- the authoritative monetary delta, if any;
- the canonical native-value post-state;
- the interpretation of historical monetary effects; and
- whether failure, reorganization, and migration preserve the required monetary
  invariants.

This document does not decide whether current recognized supply is:

- stored explicitly;
- committed explicitly;
- reconstructed from state;
- reconstructed from history;
- derived through an accumulator; or
- verified through another mechanism.

## 26. State Commitment, Snapshot, and Synchronization Boundary

Monetary semantics must exist before a commitment construction is used to prove
them.

A future authenticated-state design may need to support proofs concerning:

- value-bearing state;
- existence;
- absence;
- current quantity;
- current recognized supply;
- historical monetary state;
- or another monetary property.

Which of those are required remains unresolved.

Future snapshot and synchronization work must eventually answer:

- what monetary facts must be represented in a snapshot;
- whether current supply can be verified without replaying full history;
- which historical monetary facts remain necessary;
- how missing or malformed value state is detected;
- how version boundaries are authenticated; and
- how a synchronized node verifies the same monetary state as an independent
  verifier.

No state commitment, proof, snapshot, pruning, or archival scheme is selected.

## 27. Resource and Persistent-State Boundary

Native-value correctness must not depend on physical implementation cost.

At the same time, future value-bearing state may create:

- persistent-state growth;
- logical read or mutation work;
- structural amplification;
- dust-like populations;
- large fan-in or fan-out;
- historical-retention pressure; or
- hostile invalid-candidate work.

Those are relevant comparison and security dimensions.

This document defines no:

- resource unit;
- state charge;
- fee;
- rent;
- cleanup rule;
- dust threshold;
- pruning rule;
- persistent-growth attribution;
- numeric limit; or
- economic conversion formula.

## 28. Adversarial Scenario Matrix

The following scenarios define questions and evidence obligations only.

| Scenario | Required property | Common unresolved question | Account question | UTXO question | Evidence required |
|---|---|---|---|---|---|
| 1. Ordinary native-value movement | Applicable monetary invariant holds exactly | What monetary class is this transition? | How do changed logical quantities reconcile? | How do replaced live value records reconcile? | Abstract value-effect model |
| 2. Split | Representation change does not create value | What quantities are pre/post semantic equivalents? | How are resulting quantities represented? | How are resulting records represented? | Conservation mapping |
| 3. Merge | Representation change does not destroy value | What quantities are pre/post semantic equivalents? | How are merged quantities represented? | How are replacement records represented? | Conservation mapping |
| 4. Self-directed movement | No hidden supply effect | What counts as a real state effect? | Does the logical quantity change? | Does the replacement relation change? | State-effect semantics |
| 5. Creation of value-bearing state | Representation creation is distinguished from issuance | What monetary source accounts for the value? | What does first appearance mean? | What does new record creation mean? | Lifecycle + monetary-source proof |
| 6. Removal of value-bearing state | Representation removal is distinguished from destruction | Where does represented value go? | Can removal occur with non-zero value? | What does retirement/consumption mean? | Lifecycle proof |
| 7. Zero versus absence | No accidental supply reset | Are they semantically different? | Does zero state persist? | Are zero-valued records permitted? | Entity-lifecycle semantics |
| 8. Deletion and recreation | No accidental duplication or destruction | What monetary meaning survives lifecycle change? | Can recreated relation inherit old quantity? | Can equivalent record identity reappear? | Lifecycle/replay evidence |
| 9. Arithmetic maximum boundary | No wrap or ambiguous monetary result | What arithmetic domain applies? | Which quantity operation reaches boundary? | Which aggregate operation reaches boundary? | Arithmetic vectors |
| 10. Arithmetic underflow attempt | Deterministic failure and no partial monetary effect | When is insufficiency detected? | Does debit-like mutation fail atomically? | Does replacement fail atomically? | Negative vectors |
| 11. Failure after partial validation | No partial value or supply effect | Which provisional effects exist? | Which logical quantity changes must roll back? | Which replacements must be discarded? | Failure-atomicity evidence |
| 12. Exact replay | No second monetary realization | What canonical fact excludes it? | How does Account mapping preserve monetary effect uniqueness? | How does UTXO mapping preserve monetary effect uniqueness? | Replay/conservation combined evidence |
| 13. Conflicting candidates | No double realization of incompatible value effects | Which effects conflict? | Which logical quantities overlap? | Which live records overlap? | Conflict evidence |
| 14. Authoritative reorganization | Monetary state follows canonical history | What monetary effects are reverted? | Which quantities restore? | Which live records restore? | Reorg model |
| 15. Reapplication after reversion | Monetary effect applies exactly as specified once in new history | What state makes reapplication eligible? | How is pre-state restored? | How is replacement eligibility restored? | Reapplication vectors |
| 16. Protocol-version migration | No unexplained monetary delta | Is migration supply-neutral? | How do old/new quantities reconcile? | How do old/new live records reconcile? | Migration evidence |
| 17. Dormant value across versions | Monetary interpretation remains unambiguous | Does inaccessibility alter recognized supply? | What persistent value relation remains? | What historical/live record relation remains? | Dormancy/migration analysis |
| 18. Lost authorization evidence | Inaccessibility is not silently treated as destruction | What does consensus still recognize? | Does quantity remain recognized? | Does live value remain recognized? | Ownership/monetary boundary analysis |
| 19. Hypothetical authorized issuance | Exact delta and constitutional compliance | What authoritative rule permits it? | How is issuance distinguished from ordinary credit-like change? | How is issuance distinguished from ordinary record creation? | Only after authoritative monetary rule exists |
| 20. Hypothetical authorized destruction | Exact delta and constitutional compliance | What authoritative rule permits it? | How is destruction distinguished from ordinary removal? | How is destruction distinguished from consumption? | Only after authoritative monetary rule exists |
| 21. Hypothetical fee | Fee accounting is explicit | Transfer, destruction, redistribution, or another class? | How are affected quantities represented? | How are affected value records represented? | Only after fee rules exist |
| 22. Hypothetical reward | Reward accounting is explicit | Issuance, transfer, or another class? | How is reward distinguished from ordinary quantity creation? | How is reward distinguished from ordinary record creation? | Only after reward rules exist |
| 23. Snapshot sync | Monetary state is independently verifiable | Which current supply facts must be proven? | What logical state is necessary? | What live-value state is necessary? | Commitment/snapshot requirements |
| 24. Large fan-out/fan-in | Monetary correctness survives structural complexity | Which aggregate quantities must reconcile? | How many logical relations are touched? | How many records are replaced or created? | Structural bounds + conservation evidence |
| 25. Independent revalidation years later | Historical monetary result is reproducible | Which version's monetary rules apply? | How are old logical quantities interpreted? | How are old value records interpreted? | Multi-version conformance |
| 26. Malformed supply-changing candidate | Invalid candidate cannot create partial supply | Where is monetary authorization checked? | What provisional quantity change is possible? | What provisional record creation is possible? | Failure and hostile-work analysis |

Hypothetical issuance, destruction, fee, and reward scenarios may not be promoted
into ordinary comparison cases until authoritative monetary rules support them.

## 29. Evidence Required Before Candidate Comparison

Before native-value lifecycle and conservation can contribute to Account/UTXO
ranking, both candidate mappings must have:

- the same external semantic value case;
- explicit monetary-rule assumptions;
- explicit classification of supply-neutral versus any authoritatively supported
  supply-changing cases;
- model-specific lifecycle mapping;
- deterministic arithmetic assumptions;
- failure oracle;
- replay interaction;
- reorganization and reapplication assumptions;
- migration/version assumptions;
- ownership and authorization assumptions;
- equivalent feature scope;
- symmetric optimization opportunities;
- explicit unresolved dimensions;
- independent semantic review;
- formal or executable conservation evidence where possible; and
- conclusions limited to the frozen mapping actually tested.

No candidate may receive an assumed monetary feature that the other candidate is
not permitted to map equivalently.

## 30. Formal-Verification Obligations

Future native-value semantics should support proofs or equivalent rigorous
arguments for:

- compliance with constitutional monetary boundaries;
- correctness of every supply-neutral transition;
- correctness of every future authorized supply-changing transition;
- no hidden issuance;
- no hidden destruction;
- deterministic monetary delta;
- deterministic overflow and underflow behavior;
- failure atomicity;
- replay/double-spend interaction;
- lifecycle safety;
- deletion/recreation safety;
- migration correctness;
- reorganization/reapplication correctness;
- historical-version interpretation; and
- independent-implementation equivalence.

### Minimal Account

Future Account evidence may require proofs concerning:

- quantity reconciliation;
- multi-entity atomic mutation;
- zero/absence/deletion/recreation behavior;
- arithmetic boundaries; and
- separation of ordinary quantity change from any future monetary delta.

### Minimal UTXO

Future UTXO evidence may require proofs concerning:

- replacement-value reconciliation;
- creation versus issuance;
- consumption/retirement versus destruction;
- aggregate arithmetic boundaries; and
- separation of ordinary replacement from any future monetary delta.

These are proof topics, not selected mechanisms.

## 31. Premature-Commitment Matrix

| Classification | Items | Boundary |
|---|---|---|
| AUTHORITATIVE NOW | Zero pre-mine; zero foundation allocation; zero team allocation; zero reserved coins; no retroactive issuance; no privileged allocation; no hidden issuance; no preferential monetary treatment | Direct constitutional boundaries |
| SAFE TO RECORD NOW | Deterministic monetary interpretation; explicit overflow/underflow behavior; failure atomicity; representation change must not silently create or destroy supply; every value-affecting transition must satisfy applicable authoritative monetary/supply invariants | Model-independent requirements |
| ABSTRACT REQUIREMENT ONLY | Recognized supply, monetary delta, supply-neutral reconciliation, supply-changing reconciliation, lifecycle/supply distinction | Exact representation and mechanisms unresolved |
| KEEP AS CONDITIONAL CASE | Future issuance, destruction, rewards, fees, rent, refunds, subsidies | Only if later authorized |
| KEEP AS CANDIDATE | Minimal Account value accounting; Minimal UTXO value accounting | Neither selected nor ranked |
| BLOCKED | Balance field; amount width; transaction-value equation; output-value formula; supply accumulator; issuance formula; reward schedule; burn formula; fee formula; monetary epoch; supply cap | Formal monetary/state/transaction design unresolved |
| DEFER | Wallet balances; local accounting indexes; UI supply display; coin selection; physical database aggregation | Cannot define consensus monetary semantics |
| DO NOT ADOPT | Hidden minting; privileged discretionary minting; privileged allocations; implementation-local supply changes; database record count as native supply; deletion as implicit burn; record creation as implicit issuance | Violates authority or monetary determinism |

## 32. Decision Gates

Native DLTH lifecycle/conservation analysis remains incomplete until
decision-ready abstract answers exist for:

1. Native-value semantic domain, denomination or base-unit semantics, precision,
   and any consensus-relevant conversion or rounding requirements
2. Deterministic arithmetic and overflow/underflow requirements
3. Supply-neutral transition reconciliation
4. Classification framework for any future supply-changing transition
5. Genesis monetary boundary consistent with the Constitution
6. Issuance and destruction authority boundaries without privileged authority
7. Distinction between representation lifecycle and monetary lifecycle
8. Zero, absence, deletion, recreation, and inaccessible-value semantics
9. Failure, replay, conflict, reorganization, and reapplication interaction
10. Historical/current monetary interpretation and migration requirements
11. Account/UTXO-neutral evidence obligations
12. Auditability, commitment, snapshot, resource, and economic boundaries

These gates require reviewed abstract semantics.

They do not require selection of:

- a state model;
- a transaction format;
- a monetary policy;
- an issuance schedule;
- a fee model;
- an amount width; or
- a state commitment.

## 33. Formal Specification Boundary

Future authoritative specification will eventually need to define enough
consensus semantics for:

- native DLTH value representation;
- monetary domain;
- exact arithmetic;
- every valid native-value-affecting transition;
- supply-neutral conservation;
- every permitted supply-changing transition, if any;
- Genesis monetary semantics;
- issuance, if any;
- destruction, if any;
- rewards, if any;
- fees, if any;
- value-bearing entity lifecycle;
- transaction preconditions and effects;
- failure atomicity;
- historical monetary interpretation;
- migration;
- reorganization and reapplication;
- canonical serialization of adopted monetary structures;
- authenticated-state interaction, if and where required by a subsequently
  selected consensus-relevant state or commitment architecture; and
- any validity-affecting economic rule.

The Constitution directly requires detailed monetary mechanics to be defined by
the Formal Specification. Other items in this section are future authoritative
specification needs only where they become consensus-relevant under subsequently
selected protocol architecture. Their appearance here does not independently
select or constitutionally require a particular state, commitment, transaction,
or consensus construction.

This document defines none of those normative mechanisms.

It does not modify the Formal Specification.

## 34. Threat Model Boundary

This decision-readiness analysis does not by itself modify the Threat Model.

The analysis identifies generic monetary-security questions such as:

- unintended value creation;
- unintended value destruction;
- arithmetic overflow or underflow;
- partial monetary effects after failure;
- duplicate monetary realization through replay;
- lifecycle reset that changes recognized supply;
- migration accounting mismatch;
- reorganization double-counting;
- implementation disagreement over supply; and
- hostile structural amplification of value-affecting transitions.

After independent review, the Threat Model should be checked separately to
determine whether these questions expose a genuinely new generic threat class
not already represented by existing determinism, failure-atomicity, resource,
versioning, replay, or state-integrity coverage.

Any future threat-model addition must remain state-model-neutral unless a state
model has actually been selected.

No Account-specific balance threat or UTXO-specific value-record threat becomes
project direction through this document.

## 35. Complete TBD Register

The following remain unresolved:

- final state model;
- native DLTH representation;
- native-value semantic type;
- native monetary denomination or base unit;
- monetary precision;
- conversion or scaling semantics;
- rounding semantics, if any;
- amount width;
- arithmetic representation;
- maximum amount;
- maximum recognized supply;
- initial circulating supply;
- exact Genesis monetary state;
- launch-time monetary transition;
- post-Genesis issuance;
- issuance schedule;
- issuance recipients;
- reward mechanism;
- reward amount;
- reward schedule;
- inflation policy;
- supply cap;
- destruction;
- burn;
- fee burning;
- fees;
- fee recipient;
- fee redistribution;
- rent;
- refunds;
- subsidies;
- cleanup economics;
- dust policy;
- transaction format;
- transaction-value equation;
- state-entity format;
- Account quantity representation;
- UTXO quantity representation;
- value-record identity;
- creation semantics;
- zero semantics;
- absence semantics;
- deletion semantics;
- recreation semantics;
- inaccessible-value monetary treatment;
- historical-retention requirements;
- monetary accumulator, if any;
- authenticated-state commitment;
- proof format;
- snapshot semantics;
- synchronization semantics;
- pruning;
- archival obligations;
- replay mechanism;
- conflict rules;
- canonical ordering;
- consensus algorithm;
- fork choice;
- finality;
- reorganization semantics;
- canonical reapplication mechanics;
- resource accounting;
- numeric resource limits;
- activation mechanics;
- migration mechanics; and
- governance thresholds and HIP / Super HIP mechanics.

No item is resolved by its appearance in this document.

## 36. Current Project Impact

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

Native DLTH representation selected:
NO

Amount width selected:
NO

Native monetary denomination selected:
NO

Monetary precision selected:
NO

Conversion or rounding rule selected:
NO

Initial supply selected:
NO

Issuance mechanism selected:
NO

Issuance schedule selected:
NO

Reward mechanism selected:
NO

Destruction or burn mechanism selected:
NO

Fee mechanism selected:
NO

Supply cap selected:
NO

Monetary formula selected:
NO

Transaction format selected:
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

## 37. Next Decision Boundary

Completion and independent review of this document may clarify the third
state-model decision gate:

> Native DLTH lifecycle and conservation requirements.

The existence, completion, or review of this document does not by itself satisfy
the third state-model decision gate. That gate requires sufficiently concrete,
reviewed abstract semantics and evidence; this document is only one
decision-readiness artifact toward that determination.

It also does not satisfy the other remaining state-model gates.

The next model-neutral analytical area is:

> Abstract transaction dependencies, state effects, atomicity, and failure.

That next analysis must not use this document to silently select:

- Account;
- UTXO;
- a transaction format;
- an issuance rule;
- a fee rule;
- a reward rule;
- a supply policy; or
- a monetary mechanism.

## 38. Conclusion

- Constitutional monetary boundaries remain authoritative.
- Zero pre-mine, zero foundation allocation, zero team allocation, and zero
  reserved coins remain fixed constitutional requirements.
- Retroactive issuance, privileged allocation, hidden issuance, and preferential
  monetary treatment remain prohibited.
- Detailed monetary mechanics remain a Formal Specification responsibility.
- Conservation does not mean that this document imposes permanent zero supply
  change.
- Every value-affecting transition must satisfy the authoritative monetary and
  supply invariants applicable to that transition.
- Representation creation is not automatically issuance.
- Representation removal is not automatically destruction.
- Inaccessibility is not automatically monetary destruction.
- Supply-changing transitions, if any, require explicit authoritative rules.
- Issuance is not selected.
- Rewards are not selected.
- Destruction or burn is not selected.
- Fees are not selected.
- Rent, refunds, and subsidies are not selected.
- Amount width and arithmetic representation remain TBD.
- Minimal Account and Minimal UTXO remain co-equal candidates.
- Candidate ranking remains blocked.
- State-model decision remains NOT MADE.
- This document defines no consensus rule.
