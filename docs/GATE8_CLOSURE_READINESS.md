# Dilithia Gate 8 Closure and Decision-Readiness Record

**Status:** NON-NORMATIVE — CLOSURE CANDIDATE PENDING FOCUSED REVIEW
**State-model decision:** NOT MADE
**Gate-8 satisfaction decision:** NOT YET MADE
**Protocol adoption effect:** NONE

> This document records candidate answers for the remaining Gate-8
> decision-readiness questions.
>
> It does not select Account, UTXO, a state commitment, proof system, snapshot
> format, synchronization protocol, light-client protocol, consensus algorithm,
> cryptographic primitive, governance mechanism, or state-model ranking.
>
> Gate 8 must not be marked SATISFIED merely because this document exists.
> A focused independent review must first confirm that these answers are
> model-neutral, internally consistent, and sufficient under the Gate-8
> satisfaction standard.

## 1. Source Identity

Reviewed Gate-8 source merge:

`e4f5717fa5eb2273f75b015f40a62da015b10350`

Reviewed Gate-8 source Git blob:

`08928d77174580e500ca8f366795ddd79720b224`

Source document:

`docs/AUTHENTICATED_STATE_MEMBERSHIP_ABSENCE_SNAPSHOT_LIGHT_CLIENT_REQUIREMENTS.md`

This closure record is subordinate to the Dilithia Technical Constitution and
applicable Formal Specification.

## 2. Closure Rule

A Gate-8 answer is sufficient at this stage when it defines the required
model-neutral semantic property without silently selecting:

- Account or UTXO;
- a concrete authenticated-state construction;
- a proof-system family;
- a hash or other cryptographic primitive;
- a snapshot file format;
- a synchronization protocol;
- a light-client protocol;
- a consensus or fork-choice mechanism;
- a governance mechanism;
- a numeric resource parameter; or
- a state-model ranking.

Concrete representations and mechanisms remain for later authoritative or
evidence-based design where expressly deferred.

## 3. Question 1 — Canonical Logical Facts Requiring Authenticated Coverage

A canonical logical-state fact requires authenticated-state coverage when:

1. the fact affects current protocol validity, current authoritative state, or
   the validity or effects of a future canonical state transition; and
2. the fact cannot be deterministically derived under authoritative rules from
   other authenticated authoritative facts.

Applicable fact classes may include:

- current native-value facts;
- current ownership or control-condition facts;
- current replay- or double-spend-exclusion facts;
- current entity existence, absence, or lifecycle facts;
- protocol-version facts affecting interpretation;
- cryptographic-version facts affecting interpretation;
- validity-affecting canonical system-state facts; and
- current authenticated facts that replace historical dependencies when full
  replay is not required.

Implementation-local facts do not become authenticated protocol facts merely
because a particular implementation stores them.

**Closure disposition:** ANSWERED.

## 4. Question 2 — Required Current Membership Claims

A current membership claim must be supportable whenever the presence of a
canonical logical fact can affect:

- current validity;
- authorization;
- ownership;
- native-value interpretation;
- replay or double-spend exclusion;
- entity lifecycle;
- protocol or cryptographic version interpretation; or
- another authoritative state transition.

A candidate does not receive an easier or harder membership obligation merely
because of its preferred internal representation.

**Closure disposition:** ANSWERED.

## 5. Question 3 — Required Current Absence Claims

A current absence claim must be supportable whenever authoritative validity or
state-transition semantics depend on a canonical logical fact being absent.

Physical database non-presence is not sufficient evidence of protocol-semantic
absence.

Account and UTXO receive the same external absence requirement.

**Closure disposition:** ANSWERED.

## 6. Question 4 — Required Semantic Distinctions

Zero, empty, absent, removed, consumed, invalid, unsupported, and historical
non-current state must remain distinct whenever collapsing two categories could
change:

- validity;
- ownership;
- authorization;
- replay safety;
- double-spend safety;
- native-value interpretation;
- lifecycle interpretation;
- migration interpretation; or
- historical interpretation.

Two categories may be treated as equivalent only when applicable authoritative
rules explicitly establish that equivalence and no required semantic distinction
is lost.

**Closure disposition:** ANSWERED.

## 7. Question 5 — Required Historical Claims

Historical claims must remain provable, under the applicable accepted
cryptographic and trust assumptions, when the historical fact is required to
determine a current or required authoritative property and no sufficient current
authenticated replacement fact exists.

Relevant properties may include:

- replay safety;
- ownership interpretation;
- native-value correctness;
- lifecycle interpretation;
- migration interpretation;
- canonical reapplication; or
- another expressly required historical property.

Gate 8 does not require permanent proof of every historical fact for every
validity decision.

This does not prohibit or discourage permissionless preservation of full
canonical history.

If an authoritative current authenticated fact fully replaces a historical
dependency for the relevant validity decision, direct proof of that historical
fact is not required by Gate 8 for that decision.

The catastrophic-break limits and deferred trust questions recorded for
Questions 28 and 29 remain applicable. This requirement does not claim that
arbitrary cryptographic failure preserves all historical proofs forever.

**Closure disposition:** ANSWERED.

## 8. Question 6 — Canonical Logical-State Subject

The subject of a required authenticated claim is an unambiguous
protocol-semantic subject whose identity and scope are sufficient to evaluate
the relevant authoritative claim.

Gate 8 does not require the subject to be the smallest possible unit, a single
record, or any particular commitment granularity.

The subject is defined by protocol meaning, not by:

- a database row;
- a page;
- a cache entry;
- a file;
- an implementation object;
- an implementation-specific key; or
- another accidental physical-storage boundary.

Its concrete identifier or encoding remains unselected.

**Closure disposition:** ANSWERED.

## 9. Question 7 — Commitment Coverage Boundary

Authenticated-state coverage must include every current canonical logical fact
required under Question 1.

A fact need not be directly and redundantly committed when it is deterministically
derivable under authoritative rules from other authenticated facts without
changing any required verification property.

The following remain outside authenticated-state coverage unless future
authoritative rules explicitly make them consensus-relevant:

- implementation caches;
- database layout;
- indexes used only for acceleration;
- memory layout;
- compaction state;
- local file placement;
- other implementation-local metadata; and
- non-authoritative derived convenience data.

Historical data need not remain in current authenticated-state coverage when its
required validity role has been completely replaced by sufficient current
authenticated facts.

Exclusion from current authenticated-state coverage does not require deletion
of that data from permissionlessly preserved canonical history.

Canonical-history binding remains a separate requirement unless a future
authoritative architecture explicitly defines a relationship.

**Closure disposition:** ANSWERED.

## 10. Question 11 — Accepted State Updates and Commitments

For every accepted canonical state transition from logical state `S` to `S'`,
the authoritative authenticated-state result must correspond exactly to `S'`.

Canonical state and its consensus-relevant authenticated-state result must not
become observably inconsistent.

A rejected candidate leaves the prior canonical authenticated-state result
unchanged.

If a future design uses multiple namespaces or multiple commitments, their
collective result must still correspond deterministically to the same accepted
authoritative post-state.

Reorganization and canonical reapplication must produce the authenticated-state
result corresponding to the newly authoritative canonical logical state.

No incremental-update algorithm or commitment representation is selected here.

**Closure disposition:** ANSWERED.

## 11. Question 13 — Invalid and Unsupported Evidence Classification

Future authoritative interpretation must deterministically distinguish at least
the following semantic classes where applicable:

1. structurally malformed or non-canonical evidence;
2. unsupported or unknown version/context;
3. cryptographically invalid evidence;
4. cryptographically valid evidence for the wrong subject, claim, network,
   purpose, or authoritative history context; and
5. stale or reverted evidence that is not valid for a current-state claim.

Stale or reverted evidence may remain meaningful only for an explicitly
supported historical claim class.

Exact error codes, wire encodings, and implementation-local diagnostic text
remain unselected.

**Closure disposition:** ANSWERED.

## 12. Question 14 — Snapshot Completeness

For a target node role, a snapshot must provide enough information to:

- reconstruct every current canonical logical fact required by that role;
- verify every required authenticated-state relation;
- detect omission or inconsistency of required state; and
- bind the reconstructed state to the required authoritative history and version
  context.

A required fact may be:

- directly included;
- deterministically reconstructible;
- separately authenticated; or
- derivable from other authoritative data.

Full historical replay is not automatically required.

If a historical dependency is omitted, a sufficient current authenticated
replacement fact must preserve the same required validity result.

**Closure disposition:** ANSWERED.

## 13. Question 17 — Snapshot Bootstrap Trust

A snapshot mechanism must not introduce a new privileged trust root.

The snapshot provider, website, API, repository, founder, foundation, release
channel, or other distributor is not protocol authority merely because it
delivers snapshot data.

Any trust assumptions that remain must be limited to:

- the explicitly declared assumptions of the future selected consensus/bootstrap
  architecture; and
- the explicitly declared cryptographic assumptions required for verification.

A snapshot path must not require greater hidden trust than the authoritative
protocol design otherwise requires.

The concrete consensus/bootstrap trust model remains unselected.

**Closure disposition:** ANSWERED AT THE ABSTRACT REQUIREMENT LEVEL.

## 14. Question 18 — Current Facts Replacing Historical Dependencies

When full historical replay is not required, every historical dependency that
would otherwise affect current validity must be replaced by a sufficient current
authenticated fact or fact set.

The replacement must cause compliant implementations to reach the same required
validity and state-transition result that the retained historical dependency was
needed to establish.

Possible classes include:

- replay-exclusion state;
- current ownership/control state;
- current native-value state;
- lifecycle or generation state;
- protocol-version state;
- cryptographic-version or migration state;
- monetary-reconciliation state; and
- other current canonical facts required by future authoritative semantics.

Exact representations remain unselected.

**Closure disposition:** ANSWERED.

## 15. Question 19 — Required History and Retention

Dilithia's long-term objective is that full canonical history remain
permissionlessly preservable and independently reproducible.

Gate 8 does not require every validating node to retain the full raw canonical
history forever as a condition of protocol participation.

A future architecture must preserve the distinction between protocol truth and
which individual participants retain all historical bytes.

At the abstract requirement level:

- any party must be able to preserve full canonical history without privileged
  access;
- full-history preservation must not be restricted to a founder, foundation,
  official server, repository, or other privileged provider;
- pruning by one participant must not redefine historical truth, lifecycle
  semantics, ownership, replay safety, or canonical protocol state;
- a participant that does not retain full history must still maintain or obtain
  enough authenticated current and retained facts to perform the authoritative
  duties required by its future role;
- historical claim classes required under Question 5 must remain verifiable
  under the future selected architecture; and
- loss or unavailability of one history provider must not itself change
  canonical protocol truth.

Hardware, storage, and network capability may improve over time and may make
full-history preservation easier for more participants, but protocol correctness
and decentralization must not depend on hardware improving at any particular
rate.

Exact archival roles, replication or availability mechanisms, retention
durations, reorganization depth, incentives, storage economics, and numeric
resource limits remain deferred to their proper later design stages.

**Closure disposition:** ANSWERED AT THE ABSTRACT REQUIREMENT LEVEL.

## 16. Question 21 — Reorganization Effect on Proof and Snapshot Validity

When authoritative canonical history changes under future consensus rules, an
authenticated-state artifact bound to history that is no longer canonical must
not be accepted as evidence of current canonical state merely because its
cryptographic verification still succeeds.

A proof or snapshot bound to reverted or otherwise non-current history may
remain meaningful only for an explicitly supported historical claim under the
applicable authoritative rules.

Current-state use requires binding to the newly authoritative canonical-history
context and satisfaction of the applicable current protocol, authenticated-state,
and cryptographic-version requirements.

Gate 8 does not select fork choice, finality, reorganization depth, rollback
mechanics, checkpointing, or state reconstruction mechanics.

**Closure disposition:** ANSWERED AT THE ABSTRACT REQUIREMENT LEVEL.

## 17. Question 22 — Required Light-Client Claim Classes

There is no universal requirement that every light-client role verify every
possible state fact.

Each future light-client profile must explicitly declare the decisions and claim
classes on which it relies.

For every claim that the light client exposes as authoritative or uses to make an
authoritative decision, it must verify all applicable facts needed for that
claim, which may include:

- current membership;
- current absence;
- ownership or control state;
- replay-relevant state;
- native-value state;
- protocol-version state;
- cryptographic-version state;
- historical state where expressly required; and
- canonical-history relationship.

A cryptographically correct state proof is insufficient without the required
binding to authoritative Dilithia history.

A light client must not imply verification of claim classes outside its declared
security profile.

**Closure disposition:** ANSWERED.

## 18. Explicit Deferred Questions

The following Gate-8 checklist questions remain expressly deferred because their
concrete answers depend on later architecture that Gate 8 must not silently
select.

### Question 24

How a light client obtains its concrete head or history anchor is deferred to the
future consensus/bootstrap architecture.

Gate-8 requirement preserved:

No hidden privileged state or head provider may be introduced.

### Question 25

The exact unavoidable external trust assumptions, if any, are deferred until the
future consensus/bootstrap architecture is selected.

Gate-8 requirement preserved:

Every such assumption must be explicit and reviewed.

### Question 27

The concrete procedure used when a commitment or history-related cryptographic
primitive is deprecated is deferred to Crypto Agility, migration, and applicable
consensus design.

Gate-8 requirement preserved:

Deprecation must not silently reinterpret authoritative state or history.

### Question 28

The exact surviving assumptions after catastrophic cryptographic failure depend
on the primitive and future consensus/history architecture.

Gate-8 requirement preserved:

No claim is made that arbitrary total cryptographic failure is always recoverable
without loss or additional trust.

### Question 29

Whether historical verification remains possible after catastrophic
cryptographic failure is architecture- and failure-dependent.

Gate-8 requirement preserved:

Any new trust assumption must be exposed explicitly rather than hidden.

## 19. Forty-Question Closure Matrix

1. ANSWERED
2. ANSWERED
3. ANSWERED
4. ANSWERED
5. ANSWERED
6. ANSWERED
7. ANSWERED
8. ANSWERED BY MERGED GATE-8 REQUIREMENTS
9. ANSWERED BY MERGED GATE-8 REQUIREMENTS
10. ANSWERED BY MERGED GATE-8 REQUIREMENTS
11. ANSWERED
12. ANSWERED BY MERGED GATE-8 REQUIREMENTS
13. ANSWERED
14. ANSWERED
15. ANSWERED BY MERGED GATE-8 REQUIREMENTS
16. ANSWERED BY MERGED GATE-8 REQUIREMENTS
17. ANSWERED AT ABSTRACT REQUIREMENT LEVEL
18. ANSWERED
19. ANSWERED AT ABSTRACT REQUIREMENT LEVEL
20. ANSWERED BY MERGED GATE-8 REQUIREMENTS
21. ANSWERED AT THE ABSTRACT REQUIREMENT LEVEL
22. ANSWERED
23. ANSWERED BY MERGED GATE-8 REQUIREMENTS
24. EXPLICITLY DEFERRED
25. EXPLICITLY DEFERRED
26. ANSWERED BY MERGED GATE-8 REQUIREMENTS
27. EXPLICITLY DEFERRED
28. EXPLICITLY DEFERRED
29. EXPLICITLY DEFERRED
30. ANSWERED BY MERGED GATE-8 REQUIREMENTS
31. ANSWERED BY MERGED GATE-8 REQUIREMENTS
32. ANSWERED BY MERGED GATE-8 REQUIREMENTS
33. ANSWERED BY MERGED GATE-8 REQUIREMENTS
34. ANSWERED BY MERGED GATE-8 REQUIREMENTS
35. ANSWERED BY MERGED GATE-8 REQUIREMENTS
36. ANSWERED BY MERGED GATE-8 REQUIREMENTS
37. ANSWERED BY MERGED GATE-8 REQUIREMENTS
38. ANSWERED BY MERGED GATE-8 REQUIREMENTS
39. ANSWERED BY MERGED GATE-8 REQUIREMENTS
40. ANSWERED BY MERGED GATE-8 REQUIREMENTS

No checklist item remains intentionally unclassified by this closure candidate.

## 20. Non-Selection Register

State-model decision:

**NOT MADE**

Minimal Account selected:

**NO**

Minimal UTXO selected:

**NO**

State commitment selected:

**NO**

Proof system selected:

**NO**

Snapshot mechanism selected:

**NO**

Snapshot trust implementation selected:

**NO**

Synchronization protocol selected:

**NO**

Light-client protocol selected:

**NO**

Checkpoint selected:

**NO**

Weak subjectivity selected:

**NO**

Consensus algorithm selected:

**NO**

Cryptographic primitive selected:

**NO**

Resource meter selected:

**NO**

Numeric resource limit selected:

**NO**

Governance mechanism selected:

**NO**

Formal Specification consensus rule created by this document:

**NO**

Constitution amendment created by this document:

**NO**

Consensus implementation change:

**NONE**

## 21. Proposed Gate-8 Closure Standard

This closure candidate proposes that the Gate-8 decision-readiness questions are
now sufficiently classified at the model-neutral semantic level.

That proposal is not yet the final Gate-8 satisfaction decision.

Before Gate 8 may be explicitly recorded as SATISFIED:

1. this exact closure candidate must receive focused independent adversarial
   review;
2. any material findings must be resolved;
3. the reviewed bytes must be identified reproducibly;
4. the final record must explicitly state the Gate-8 satisfaction decision; and
5. the state-model decision must remain NOT MADE.

Until those conditions are complete:

**GATE 8 SATISFACTION DECISION: NOT YET MADE**

**STATE MODEL DECISION: NOT MADE**

**DO NOT START GATE 9 YET**