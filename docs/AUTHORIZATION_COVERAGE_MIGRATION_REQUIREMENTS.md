# Dilithia Authorization Coverage and Migration Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records terminology, comparison variables, unresolved policy
> branches, information boundaries, adversarial questions, and evidence gates.
> It defines no consensus rule, cannot resolve a Formal Specification TBD, and
> does not constitute protocol adoption. It selects no state model,
> authorization architecture, ownership representation, migration mechanism,
> cryptographic algorithm, transaction format, signing format, recovery
> mechanism, or numeric limit. The Dilithia Constitution and Formal
> Specification remain authoritative.

## 1. Status and Purpose

Dilithia is Pre-Genesis. The Formal Specification's Crypto Agility,
transactions, state, consensus, governance, and mechanical HIP / Super HIP
sections remain pending. Minimal Account and minimal UTXO remain co-equal
candidates.

This document narrows two prerequisites for a future comparison:

1. what authorization coverage a native DLTH effect may require; and
2. what outcome, assumptions, and information boundary a future cryptographic
   migration guarantee may require.

It exists to prevent either candidate from receiving favorable unstated
assumptions. Terms such as "must" and "required" below express authority-derived
constraints, model-independent safety properties, or decision gates within this
non-normative artifact. They do not adopt protocol behavior.

## 2. Authority and Non-Normative Boundary

The authority order applied here is:

1. Dilithia Technical Constitution
2. Dilithia Formal Specification
3. Ratified HIP / Super HIP material, if any
4. Normatively adopted conformance material
5. Implementation
6. `PROJECT_STATE.md`
7. Non-normative design, threat, resource, and benchmark documents
8. AI or other design analysis

The relevant current boundaries are:

- no privileged cryptographic authority may exist;
- protocol changes must not degrade cryptographic or consensus security;
- protocol evolution must respect the Constitution's long-term ownership and
  state-preservation requirements;
- consensus-critical interpretation remains deterministic, canonical where
  specified, versioned, domain-separated where required, and reproducible by
  independent implementations; and
- protocol validity comes only from the authoritative protocol process, not
  from this document or an implementation precedent.

No ratified HIP or Super HIP currently supplies the pending ownership,
authorization, transaction, state, Crypto Agility, or migration rules.

## 3. Terminology

These distinctions do not require one-to-one mappings, separate encoded fields,
or separate on-chain objects.

| Term | Decision-readiness meaning |
|---|---|
| Consensus authority relation | A protocol-defined relation evaluated, alone or with other required relations, when determining whether a specified canonical effect is permitted under the applicable rules and state |
| Independent authorizer | One of the distinct consensus authority relations whose satisfaction is independently required |
| Credential | Cryptographic material or another future mechanism used to satisfy an authorization relation |
| Authorization evidence | Canonically interpreted evidence presented to satisfy one or more authorization relations |
| Cryptographic verification operation | One algorithm-dependent verification action performed while evaluating evidence |
| Funding or value source | A logical source of value or authority relevant to a candidate effect; no representation is implied |
| Affected state entity | A logical state entity read, created, changed, removed, or otherwise relevant to validation; no state model is implied |
| Authorization scope | The set of effects canonically bound to evidence under the applicable protocol rules |
| Grouping | A possible architecture in which one authorization relation or evidence object covers several sources, effects, or relations |
| Evidence reuse | A possible use of the same evidence for more than one canonically bound item within a permitted scope |
| Crypto version | The applicable protocol interpretation of a cryptographic or authorization rule; no field or registry is implied |
| Migration eligibility | Whether and under what adopted rules a current ownership or authorization relation may evolve |
| Historical validation | Interpretation of old canonical data under the rules applicable to that history |
| Current acceptance | Whether evidence or an algorithm may authorize a new effect under current rules |
| Distinguishing information or basis | Independent, pre-existing, consensus-interpretable information or authority basis capable of distinguishing authorization outcomes relevant to a claimed guarantee |

Independent-authorizer count does **not** mean human-party count, company or
legal-entity count, credential count, threshold-share count, proof count,
signature count, or cryptographic verification-operation count.

A protocol evaluates canonical evidence and protocol state. It cannot directly
observe external human identity, subjective intent, possession history, or moral
entitlement.

## 4. Authorization Coverage Variables

Future comparison must keep these variables distinct:

| Variable | Question | Why it matters |
|---|---|---|
| Independent-authorizer count | How many distinct authority relations must independently be satisfied? | Determines required authority coverage, not proof count |
| Credential count | How many credentials may exist per authority relation? | Affects lifecycle, state, and compromise exposure |
| Funding or value-source count | How many logical sources participate? | Affects transaction shape without determining evidence count |
| Affected-state-entity count | How much logical state participates? | Affects state discovery and persistent exposure |
| Verification-operation count | How many cryptographic operations actually occur? | Determines algorithm-dependent work |
| Grouping | May one relation or evidence object cover several items? | Affects scope, coupling, and possible PQ overhead |
| Evidence reuse | May one evidence object safely serve several bound items? | Affects replay surface, scope, and artifact volume |
| Threshold behavior | Must several components satisfy one authority relation? | Must not be equated with authorizer or signature count |
| Aggregation or batching | Can work or artifacts be combined under a future algorithm? | Algorithm- and format-dependent |
| Authorization scope | Which effects are bound to the evidence? | Controls over-authorization and substitution risk |
| Crypto-version diversity | Can several current or historical interpretations coexist? | Affects migration, parsing, and downgrade exposure |

Account does not imply one authorizer. UTXO input count does not imply signature
count. One owner does not imply one credential. Multiple credentials do not
necessarily imply multiple owners. One authorization object does not necessarily
imply one verification operation.

## 5. Parameters, Architecture Branches, and Prerequisites

The following categories must not be averaged together.

### Numeric or Workload Parameters

Examples include:

- source-count and affected-entity distributions;
- credential counts;
- evidence byte sizes;
- verification-operation counts;
- malformed-input workload; and
- potential authorization metadata volume.

These may be explored through ranges and sensitivity analysis without selecting
a protocol maximum or numeric limit.

### Architecture Branches

Examples include:

- grouped versus ungrouped authorization;
- native multi-authorizer capability;
- mixed-version coexistence;
- evidence reuse; and
- direct versus indirect ownership representation, if either is considered.

An architecture branch is not merely a scalar parameter. Comparisons must apply
the branch symmetrically or demonstrate robustness across its alternatives.

### Prerequisite or Policy Decisions

Examples include:

- the required migration guarantee;
- owner-participation assumptions;
- any dormant-asset promise; and
- whether a promised guarantee needs independent information to pre-exist a
  failure.

A final Account/UTXO winner must not average across incompatible prerequisite
semantics.

## 6. Authorization-Scope Requirement

The mechanism-neutral property is:

> Authorization evidence may enable only the effects canonically bound to that
> evidence by the applicable protocol rules, and no others.

Every authorization relation required by the applicable rules must be satisfied
before a canonical effect is valid. This property does not define whole-
transaction signing, per-input signing, per-effect signing, signing bytes,
transaction identifiers, replay state, or an authorization format.

Future review must distinguish asset-transfer authority from credential-change
authority if credential change is ever adopted. Their distinction, identity, or
absence remains unresolved.

## 7. Genesis Independent-Authorizer Question

Status: **UNRESOLVED — ARCHITECTURE BRANCH**.

Current evidence does not establish native support for more than one independent
authorizer as mandatory at Genesis. Institutional, exchange, treasury, or
corporate custody usefulness alone does not create a consensus requirement.
Off-chain controls may enforce internal approval but do not prove that consensus
enforced every independent party's approval.

Catastrophic resilience does not itself prove native multisig,
multi-authorizer support, threshold cryptography, scripts, or several evidence
objects are required. The relevant impossibility boundary is about missing
distinguishing information, not one selected architecture.

Retrofit limitations still matter: a stronger capability may be addable for new
assets later, while dormant existing assets may be unable to acquire it without
valid current authority. No representation or migration procedure is selected.

## 8. Same-Authority Grouping Question

Status: **UNRESOLVED — ARCHITECTURE BRANCH**.

It is unresolved whether one authorization relation or evidence object may cover
several funding or value sources controlled under the same authority relation.
Grouping may reduce PQ artifact or verification overhead, but may also affect:

- authorization scope;
- replay exposure;
- conflict coupling;
- the failure domain;
- transaction composition; and
- malformed or late-invalid validation work.

Sensitivity analysis may compare grouped and ungrouped branches. A final ranking
may not depend on grouping unless grouping becomes an adopted requirement or the
ranking remains robust across both branches. No grouping semantics are defined.

## 9. PQ-Cost Decomposition

PQ cost depends on more than a state-model label.

| Category | Future evidence variables |
|---|---|
| Algorithm properties | Public-key and evidence sizes, verification work, malformed-input behavior, scratch memory, aggregation or batching capability |
| Authorization architecture | Independent authorizers, credentials per relation, scope, grouping, evidence reuse, mixed-version behavior |
| State model | Logical source and entity counts, potential metadata placement, state discovery, representative and adversarial effect shapes |

Future comparison may need canonical authorization bytes, persistent metadata,
valid and invalid verification work, mixed-version exposure, and aggregate
transaction- or block-level workloads. No algorithm, artifact size, operation
taxonomy, grouping rule, or numerical estimate is selected.

## 10. Security Tradeoffs

Fewer evidence objects or verification operations are not automatically safer:

- a broadly defined canonical authorization scope can enlarge the set of effects
  authorized by the evidence and therefore enlarge compromise or blast-radius
  exposure;
- grouping can enlarge coupling and failure domains;
- evidence reuse can enlarge replay or substitution exposure;
- shared credentials can enlarge compromise blast radius;
- aggregation can obscure per-relation failure attribution; and
- narrow scope can improve least privilege while increasing artifact and work
  counts.

Security properties and PQ/resource cost must be evaluated separately before
their tradeoffs are compared. No branch is preferred here.

## 11. Hostile-Validation Work

Not every semantic authorization quantity must be visible in the initial
encoding. Future validation must distinguish:

1. directly transaction-structural data;
2. state-dependent data reachable through bounded lookup; and
3. algorithm- or version-dependent work known after bounded interpretation.

Before each expensive action, validation must have a deterministic bound check
or conservative reservation sufficient to prevent unbounded hostile work. A
state lookup may itself require prior capacity, and a later-discovered semantic
quantity remains constrained by its containing attempt envelope.

Relevant hostile shapes include many claimed relations, malformed credentials,
mixed versions, many sources under one relation, many independent relations, and
late-invalid evidence. This document defines no counter, unit, weight, dimension,
numeric limit, or exact validation order.

## 12. Corrected Migration-Guarantee Ladder

The ladder is a classification tool, not a selected policy.

| Level | Conceptual guarantee | Classification | Unresolved assumptions |
|---|---|---|---|
| A | Protocol evolution does not silently or arbitrarily destroy or change the protected ownership/control outcome; explicit deterministic versioned evolution may change representation or interpretation rules | Constitutional outcome constraint / abstract requirement | Authoritative migration and version rules |
| B | An active owner may be able to migrate while current authority remains secure and usable under an adopted procedure | Achievable candidate guarantee; not currently normative | Migration eligibility, procedure, scope, and conflict rules |
| C | A dormant owner retains a usable evolution path after long inactivity while the old primitive and credential remain secure | **UNRESOLVED POLICY CHOICE** | Current acceptance, participation, deprecation, migration eligibility, and possibly additional architecture |
| D | Protection survives loss, invalidation, or deprecation of a primary path because independent distinguishing information or basis existed before failure | Possible only with extra pre-existing assumptions; mechanism unselected | Nature, authority, scope, and timing of the pre-existing basis |
| E | Exclusive control plus unconditional recoverability survives full forgeability of sole evidence with no independent prior distinguishing basis | **IMPOSSIBLE WITHOUT EXTRA ASSUMPTION** | The premise removes the information consensus would need |

Level C is not currently justified or constitutionally guaranteed by this
document. Level B does not imply that an old algorithm remains accepted forever.
Level D does not establish a second credential, alternate authority, multisig,
threshold scheme, recovery path, identity layer, or other mechanism.

## 13. Catastrophic Information Boundary

Consider the model-independent case:

1. sole authorization evidence controls an asset under the applicable rules;
2. that evidence later loses its distinguishing power and becomes forgeable;
3. no independent pre-existing, consensus-interpretable distinguishing
   information or authority basis exists; and
4. valid-looking evidence from the external owner and an attacker can satisfy
   the same protocol predicate.

Consensus evaluates evidence and protocol state, not external human identity or
intent. It cannot reconstruct a distinction absent from those inputs.

Consequently:

- accepting the broken evidence may preserve nominal access while sacrificing
  exclusive control;
- rejecting it may preserve a new security policy while sacrificing access; and
- Crypto Agility cannot recreate missing distinguishing information after the
  fact.

This is an informational impossibility boundary only. It establishes no second
credential, alternate authority, multisig, threshold rule, recovery, identity,
privileged rescue, or migration mechanism.

## 14. Article 5 and Article 7 Boundary

Both constitutional constraints must be preserved without pretending their
future application is already resolved:

- Article 5: protocol change and implementation practice must not degrade
  cryptographic or consensus security.
- Article 7: protocol evolution must not itself cause prohibited ownership,
  spendability, reachability, or migration loss.

Article 7 protects protocol-evolution outcomes. It does not necessarily require
old evidence to remain directly usable for new effects indefinitely. Explicit,
deterministic, versioned evolution may change representation or interpretation
rules through the authoritative protocol process without silently or arbitrarily
changing the protected ownership/control outcome.

This document infers none of the following:

- perpetual direct acceptance of every old algorithm;
- automatic deprecation;
- automatic or global migration;
- lost-credential recovery;
- compromised-credential recovery;
- recognition of external human intent;
- migration without owner participation; or
- privileged rescue.

## 15. Historical Validation, Current Acceptance, and Migration Eligibility

These are separate questions:

| Question | Meaning | Current status |
|---|---|---|
| Historical validation | How old canonical evidence is interpreted under the rules applicable to that history | Deterministic interpretability is required; exact retention and verifier obligations remain unresolved |
| Current acceptance | Whether old evidence or an old algorithm may authorize a new effect now | **TBD** |
| Migration eligibility | Whether and under what adopted rules a current ownership or authorization relation may evolve | **TBD** |

Historical interpretability does not imply continued current acceptance.
Migration eligibility does not imply direct acceptance for an ordinary new
effect. This document selects no checkpointing, pruning, archival, verifier-
retention, or version-activation policy.

## 16. Dormant-Asset Cases

The matrix exposes assumptions and questions; it supplies no deadline, policy,
or answer.

| Case | Protected safety property | Participation assumption | Current-acceptance assumption | Independent prior information | Retrofit concern |
|---|---|---|---|---|---|
| 1. Owner active; primitive secure | No protocol-caused loss or authority outside adopted rules | Owner can act under some adopted rule | Whether old evidence directly authorizes migration or another effect is TBD | Not established as necessary for ordinary secure-era action | Procedure may be addable while current authority remains valid |
| 2. Owner dormant; primitive secure | Inactivity does not silently reassign control | Whether eventual action may be required is TBD | Continued old-evidence acceptance is TBD | Depends on the chosen guarantee | Level C remains unresolved |
| 3. Primitive secure; deprecation considered | Articles 5 and 7 are evaluated together | Owner-action requirement is TBD | Whether a secure old algorithm may be disabled is TBD | Depends on the promised dormant outcome | A cutoff may strand assets unless a valid evolution path exists |
| 4. Primitive weakened but not fully broken | No silent security downgrade or arbitrary loss | Timing assumptions are TBD | Continued acceptance and evidence for policy change are TBD | May be necessary for guarantees after primary rejection | Delay may narrow safe options |
| 5. Primitive becomes forgeable | Impossibility and security/access tradeoff are explicit | Participation may already be too late | Accepting or rejecting broken evidence has different losses | Required for any stronger distinguishing guarantee, without selecting its form | Missing distinction cannot be reconstructed |
| 6. Credential lost | Loss is not silent protocol reassignment | Owner cannot supply current evidence | Current algorithm security does not restore the credential | Needed only if a recovery-like guarantee were adopted | No recovery is currently required |
| 7. Credential stolen | Consensus does not infer external legitimacy | Competing presenters may act | Accepted evidence has only protocol-defined scope | Needed for any adopted override distinction | Later claims cannot establish human intent by themselves |
| 8. Owner offline; credential exists | Offline status alone conveys no new authority | Duration and eventual participation remain TBD | Direct old-evidence usability is not promised | Depends on Level C or stronger policy | Future cutoff consequences remain unresolved |
| 9. Asset predates later migration capability | No retroactive authority invention | Existing owner might opt in while authority remains valid | Applicable version rules are TBD | Stronger dormant guarantees may require prior information | Prospective capability may not protect dormant legacy assets |

## 17. Deprecation Questions

All of the following remain unresolved:

- whether a still-secure old algorithm may be disabled for new effects;
- whether continued acceptance can itself become unsafe;
- whether an old algorithm becomes history-only;
- what evidence justifies a policy change;
- whether owner action is required;
- whether dormant assets may become inaccessible under a proposed rule;
- whether any deprecation or migration interval exists; and
- how Articles 5 and 7 apply under the stated cryptographic and participation
  assumptions.

Scheduled deprecation is a scenario, not a project direction. This document
defines no sunset height, deadline, grace period, migration window, compulsory
owner action, or automatic deprecation.

## 18. Owner-Participation Questions

Possible policy branches include no owner action, eventual action, action before
a future deprecation, and action only when exercising a future effect. All are
unresolved.

Future review must ask:

- which participation assumption is necessary for each proposed guarantee;
- whether inactivity alone may change migration eligibility;
- whether an owner must act while current authority remains valid;
- how owners who never act are treated without inventing recovery; and
- whether a policy that requires action can satisfy Articles 5 and 7.

No branch is selected, and the list does not imply that a deadline or migration
window will exist.

## 19. Genesis Information-Boundary Decision

The precise pre-Genesis decision gate is:

> Does Dilithia promise any protection for already-created dormant assets that
> logically requires distinguishing information to exist before their current
> authority fails?

If the authoritative answer is yes, the required information boundary must be
addressed before affected assets are created, although its representation and
mechanism remain unselected. If the authoritative answer is no, Genesis should
not acquire machinery merely to promise theoretical universal recovery.

This document answers neither yes nor no. It does not require all migration
guarantee strength to be finalized before Genesis. Prospective capabilities may
potentially be added later, and an existing owner may potentially opt in while
current authority remains valid; neither possibility is a selected mechanism.

## 20. Retrofit and Irreversibility Analysis

Only these conditional information properties are recorded:

- prospective capabilities may be addable later for new assets;
- an existing owner may potentially opt in while current authority remains
  secure and valid under future adopted rules;
- dormant assets may be unable to acquire stronger guarantees without valid
  current authority;
- a capability added later does not automatically protect every earlier asset;
  and
- after sole evidence loses its distinguishing power, missing independent
  distinguishing information cannot be reconstructed.

The future protocol must separately analyze prospective adoption, owner-
authorized opt-in, treatment of dormant legacy assets, and post-failure claims.
No retrofit transaction, state update, field, or migration operation is defined.

## 21. Account-Neutral Questions

Minimal Account remains a candidate only. Questions include:

- How would required consensus authority relations be represented, if an
  Account design is selected?
- How would canonical authorization scope apply without assuming one Account is
  one authority domain?
- Could several independent relations or several crypto versions coexist if
  later required?
- How would dormant Account-related value satisfy whichever migration guarantee
  is authoritatively selected?
- What potential metadata, state lookup, evidence, and verification costs arise
  under each architecture branch?
- Would a stronger guarantee require distinguishing information to exist before
  primary evidence fails?
- Which quantities are structural, state-dependent, or algorithm-dependent?

These questions assume no mutable authorization metadata, stable identity,
global migration, credential count, authorizer count, Account fields, or Account
transition semantics.

## 22. UTXO-Neutral Questions

Minimal UTXO remains a candidate only. Questions include:

- May several sources share authorization coverage if grouping is later
  required or permitted?
- How would several independent authority relations compose, if required?
- Could several crypto versions coexist if later required?
- How would dormant UTXO-related value satisfy whichever migration guarantee is
  authoritatively selected?
- How would grouping affect scope, conflict coupling, and failure domains?
- Would a stronger guarantee require distinguishing information already
  embedded or otherwise consensus-interpretable before failure?
- Which quantities are structural, state-dependent, or algorithm-dependent?

These questions assume no permanently immutable condition, credential per
output, signature per input, spend-time-only migration, output field, output
identifier, or UTXO transition semantics.

## 23. Coverage and Migration Interaction

Coverage and migration variables interact without establishing a winner:

- several authority relations may provide additional distinguishing information
  in some future architecture but increase state and authorization complexity;
- grouping may reduce PQ overhead while enlarging scope and failure coupling;
- mixed versions may increase evolution flexibility while enlarging parsing,
  downgrade, and hostile-verification exposure;
- narrow authorization scope may improve least privilege while increasing
  evidence volume; and
- evidence reuse may reduce artifact cost while increasing replay and coupling
  risk.

None of these observations requires native multi-authorizer support, grouping,
mixed versions, an alternate authority, or a migration mechanism.

## 24. Ranking Decision Gate

For every materially relevant unresolved requirement, future comparison must do
one of the following:

1. decide the requirement through the authoritative protocol process; or
2. demonstrate that the Account/UTXO comparison result remains robust across all
   materially different alternatives.

Sensitivity and branch analysis may proceed before final decisions. A single
winner may not depend on a hidden authorizer count, grouping assumption,
ownership architecture, migration guarantee, owner-participation rule, dormant-
asset promise, current-acceptance policy, or independent-information premise.

## 25. Account/UTXO Comparison Matrix

The matrix records symmetric questions and dependencies. It assigns no score.

| Requirement or branch | Account questions | UTXO questions | Current evidence | Blocked dependency |
|---|---|---|---|---|
| One independent authorizer | What potential relation and state are consulted? | What potential relation and state are consulted? | Both can be analyzed abstractly | Ownership and authorization semantics |
| Several same-authority sources | What counts as several sources under the candidate? | May several candidate sources share coverage? | Grouping is unresolved | Effect and transaction shape |
| Several independent authorizers | Could one candidate effect require several relations? | Could one candidate effect require several relations? | No current authoritative Genesis requirement has been established | Capability decision |
| Grouped authorization | Which effects could be grouped if permitted? | Which sources or effects could be grouped if permitted? | Possible PQ tradeoff only | Scope and replay rules |
| Evidence reuse | What candidate scope would bound reuse? | What candidate scope would bound reuse? | No reuse rule exists | Canonical effect binding |
| Mixed-version coexistence | Could one candidate effect consult several versions? | Could one candidate effect consult several versions? | Architecture branch | Crypto Agility rules |
| Credential change | Would it require authority distinct from transfer? | Would it require authority distinct from transfer? | Distinction unresolved | Ownership lifecycle |
| Active secure-era migration | What candidate authority relation permits evolution? | What candidate authority relation permits evolution? | Level B candidate only | Migration eligibility |
| Dormant secure-era value | How could a selected Level C policy apply? | How could a selected Level C policy apply? | Level C unresolved | Participation and acceptance policy |
| Deprecation | How could a candidate preserve the selected outcome? | How could a candidate preserve the selected outcome? | No policy selected | Articles 5 and 7 analysis |
| Catastrophic break | What prior distinguishing basis existed, if any? | What prior distinguishing basis existed, if any? | Same impossibility applies | Chosen guarantee level |
| Historical validation | How is prior Account-related context selected? | How is prior UTXO-related context selected? | Historical/current distinction exists | Version rules |
| Current acceptance | What current evidence may authorize a new effect? | What current evidence may authorize a new effect? | **TBD** | Deprecation and version policy |
| Authorization metadata | What potential data persists under each branch? | What potential data persists under each branch? | Neither candidate is proven smaller | Logical schemas |
| Hostile verification | Which work is structural, state-dependent, or algorithm-dependent? | Which work is structural, state-dependent, or algorithm-dependent? | Work must be bounded | Formats and workflows |
| Retrofit | Can an existing asset adopt a future capability while current authority is valid? | Can an existing asset adopt a future capability while current authority is valid? | Prospective and dormant cases differ | Migration and ownership rules |

## 26. Adversarial Scenario Matrix

| Scenario | Safety property | Unresolved assumptions | Account questions | UTXO questions | Evidence required |
|---|---|---|---|---|---|
| 1. Ordinary native DLTH effect | Only canonically bound effects become valid | Required relations and scope | Which potential relations and entities participate? | Which potential relations and entities participate? | Abstract coverage obligations |
| 2. Same authority controls many sources | Coverage omits and adds nothing | Grouping branch | What are the candidate's logical sources? | May candidate sources share coverage? | Grouped/ungrouped sensitivity |
| 3. Several independent relations | Every required relation is satisfied | Native capability branch | Could the candidate express the requirement? | Could the candidate express the requirement? | Capability justification |
| 4. One evidence object covers several effects | No over-authorization | Canonical scope and reuse | What bounds the candidate scope? | What bounds the candidate scope? | Scope proof |
| 5. Evidence limited to one effect | No cross-effect reuse | Scope granularity | What metadata or evidence cost results? | What metadata or evidence cost results? | Least-privilege analysis |
| 6. Secure credential change | No ambiguous concurrent authority | Credential change may be absent | What candidate relation changes, if any? | What candidate relation changes, if any? | Conflict requirements |
| 7. Secure algorithm migration | Protected outcome preserved under stated assumptions | Level B eligibility and participation | What candidate questions apply? | What candidate questions apply? | Migration proof obligations |
| 8. Owner remains dormant | Inactivity creates no inferred new authority | Level C policy | How could the selected guarantee apply? | How could the selected guarantee apply? | Dormant-state analysis |
| 9. Secure primitive considered for deprecation | Articles 5 and 7 both remain effective | Current acceptance and participation | What candidate paths remain possible? | What candidate paths remain possible? | Constitutional policy review |
| 10. Primitive weakens | No silent downgrade or arbitrary loss | Evidence threshold and timing | What candidate exposure changes? | What candidate exposure changes? | Cryptanalytic and policy evidence |
| 11. Primitive becomes forgeable | Information boundary is explicit | Prior distinguishing basis exists or not | What prior candidate information existed? | What prior candidate information existed? | Formal impossibility argument |
| 12. Sole credential is lost | No silent reassignment | Recovery absent unless adopted | Is any prior candidate basis relevant? | Is any prior candidate basis relevant? | Explicit recovery decision or absence |
| 13. Sole credential is stolen | Consensus does not infer external legitimacy | Conflict and current acceptance | What effects are canonically bound? | What effects are canonically bound? | Compromise and scope analysis |
| 14. Several crypto versions coexist | No confusion or downgrade | Coexistence architecture branch | Could the candidate consult several versions? | Could the candidate consult several versions? | Cross-version tests |
| 15. One version becomes history-only | Historical and current meaning remain distinct | Deprecation policy | How is historical context selected? | How is historical context selected? | Historical/current conformance |
| 16. Many malformed evidence objects | Rejection work is bounded | Structural and algorithm-dependent bounds | Which work is known at each stage? | Which work is known at each stage? | Worst-case validation evidence |
| 17. Mixed valid and invalid relations | Failure is atomic | Required-set and validation-stage rules | What candidate effects remain provisional? | What candidate effects remain provisional? | Failure proof |
| 18. Late authorization failure | Attempted work remains bounded | State-dependent discovery | What candidate lookup may precede failure? | What candidate lookup may precede failure? | Attempt-envelope analysis |
| 19. Independent prior distinction exists | It grants no authority beyond adopted scope | Nature of the basis unselected | What candidate information could be consulted? | What candidate information could be consulted? | Separate threat and authority review |
| 20. No independent prior distinction exists | No impossible recovery promise | Sole evidence loses distinction | Same impossibility assumption | Same impossibility assumption | Information-boundary proof |
| 21. General rule changes current acceptance | No per-owner adjudication | Governance and Articles 5/7 | How is the candidate outcome preserved? | How is the candidate outcome preserved? | Constitutional and version review |
| 22. Historical data uses deprecated evidence | Historical interpretation stays deterministic | Verifier-retention policy | How is prior context selected? | How is prior context selected? | Cross-version conformance |
| 23. Reorg crosses an evolution event | Each branch uses deterministic applicable rules | Reorg and activation remain TBD | What candidate effects revert? | What candidate effects revert? | Reorg and version model |
| 24. Dormant asset predates later capability | No retroactive authority invention | Opt-in and legacy treatment | Can current authority adopt it before failure? | Can current authority adopt it before failure? | Retrofit feasibility evidence |

## 27. Formal-Verification Questions

Future formal work may need to establish:

- evidence enables no effects outside its canonical protocol scope;
- every protocol-required authority relation is satisfied;
- no unauthorized authority is inferred;
- grouped authorization does not omit or add effects;
- version selection is deterministic;
- migration does not create unintended concurrent authority;
- historical and current interpretation are unambiguous;
- failed authorization leaves no partial canonical effect;
- hostile authorization work is bounded; and
- every catastrophic guarantee states all independent-information assumptions.

These are proof obligations only. This document defines no formal state machine,
transition relation, signing function, or verification algorithm.

## 28. Premature-Commitment Matrix

| Classification | Items | Boundary |
|---|---|---|
| SAFE TO RECORD NOW | Catastrophic information boundary; consensus cannot infer external human intent; no privileged rescue; historical/current distinction | Authority-derived or model-independent facts |
| ABSTRACT REQUIREMENT ONLY | No protocol-caused ownership loss; deterministic versioned evolution; bounded hostile authorization work | Outcomes and safety properties, not mechanisms |
| COMPARISON VARIABLE | Authorizer count; credential count; source distributions; evidence and verification cost | May be explored parametrically |
| ARCHITECTURE BRANCH | Grouping; native multi-authorizer capability; mixed-version coexistence; evidence reuse | Apply symmetrically or demonstrate robustness |
| KEEP AS CANDIDATE | Ordinary rotation; prospective migration | Neither is selected or required |
| CONDITIONAL PREREQUISITE | Sufficient independent pre-existing consensus-interpretable distinguishing information or authority basis | Logically required before failure only if Dilithia adopts a guarantee preserving exclusive-control continuity after sole primary evidence loses its distinguishing power; form and mechanism remain unselected |
| BLOCKED / UNRESOLVED POLICY | Dormant Level C; global migration; owner participation; deprecation timing; indefinite old-algorithm acceptance | Requires authoritative policy and architecture decisions |
| DEFER | Multisig; recovery; delegation; concrete alternate-authority mechanism | No current Genesis requirement |
| IMPOSSIBLE WITHOUT EXTRA ASSUMPTION | Exclusive control plus unconditional recovery after sole-evidence forgeability with no independent prior distinction | Missing information cannot be recreated afterward |

Stable identity and direct credential-bound ownership remain unselected: stable
identity is blocked, and direct credential binding remains a candidate only.
Neither is inferred by this matrix.

## 29. Threat-Model Impact

**NO THREAT MODEL UPDATE JUSTIFIED YET.**

The current Threat Model already records primitive breakage, forgery,
compromised credentials, dormant addresses and unspent outputs as exposure,
hostile validation work, state and resource exhaustion, and failure atomicity.
This document refines information and decision boundaries but identifies no new
generic adversary or attack class.

A future update may be justified after a selected ownership, authorization,
migration, recovery, or deprecation direction creates model-specific threats, or
if an independently reviewed analysis discovers a genuinely new generic threat
class.

## 30. Formal-Specification Dependencies

Future authoritative specification may need to define, without this document
defining them now:

- authorization coverage and required authority relations;
- canonical authorization scope;
- transfer and credential-change authority, if credential change exists;
- current and historical crypto-version interpretation;
- migration eligibility and required guarantee assumptions;
- owner-participation policy;
- dormant-asset treatment;
- algorithm deprecation and current acceptance;
- failure, conflict, and atomicity behavior;
- canonical authorization encoding and signing scope; and
- validity-affecting structural and resource bounds.

No normative specification change is justified merely by recording these
questions. Implementation must not resolve them by precedent.

## 31. Complete TBD Register

The following remain unresolved:

- state model;
- ownership representation;
- address role and format;
- stable identity;
- credential representation;
- authorization representation;
- Genesis independent-authorizer capability;
- independent-authorizer requirements after Genesis;
- grouping semantics;
- evidence reuse;
- mixed-version coexistence;
- transfer/credential-change distinction and any mechanism;
- authorization scope encoding;
- signing bytes;
- transaction format;
- replay semantics;
- cryptographic algorithms and parameters;
- key rotation;
- algorithm-migration mechanism;
- owner participation;
- dormant-asset guarantee level;
- algorithm deprecation;
- current acceptance;
- migration eligibility;
- migration windows, deadlines, grace periods, or sunset behavior;
- independent distinguishing-information architecture;
- recovery;
- multisig;
- threshold authorization;
- delegation;
- `ChainId` representation;
- `NetworkId` discriminant values;
- domain-tag registry;
- state commitment;
- resource counters, units, and numeric limits;
- fees and economic rules;
- consensus and finality;
- activation and migration mechanics; and
- governance thresholds and HIP / Super HIP mechanics.

No item is resolved by its appearance in this document.

## 32. Exit Criteria for Future State-Model Ranking

A final Minimal Account versus minimal UTXO ranking is not decision-ready until:

1. each material authorization-coverage requirement is authoritatively decided
   or ranking robustness is demonstrated across its alternatives;
2. workload parameters are separated from architecture branches and policy
   prerequisites;
3. the required migration guarantee and all cryptographic, participation,
   current-acceptance, and dormant-asset assumptions are explicit;
4. the Genesis information-boundary question is authoritatively answered if a
   promised guarantee may be impossible to retrofit to dormant existing assets;
5. Account and UTXO are evaluated under identical guarantee and authority
   assumptions without presumed model-specific mechanisms;
6. PQ and hostile-validation evidence covers representative and adversarial
   branch combinations;
7. historical validation, current acceptance, and migration eligibility remain
   explicitly distinct;
8. no impossible post-failure guarantee is used as a ranking criterion; and
9. the comparison passes independent adversarial review.

Sensitivity and branch analysis may proceed before every exit criterion closes.
No single winner may be declared from hidden assumptions. This document selects
no state model and defines no consensus rule.
