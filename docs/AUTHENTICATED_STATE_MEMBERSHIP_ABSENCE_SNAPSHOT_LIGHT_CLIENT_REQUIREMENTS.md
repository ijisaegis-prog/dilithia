# Dilithia Authenticated-State Membership, Absence, Snapshot, and Light-Client Decision Requirements

**Status:** NON-NORMATIVE — PRE-GENESIS DECISION-READINESS WORKING DRAFT\
**Review status:** UNREVIEWED WORKING DRAFT\
**State-model decision:** NOT MADE\
**Protocol adoption effect:** NONE

> This document is a non-normative decision-readiness artifact.
>
> It does not itself define Dilithia consensus behavior, select a state model,
> select a state commitment, select a proof system, select a snapshot mechanism,
> select a synchronization protocol, select a light-client protocol, or amend
> the Dilithia Technical Constitution or Formal Specification.
>
> The Dilithia Technical Constitution and the Dilithia Formal Specification
> remain authoritative according to their applicable status and adoption rules.
> Supporting requirement documents, comparison frameworks, workload models,
> threat models, experiments, implementations, tests, and review records do not
> become protocol authority merely because they are reviewed or merged.

Working-draft provenance:

- repository base commit:
  `ae1d6e4667a8ecd48f33b38621444533e9936b76`
- Gate-8 scope scan SHA-256:
  `FE52793F6E36198648C8E81A87B5A5D6DB2B8D9DDDE78AB4A2DDF615414A3978`
- Gate label:
  **Authenticated-state membership, absence, snapshot, and light-client requirements.**

The provenance above identifies the evidence base used for this working draft.
It is not a consensus identifier and does not make this draft authoritative.

---

## 1. Existing Authoritative Boundaries

This analysis inherits existing authoritative boundaries without expanding them.

The current Constitution requires, among other things:

- no administrator, master, emergency, foundation, or equivalent privileged
  cryptographic authority;
- permanent migratability of address formats and state representations without
  loss of ownership, and prohibition on upgrades rendering previously valid
  assets unreachable, unspendable, or un-migratable;
- canonical, versioned, and domain-separated consensus-critical
  serialization;
- Formal Specification primacy;
- support for independent compliant implementations;
- deterministic consensus-critical behavior; and
- rejection of protocol changes that impermissibly reduce required security.

The current Formal Specification provides partial authoritative or draft
boundaries relevant to this analysis, including:

- canonical and strict serialization;
- deterministic interpretation;
- purpose-specific domain separation for applicable hashes and signatures over
  canonical bytes;
- a post-quantum Genesis direction;
- protocol identity that is not intended to depend permanently on one
  cryptographic primitive; and
- pending Crypto Agility, transaction, state, and consensus sections.

The current Formal Specification does not yet define Dilithia's state model,
authenticated-state structure, state commitment, membership proof,
absence-proof construction, snapshot format, synchronization protocol, or
light-client protocol.

This document does not fill those authoritative gaps.

---

## 2. Purpose of the Eighth State-Model Gate

The eighth state-model gate exists because Account and UTXO cannot be compared
fairly on proof efficiency, synchronization, snapshot behavior, light-client
support, or authenticated-state cost while the required authenticated-state
capabilities themselves remain unstated.

The exact unresolved decision is:

> Which membership, absence, update, snapshot, synchronization, and light-client
> capabilities are required?

The exact reason selection remains blocked is:

> Proof and synchronization comparisons depend on the unchosen commitment
> requirements and construction.

This gate therefore defines enough model-neutral requirements to prevent a
future comparison from silently giving either candidate its preferred
commitment, proof, snapshot, or light-client assumptions.

It does not select the construction that will satisfy those requirements.

---

## 3. Gate 8 Does Not Select a State Model

Nothing in this document selects or ranks:

- Minimal Account;
- Minimal UTXO;
- Hybrid state;
- Generalized Object state;
- another state architecture; or
- a Genesis state-model decision.

Both Minimal Account and Minimal UTXO remain co-equal comparison candidates.

Candidate labels must not be treated as proof architectures.

In particular:

- Account does not imply a trie;
- UTXO does not imply a Merkle set;
- Account does not imply one authenticated record;
- UTXO does not imply one proof per input;
- Account does not imply simpler absence proofs;
- UTXO does not imply simpler membership proofs; and
- neither label implies a particular snapshot or light-client architecture.

---

## 4. Scope

This document addresses model-neutral decision-readiness requirements for:

- canonical logical-state commitment;
- current membership;
- current absence;
- state update authentication;
- current versus historical claim interpretation;
- proof and witness semantics;
- snapshot meaning;
- snapshot verification;
- synchronization from untrusted data;
- light-client verification;
- protocol-version binding;
- cryptographic-version binding;
- migration compatibility;
- reorganization and canonical-history interaction;
- independent implementation reproducibility;
- hostile proof and snapshot inputs;
- fair Account/UTXO comparison assumptions;
- authenticated-state implications of cryptographic evolution; and
- handoff boundaries to consensus, Crypto Agility, governance, the Threat Model,
  and the ninth state-model gate.

---

## 5. Explicit Non-Goals

This document does not select or define:

- a tree;
- a trie;
- a Merkle structure;
- a sparse Merkle structure;
- a Verkle structure;
- an accumulator;
- an authenticated dictionary implementation;
- a vector commitment;
- a polynomial commitment;
- a zero-knowledge proof system;
- a SNARK;
- a STARK;
- a hash algorithm;
- a state-root width;
- a state-root encoding;
- a membership-proof format;
- an absence-proof format;
- a witness format;
- a proof aggregation mechanism;
- a proof batching mechanism;
- a proof recursion mechanism;
- a snapshot file format;
- a snapshot chunk format;
- a snapshot cadence;
- a checkpoint format;
- a checkpoint authority;
- a weak-subjectivity rule;
- a bootstrap protocol;
- a synchronization transport;
- a peer-selection rule;
- a data-availability protocol;
- a light-client protocol;
- a block-header format;
- a consensus algorithm;
- a fork-choice rule;
- a finality mechanism;
- a reorganization depth;
- a pruning policy;
- a history-retention period;
- an archival-node requirement;
- a database backend;
- a resource meter;
- a resource unit;
- a numeric resource limit;
- a fee rule;
- a state-rent rule;
- a transaction format;
- an Account nonce;
- a UTXO consumption mechanism;
- a governance weighting mechanism;
- an emergency-upgrade activation mechanism; or
- a state-model ranking.

No item becomes selected merely because this document discusses the
requirement that a future design must satisfy.

---

## 6. Terminology

For this document, the following terms are model-neutral.

**Canonical logical state**

The protocol-level state facts whose meaning is defined by authoritative
protocol rules, independent of a particular database representation.

**State subject**

A logical subject about which the protocol may need to make an authenticated
claim. Its concrete identifier and representation remain unselected.

**Current membership**

A claim that a specified logical subject or fact is present in the applicable
current canonical logical state under the applicable semantics.

**Current absence**

A claim that a specified logical subject or fact is absent under the applicable
current canonical logical-state semantics.

**Logical availability**

A lifecycle or protocol-semantic concept indicating whether a subject or fact
is usable or available under applicable rules. It is not automatically the same
as existence or data availability.

**Data availability**

Whether the underlying data needed for validation, reconstruction, or use can
actually be obtained. A cryptographic commitment alone does not establish data
availability.

**Authenticated-state commitment**

An abstract cryptographic commitment to defined canonical logical-state
semantics under an applicable version and cryptographic context.

**Proof or witness**

An artifact used to support verification of an authenticated-state claim.
No proof-system family or encoding is implied by this term.

**State update**

A canonical logical-state change produced by applicable authoritative state
transition rules.

**Snapshot**

A representation intended to assist reconstruction or synchronization of
state at a defined authoritative state/history context.

**Synchronization**

A process by which a compliant node obtains enough verified information to
reach the authoritative state interpretation required by its role.

**Light client**

A client that intentionally validates less data or maintains less state than a
full validating node while still verifying explicitly defined claim classes
under explicitly defined trust assumptions.

**Canonical-history anchor**

An abstract authoritative reference that binds an authenticated-state claim to
the applicable canonical history or consensus context.

This term does not select a block hash, checkpoint, finality certificate,
header chain, consensus proof, or another concrete anchor.

---

## 7. Semantic-First Rule

Authenticated-state construction must follow state semantics, not create them.

A commitment, proof, witness, state root, snapshot, or light-client mechanism
must not invent:

- ownership semantics;
- authorization semantics;
- replay semantics;
- native-value semantics;
- entity existence semantics;
- absence semantics;
- deletion semantics;
- recreation semantics;
- conflict semantics;
- transaction ordering semantics;
- historical interpretation; or
- migration semantics

that do not already exist in the applicable authoritative logical-state rules.

The authenticated-state layer proves or commits to adopted semantics.

It is not a substitute for defining those semantics.

---

## 8. Canonical Logical State and Physical Storage Are Distinct

Canonical logical state must remain distinguishable from implementation
storage.

A future authenticated-state design must not make consensus meaning depend on:

- database engine;
- database page layout;
- key-value encoding chosen only by an implementation;
- cache state;
- index structure;
- memory layout;
- allocator behavior;
- file placement;
- compaction behavior;
- host filesystem;
- thread scheduling; or
- implementation-local optimization.

Different compliant implementations may use different physical storage while
representing the same canonical logical state.

If physical bytes become consensus-relevant, that relevance must come from an
explicit authoritative format, not from an accidental implementation layout.

---

## 9. State Subjects Must Have Unambiguous Logical Meaning

Before a state subject can be authenticated, future authoritative rules must
make its logical meaning unambiguous.

Where applicable, this includes distinctions among:

- subject identity;
- subject existence;
- subject content;
- subject ownership condition;
- authorization-relevant metadata;
- replay-relevant metadata;
- lifecycle version;
- protocol version;
- current value-bearing state;
- non-value system state; and
- other explicitly adopted state facts.

This document does not require all categories to exist.

It requires that a proof not become the place where their meaning is guessed.

---

## 10. Membership Semantics

A future membership claim must identify enough context that independent
compliant implementations agree on what is being claimed.

At minimum, the future design must determine, where applicable:

- what logical subject is claimed present;
- what logical fact is claimed present;
- whether the claim concerns current or historical state;
- which protocol version governs interpretation;
- which commitment or authenticated-state version applies;
- which canonical-history context applies; and
- whether additional domain or network binding is required.

A membership proof must not be accepted merely because its cryptographic
equation verifies if the proof is bound to the wrong logical claim or wrong
state context.

---

## 11. Absence Semantics

Absence must be defined semantically before it is proven cryptographically.

Future authenticated-state work must preserve existing distinctions where the
applicable logical rules require them, including possible distinctions among:

- currently absent;
- zero-valued;
- empty-valued;
- never created;
- previously existing but removed;
- previously existing but consumed;
- unavailable under a candidate-specific rule;
- present only in non-current history;
- malformed or non-canonical referent; and
- a referent that is validly formed but not present.

This document does not decide which of these categories will exist in the
final protocol.

It requires that a commitment or proof construction not silently collapse
categories that authoritative semantics keep distinct.

Neither Account nor UTXO may receive an easier definition of absence merely
because a preferred commitment structure makes that definition convenient.

---

## 12. Zero, Empty, Absence, and Invalid Referent Must Not Be Accidentally Conflated

Where the future logical-state specification distinguishes these concepts:

- zero;
- empty;
- absent;
- deleted;
- consumed;
- invalid;
- unknown version; and
- unavailable

must remain distinguishable through authenticated-state interpretation.

A proof system that cannot express a required distinction is not sufficient
merely because it is otherwise efficient.

A construction must adapt to the required semantics, not weaken the semantics
to fit the construction.

---

## 13. Logical Availability and Data Availability Are Separate

A proof that a logical subject exists does not by itself prove that every
underlying byte needed by every node role is retrievable.

Likewise, inability to retrieve data from one peer does not by itself prove
logical absence.

Future design must distinguish at least:

- authenticated logical existence;
- authenticated logical absence;
- logical availability under protocol rules;
- network retrievability;
- snapshot availability;
- historical-data availability; and
- implementation-local storage availability.

This document does not select a data-availability protocol.

It prohibits hidden substitution of a data-provider assumption for an
authenticated-state proof requirement.

---

## 14. Current and Historical Claims Are Distinct

A subject that existed in authoritative history does not necessarily exist in
current canonical logical state.

Current absence does not necessarily mean that the subject never existed.

Future authenticated-state design may need to support distinct claim classes
for:

- current membership;
- current absence;
- current content;
- historical existence;
- historical non-current existence;
- historical consumption;
- historical removal;
- replay-relevant historical fact;
- migration-relevant historical fact; or
- another explicitly adopted historical property.

This document does not decide which historical claims must be provable.

That decision must be made before a proof architecture is ranked.

---

## 15. Canonical Identity and Authenticated-State Commitment Are Distinct

Transaction identity, output identity, replay identity, semantic identity, and
authenticated-state commitment are separate design surfaces.

A state commitment does not become a transaction identifier merely because it
is a hash-like value.

A transaction identifier does not prove state membership merely because it is
cryptographically derived.

Replay exclusion must not be inferred from membership.

Membership must not be inferred from replay identity.

Future authoritative rules must define any relationship explicitly.

---

## 16. Abstract Commitment Requirements

Without selecting a construction, a future authenticated-state commitment must
be capable of satisfying all adopted requirements that apply to it.

The minimum abstract requirements for comparison include:

- deterministic commitment to canonical logical state;
- unambiguous commitment coverage;
- canonical interpretation of committed logical subjects;
- canonical interpretation of existence and absence;
- reproducibility across independent compliant implementations;
- explicit applicable version context;
- explicit cryptographic context where required;
- safe protocol evolution;
- migration compatibility;
- resistance to ambiguous interpretation;
- membership verification where required;
- absence verification where required;
- snapshot verification capability where required; and
- synchronization verification capability where required.

These are capability requirements, not a construction selection.

---

## 17. Commitment Coverage Must Be Explicit

A future commitment design must define exactly what logical facts it commits.

It must not leave consensus-relevant coverage to implementation convention.

Questions that must eventually have explicit answers include:

- Does the commitment cover all current consensus state?
- Are some state namespaces committed separately?
- Are protocol-version facts committed?
- Are replay-relevant facts committed?
- Are ownership-condition versions committed?
- Are lifecycle facts committed?
- Are monetary reconciliation facts committed?
- Are validity-affecting resource-state facts committed, if such state exists?
- Are historical facts committed separately or not committed?
- What is intentionally outside the commitment?

This document supplies no answer to those questions.

It requires the answers before commitment-dependent ranking.

---

## 18. Independent Implementations Must Reproduce the Same Commitment Semantics

For the same canonical logical state, applicable protocol version,
authenticated-state version, and cryptographic profile, compliant
implementations must not disagree on the authoritative commitment result once
a construction is selected.

This requirement does not require identical:

- database structure;
- cache behavior;
- internal traversal;
- parallel schedule;
- temporary memory use;
- proof-generation algorithm; or
- physical storage bytes

unless a future authoritative rule explicitly makes one of those properties
consensus-relevant.

Authenticated-state determinism must follow the Constitution's broader
protocol-determinism requirement.

---

## 19. Version and Context Binding

Future authenticated-state artifacts must not be interpreted outside the
version and context in which they are valid.

The future design must determine, where applicable, how an artifact is bound
to:

- protocol version;
- authenticated-state construction version;
- cryptographic algorithm or profile version;
- network or chain context;
- claim purpose;
- canonical-history context; and
- any migration or coexistence context required for correct interpretation.

The exact fields and encoding remain unselected.

Existing domain-separation requirements for applicable hashes and signatures
remain applicable according to the Formal Specification.

This document does not invent a new domain-tag registry.

---

## 20. Update Semantics

Authenticated-state update behavior must correspond to authoritative canonical
logical-state transitions.

The future design must eventually define:

- which logical state changes alter authenticated commitments;
- how creation affects authenticated state;
- how removal affects authenticated state;
- how replacement affects authenticated state;
- how protocol-version changes affect authenticated state;
- how cryptographic-version changes affect authenticated state;
- how reorganization and canonical reapplication affect authenticated state;
- whether multiple logical namespaces update independently; and
- what atomicity relationship exists between accepted canonical state and its
  authoritative commitment.

No incremental-update algorithm is selected here.

No proof-update mechanism is selected here.

No path-update representation is selected here.

---

## 21. Rejected Candidates Must Not Create Partial Authenticated-State Effects

A transaction, block, proof, snapshot, synchronization input, or other
candidate that is rejected must not leave a partial canonical state transition
merely because authenticated-state processing began.

If a future design maintains consensus-relevant authenticated-state metadata,
failure atomicity must include that metadata.

Physical cleanup after failure remains an implementation concern unless a
future authoritative rule makes it consensus-relevant.

This document does not select rollback, journal, overlay, copy-on-write, or
database-transaction mechanics.

---

## 22. Proof and Witness Semantics

A cryptographically valid proof is insufficient unless it proves the intended
claim.

A future proof or witness system must make it possible for a verifier to
determine, where applicable:

- claim type;
- logical subject;
- claimed value or state fact;
- membership or absence meaning;
- current or historical scope;
- protocol version;
- authenticated-state version;
- cryptographic profile;
- canonical-history context; and
- any additional purpose binding required by the authoritative rules.

The exact proof encoding remains unselected.

The exact proof primitive remains unselected.

Proof aggregation and batching remain unselected.

---

## 23. Malformed, Unsupported, and Invalid Authenticated-State Evidence

Future authoritative rules must classify hostile or unsupported evidence
deterministically.

Relevant categories may include:

- malformed encoding;
- non-canonical encoding;
- unsupported authenticated-state version;
- unknown cryptographic version;
- invalid proof;
- proof for the wrong subject;
- proof for the wrong claim type;
- proof for the wrong network or chain context;
- proof for the wrong history anchor;
- stale proof;
- proof for reverted history;
- inconsistent proof components;
- oversized evidence;
- truncated evidence; and
- duplicate or contradictory evidence.

This document does not define the exact error taxonomy.

It requires that equivalent compliant implementations not silently disagree on
consensus-relevant interpretation.

---

## 24. Snapshot Is Not Protocol Authority by Itself

A snapshot is data.

A snapshot file, website, API response, release artifact, peer response, or
founder-provided archive does not become authoritative merely because it is
called an official snapshot.

Future snapshot design must make untrusted snapshot data independently
verifiable against the authoritative state/history commitments required by the
future protocol design.

A snapshot provider must not silently become:

- a state authority;
- a balance authority;
- an ownership authority;
- a protocol-version authority;
- a checkpoint authority;
- a fork-choice authority; or
- a privileged recovery authority.

---

## 25. Snapshot Completeness Requirements Must Be Explicit

A future snapshot design must define what a synchronizing node needs in order
to reconstruct the authoritative state required by its role.

The required content may depend on future decisions concerning:

- ownership;
- authorization;
- replay;
- native DLTH;
- entity lifecycle;
- protocol versions;
- cryptographic versions;
- validity-affecting resource state;
- retained history;
- reorganization support; and
- consensus metadata.

This document does not state that all such facts must be stored directly in a
snapshot.

A fact may instead be:

- included;
- reconstructible;
- separately provable;
- derivable from authoritative data; or
- not required for the node role

under future reviewed rules.

The completeness requirement must be explicit before snapshot size or
synchronization cost is compared.

---

## 26. Snapshot Verification Requirements

A future synchronizing node must be able to detect, as applicable:

- corrupted snapshot data;
- omitted required data;
- duplicated data;
- reordered data where order is meaningful;
- mixed protocol-version data;
- mixed cryptographic-version data;
- state from the wrong canonical-history context;
- stale state presented as current;
- partial migration state presented as complete;
- monetary inconsistency;
- ownership inconsistency;
- replay-relevant inconsistency; and
- commitment mismatch.

The exact mechanism for this verification remains unselected.

---

## 27. Snapshot Trust Model Must Be Explicit

Every future snapshot mechanism must state its trust assumptions.

Questions that must eventually be answered include:

- What does the snapshot provider need to be trusted for, if anything?
- Can the snapshot provider create a false state that still passes verification?
- Can the provider omit data without detection?
- Can the provider choose the canonical chain for the receiver?
- How does the receiver obtain the state/history anchor used for verification?
- Can two providers equivocate?
- Does verification require full historical replay?
- Does verification require an external checkpoint?
- Does verification require another node role?
- What happens when providers disagree?

A design with an unavoidable external trust assumption must state that
assumption explicitly.

It must not market the assumption as trustless verification.

---

## 28. Synchronization Success Must Have a Protocol-Level Meaning

Synchronization is successful only when the node has enough verified
information to perform the authoritative duties required by its node role.

A future design must not define success merely as:

- downloaded all files;
- matched a server checksum;
- received a state root from a peer;
- matched an explorer;
- matched an official RPC response; or
- loaded a database image.

For a full validating node, synchronization must eventually result in an
authoritative state interpretation sufficient for full validation under the
applicable protocol rules.

Other node roles may have different requirements, but those requirements and
their trust assumptions must be explicit.

---

## 29. Full Historical Replay Is Not Assumed

This document does not require every node to replay all history forever.

It also does not assume that replay can be discarded.

The future design must determine what current or retained authenticated facts
are sufficient for:

- replay safety;
- ownership interpretation;
- native-value reconciliation;
- lifecycle interpretation;
- cryptographic migration;
- protocol-version interpretation;
- canonical reapplication;
- snapshot verification; and
- node bootstrap.

If historical data is no longer required for a particular validity property,
the authoritative current fact replacing that historical dependency must be
defined.

---

## 30. Historical Interpretability Does Not Require Permanent Current-State Retention

The ability to interpret historical state does not require every old entity,
proof, record, deletion marker, or prior value to remain in current logical
state forever.

Future architecture may distinguish:

- current consensus state;
- historical blocks;
- historical transactions;
- historical commitments;
- historical proofs;
- archival data;
- snapshot data;
- reorganization-support data;
- temporary validation data; and
- implementation-local indexes.

This document selects no retention period and no pruning policy.

---

## 31. Pruning Is Not Logical Deletion

Physical or logical retention policy must not silently redefine lifecycle
semantics.

A locally pruned historical record does not by itself mean:

- the entity never existed;
- the entity is currently absent;
- the entity was deleted;
- the entity was consumed;
- the ownership never existed; or
- replay protection may reset.

Any future relationship between pruning and authenticated-state evidence must
be explicit.

---

## 32. Canonical History and Reorganization

Authenticated-state interpretation must remain deterministic when the
authoritative canonical history changes under whatever future consensus rules
are adopted.

The future design must eventually address:

- state produced by the old canonical history;
- state produced by the new canonical history;
- commitment interpretation across authoritative reversion;
- proof validity after reversion;
- snapshot validity after reversion;
- canonical reapplication;
- protocol-version interpretation;
- cryptographic-version interpretation; and
- historical claims referring to non-current history.

This document does not select:

- fork choice;
- finality;
- reorganization depth;
- undo logs;
- checkpointing;
- journals; or
- state reconstruction mechanics.

---

## 33. Light-Client Purpose Must Be Defined Before Light-Client Construction

A light client must be evaluated by the claims it must verify, not by the name
of a particular proof system.

Before selecting a light-client construction, the protocol must identify the
claim classes the light client is expected to verify.

Possible claim classes include:

- current state membership;
- current state absence;
- ownership-relevant state;
- authorization-version state;
- replay-relevant state;
- native-value state;
- protocol-version state;
- authenticated-state version;
- historical claim where required; and
- canonical-history relationship.

This list is a requirements inventory, not a declaration that every light
client must verify every item.

---

## 34. A Correct State Proof Does Not Alone Prove Canonical Chain Membership

A proof may correctly establish a fact relative to a state commitment while the
state commitment itself belongs to:

- stale history;
- reverted history;
- an alternative chain;
- an adversarially fabricated context; or
- a context not authoritative for Dilithia.

Therefore, any light client that claims to verify current canonical Dilithia
state must have an explicitly defined verification relationship between:

- the state claim;
- the authenticated-state commitment; and
- the applicable authoritative canonical-history or consensus context.

This document does not select how that relationship is represented.

In particular, it does not select:

- a header chain;
- a consensus proof;
- a finality certificate;
- a checkpoint;
- weak subjectivity;
- a multi-commitment header; or
- another canonical-history mechanism.

---

## 35. Head, Anchor, and Bootstrap Distribution Must Not Hide a Trusted Third Party

A future light client or snapshot client must not silently obtain protocol truth
from:

- the project founder;
- a foundation;
- one official server;
- one RPC provider;
- one explorer;
- one repository;
- one release channel;
- one security-review team; or
- another privileged distributor

unless the design explicitly admits that party as a trust assumption.

A claim of trust minimization requires the client to verify the authoritative
relationship required by the selected consensus and authenticated-state
design.

The exact bootstrap trust model remains unselected.

---

## 36. Privileged Authenticated-State Authority Is Not Introduced by This Gate

This document does not authorize:

- an admin state signer;
- an emergency state signer;
- a foundation checkpoint signer;
- a founder snapshot signer;
- a privileged light-client oracle;
- a privileged recovery signer; or
- a hidden state-override authority.

A future architecture must be reviewed for whether an apparently operational
role becomes a de facto protocol authority.

"No privileged key" must not be bypassed by creating an equivalent privileged
state, snapshot, checkpoint, or bootstrap role.

---

## 37. Crypto Agility Applies to Authenticated-State Dependencies

A future authenticated-state architecture must not assume that one
cryptographic primitive remains secure forever.

Where a state commitment, proof, witness, snapshot authenticator, or
history-binding mechanism depends on cryptographic primitives, the future
design must address:

- algorithm identification;
- protocol-version interpretation;
- cryptographic-version interpretation;
- coexistence where required;
- deprecation;
- migration;
- historical interpretation;
- unsupported-version behavior; and
- failure behavior when an assumption is no longer accepted.

This document does not select the primitive, registry, migration schedule, or
coexistence mechanism.

---

## 38. Post-Quantum Authorization Does Not Automatically Make State Commitments Post-Quantum Safe

Authorization signatures and state commitments are separate cryptographic
surfaces.

A post-quantum signature choice does not by itself establish the security of:

- a hash-based state commitment;
- an accumulator;
- a proof system;
- a vector commitment;
- a snapshot authenticator;
- a history-binding mechanism; or
- a light-client proof.

Every selected cryptographic dependency must be analyzed under its own
security assumptions and required security margin.

This document selects none of those cryptographic dependencies.

---

## 39. Catastrophic Cryptographic Break and Historical Verification Boundary

A future catastrophic break may invalidate assumptions used to authenticate
past state or past history.

This document does not assume that every past cryptographic artifact remains
meaningful forever after arbitrary cryptanalytic failure.

The future architecture must therefore explicitly analyze:

- which historical verification assumptions depend on each primitive;
- what happens when a primitive is deprecated before failure;
- what happens when a primitive is unexpectedly broken;
- whether current state can remain authoritative without re-trusting broken
  evidence;
- whether bootstrap from Genesis remains possible under the selected future
  consensus and cryptographic architecture;
- whether another trust assumption becomes necessary; and
- how that assumption, if any, is exposed rather than hidden.

This document does not select checkpoints.

This document does not select weak subjectivity.

This document does not claim that arbitrary catastrophic cryptographic failure
can always be repaired without loss or additional trust assumptions.

---

## 40. Dormant Ownership and Authenticated-State Handoff

Authenticated-state design must preserve the distinction between:

- proving that a dormant asset or entity exists;
- proving the authorization condition governing it;
- proving the cryptographic version governing that authorization; and
- deciding whether that authorization method remains acceptable.

The authenticated-state layer must not invent a privileged recovery path when
an authorization primitive is broken.

Questions concerning proactive alternate authorization anchors, public-key
exposure minimization, cryptographic sunset policy, or dormant-asset migration
remain authorization and Crypto Agility design questions.

They are not selected by this Gate 8 document.

---

## 41. Native DLTH, Ownership, Replay, and Lifecycle Facts Must Remain Consistent

Authenticated-state evidence must not permit a state interpretation that
contradicts already-reviewed model-neutral obligations concerning:

- legitimate ownership;
- authorization;
- replay exclusion;
- double-spend exclusion;
- native-value conservation;
- failure atomicity;
- entity existence;
- entity absence;
- creation;
- removal;
- recreation;
- historical interpretation; or
- canonical reapplication.

A commitment cannot make an invalid monetary or ownership state valid.

A cryptographically valid proof of incorrectly specified semantics is still
insufficient.

---

## 42. Account and UTXO Must Receive the Same External Authenticated-State Requirements

Fair comparison requires the same external semantic requirements to be applied
to both candidates before candidate mapping.

Common requirements may include, where applicable:

- proof of required current membership;
- proof of required current absence;
- proof of ownership-relevant facts;
- proof of replay-relevant facts;
- snapshot verification;
- synchronization correctness;
- protocol-version interpretation;
- cryptographic-version interpretation;
- canonical-history binding; and
- hostile evidence handling.

Candidate mappings may then produce different internal results.

Those differences are evidence.

They are not permission to redefine the common requirement.

---

## 43. Candidate Outputs Remain Candidate Outputs

Once a future commitment construction and evidence profile are frozen fairly,
candidate-specific outputs may include:

- number of committed logical subjects;
- number of internal authenticated records;
- commitment-update count;
- proof count;
- proof artifact bytes;
- witness bytes;
- verification-operation count;
- update-operation count;
- logical access required for proof generation;
- logical access required for proof verification;
- snapshot bytes;
- synchronization bytes;
- restoration work;
- retained auxiliary authenticated state;
- conflict amplification caused by authenticated representation;
- malformed-proof failure stage; and
- implementation performance evidence.

No current value is assigned to these outputs.

No current output ranks Account or UTXO.

---

## 44. Commitment-Dependent Evidence Must Be Explicitly Profiled

Commitment-dependent comparison must use an explicitly frozen profile.

At minimum, a future evidence profile must identify material assumptions such
as:

- commitment construction identifier;
- construction version;
- cryptographic profile;
- proof capability profile;
- absence-proof capability;
- update capability;
- snapshot capability;
- synchronization capability;
- light-client claim profile;
- protocol-version context;
- history or reorganization profile where material;
- candidate population;
- hostile-input profile; and
- measurement methodology.

A commitment assumption must not be introduced inside only one candidate's
mapping.

If a material profile changes, the evidence identity must change.

---

## 45. Hostile Authenticated-State Evidence Cases

Future evidence should include adversarial cases sufficient to test the adopted
requirements.

Relevant case classes include:

- malformed proof;
- non-canonical proof representation;
- truncated proof;
- oversized proof;
- unsupported proof version;
- unknown cryptographic version;
- valid proof for wrong state subject;
- valid proof for wrong claim type;
- valid proof for wrong network or chain;
- valid proof for wrong protocol version;
- valid proof for wrong authenticated-state version;
- valid proof for stale state;
- valid proof for reverted history;
- false membership claim;
- false absence claim;
- zero-versus-absence confusion;
- current-versus-historical confusion;
- duplicate proof material;
- contradictory proof material;
- many invalid proofs;
- proof or witness expansion;
- corrupted snapshot;
- omitted snapshot component;
- duplicated snapshot component;
- mixed-version snapshot;
- incomplete migration snapshot;
- snapshot from non-canonical history;
- light-client head substitution;
- eclipse-assisted alternative-history presentation;
- commitment mismatch between independent implementations;
- state update followed by failure;
- reorganization after snapshot;
- cryptographic-version transition;
- cryptographic deprecation; and
- catastrophic cryptographic-assumption failure analysis.

This list does not define a final test suite.

It defines the hostile surfaces that future selection evidence must not ignore.

---

## 46. Resource Exhaustion Is a Handoff, Not a Reason to Weaken Verification

Authenticated-state and proof mechanisms may create large validation,
synchronization, memory, bandwidth, or persistent-state costs.

The Threat Model already treats hostile proof work, witness expansion,
late-invalid candidates, state workload, synchronization burden, and persistent
growth as resource-security concerns.

Gate 8 must expose the operations and artifacts that Gate 9 will need to bound.

Gate 8 does not select:

- resource counters;
- CPU-time limits;
- wall-clock limits;
- proof weights;
- gas;
- fees;
- memory limits;
- bandwidth prices;
- attempt accounting;
- invalid-proof charges; or
- numeric limits.

Verification must not be weakened merely because valid verification is
expensive.

The ninth gate is responsible for deterministic resource-bound requirements.

---

## 47. Cheap Early Rejection Is Compatible With Strong Verification

A future implementation may reject obviously malformed or structurally invalid
evidence before expensive cryptographic work where the authoritative rules
permit that behavior.

However:

- early rejection must not change canonical validity;
- different implementations must not disagree on validity because their
  optimization order differs;
- an invalid unauthenticated submission must not gain authority to mutate
  another user's canonical state merely because validation consumed resources;
- local network defenses must remain distinct from consensus validity; and
- optimization must not reduce required verification completeness.

This document does not select a fee or upfront-reservation mechanism.

---

## 48. Emergency and Governance Boundary

This document does not create an Emergency Security Upgrade Track.

It does not select an emergency activation mechanism.

It does not select governance weighting or thresholds.

However, future emergency or governance mechanisms must not use authenticated
state, snapshots, checkpoints, light-client roots, or proof-version overrides
as a hidden way to create a privileged protocol authority.

An emergency response must not silently redefine:

- what state exists;
- who owns state;
- which native value exists;
- what counts as absence;
- which old history is authoritative; or
- which state provider is trusted

without the authoritative procedure required for those semantics.

Detailed emergency liveness, false-emergency resistance, severity
classification, activation, and governance design belong to their separate
future review.

---

## 49. Security Review Handoff: Hidden Authority

Future design review must ask whether any of the following becomes a de facto
privileged authority even without an explicit admin key:

- snapshot publisher;
- checkpoint distributor;
- light-client head provider;
- canonical-state API;
- proof-generation service;
- security-review group;
- release channel;
- repository;
- specification mirror;
- consensus bootstrap service; or
- cryptographic migration coordinator.

A convenience service is not automatically a protocol authority.

If the protocol requires trusting such a service for correctness, the trust
assumption must be explicit and reviewed.

---

## 50. Threat Model Boundary

The current Threat Model already includes relevant generic concerns such as:

- untrusted peer state claims;
- resource exhaustion;
- invalid-candidate late failure;
- proof and witness expansion;
- state workload;
- synchronization burden;
- retained-history growth;
- migration ambiguity;
- cryptographic evolution;
- supply-chain compromise;
- nondeterminism; and
- version drift.

This document does not claim that the Threat Model already contains a dedicated
authenticated-state, snapshot-substitution, light-client-head-substitution, or
catastrophic-history-cryptography threat class.

After Gate 8 requirements are independently reviewed, the Threat Model should
be reviewed to determine whether new explicit threat classes or revisit
triggers are justified.

Threat Model update selected by this document:

**NONE**

---

## 51. Formal Specification Handoff

If authenticated-state mechanisms are later adopted, the authoritative Formal
Specification must eventually define every consensus-relevant property required
for independent implementation.

Depending on the selected architecture, this may include:

- canonical logical-state subjects;
- commitment coverage;
- commitment encoding;
- commitment versioning;
- cryptographic profile binding;
- membership validity;
- absence validity;
- proof validity;
- witness validity;
- update semantics;
- failure semantics;
- protocol-version interpretation;
- historical interpretation;
- migration behavior;
- reorganization behavior;
- snapshot validity rules;
- synchronization validity rules;
- light-client security assumptions;
- canonical-history binding;
- unsupported-version behavior; and
- validity-affecting resource semantics.

This document does not create those consensus rules.

Formal Specification consensus rule created by this document:

**NO**

---

## 52. Conformance and Machine-Checkable Evidence Handoff

Because authenticated-state behavior may be implemented independently, future
selection evidence should include strong cross-implementation conformance
evidence.

Useful future evidence may include:

- conformance vectors whose normative force, if any, exists only to the extent
  explicitly established by the applicable Formal Specification through the
  valid authoritative protocol process;
- canonical valid examples;
- canonical invalid examples;
- malformed-input corpora;
- independent implementation differential testing;
- differential fuzzing;
- property-based testing;
- formal invariants;
- mechanically checkable models where suitable; and
- executable reference semantics where suitable.

This document does not select Lean, Coq, K Framework, or another formal tool.

Machine-checkable models, executable reference semantics, and conformance
vectors are subordinate derived artifacts. They have no independent protocol
authority.

The applicable Formal Specification remains controlling for protocol behavior.
If a machine-checkable model, executable reference semantics, or conformance
vector conflicts with the applicable Formal Specification, that artifact is
incorrect and must not be treated as a co-equal source of protocol authority.

If such an artifact is adopted, its derivation, scope, versioning, and
relationship to the applicable authoritative rules must be defined explicitly.

---

## 53. Ninth-Gate Resource Handoff

The next state-model decision gate remains:

> **Logical access, mutation, persistent-growth, and invalid-candidate resource
> requirements.**

Gate 8 must provide enough authenticated-state structure to let Gate 9 ask:

- how many logical state accesses may be required;
- how many authenticated updates may be required;
- how large required artifacts may become;
- how malformed proof work is bounded;
- how snapshot verification work is bounded;
- how synchronization work is bounded;
- how retained authenticated state grows;
- how cryptographic-version coexistence affects resource exposure; and
- which bounds affect canonical validity.

Gate 8 does not answer those numeric or accounting questions.

---

## 54. Premature-Commitment Matrix

| Classification | Item | Gate-8 disposition |
|---|---|---|
| SAFE TO RECORD NOW | Commitment follows canonical logical semantics | Requirement |
| SAFE TO RECORD NOW | Membership meaning must be explicit | Requirement |
| SAFE TO RECORD NOW | Absence meaning must be explicit | Requirement |
| SAFE TO RECORD NOW | Zero/empty/absence must not be accidentally conflated | Requirement |
| SAFE TO RECORD NOW | Current and historical claims are distinct | Requirement |
| SAFE TO RECORD NOW | Independent implementations must agree on consensus-visible authenticated-state meaning | Requirement |
| SAFE TO RECORD NOW | Snapshot data is not authoritative merely because of its source | Requirement |
| SAFE TO RECORD NOW | Light-client canonical-state claims require canonical-history binding | Requirement |
| SAFE TO RECORD NOW | Hidden trusted state/bootstrap authority must be exposed | Requirement |
| SAFE TO RECORD NOW | Crypto Agility applies to authenticated-state dependencies | Requirement |
| SAFE TO RECORD NOW | Catastrophic cryptographic failure may invalidate historical assumptions | Required analysis |
| SAFE TO RECORD NOW | Commitment-dependent candidate evidence must use frozen common profiles | Requirement |
| KEEP AS CANDIDATE | Merkle-family construction | Not selected |
| KEEP AS CANDIDATE | Verkle-family construction | Not selected |
| KEEP AS CANDIDATE | Accumulator construction | Not selected |
| KEEP AS CANDIDATE | Vector/polynomial commitment | Not selected |
| KEEP AS CANDIDATE | Multiple concurrent commitments | Not selected |
| KEEP AS CANDIDATE | Public-key exposure minimization | Authorization/Crypto handoff |
| KEEP AS CANDIDATE | Alternate pre-registered authorization anchor | Authorization/Crypto handoff |
| BLOCKED | Exact state commitment | Requires reviewed semantics and evidence |
| BLOCKED | Exact proof system | Commitment/crypto selection unresolved |
| BLOCKED | Exact snapshot format | State/commitment/history decisions unresolved |
| BLOCKED | Exact synchronization protocol | Consensus/network/state decisions unresolved |
| BLOCKED | Exact light-client protocol | Consensus/commitment/bootstrap decisions unresolved |
| BLOCKED | Checkpoint design | Consensus/history architecture unresolved |
| BLOCKED | Weak subjectivity | Consensus architecture unresolved |
| BLOCKED | Hash algorithm for authenticated state | Crypto design unresolved |
| BLOCKED | Numeric proof or snapshot resource limits | Gate 9 |
| BLOCKED | Account/UTXO ranking | Evidence not yet sufficient |

---

## 55. Decision-Readiness Checklist

Gate 8 cannot be considered decision-ready until reviewed answers exist for at
least the following questions.

1. What canonical logical facts require authenticated commitment?
2. Which current membership claims must be supportable?
3. Which current absence claims must be supportable?
4. Which distinctions among zero, empty, absent, removed, consumed, invalid, and
   historical non-current state are authoritative?
5. Which historical claims, if any, must be provable?
6. What constitutes the canonical logical-state subject of each required claim?
7. What exact state is inside and outside commitment coverage?
8. What protocol/version context must bind authenticated-state artifacts?
9. What cryptographic-version context must bind authenticated-state artifacts?
10. What network, chain, or purpose binding is required?
11. How do accepted canonical state updates affect authenticated commitments?
12. How does failure avoid partial authenticated-state effect?
13. How are malformed, unsupported, unknown-version, and stale proofs classified?
14. What must a snapshot contain, reconstruct, or separately prove?
15. What does successful snapshot restoration mean?
16. How is an untrusted snapshot verified?
17. What trust assumptions, if any, remain in snapshot bootstrap?
18. What current facts replace historical dependencies if full replay is not
    required?
19. What history or retention assumptions remain necessary?
20. How does pruning remain distinct from lifecycle deletion?
21. How does reorganization affect proof and snapshot validity?
22. What claim classes must a light client verify?
23. How does a light client bind a correct state proof to authoritative canonical
    history?
24. How does a light client obtain its head or history anchor without a hidden
    privileged state provider?
25. What explicit external trust assumptions remain, if any?
26. How does Crypto Agility affect commitments, proofs, snapshots, and history
    interpretation?
27. What happens when a commitment or history-related primitive is deprecated?
28. What assumptions remain after catastrophic cryptographic break?
29. Can historical verification remain meaningful under those assumptions?
30. Which consequences belong to consensus design rather than Gate 8?
31. Which consequences belong to authorization/Crypto Agility rather than
    Gate 8?
32. Which authenticated-state operations and artifacts must Gate 9 bound?
33. How are Account and UTXO given identical external authenticated-state
    requirements?
34. Which candidate differences remain legitimate outputs?
35. What hostile proof, snapshot, synchronization, and bootstrap cases are
    required in future evidence?
36. What conformance evidence is required across independent implementations?
37. Does any operational service become a hidden protocol authority?
38. Does any proposed solution introduce a founder, foundation, reviewer, API,
    snapshot, checkpoint, or release-channel trust root?
39. Does any requirement silently select a state model?
40. Does any requirement silently select a commitment or consensus mechanism?

A reviewed answer may remain deferred where the question properly belongs to a
later gate, provided that the deferral is explicit and does not introduce a
candidate-specific default.

---

## 56. Current Project Impact

Creation of this working draft has the following effect.

State-model decision:

**NOT MADE**

Minimal Account selected:

**NO**

Minimal UTXO selected:

**NO**

Hybrid selected:

**NO**

Generalized Object selected:

**NO**

State-model ranking justified:

**NO**

State commitment selected:

**NO**

Authenticated dictionary selected:

**NO**

Merkle construction selected:

**NO**

Verkle construction selected:

**NO**

Accumulator selected:

**NO**

Hash algorithm for authenticated state selected:

**NO**

Membership-proof format selected:

**NO**

Absence-proof construction selected:

**NO**

Witness format selected:

**NO**

Proof system selected:

**NO**

Proof aggregation selected:

**NO**

Snapshot mechanism selected:

**NO**

Snapshot trust model selected:

**NO**

Synchronization protocol selected:

**NO**

Bootstrap protocol selected:

**NO**

Light-client protocol selected:

**NO**

Checkpoint mechanism selected:

**NO**

Weak subjectivity selected:

**NO**

Canonical-history anchor representation selected:

**NO**

Consensus algorithm selected:

**NO**

Fork-choice mechanism selected:

**NO**

Finality mechanism selected:

**NO**

Reorganization depth selected:

**NO**

Pruning policy selected:

**NO**

History-retention policy selected:

**NO**

Archival requirement selected:

**NO**

Database backend selected:

**NO**

Resource meter selected:

**NO**

Numeric resource limit selected:

**NO**

Fee mechanism selected:

**NO**

Emergency activation mechanism selected:

**NO**

Governance weighting selected:

**NO**

Formal Specification consensus rule created by this document:

**NO**

Constitution amendment created by this document:

**NO**

Threat Model update selected:

**NONE**

Consensus implementation change:

**NONE**

---

## 57. Gate Satisfaction Is Not Automatic

The existence of this document does not satisfy the eighth state-model gate.

Completion of this draft does not satisfy the eighth state-model gate.

Independent review of this draft does not automatically satisfy the eighth
state-model gate.

Merge of this document does not itself constitute protocol adoption.

Gate satisfaction requires enough reviewed abstract semantics to prevent
state-model comparison from relying on unstated authenticated-state,
commitment, snapshot, synchronization, or light-client assumptions.

A later decision must explicitly record whether that standard has been met.

---

## 58. Relationship to Future Construction Selection

After the required abstract semantics are sufficiently reviewed, the project
may evaluate one or more authenticated-state construction candidates.

Such evaluation must:

- preserve the frozen common semantic requirements;
- identify construction-specific assumptions;
- identify cryptographic assumptions;
- identify migration assumptions;
- identify historical verification assumptions;
- expose bootstrap trust assumptions;
- expose candidate-specific proof and update costs;
- expose hostile-input behavior;
- avoid hidden state-model assumptions;
- avoid hidden privileged authorities; and
- preserve evidence provenance.

A construction that is fast but cannot express required semantics is not
acceptable merely because it benchmarks well.

A construction that is secure only by trusting an undeclared privileged state
provider is not trust-minimized merely because its proofs are cryptographically
sound.

---

## 59. Relationship to Long-Term Protocol Evolution

Authenticated-state design must be compatible with Dilithia's long-term
evolution objective.

Future protocol evolution may require:

- new state versions;
- new cryptographic primitives;
- new proof formats;
- new authenticated-state constructions;
- new node roles;
- new synchronization methods; or
- new light-client capabilities.

Evolution must not silently reinterpret old authoritative state.

Migration must remain deterministic under applicable authoritative rules.

Old cryptographic assumptions must not be treated as permanently valid merely
because they were valid at Genesis.

The exact Evolution Engine, HIP, Super HIP, Emergency Security Upgrade, and
migration activation mechanics remain outside this document.

---

## 60. Conclusion

Authenticated-state design must begin with authoritative logical semantics.

A commitment is not a substitute for state semantics.

A proof is not sufficient merely because its cryptographic equation verifies.

Absence must mean what the protocol says absence means.

A snapshot is not authoritative merely because a trusted project service
published it.

A light client that verifies a state proof must also have an explicit path for
determining whether that state belongs to the authoritative Dilithia history
under the future selected consensus architecture.

Cryptographic agility applies not only to authorization but also to
authenticated-state and history-verification dependencies.

Future catastrophic cryptographic failure may challenge historical
verification assumptions and must be analyzed explicitly rather than hidden
behind a claim of permanent cryptographic safety.

Account and UTXO remain co-equal.

No commitment, proof system, snapshot mechanism, light-client protocol,
consensus algorithm, cryptographic primitive, governance weighting, or
state-model ranking is selected.

State-model decision:

**NOT MADE**

The next state-model decision gate remains:

> **Logical access, mutation, persistent-growth, and invalid-candidate resource
> requirements.**
