# Dilithia Entity Lifecycle and Historical Meaning Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records model-neutral requirements, unresolved questions,
> comparison variables, adversarial scenarios, and evidence gates for entity
> existence, creation, deletion, recreation, and historical meaning.
>
> It defines no consensus rule, resolves no Formal Specification TBD, selects no
> state model, entity representation, identity architecture, Account lifecycle,
> UTXO lifecycle, replay mechanism, transaction format, state commitment,
> pruning policy, archival policy, fee rule, resource meter, or storage
> architecture.

## Status

This document is a Pre-Genesis decision-readiness artifact.

It exists to clarify the fifth state-model decision gate:

> Entity existence, creation, deletion, recreation, and historical meaning.

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
- transaction state-effect, atomicity, and failure requirements;
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
- Generalized Object State;
- an account representation;
- a UTXO representation;
- a generic object representation;
- a state-entity byte format;
- an entity identifier;
- an account identifier;
- an output identifier;
- a stable logical identity architecture;
- a balance field;
- a nonce or sequence field;
- explicit-consumption semantics;
- an existence flag;
- a deletion flag;
- a spent flag;
- a tombstone;
- a generation number;
- an incarnation number;
- a version counter;
- an entity-type registry;
- an entity namespace;
- a transaction format;
- a transaction identity;
- a state commitment;
- a proof system;
- an absence-proof construction;
- a snapshot format;
- a pruning policy;
- a history-retention policy;
- an archival requirement;
- a rollback mechanism;
- a database deletion mechanism;
- garbage collection;
- state rent;
- cleanup rewards;
- refunds;
- fees;
- gas;
- resource units;
- resource numeric limits;
- conflict rules;
- canonical transaction ordering;
- block scheduling;
- parallel execution; or
- any state-model ranking.

## 1. Existing Authoritative Boundaries

The Constitution establishes model-independent boundaries relevant to entity
lifecycle analysis.

Consensus-critical behavior must be defined by authoritative specification rather
than accidental implementation behavior.

Consensus-critical operations must be deterministic across compliant
implementations.

Consensus-critical data that is eventually defined must satisfy applicable
canonical, versioned, and domain-separated serialization requirements.

Previously valid canonical encodings must remain unambiguously decodable or
safely migratable under the Formal Specification.

Address formats and state representations must remain migratable without loss of
ownership.

A protocol upgrade must not render previously valid assets unreachable,
unspendable, or un-migratable.

Consensus correctness is defined by the Constitution and Formal Specification,
not by one implementation or one database representation.

Economic-safety requirements constrain future persistent-state architecture, but
they do not by themselves select state rent, fees, cleanup rewards, refunds,
pruning, deletion, or another lifecycle mechanism.

Detailed state-entity and lifecycle semantics remain a future Formal
Specification responsibility where they become consensus-relevant.

This document does not amend the Formal Specification.

## 2. Purpose of the Fifth State-Model Gate

A fair state-model comparison is premature if "exists," "absent," "created,"
"deleted," or "historical" silently mean different things for different
candidates.

Before Account and UTXO can be compared fairly, the project needs sufficiently
concrete abstract answers to questions such as:

- what it means for a protocol-relevant entity to exist;
- what it means for such an entity to be absent;
- whether different forms of absence must be distinguished;
- whether a zero-valued or empty entity is equivalent to absence;
- what logical event constitutes creation;
- what logical event constitutes deletion or other removal from current state;
- whether and under what conditions a previously used referent may become
  current again;
- how lifecycle transitions interact with replay exclusion;
- how lifecycle transitions interact with ownership and authorization;
- how lifecycle transitions interact with native-DLTH conservation;
- how rejected transitions avoid partial lifecycle effects;
- how protocol versions interpret historical entities;
- what distinction exists between current logical state and historical facts;
- which historical distinctions must remain protocol-interpretable;
- which questions belong to later commitment, conflict, PQ, and resource gates;
  and
- which lifecycle assumptions must be exposed before candidate comparison.

These are semantic decision-readiness requirements.

They are not state-encoding, database, proof-system, or pruning requirements.

## 3. Analytical Terminology

For this document only, the following terms are analytical vocabulary.

**Entity**

A protocol-relevant logical referent or state-bearing subject whose existence,
content, availability, control, or lifecycle may affect consensus semantics.

The term does not imply an Account, UTXO, object, database row, key-value entry,
address, credential, or transaction output.

**Existence**

A protocol-semantic condition under which an entity or relevant state referent
is recognized as currently existing under the applicable authoritative rules.

**Absence**

A protocol-semantic condition under which the relevant entity or state referent
is not recognized as currently existing in the required sense.

Absence does not by itself mean:

- never existed;
- deleted;
- consumed;
- pruned;
- physically missing from a database;
- unknown to a local node; or
- malformed.

**Creation**

A canonical lifecycle transition by which an entity or state referent becomes
recognized as existing under the applicable authoritative rules.

Creation of a representation does not by itself mean creation or issuance of
native DLTH.

**Deletion**

A generic analytical term for a future authoritative lifecycle transition that
causes an entity to cease being recognized as currently existing in some
specified sense.

The word does not select physical database deletion or require that Dilithia
support a deletion operation.

**Recreation**

A generic analytical term for a possible future condition in which a referent
that is related to an earlier non-current entity becomes current again.

This term does not imply that recreation is permitted.

It also does not define what identity or equivalence relation makes the later
entity "the same" as, related to, or distinct from the earlier entity.

**Historical meaning**

The authoritative interpretation of an entity, lifecycle fact, or state
relation under the protocol version and canonical-history context applicable to
it.

Historical meaning does not imply permanent retention of all historical data in
current consensus state.

**Current logical state**

The protocol-semantic facts required to interpret the current authoritative
state.

This term does not specify a commitment, database, snapshot, or storage layout.

## 4. Entity Representation Remains Unselected

Abstract lifecycle semantics must not be mistaken for an entity representation.

This document defines no:

- entity header;
- entity tag;
- account record;
- output record;
- object record;
- identifier width;
- key format;
- value format;
- existence bit;
- deleted bit;
- spent bit;
- tombstone bit;
- lifecycle enum;
- generation counter;
- incarnation counter;
- version field;
- creation-height field;
- deletion-height field;
- history pointer;
- parent reference;
- namespace identifier; or
- canonical state encoding.

Future concrete structures require separate authoritative specification.

## 5. Existence Must Be Protocol-Semantic

Where existence affects validity or canonical effects, its meaning must
eventually be deterministic and implementation-independent.

Two compliant implementations must not disagree about a consensus-relevant
existence fact merely because their physical storage differs.

Existence must not be defined by accidental implementation events such as:

- whether a database key is physically present;
- whether a cache entry exists;
- whether a record has been compacted;
- whether a database retains a deleted row;
- whether a local index contains an entry;
- whether archival data is locally available; or
- whether a storage engine uses a null value, missing key, or special marker.

A future authoritative architecture may encode existence in any reviewed
deterministic way.

No encoding is selected here.

## 6. Absence Semantics

A future state architecture must define the consensus meaning of any form of
absence that affects validity or canonical state.

Potential analytical categories include:

- never-current under the relevant history;
- formerly current but no longer current;
- currently unavailable under a candidate-specific lifecycle rule;
- unresolved because a supplied referent is invalid;
- not part of the relevant current state namespace; or
- another explicitly adopted semantic category.

This list does not require every category to exist.

A future authoritative design may equate categories where doing so is safe,
deterministic, and consistent with all applicable replay, ownership, monetary,
history, migration, commitment, and resource requirements.

The protocol must not accidentally derive semantic distinctions from a physical
database's missing-key behavior.

## 7. Zero, Empty, and Absent Are Not Automatically Equivalent

A numerically zero, logically empty, default-valued, or metadata-only state
representation is not automatically equivalent to absence.

Likewise, absence is not automatically equivalent to a zero value.

If a selected architecture permits a concept such as zero value or empty
content, future authoritative semantics must specify whether that state:

- exists;
- can receive effects;
- can authorize effects;
- retains replay-relevant facts;
- retains ownership-relevant facts;
- has historical significance;
- is eligible for removal from current state; or
- is semantically equivalent to absence for any particular rule.

This document does not require zero-valued Account records.

It does not require zero-valued UTXO records.

It does not define a zero-balance deletion rule.

## 8. Invalid Referent Versus Absent Entity

A malformed, non-canonical, unsupported, or otherwise invalid referent must not
be silently treated as an ordinary absent entity unless future authoritative
rules explicitly establish that result.

The future protocol may need to distinguish:

- syntactically invalid representation;
- non-canonical representation;
- validly formed referent whose target is absent;
- unsupported protocol version;
- valid current entity;
- historical but non-current entity; and
- candidate-specific unavailable state.

The exact classification remains TBD.

This document defines no entity identifier grammar.

## 9. Creation Semantics

If a selected architecture supports entity creation, authoritative semantics must
eventually define:

- the preconditions for creation;
- what logical fact becomes current;
- whether a prior related entity may affect creation validity;
- which authorization requirements apply;
- which replay requirements apply;
- which native-value rules apply;
- the atomicity boundary;
- the protocol-version interpretation;
- relevant resource consequences; and
- relevant conflict dependencies.

This document does not select a creation mechanism.

It does not require account creation.

It does not require output creation.

It does not require object creation.

## 10. Creation and Native-Value Issuance Are Distinct

Creation of an entity or representation must not be assumed to create native
DLTH.

A lifecycle transition may create a new representation while preserving the
same total value under the applicable authoritative monetary rules.

Conversely, if future authoritative monetary rules permit issuance, the fact
that issuance occurs does not by itself determine the state-model representation
used to record it.

This document selects no:

- initial supply;
- issuance mechanism;
- reward;
- burn;
- fee;
- supply cap;
- amount representation; or
- value-record format.

Native-value semantics remain governed by the applicable authoritative monetary
rules and their separate decision-readiness analysis.

## 11. Creation Uniqueness and Aliasing Boundary

A future architecture must prevent lifecycle ambiguity that would allow two
compliant implementations to interpret one consensus-relevant referent as
different current entities or different referents as the same current entity
when that difference changes validity or effects.

This property may require a future identity, uniqueness, or anti-aliasing rule.

No such mechanism is selected here.

This document does not select:

- transaction-hash-derived identity;
- output indexes;
- account identifiers;
- object identifiers;
- random identifiers;
- counters;
- generation numbers; or
- globally unique identifiers.

Identity requirements remain coordinated with replay and canonical-identity
analysis.

## 12. Deletion Semantics

If a selected architecture permits deletion or equivalent removal from current
logical state, future authoritative semantics must define:

- what ceases to be current;
- which preconditions permit the transition;
- which authorization applies;
- whether native value may remain associated with the removed entity;
- how replay safety is preserved;
- whether later related creation is permitted;
- how history interprets the prior entity;
- what current-state facts, if any, must remain;
- what commitment semantics are required; and
- what resource consequences apply.

This document does not determine that deletion exists.

It does not select a deletion encoding or physical removal mechanism.

## 13. Logical Deletion Is Not Physical Database Deletion

A protocol-semantic lifecycle transition must not be confused with storage-engine
behavior.

Logical deletion does not require:

- deletion of a database row;
- erasure of historical blocks;
- erasure of archival data;
- compaction;
- garbage collection;
- removal of a commitment node;
- deletion from every index;
- a tombstone;
- physical secure erasure; or
- immediate storage reclamation.

Likewise, physical removal of implementation-local data must not change
authoritative lifecycle semantics.

Physical database management remains implementation and benchmark evidence unless
future authoritative rules explicitly make some abstract property
consensus-relevant.

## 14. Deletion and Native-Value Destruction Are Distinct

Removal of an entity representation is not automatically destruction or burn of
native DLTH.

A future lifecycle rule must not silently destroy native value merely because a
record, entity, reference, or representation ceases to be current.

Any actual native-value destruction requires authority from the applicable
monetary rules.

This document selects no burn mechanism.

## 15. Deletion Must Not Create a Replay Reset

Lifecycle semantics must not create a path by which a previously exercised
spendable effect becomes spendable again contrary to the applicable replay
rules.

This requirement does not require a tombstone.

It does not require a persistent nonce.

It does not require explicit-consumption state.

It does not require permanent retention of every historical entity.

A future architecture may preserve replay safety through any reviewed mechanism
that satisfies the authoritative replay contract.

The lifecycle design must therefore expose enough semantics to determine whether
deletion or removal changes replay-relevant facts.

## 16. Recreation Semantics

A future architecture must explicitly determine whether any category of
recreation exists.

If recreation is permitted, authoritative semantics must eventually define:

- what makes the later entity related to the earlier entity;
- whether it is semantically the same entity, a new entity, or another category;
- which prior facts remain relevant;
- which ownership facts remain relevant;
- which authorization facts remain relevant;
- which replay facts remain relevant;
- whether prior native-value history matters;
- which protocol version interprets each lifecycle phase;
- what conflicts exist; and
- what historical facts remain distinguishable.

This document does not permit or prohibit recreation globally.

## 17. Recreation Must Not Recreate Spendability Accidentally

No future recreation rule may make an already exercised native-value effect
spendable again while the earlier exercise remains authoritative under the
applicable canonical history.

Any future expressly authorized new native-value effect must remain
canonically distinct, for replay purposes, from the previously exercised effect
under the applicable authoritative replay and monetary rules.

Canonical reapplication after authoritative reversion of the earlier exercise is
a separate case whose semantics remain unresolved and must be defined by future
authoritative rules.

A lifecycle reset must not silently reset replay protection.

This is a semantic requirement only.

It does not select:

- nonce persistence;
- sequence persistence;
- output tombstones;
- permanent spent records;
- identity generation numbers; or
- another replay mechanism.

## 18. Ownership and Authorization Boundary

Entity lifecycle and ownership are related but distinct.

Existence of an entity does not by itself establish:

- who owns it;
- what credential controls it;
- that it has one owner;
- that it has one credential;
- that its identifier is an owner identifier; or
- that control survives every possible credential failure.

If creation, deletion, recreation, migration, or other lifecycle transitions can
affect control, the future authoritative rules must preserve applicable
authorization and constitutional ownership guarantees.

This document does not select:

- stable identity;
- credential-bound identity;
- authorization descriptors;
- key rotation;
- recovery;
- delegation;
- multisig; or
- alternate authority.

## 19. Migration and Lifecycle Continuity

A protocol upgrade that changes state representation or lifecycle interpretation
must remain consistent with constitutional ownership and state-preservation
requirements.

A future migration must not silently:

- make legitimately controlled value unreachable;
- make legitimately controlled value unspendable;
- erase ownership-required state;
- create new spendability from an exercised effect;
- destroy native value;
- create native value;
- reinterpret an old entity inconsistently across compliant implementations; or
- make the result depend on implementation-specific storage history.

The exact migration mechanism remains TBD.

## 20. Historical Meaning Is Version-Relative Where Required

Historical entity interpretation must eventually be precise enough that
compliant implementations can interpret prior canonical state and effects under
the rules applicable to them.

A current protocol version must not accidentally reinterpret an old canonical
entity solely according to current implementation behavior.

Future authoritative rules may need to determine:

- which protocol version governs creation;
- which protocol version governs a later lifecycle transition;
- which historical representation remains decodable;
- whether a migration changes current representation while preserving prior
  meaning;
- which old lifecycle facts remain relevant to current validity; and
- how authoritative reapplication interprets historical effects.

This document selects no version field or activation mechanism.

## 21. Historical Existence and Current Existence Are Distinct Concepts

An entity having existed in an authoritative past state does not automatically
mean it exists in current logical state.

Likewise, current absence does not automatically mean the entity never existed.

The protocol may eventually require distinctions among:

- currently existing;
- currently absent;
- previously existing under authoritative history;
- removed under an authoritative lifecycle transition;
- unavailable under a candidate-specific rule;
- part of a reverted or non-current history; and
- never existing under the relevant authoritative history.

This document does not require all these categories to be materialized in
current consensus state.

## 22. Historical Meaning Does Not Imply Permanent Current-State Retention

The ability to interpret historical lifecycle facts does not by itself require
every historical entity, version, deletion marker, or prior value to remain in
current logical state forever.

Future architecture may distinguish:

- current consensus state;
- historical blocks and transactions;
- historical commitments or proofs;
- archival data;
- snapshot data;
- reorganization-support data;
- temporary validation data; and
- implementation-local indexes and caches.

Which data must be retained, pruned, archived, reconstructible, or provable
belongs to later commitment, synchronization, consensus, and resource analysis
unless required earlier by a specific authoritative semantic rule.

This document selects no retention period or pruning rule.

## 23. Pruning Is Not Deletion Semantics

Pruning is a data-retention or state-management concept.

Deletion is a lifecycle semantic concept in this document.

They must not be silently equated.

A logically non-current entity may still have historical data retained.

A locally pruned historical representation does not by itself change whether the
entity existed under authoritative history.

This document defines no:

- pruning eligibility;
- pruning horizon;
- archival node role;
- history expiry;
- cleanup transaction;
- cleanup reward;
- state rent;
- refund; or
- garbage-collection rule.

## 24. Snapshot and Synchronization Boundary

Entity lifecycle semantics must eventually be precise enough that independently
synchronizing compliant nodes can reach the same authoritative interpretation of
current existence and lifecycle-relevant state.

This requirement does not select a snapshot mechanism.

It does not define:

- snapshot contents;
- trust model;
- proof format;
- checkpoint;
- state root;
- bootstrap protocol;
- history requirement; or
- archival obligation.

Authenticated-state and synchronization requirements belong to their separate
later gate.

## 25. Commitment Boundary

Logical entity existence and authenticated-state commitment are distinct design
surfaces.

The lifecycle layer must eventually provide clear logical subjects that a later
commitment design can represent where required, such as:

- current existence;
- current absence;
- current availability;
- current entity content;
- or another adopted lifecycle fact.

Whether historical existence, prior deletion, prior consumption, or another
historical fact requires a proof is not selected here.

No tree, trie, accumulator, hash algorithm, proof system, state root, or
absence-proof construction is selected.

A commitment construction must not be used to invent lifecycle semantics missing
from the logical state specification.

## 26. Reorganization and Canonical History Boundary

Lifecycle semantics must remain deterministic when the authoritative canonical
history changes under whatever future consensus rules are eventually adopted.

The project must distinguish:

- replay of a candidate;
- canonical reapplication after authoritative history change;
- restoration of a previously current state;
- removal of effects from a reverted history; and
- historical interpretation of non-current history.

This document does not select:

- fork choice;
- finality;
- reorganization depth;
- undo logs;
- rollback storage;
- journals;
- snapshots;
- database transactions; or
- state reconstruction architecture.

## 27. Failure Atomicity for Lifecycle Effects

A rejected candidate must not leave only part of a required lifecycle transition
canonical.

For example, if a future transaction class requires a logically coupled set of
effects involving existence, ownership, replay, or native value, rejection must
not leave an inconsistent subset canonical.

This requirement does not select a rollback mechanism.

Physical rollback, overlay, journal, immutable-state, copy-on-write, and database
transaction techniques remain implementation choices unless future authoritative
rules establish otherwise.

## 28. Lifecycle and Transaction Dependencies

A future transaction architecture must expose enough consensus-semantic
information to determine lifecycle-relevant preconditions and effects
deterministically.

Possible abstract questions include:

- must a referenced entity currently exist;
- must it currently be absent;
- may an effect create a currently absent entity;
- may an effect make a current entity non-current;
- does a prior related lifecycle fact affect validity;
- does lifecycle state affect authorization;
- does lifecycle state affect replay;
- does lifecycle state affect native value; and
- does lifecycle state affect a future conflict relation.

This document does not require an explicit lifecycle field, access list, read set,
write set, or declared dependency list.

## 29. Lifecycle and Conflict Boundary

Lifecycle semantics may expose conditions that later create transaction conflicts.

Examples may include competing attempts involving:

- the same current entity;
- the same creation referent;
- one creation and one deletion;
- one deletion and one mutation;
- incompatible recreation attempts; or
- another lifecycle-sensitive dependency.

These examples do not define a conflict.

The later conflict and ordering gate must determine:

- what constitutes a conflict;
- what order is canonical;
- whether conflicts are rejected;
- whether one candidate wins;
- whether effects commute;
- and how serial and parallel execution remain equivalent.

No conflict rule is selected here.

## 30. Lifecycle and Resource Boundary

Entity creation, retention, replacement, deletion, recreation, and historical
requirements can affect persistent-state exposure.

The fifth gate must therefore expose lifecycle semantics sufficiently for the
later resource gate to evaluate:

- logical population growth;
- persistent current-state growth;
- metadata growth;
- historical-retention exposure;
- high-churn creation/removal behavior;
- invalid-candidate lifecycle work; and
- migration exposure.

This document does not select:

- logical resource units;
- counters;
- numeric limits;
- storage pricing;
- fees;
- rent;
- refunds;
- cleanup;
- dust rules;
- retention limits; or
- pruning.

Physical database write amplification, compaction, cache behavior, indexes, and
page layout remain implementation and benchmark evidence.

## 31. Minimal Account Candidate Questions

Minimal Account remains a candidate only.

Later Account mapping must answer questions such as:

- What makes an account-like entity currently exist?
- Can a valid account-like referent be absent?
- Does a zero native-value state remain an existing entity?
- Can an account-like entity exist with no native value?
- If an entity becomes zero-valued, is any lifecycle transition implied?
- Is deletion possible?
- If deletion is possible, what logical facts cease to be current?
- What happens to replay-relevant facts when deletion occurs?
- What happens to authorization-relevant facts when deletion occurs?
- Can an account-like referent later become current again?
- If so, is that recreation or a new entity?
- Can recreation accidentally reset replay exclusion?
- Can recreation accidentally change ownership interpretation?
- How does migration affect dormant or previously existing account-like state?
- What historical facts remain protocol-interpretable?
- Which current-state facts must remain for later commitment analysis?
- Which logical lifecycle operations contribute to persistent-state exposure?
- How does authoritative history change restore or remove lifecycle effects?
- Which lifecycle dependencies must be exposed to later conflict analysis?

These questions do not select:

- an account identifier;
- a balance field;
- a nonce;
- a sequence;
- an existence bit;
- a tombstone;
- a deletion rule;
- a recreation rule;
- persistent authorization metadata; or
- account creation semantics.

## 32. Minimal UTXO Candidate Questions

Minimal UTXO remains a candidate only.

Later UTXO mapping must answer questions such as:

- What makes a referenced value-like entity currently available?
- What semantic relationship exists between existence and availability?
- What distinguishes never-existing from previously available state where that
  distinction matters?
- What lifecycle event, if any, makes a referenced entity unavailable?
- Is "consumption" the selected lifecycle mechanism or only one candidate
  possibility?
- If an entity becomes unavailable, must any current logical fact remain?
- Can a previously used referent ever become available again?
- What identity or uniqueness requirement prevents unintended recreation?
- How does ownership interpretation apply across the lifecycle?
- How does native-value conservation apply to created and non-current
  representations?
- How are dormant old-version entities interpreted after migration?
- What historical facts remain protocol-interpretable?
- Which current-state facts must remain for later commitment analysis?
- Which logical lifecycle operations contribute to persistent-state exposure?
- How does authoritative history change restore or remove lifecycle effects?
- Which lifecycle dependencies must be exposed to later conflict analysis?

These questions do not select:

- transaction inputs;
- transaction outputs;
- output identifiers;
- output indexes;
- explicit consumption;
- spent flags;
- tombstones;
- creation rules;
- deletion rules;
- recreation rules; or
- change-output semantics.

## 33. Account and UTXO Neutrality

Minimal Account and Minimal UTXO remain co-equal candidates.

Neither candidate may assume its preferred lifecycle semantics as a common
requirement.

Neither candidate may receive an easier definition of absence.

Neither candidate may receive weaker replay obligations after deletion,
consumption-like transitions, or recreation.

Neither candidate may assume that its current-state representation is also the
required historical representation.

Neither candidate may assume its preferred pruning, retention, or commitment
architecture.

No current evidence establishes that:

- Account has simpler lifecycle semantics;
- UTXO has simpler lifecycle semantics;
- Account requires less persistent state;
- UTXO requires less persistent state;
- Account deletion is cheaper;
- UTXO consumption is cheaper;
- Account recreation is inherently unsafe;
- UTXO recreation is inherently impossible;
- Account absence proofs are simpler;
- UTXO absence proofs are simpler;
- one model has easier migration;
- one model has easier historical verification; or
- one model is preferable for Genesis.

Those remain hypotheses until the relevant lifecycle, replay, identity,
authorization, commitment, workload, and resource assumptions are frozen.

## 34. Generalized Object and Hybrid Boundary

Generalized Object State remains deferred.

This fifth-gate analysis does not use generic-object terminology to introduce:

- object identifiers;
- object versions;
- mutable shared objects;
- split/merge semantics;
- references between arbitrary objects;
- object schemas;
- a runtime; or
- a VM.

An active multiple-native-value-model Hybrid remains deferred.

Namespace-separated non-value state remains only a candidate concept and gains no
entity representation or lifecycle rule through this document.

No generalized entity framework is selected merely because this document uses
the analytical word "entity."

## 35. Adversarial Scenario Matrix

Future evidence should include equivalent external semantic scenarios for both
candidate mappings where applicable.

At minimum, lifecycle analysis should cover:

| Scenario | Model-neutral property |
|---|---|
| Required entity currently exists | Deterministic current-existence interpretation |
| Required entity currently absent | Deterministic absence interpretation |
| Valid referent with no current entity | Distinct from malformed input where required |
| Zero or empty candidate state | Explicitly distinguished from or equated with absence |
| Ordinary lifecycle creation | Deterministic authorized creation semantics if supported |
| Duplicate or competing creation | No ambiguous current entity or accidental alias |
| Lifecycle removal | Deterministic current-state result if supported |
| Removal with native value involved | No unauthorized destruction or conservation violation |
| Removal with replay-relevant state involved | No accidental recreation of spendability |
| Recreation attempt | Deterministic permitted or rejected semantics |
| Recreation after exercised spendability | No replay reset |
| Recreation across authorization migration | Ownership and authorization remain correctly interpreted |
| Historical old-version entity | Interpretation under applicable historical rules |
| Current absence after prior existence | Current and historical meaning not accidentally conflated |
| Reorganization removing creation | Deterministic authoritative state interpretation |
| Reorganization reversing removal | Deterministic authoritative state interpretation |
| Protocol-version lifecycle transition | No implementation-dependent reinterpretation |
| Large entity-creation population | Exposes persistent-growth requirements without selecting counters |
| High lifecycle churn | Exposes mutation/resource requirements without selecting storage mechanics |
| Missing historical local data | Does not silently redefine authoritative semantics |
| Malformed or non-canonical referent | Deterministic classification with no partial canonical lifecycle effect |

Scenarios are analytical requirements.

They do not establish that any particular lifecycle operation exists.

## 36. Evidence Required Before Candidate Comparison

Account/UTXO lifecycle comparison requires evidence based on equivalent external
semantic cases.

Candidate mappings must explicitly state:

- the semantic entity or referent being considered;
- current-existence semantics;
- absence semantics;
- zero or empty semantics where applicable;
- creation semantics where applicable;
- removal or unavailability semantics where applicable;
- recreation semantics where applicable;
- authorization assumptions;
- replay assumptions;
- native-value assumptions;
- identity assumptions;
- historical interpretation;
- version-transition assumptions;
- reorganization assumptions;
- persistent-current-state exposure;
- historical-retention assumptions; and
- unresolved commitment/resource assumptions.

Evidence must distinguish:

- common external requirements;
- candidate-specific lifecycle representation;
- candidate-specific identity assumptions;
- candidate-specific optimization;
- architecture assumptions;
- measured implementation behavior;
- current-state storage;
- historical storage; and
- authoritative protocol requirements.

Implementation measurements alone cannot establish lifecycle semantics.

Independent semantic review should check that neither mapping received hidden
advantages.

Conclusions must remain mapping-limited until broader evidence exists.

## 37. Formal-Verification Obligations

Future formal or executable evidence should be capable of expressing properties
such as:

- deterministic existence interpretation;
- deterministic absence interpretation;
- deterministic lifecycle transition results;
- authorization correctness across lifecycle transitions;
- replay exclusion across lifecycle transitions;
- applicable native-value conservation;
- no accidental recreation of exercised spendability;
- rejection without partial lifecycle effects;
- historical-version interpretation;
- migration continuity under stated assumptions;
- deterministic reapplication under authoritative history change;
- and equivalence between permitted implementation strategies.

A possible abstract analytical relation may eventually resemble:

`Lifecycle(version, history_context, prior_state, candidate) -> result`

This notation is not a normative protocol function.

It does not define:

- state types;
- transaction types;
- entity identifiers;
- history representation;
- Account;
- UTXO;
- a VM; or
- an API.

A proof model must not silently encode candidate-specific lifecycle assumptions
into the common specification.

## 38. Premature-Commitment Matrix

| Classification | Item |
|---|---|
| AUTHORITATIVE NOW | Constitution and currently adopted Formal Specification constraints |
| SAFE MODEL-INDEPENDENT REQUIREMENT | deterministic consensus-relevant existence and lifecycle interpretation |
| SAFE MODEL-INDEPENDENT REQUIREMENT | implementation-independent distinction or explicit equivalence for lifecycle states that affect consensus |
| SAFE MODEL-INDEPENDENT REQUIREMENT | lifecycle changes must not violate applicable ownership preservation |
| SAFE MODEL-INDEPENDENT REQUIREMENT | lifecycle changes must not accidentally recreate spendability contrary to replay rules |
| SAFE MODEL-INDEPENDENT REQUIREMENT | lifecycle representation creation/removal does not by itself imply native-value issuance/destruction |
| SAFE MODEL-INDEPENDENT REQUIREMENT | rejected lifecycle transition leaves no partial canonical lifecycle effect |
| ABSTRACT ONLY | entity |
| ABSTRACT ONLY | existence |
| ABSTRACT ONLY | absence |
| ABSTRACT ONLY | creation |
| ABSTRACT ONLY | deletion/removal |
| ABSTRACT ONLY | recreation |
| ABSTRACT ONLY | historical meaning |
| CANDIDATE | Minimal Account |
| CANDIDATE | Minimal UTXO |
| BLOCKED | Account existence representation |
| BLOCKED | Account zero/deletion/recreation rules |
| BLOCKED | Account nonce or sequence semantics |
| BLOCKED | UTXO entity representation |
| BLOCKED | UTXO creation/consumption representation |
| BLOCKED | output identity |
| BLOCKED | stable logical identity |
| BLOCKED | tombstone or generation mechanism |
| BLOCKED | state commitment and absence-proof construction |
| BLOCKED | pruning and history-retention policy |
| BLOCKED | conflict and canonical ordering |
| BLOCKED | deterministic resource units and numeric limits |
| DEFERRED | physical database deletion and compaction |
| DEFERRED | local indexes and caches |
| DEFERRED | archival product policy beyond future protocol requirements |
| DO NOT ADOPT | physical missing-key behavior as lifecycle consensus semantics |
| DO NOT ADOPT | deletion/recreation that silently resets replay protection |
| DO NOT ADOPT | representation removal as implicit native-DLTH destruction |
| DO NOT ADOPT | representation creation as implicit native-DLTH issuance |
| DO NOT ADOPT | permanent historical retention merely because an entity once existed |
| DO NOT ADOPT | hidden Account- or UTXO-specific assumptions in common comparison cases |

## 39. Fifth-Gate Decision Gates

The fifth state-model gate remains blocked until sufficiently concrete, reviewed
abstract answers exist for:

1. the semantic scope of an entity or lifecycle-relevant referent;
2. current-existence semantics;
3. current-absence semantics;
4. the distinction, where consensus-relevant, between malformed,
   non-canonical, unsupported, or otherwise invalid referents and validly formed
   referents denoting absent state;
5. zero, empty, or default-state interpretation where applicable;
6. creation semantics where applicable;
7. deletion, removal, or unavailability semantics where applicable;
8. recreation or referent-reuse semantics where applicable;
9. lifecycle interaction with replay exclusion;
10. lifecycle interaction with ownership and authorization;
11. lifecycle interaction with applicable native-value invariants;
12. historical-versus-current interpretation;
13. protocol-version and migration interpretation;
14. authoritative reapplication and history-change interpretation;
15. sufficient lifecycle semantics for later commitment and absence analysis;
16. sufficient lifecycle semantics for later conflict and ordering analysis; and
17. sufficient lifecycle semantics for later persistent-growth and resource
    analysis without selecting resource mechanisms.

These gates require decision-ready semantics and evidence.

They do not require a concrete state encoding, transaction encoding, database
layout, state commitment, or state-model selection.

The existence, completion, or review of this document alone satisfies none of
them automatically.

## 40. Formal Specification Boundary

Future authoritative specification will need to define lifecycle semantics where
they become consensus-relevant for the architecture ultimately selected.

Depending on the selected architecture, this may include:

- entity or state-referent semantics;
- canonical representation;
- current existence;
- current absence;
- zero or empty interpretation;
- creation;
- update;
- removal or unavailability;
- recreation or referent reuse;
- identity or uniqueness semantics;
- authorization interaction;
- replay interaction;
- native-value interaction;
- lifecycle atomicity;
- protocol-version interpretation;
- migration behavior;
- historical interpretation;
- reorganization and reapplication behavior;
- canonical commitment subjects;
- conflict interaction;
- and validity-affecting resource semantics.

Some items are architecture-contingent.

Their appearance here does not independently constitutionally require:

- deletion;
- recreation;
- tombstones;
- permanent historical state;
- a particular identity scheme;
- an Account model;
- a UTXO model;
- a commitment construction;
- a pruning system; or
- a resource mechanism.

This document defines none of those normative mechanisms.

It does not modify the Formal Specification.

## 41. Threat Model Boundary

Creation of this document alone does not justify a Threat Model change.

The current Threat Model already records generic categories including:

- nondeterminism and version drift;
- invalid-candidate work and resource exhaustion;
- state workload and persistent-state growth;
- resource-accounting integrity and failure atomicity;
- migration ambiguity; and
- cryptographic evolution or primitive failure.

Replay and duplicate-spend safety are separately tracked by replay and
canonical-identity decision-readiness analysis and may interact with entity
lifecycle semantics. Their appearance in this document does not assert that the
current Threat Model already contains a dedicated replay or duplicate-spend
threat class.

Lifecycle analysis should nevertheless test for candidate-specific risks such as:

- accidental recreation of previously exercised spendability;
- ownership loss during lifecycle migration;
- ambiguous current-versus-historical interpretation;
- unbounded persistent lifecycle metadata;
- identifier aliasing;
- stale-state acceptance; and
- implementation-specific absence handling.

Candidate-specific findings do not automatically become generic Threat Model
requirements.

A Threat Model update should be proposed only if analysis discovers a genuinely
new generic threat class or a selected architecture creates a concrete
model-specific threat requiring normative treatment.

## 42. Complete TBD Register

The following remain unresolved:

- final state model;
- entity representation;
- state-entity identity;
- stable logical identity;
- Account identity;
- Account existence semantics;
- Account zero semantics;
- Account deletion semantics;
- Account recreation semantics;
- Account balance representation;
- Account nonce or sequence semantics;
- UTXO entity representation;
- transaction input representation;
- transaction output representation;
- output identity;
- explicit-consumption semantics;
- spent-state representation;
- creation semantics;
- deletion/removal semantics;
- recreation semantics;
- tombstones;
- lifecycle generation or incarnation numbers;
- historical-state representation;
- historical-retention requirements;
- archival requirements;
- pruning;
- cleanup;
- garbage collection;
- transaction format;
- transaction identity;
- replay mechanism;
- ownership representation;
- authorization representation;
- credential format;
- native-DLTH representation;
- amount width;
- denomination;
- monetary precision;
- issuance;
- rewards;
- burn or destruction;
- fees;
- state commitment;
- absence-proof construction;
- proof format;
- snapshot format;
- synchronization semantics;
- conflict definition;
- canonical transaction ordering;
- block scheduling;
- parallel execution;
- deterministic resource units;
- resource counters;
- numeric resource limits;
- state rent;
- refunds;
- cleanup economics;
- fork choice;
- finality;
- reorganization depth;
- activation mechanics;
- migration mechanics;
- cryptographic algorithms and parameters;
- PQ authorization structure;
- multisignature behavior;
- authorization grouping; and
- governance thresholds and HIP / Super HIP mechanics.

No item is resolved by appearing in this register.

## 43. Current Project Impact

Creation of this document alone has the following effect:

```text
Account selected:
NO

UTXO selected:
NO

Hybrid selected:
NO

Generalized Object selected:
NO

State-model decision:
NOT MADE

Entity representation selected:
NO

Entity identifier selected:
NO

Stable logical identity selected:
NO

Account existence semantics selected:
NO

Account zero semantics selected:
NO

Account deletion semantics selected:
NO

Account recreation semantics selected:
NO

Account nonce or sequence selected:
NO

UTXO input/output structure selected:
NO

UTXO consumption mechanism selected:
NO

Output identity selected:
NO

Generic creation mechanism selected:
NO

Generic deletion mechanism selected:
NO

Generic recreation mechanism selected:
NO

Tombstone mechanism selected:
NO

Replay mechanism selected:
NO

Transaction format selected:
NO

Transaction identity selected:
NO

Native DLTH representation selected:
NO

Issuance mechanism selected:
NO

Burn mechanism selected:
NO

State commitment selected:
NO

Absence-proof construction selected:
NO

Snapshot mechanism selected:
NO

Pruning policy selected:
NO

History-retention policy selected:
NO

Archival requirement selected:
NO

Conflict rule selected:
NO

Canonical ordering selected:
NO

Parallel scheduling selected:
NO

Resource meter selected:
NO

State rent selected:
NO

Fee mechanism selected:
NO

Rollback mechanism selected:
NO

Database deletion mechanism selected:
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

## 44. Next Decision Boundary

Completion and independent review of this document may clarify the fifth
state-model decision gate:

> Entity existence, creation, deletion, recreation, and historical meaning.

The existence, completion, or review of this document does not by itself satisfy
the fifth state-model decision gate.

It also does not satisfy the other remaining state-model gates.

The next model-neutral analytical area is:

> PQ authorization-count and artifact assumptions sufficient for comparison.

That analysis must not use this document to silently select:

- Account;
- UTXO;
- one-authorizer-per-account semantics;
- one-authorizer-per-input semantics;
- a credential format;
- a cryptographic algorithm;
- multisig;
- aggregation;
- batching;
- key reuse;
- a transaction format; or
- a state-model ranking.

## 45. Conclusion

- This document is non-normative.
- The Formal Specification remains authoritative for protocol behavior.
- Entity representation remains unselected.
- Entity identity remains unselected.
- Stable logical identity remains unselected.
- Current existence must eventually have deterministic protocol semantics where
  consensus-relevant.
- Absence must not derive accidentally from physical database behavior.
- Zero or empty state is not automatically equivalent to absence.
- Creation semantics remain architecture-contingent and unselected.
- Creation of a representation does not itself mean native-DLTH issuance.
- Deletion semantics remain architecture-contingent and unselected.
- Logical deletion does not mean physical database deletion.
- Removal of a representation does not itself mean native-DLTH destruction.
- Recreation semantics remain architecture-contingent and unselected.
- Lifecycle transitions must not silently reset replay protection.
- No tombstone mechanism is selected.
- No generation or incarnation mechanism is selected.
- Historical existence and current existence are distinct analytical concepts.
- Historical interpretability does not itself require permanent current-state
  retention.
- Pruning is not automatically lifecycle deletion.
- No pruning or history-retention policy is selected.
- No state commitment or absence-proof construction is selected.
- No Account existence, zero, deletion, or recreation rule is selected.
- No Account nonce or sequence mechanism is selected.
- No UTXO input/output, output-identity, or consumption mechanism is selected.
- Replay mechanism remains unselected.
- Ownership and authorization architecture remain unselected.
- Native-DLTH representation and monetary mechanics remain unselected.
- Conflict and ordering semantics remain unresolved.
- Parallel scheduling remains unresolved.
- Resource meters, numeric limits, state rent, fees, refunds, and cleanup
  economics remain unresolved.
- Minimal Account and Minimal UTXO remain co-equal candidates.
- Candidate ranking remains blocked.
- State-model decision remains NOT MADE.
- The fifth state-model decision gate is not satisfied merely by this document
  existing, being completed, or being reviewed.
- This document defines no consensus rule.
