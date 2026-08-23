# Dilithia Ownership and Authorization Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records requirements, unresolved questions, comparison
> variables, adversarial scenarios, and evidence gates only. It defines no
> consensus rule, resolves no Formal Specification TBD, selects no state model,
> ownership representation, authorization architecture, credential, recovery
> mechanism, cryptographic algorithm, or transaction format, and does not
> constitute protocol adoption. The Dilithia Constitution and Formal
> Specification remain authoritative.

## Status, Authority, Purpose, and Scope

Dilithia is Pre-Genesis. Crypto Agility, transactions, state, consensus, and the
mechanical HIP / Super HIP process remain pending in the Formal Specification.
This document exists to identify what must be understood before ownership and
authorization architecture can be selected and before Account and UTXO can be
compared fairly.

The authority order applied here is:

1. Dilithia Technical Constitution
2. Dilithia Formal Specification
3. Ratified HIP / Super HIP material, if any
4. Normatively adopted conformance vectors or tests
5. Implementation
6. `PROJECT_STATE.md`
7. Non-normative design, threat, resource, and benchmark documents
8. AI or other design discussion

Minimal Account and minimal UTXO remain co-equal candidates. Generalized Object
and active multiple-value-model Hybrid designs remain outside this document's
selection scope. This document does not select stable identity, direct
credential-bound ownership, address indirection, a credential registry, an
authorization descriptor, recovery, rotation, migration, multisig, delegation,
a signing format, state fields, a commitment construction, or numeric limits.

## 1. Terminology

The following distinctions prevent one unresolved mechanism from being treated
as another. They do not require one-to-one mappings or separate on-chain
objects.

| Concept | Decision-readiness meaning | Current status |
|---|---|---|
| Ownership | An entitlement or control relation over a permitted state or value effect | Its consensus semantics and representation remain unresolved |
| Authorization | Consensus determination that supplied evidence permits a particular effect under the applicable rules and state | The validity relation is necessary; its mechanism is unresolved |
| Credential | Cryptographic material or another mechanism used to satisfy an authorization rule | Format, storage, lifecycle, and algorithm remain unresolved |
| Identity | A possible persistent referent distinct from current credentials | Optional and **BLOCKED** |
| Address | A possible locator or encoding used by a wallet or consensus | Consensus role and format remain unresolved |
| Recovery | A previously specified alternate authority capable of restoring or changing control | Not currently required; mechanism deferred |
| Migration | Protocol evolution across representation or cryptographic eras | Required outcomes and assumptions need definition; mechanism unresolved |

A protocol evaluates authorization evidence and protocol state. It cannot
directly observe external human identity, intent, possession history, or moral
entitlement. One owner need not imply one credential; one credential need not
cover only one effect; and an address need not be identical to an owner,
credential, or state entity.

## 2. Constitutional Boundaries

The current authoritative boundaries include:

- No administrator, master, emergency, foundation, or equivalent privileged
  cryptographic authority may exist.
- An upgrade must not degrade cryptographic or consensus security.
- Consensus-critical data uses deterministic, canonical, versioned, and
  domain-separated interpretation where the Constitution and Formal
  Specification require it.
- Protocol-defined representation changes and upgrades must not themselves
  cause ownership or spendability loss contrary to Constitution Article 7.
- Consensus interpretation remains reproducible by independent implementations
  and independent of host-specific behavior.

Article 7 is not interpreted here as guaranteeing:

- recovery after loss of the only credential;
- recovery after credential compromise;
- consensus recognition of external human intent;
- migration without owner participation;
- permanent current acceptance of a broken cryptographic primitive;
- stable logical identity;
- mutable authorization records; or
- privileged rescue.

Those scenarios may expose limitations or unresolved design obligations, but
they do not authorize this document to invent a mechanism.

## 3. Catastrophic-Algorithm Impossibility Boundary

Consider this model-independent case:

1. Algorithm A was accepted by the applicable protocol rules.
2. An asset can be authorized only through evidence under A.
3. A later becomes practically or universally forgeable.
4. The external owner has not migrated.
5. No second credential, alternate pre-authorized condition, recovery path,
   identity indirection, or other independent authority exists.

Evidence produced by the external owner and evidence forged by an attacker may
then satisfy the same protocol authorization predicate. Consensus observes the
evidence and protocol state, not the external party's human identity or intent.
It therefore cannot distinguish the two authorizations on that basis.

Consequently:

- continuing to accept A may preserve nominal access while no longer
  preserving exclusive control;
- rejecting A may protect the new security policy while making unmigrated
  assets inaccessible through A; and
- Crypto Agility cannot retroactively recreate ownership information that is no
  longer distinguishable through the accepted evidence.

Migration windows, alternate credentials, pre-authorized conditions, multisig,
and indirection are candidate design directions only. This document proposes
none of them and does not assume that every catastrophic break has a solution
after the fact.

## 4. Safe Migration Requirement

The outcome-oriented requirement is:

> The ownership and authorization design must support protocol evolution
> without protocol-caused loss of ownership or spendability, while preserving
> deterministic interpretation of historical and current authorization rules.

Migration guarantees depend on explicit cryptographic, authorization,
participation, timing, and historical-validation assumptions. Those assumptions
must be stated before a proposed guarantee can be evaluated.

This requirement does not select spend-time migration, global migration,
dormant-owner migration, indefinite acceptance of an old algorithm, alternate
credentials, identity indirection, mutable authorization state, or a recovery
path. It also does not promise that unavailable or forgeable sole evidence can
be reconstructed into distinguishable ownership intent.

## 5. Lost Credential, Confiscation, and Recovery

The following cases are distinct:

| Case | Meaning |
|---|---|
| Protocol confiscation or reassignment | A protocol rule removes or redirects control |
| Rejected authorization evidence | Supplied evidence does not satisfy the applicable authorization rule |
| Loss of the only credential | No party can produce the currently required evidence |
| Explicitly pre-authorized recovery | A separately specified authority path changes or restores control |

Loss of the only valid credential may make value practically inaccessible
without implying that the protocol confiscated or reassigned it. The protocol
must not silently reassign or seize assets merely because a credential is
unavailable.

Recovery is not required by this document. Any recovery design would require a
separately justified and pre-specified authority model. Protocol-admin recovery
or any equivalent privileged fallback is constitutionally prohibited.

## 6. Compromised Credential Boundary

No evidence may authorize effects outside the authority assigned to that
evidence by the applicable protocol rules.

If accepted evidence is compromised or forgeable, consensus may be unable to
distinguish legitimate external intent from adversarial use. The protocol must
not be assumed to know which presenter is the external "real owner."

A compromised-credential race is therefore a deterministic validity and
ordering problem under the adopted rules, not a process in which consensus
discovers human intent. This document neither guarantees compromised-key
recovery nor selects a rotation or alternate-authority mechanism.

## 7. Stable Logical Identity

Classification: **BLOCKED — not selected now and not permanently forbidden**.

Possible benefits remain hypotheses: continuity across credential changes,
wallet usability, and reduced rewriting during migration.

Risks include:

- mutable identity-to-credential state;
- a credential registry;
- retained rotation history;
- recovery expectations;
- account-abstraction pressure;
- stale-credential or multiple-current-credential ambiguity;
- persistent-state growth;
- proof, snapshot, and historical-version complexity; and
- discretionary ownership-resolution pressure.

No requirement in this document means that the protocol knows a real owner,
that ownership survives independently of all evidence, or that credentials can
always be replaced.

## 8. Direct Credential-Bound Ownership

Classification: **KEEP AS CANDIDATE ONLY**.

Unresolved concerns include:

- obsolete or broken algorithms attached to dormant value;
- duplicated credentials or conditions;
- historical interpretation;
- migration exposure;
- deprecation behavior;
- algorithm-version coexistence; and
- the effect of credential compromise or loss.

No current evidence establishes that direct credential binding minimizes state,
simplifies migration, favors UTXO, disfavors Account, or handles dormant assets
better than another candidate.

## 9. Authorization Coverage

Future comparisons must distinguish these variables:

- independent-authorizer count;
- credential count per authorizer;
- number of logical value or state sources;
- authorization grouping;
- cryptographic verification-operation count;
- evidence reuse;
- threshold or aggregation behavior;
- cryptographic-version diversity; and
- authorization scope over actions or effects.

Account does not imply one authorizer. UTXO input count does not imply signature
count. One owner does not imply one credential. One authorization object does
not necessarily imply one cryptographic verification.

The required authorization coverage for native DLTH is a decision gate; no
coverage mechanism is defined here.

## 10. PQ and Cryptographic Cost Boundary

Future evidence must keep three categories separate:

| Category | Candidate variables |
|---|---|
| Algorithm properties | Key size, evidence size, verification work, malformed-input behavior, aggregation or batching capability, scratch memory, portable implementation behavior |
| Authorization-architecture properties | Independent authorizers, grouping, credential multiplicity, evidence reuse, version coexistence, authorization scope |
| State-model properties | Number of referenced value entities, potential ownership-metadata placement or duplication, state discovery, ordinary and adversarial transaction shapes |

No algorithm, parameter, key format, signature format, grouping rule, or batch
rule is selected. Benchmark timing and host behavior cannot become consensus
authorization semantics.

## 11. Rotation, Migration, and Emergency Deprecation

These remain distinct:

- key rotation replaces credential K1 with K2 within one primitive;
- algorithm migration changes interpretation from primitive A to primitive B;
- emergency deprecation responds after A loses the required security; and
- lost-credential recovery attempts to restore control without the ordinary
  current credential.

They may require different authority, replay, conflict, historical, and
resource semantics. This document defines no common mechanism. Ordinary key
rotation remains candidate-only.

## 12. Multisig and Alternate Authority

Native multisig is not currently a Genesis requirement, and no multisig
mechanism is selected. Privileged fallback is prohibited.

A migration guarantee stronger than "migration succeeds while the current
credential remains trustworthy and available" may constrain a future design
toward some independently pre-authorized authority path. The required guarantee
strength itself remains unresolved.

This observation does not establish that an alternate path is required and does
not select multisig, recovery, identity indirection, a backup key, or another
mechanism.

## 13. Delegation and Recovery

| Concept | Classification |
|---|---|
| Delegation | **DEFER** |
| Social or guardian recovery | **DEFER** |
| Protocol-admin recovery | **DO NOT ADOPT — constitutionally prohibited** |

This document defines no session key, spending allowance, guardian, delay,
revocation rule, capability, or recovery process.

## 14. Domain and Replay Boundary

The current Formal Specification provides these boundaries:

- `NetworkId` is included in every signed structure to prevent cross-network
  replay; its exact discriminant values remain TBD.
- Every hash or signature over DCS bytes uses a fixed purpose-specific domain
  tag; the exact domain-tag registry remains TBD.
- `ChainId` remains TBD alongside the future consensus chain-identification
  requirements.

Still unresolved are the complete signed bytes and any binding to an action,
referenced state, resource or economic declaration, authorization version, or
other effect context. No signing message or replay mechanism is defined.

## 15. Failure Atomicity

Model-independent requirements include:

- failed authorization creates no partial canonical effect;
- failure in later validation also leaves no partial canonical transition; and
- a failed credential-management operation, if one is later adopted, cannot
  partially update canonical authority state.

Canonical effects, hostile attempted-validation or resource work, and local
cache or policy effects are separate domains. Attempted hostile work is not
implicitly refunded by canonical rollback, and local state cannot redefine
consensus validity.

No exact validation order is defined. Cheap structural and safety checks may be
considered separately from expensive authorization verification.

## 16. Hostile Verification and Denial of Service

Future evidence must cover:

- many invalid cryptographic proofs or signatures;
- malformed authorization evidence;
- unknown versions;
- mixed algorithms;
- late authorization failure;
- repeated evidence; and
- state lookup before authorization failure.

Hostile verification exposure must eventually be structurally bounded,
deterministic, and implementation-independent. No resource counter, unit,
weight, dimension, or numeric limit is defined here.

## 17. Historical Versus Current Validation

Historical interpretation means validating old canonical data according to the
rules applicable to that history. Current acceptance asks whether an old
algorithm or authorization version may authorize a new transition now.

Historical acceptance does not by itself require continued current acceptance.
Future rules must make the distinction deterministic across implementations and
protocol versions.

This document selects no checkpointing, pruning, archival, snapshot, or
verifier-retention policy.

## 18. Governance Boundary

HIP / Super HIP governance may change general protocol rules through permitted
constitutional procedures. It must not become per-owner recovery,
discretionary asset reassignment, credential override, a hidden owner registry,
or an emergency signing authority.

A general deterministic migration rule adopted through the authoritative
process is distinct from governance deciding who is the external "real owner"
of one asset. No activation, voting, threshold, or migration mechanics are
defined.

## 19. Account Candidate Questions

Minimal Account remains a candidate only. Questions include:

- What relationship, if any, exists between a potential account identity and
  ownership?
- Can credentials change while another account-related referent remains stable,
  and what would justify that indirection?
- Where, if anywhere, is potential account-scoped authorization state stored?
- How would authorization interact with a still-undefined replay mechanism?
- What would deletion or recreation mean for authorization history?
- Could several independent authorizers control one account if that capability
  is required?
- Which potential metadata persists, and how is its growth bounded?
- What are the candidate migration implications for active and dormant state?
- How would a rotation candidate conflict with spending or another rotation?
- What canonical effects would be discarded after authorization or later
  validation failure?

Storage location, fields, lifecycle, replay, rotation, and migration semantics
remain unresolved. No Account structure or transition is defined.

## 20. UTXO Candidate Questions

Minimal UTXO remains a candidate only. Questions include:

- Is potential authorization state output-scoped, indirectly referenced, or
  represented another way?
- Could one authorization cover several inputs if future coverage requirements
  permit it?
- How would old-version conditions remain historically interpretable?
- Can several authorization versions coexist in one candidate transition?
- How would threshold or multiple-party control be represented if ever required?
- What metadata might be duplicated, and how would persistent growth be bounded?
- What are the candidate migration implications for dormant state?
- How would unknown or deprecated condition versions be treated?
- Can indirect metadata avoid duplication without creating a global identity
  registry?

Storage location, output fields, identifiers, consumption semantics, grouping,
and migration remain unresolved. No UTXO structure or transition is defined.

## 21. Account and UTXO Neutrality

Minimal Account and minimal UTXO remain co-equal candidates. No current evidence
establishes either as more secure, more PQ-efficient, more compact, easier to
migrate, easier to verify, or preferable for Genesis.

Any candidate-specific advantage or disadvantage remains a comparison
hypothesis until authorization coverage, transaction shape, migration
assumptions, state schema, commitment requirements, and resource behavior are
sufficiently concrete.

## 22. System-State Authority

Future design must distinguish:

- transferable user-controlled native value;
- canonical protocol or system state;
- governance-authorized protocol evolution; and
- local implementation state.

Governance is not an owner credential. System-state evolution must not be
silently generalized into user ownership, recovery, or authorization. No
governance state or system-state format is defined.

## 23. Adversarial Scenario Matrix

The matrix supplies questions and evidence gates, not mechanisms or answers.

| Scenario | Required property | Unresolved question and assumptions | Account questions | UTXO questions | Evidence required |
|---|---|---|---|---|---|
| 1. Ordinary native DLTH authorization | Only evidence valid for the permitted effect succeeds | What evidence and effect scope exist? | What potential account-related state is consulted? | What potential value-source state is consulted? | Abstract authorization and conservation model |
| 2. One owner uses several logical funding sources | Coverage is complete and non-duplicative | Can one authorization cover several sources? | Can one authorization cover all affected account-related sources, if several exist? | Can one authorization cover several inputs if allowed? | Coverage rules and transaction-shape evidence |
| 3. Multiple independent authorizers | Every required independent authority participates | Is native multi-authorizer support required? | Could several authorizers govern one potential account effect? | Could several ownership groups govern one effect? | Genesis capability decision and formal obligations |
| 4. Key rotation | No partial or ambiguous credential change | Is ordinary rotation supported and what authority would approve it? | Where might potential account-scoped rotation state reside? | Which potential conditions or indirect references would be affected? | Rotation requirements and conflict analysis |
| 5. Algorithm migration | Protocol migration does not itself orphan assets; assumptions are explicit | Which cryptographic and participation assumptions make the proposed guarantee feasible? | What are the candidate implications for active and dormant account-related state? | What are the candidate implications for active and dormant output-related state? | Crypto Agility and dormant-state analysis |
| 6. Sole algorithm breaks before migration | The impossibility and resulting security/access tradeoff are explicit | No alternate authority exists; consensus cannot infer human intent | Which candidate state, if any, could have carried a pre-authorized alternative without selecting one now? | Which candidate condition, if any, could have carried a pre-authorized alternative without selecting one now? | Formal impossibility argument and threat analysis |
| 7. Credential compromised before rotation | Accepted evidence grants no scope beyond its protocol authority; human intent is not inferred | Which competing effects verify, and how are conflicts determined? | How would spend and rotation candidates conflict? | How would spend and condition-change candidates conflict? | Deterministic validity and ordering model |
| 8. Credential lost | Protocol does not silently reassign value; recovery is not assumed | Is any pre-authorized recovery capability required? | Would a candidate account design include one only if later justified? | Would a candidate ownership condition include one only if later justified? | Explicit recovery decision or documented absence |
| 9. Two competing rotations | At most the effects permitted by canonical conflict rules become canonical | Is rotation adopted, and how are conflicts recognized? | What potential account-scoped state would conflict? | What potential condition changes would conflict? | Conflict and ordering semantics |
| 10. Spend versus rotation | No ambiguous authority or partial canonical effect | Which authorization version applies to each candidate? | How would replay and credential-change candidates interact? | How would spend and condition-change candidates interact? | Version and conflict rules |
| 11. Ownership transfer versus credential change | Transfer and credential change cannot accidentally retain or confer the other's authority | Are the concepts distinct, identical, or absent? | What would each candidate effect mean without assuming persistent identity? | What would each candidate effect mean without assuming condition structure? | Authority-scope semantics |
| 12. Replay under an old authorization version | Previously exercised effects are not recreated through version ambiguity | When is old evidence current, historical, or invalid? | How would a still-undefined replay model interact with version changes? | How would a still-undefined consumption model interact with version changes? | Replay and historical-version rules |
| 13. Cross-chain replay | Authorization is chain-bound if the future protocol requires it | `ChainId` remains TBD | What future signed domain would apply? | What future signed domain would apply? | ChainId and signing-domain decisions |
| 14. Cross-network replay | Every signed structure respects the authoritative NetworkId requirement | NetworkId values remain TBD | How would NetworkId enter the future authorization context? | How would NetworkId enter the future authorization context? | NetworkId discriminants and canonical signing format |
| 15. Unknown authorization version | Failure is deterministic and cannot escalate authority | How are unknown current and historical versions distinguished? | Where would version interpretation come from? | Where would version interpretation come from? | Version-selection and error semantics |
| 16. Malformed PQ evidence | Rejection is bounded and failure-atomic | Algorithm and representation remain unselected | What state must be known before verification cost is determined? | What state must be known before verification cost is determined? | Parser, algorithm, and attempted-work evidence |
| 17. Many expensive invalid authorizations | Hostile attempted work remains bounded | Which counts, algorithms, and sizes are visible before verification? | Which potential account-related metadata affects cost? | Which potential input or condition metadata affects cost? | Structural bounds and validation-stage analysis |
| 18. Dormant state under obsolete cryptography | The design states under which assumptions spendability can survive deprecation | Owner participation and alternate authority are unresolved | What candidate migration obligations apply to dormant account-related state? | What candidate migration obligations apply to dormant output-related state? | Dormant-state population and migration evidence |
| 19. Historical verification after deprecation | Old history remains deterministically interpretable | Historical acceptance is distinct from new-transition acceptance | How is the historical account-related authorization context selected? | How is the historical output-related authorization context selected? | Cross-version conformance and archival assumptions |
| 20. Reorg crosses an authorization change | Each branch is interpreted under deterministic applicable rules | Reorg and finality semantics remain TBD | How would potential authorization state be reverted and reapplied? | How would potential condition changes be reverted and reapplied? | Consensus, version, and rollback semantics |
| 21. Protocol version changes authorization interpretation | Historical meaning does not silently drift | Activation mechanics remain TBD | Which potential account-related data determines applicable interpretation? | Which potential output-related data determines applicable interpretation? | Versioned specification and independent implementation tests |
| 22. Several cryptographic versions coexist | No cross-version confusion or downgrade | Coexistence is candidate-only | Could one potential account effect require several versions? | Could one candidate transaction reference several versions? | Crypto Agility and authorization-coverage requirements |
| 23. One authorization may cover several UTXO inputs | Coverage cannot omit, add, or mutate an effect | Grouping is not selected | What equivalent comparison variable exists for Account? | Under what future rules, if any, would grouping be permitted? | Canonical transaction and signing semantics |
| 24. One Account may have several authorizers | Every required authority is enforced without abstraction leakage | Multi-authorizer support is not selected | Under what future rules, if any, would this be permitted? | What equivalent comparison variable exists for UTXO? | Native multi-authorizer requirement and formal model |
| 25. Authorization metadata growth | Persistent exposure remains bounded | Schema, retention, and economics remain TBD | Which potential account-scoped metadata could accumulate? | Which potential output-scoped or indirect metadata could accumulate? | Logical schemas, population models, and resource evidence |
| 26. Exceptional or recovery path abuse | Any adopted path grants no authority beyond its pre-specified scope | Adoption is explicitly conditional and privileged fallback is prohibited | What candidate implications would exist only if such a path were adopted? | What candidate implications would exist only if such a path were adopted? | Separate justification, threat review, and formal verification before adoption |

## 24. Premature-Commitment Matrix

| Classification | Items | Boundary |
|---|---|---|
| SAFE TO RECORD NOW | No privileged authority; deterministic authorization interpretation; canonical and domain-separated consensus interpretation where authoritative; failure atomicity | These are authority-derived or model-independent properties, not mechanisms |
| ABSTRACT REQUIREMENT ONLY | Protocol-caused migration safety; bounded hostile verification; ability to evolve cryptographic primitives without treating one algorithm as permanent protocol identity | Guarantee assumptions and mechanisms remain unresolved |
| COMPARISON HYPOTHESIS ONLY | Account one-authorization advantage; UTXO per-input authorization disadvantage; stable identity simplifies migration; direct credentials reduce state | No supporting architecture or evidence exists |
| KEEP AS CANDIDATE | Direct credential-bound ownership; versioned ownership conditions; spend-time migration; global migration; ordinary key rotation; multiple crypto versions; multi-algorithm authorization | None is selected |
| BLOCKED | Stable logical identity; address indirection; credential registry; authorization descriptor structure; Account authorization-state structure; UTXO ownership-condition structure; concrete signing message | Required state, transaction, ownership, and Crypto Agility decisions are absent |
| DEFER | Native multisig mandate; social or guardian recovery; delegation | No current authoritative Genesis requirement |
| DO NOT ADOPT | Privileged protocol-admin recovery or equivalent emergency ownership override | Prohibited by Constitution Article 2 |

## 25. Decision Gates

Ownership/authorization architecture selection and Account/UTXO ranking remain
blocked until sufficiently concrete abstract answers exist for:

1. Required authorization coverage
2. Independent-authorizer requirements
3. Migration-continuity guarantee strength and its assumptions
4. Dormant-asset expectations under deprecation and catastrophic failure
5. Historical versus current cryptographic-version behavior
6. Replay and domain properties
7. Credential-change requirements, if any
8. Failure, conflict, and ordering properties
9. Hostile verification and persistent-state exposure

These gates require decision-ready semantics and evidence, not selected fields,
formats, algorithms, or mechanisms.

## 26. Genesis Minimality

| Classification | Items |
|---|---|
| Genuinely necessary or near-unavoidable at the property level | Relationship between native DLTH control and accepted authorization evidence; deterministic authorization interpretation; replay and domain protection sufficient for native effects |
| Strong requirements or principles | Failure atomicity; bounded hostile authorization verification; protocol evolution must not be foreclosed |
| Premature | Explicit authorization-version field; key-rotation implementation; native multi-owner support; native multisig; alternate credential; recovery; stable identity |

Genesis convenience or hypothetical future extensibility is insufficient to
expand the consensus authorization surface.

## 27. Formal Specification Boundary

Future authoritative specification may need to define:

- accepted authorization evidence;
- ownership or control semantics;
- authorization scope;
- domain and replay binding;
- historical and current version interpretation;
- cryptographic deprecation and migration behavior;
- failure and conflict behavior;
- dormant-asset treatment and required assumptions;
- an alternate-authority path, if one is ever adopted;
- canonical authorization encoding; and
- validity-affecting resource bounds.

This document defines none of those rules and does not modify the Formal
Specification.

## 28. Threat Model Boundary

**NO THREAT MODEL UPDATE JUSTIFIED YET.**

The current Threat Model already records future primitive breakage, dormant
addresses and unspent outputs as exposure, key compromise, cryptographic
resource exhaustion, state growth, failure atomicity, and version drift. The
catastrophic-algorithm indistinguishability consequence is recorded here as a
decision requirement and impossibility boundary.

A Threat Model update should follow when a selected ownership, authorization,
recovery, or migration direction creates concrete model-specific threats, or if
a genuinely new generic threat class is discovered.

## 29. Complete TBD Register

The following remain unresolved:

- state model;
- ownership representation;
- identity architecture;
- address role and format;
- authorization representation;
- credential format;
- cryptographic algorithms and parameters;
- key rotation;
- algorithm-migration mechanism;
- emergency-deprecation behavior;
- dormant-asset handling;
- recovery;
- multisig;
- threshold authorization;
- delegation;
- independent-authorizer requirements;
- authorization grouping;
- signing bytes;
- transaction format;
- replay semantics;
- `ChainId`;
- `NetworkId` discriminant values;
- domain-tag registry;
- state commitment;
- resource units and numeric limits;
- fees and economic rules;
- consensus and finality;
- activation and migration mechanics; and
- governance thresholds and HIP / Super HIP mechanics.

No item is resolved by its appearance in this document.

## 30. Conclusion

- No ownership representation is selected.
- No authorization mechanism is selected.
- Stable logical identity remains blocked, not permanently forbidden.
- Direct credential-bound ownership remains candidate-only.
- Recovery and delegation remain deferred.
- Native multisig is not currently required.
- Privileged recovery remains constitutionally prohibited.
- Minimal Account and minimal UTXO remain co-equal candidates.
- Cryptographic migration cannot guarantee recovery of external human ownership
  intent after the sole authoritative evidence becomes forgeable or unavailable.
- Stronger migration guarantees may constrain future architecture toward some
  pre-authorized alternate authority path, but no such path or mechanism is
  selected.
- This document defines no consensus rule.
