# Dilithia PQ Authorization-Count and Artifact Assumptions Decision Requirements

> **NON-NORMATIVE DECISION-READINESS DOCUMENT**
>
> This document records model-neutral requirements, unresolved questions,
> comparison variables, adversarial scenarios, and evidence gates for
> post-quantum authorization count and authorization-artifact assumptions.
>
> It defines no consensus rule, resolves no Formal Specification TBD, selects no
> state model, ownership representation, authorization architecture, credential
> format, cryptographic primitive, parameter set, signature format, proof format,
> aggregation rule, batching rule, multisignature mechanism, threshold mechanism,
> key-reuse policy, transaction format, state commitment, resource meter, fee
> rule, or candidate ranking.

## Status

This document is a Pre-Genesis decision-readiness artifact.

It exists to clarify the sixth state-model decision gate:

> PQ authorization-count and artifact assumptions sufficient for comparison.

The existence, completion, or review of this document does not by itself satisfy
that gate.

Minimal Account and Minimal UTXO remain co-equal candidates.

State-model selection remains **NOT MADE**.

Dilithia's current Formal Specification identifies post-quantum cryptography as
part of the protocol's Genesis direction while the exact cryptographic
primitives, parameters, registry mechanics, coexistence rules, and Crypto
Agility runtime mechanisms remain unresolved.

This document therefore analyzes comparison assumptions without selecting a
concrete PQ algorithm or authorization architecture.

Protocol authority and supporting evidence are distinguished as follows.

Authoritative protocol sources are:

1. the Dilithia Technical Constitution;
2. the Dilithia Formal Specification; and
3. HIP or Super HIP material only to the extent that it has been validly
   adopted, activated, and given protocol effect through the authoritative
   protocol process.

Supporting material includes:

- decision-readiness requirement documents;
- comparison frameworks;
- workload models;
- benchmark methodologies;
- threat models;
- project-state summaries;
- experiments;
- implementations;
- tests;
- review records; and
- other design-analysis artifacts.

Supporting material does not become protocol authority merely because it is
reviewed, implemented, benchmarked, or included in the repository.

---

## 1. Existing Authoritative Boundaries

This analysis inherits the following existing authoritative boundaries without
expanding them.

- No administrator, master, emergency, foundation, or equivalent privileged
  cryptographic authority may become a protocol control path.
- Consensus-critical interpretation must remain deterministic across compliant
  implementations.
- Consensus-critical serialization and cryptographic use must obey the
  applicable canonical, versioned, and domain-separated rules.
- Previously valid canonical encodings must remain unambiguously decodable or
  safely migratable under the applicable authoritative rules.
- Ownership and authorization evolution must remain consistent with the
  Constitution's protected ownership and migration outcomes.
- Cryptographic evolution must not silently degrade required protocol security.
- The Formal Specification remains authoritative for actual protocol behavior.

The current Formal Specification also establishes relevant partial boundaries:

- Dilithia is intended to use post-quantum cryptography from Genesis;
- protocol identity is not tied to one immutable cryptographic algorithm;
- purpose-specific domain separation applies to applicable hashes and
  signatures over canonical bytes;
- the exact domain-tag registry remains TBD; and
- the Crypto Agility mechanism remains pending.

Separately, as a non-normative model-independent decision-readiness
requirement, historical and current protocol interpretation must not depend on
silent reinterpretation across protocol versions.

These boundaries do not select:

- a signature algorithm;
- a KEM algorithm;
- an authorization algorithm;
- a parameter set;
- a key format;
- a signature format;
- a proof system;
- an algorithm identifier;
- a registry structure;
- a credential format;
- a transaction signing format; or
- an authorization architecture.

---

## 2. Purpose of the Sixth State-Model Gate

A state-model comparison can become misleading if cryptographic cost is derived
from the labels "Account" or "UTXO" rather than from explicit authorization
requirements.

Examples of unsafe assumptions include:

- one Account implies one authorizer;
- one Account implies one credential;
- one Account transaction implies one signature;
- one UTXO input implies one independent authorizer;
- one UTXO input implies one credential;
- one UTXO input implies one signature;
- one authorization object implies one cryptographic verification;
- one owner implies one credential;
- one credential implies one submitted artifact;
- one submitted artifact implies one verification operation;
- several value sources necessarily require several independent
  authorizations; or
- PQ bandwidth follows directly from the number of Account or UTXO entities.

None of those equivalences is currently established.

The sixth gate exists to make the assumptions behind such comparisons explicit
before PQ cost, bandwidth, verification work, or related state-model claims are
used for candidate ranking.

---

## 3. Analytical Terminology

The following terms are analytical only.

They do not define protocol objects.

### Authority relation

A required semantic relation stating that some independently required authority
condition must be satisfied for an effect to be permitted.

It does not imply:

- a human;
- an owner representation;
- a public key;
- a credential;
- a signature;
- a proof;
- a transaction input;
- an Account;
- a UTXO;
- an object; or
- a stored protocol record.

### Independent authorizer

An analytical description of an authority whose required participation cannot be
derived merely from another required authority relation in the same comparison
case.

This document does not define how such independence would be encoded or proven.

### Credential

Cryptographic material or another future mechanism capable of participating in
an authorization rule.

Its format, representation, storage, lifecycle, algorithm, and protocol role
remain unresolved.

### Authorization evidence

Candidate material used to demonstrate that an applicable authorization
condition is satisfied.

This term does not imply a signature-only design.

### Authorization artifact

A comparison-level term for cryptographic or authorization-related material
whose count, size, placement, transmission, storage, parsing, or verification may
matter to a claim.

An artifact may be candidate-specific.

The term does not itself define a wire object or canonical protocol type.

### Authorization grouping

A possible relationship under which one item of authorization evidence,
condition, or mechanism covers more than one semantic source, action, effect, or
required authority relation.

No grouping mechanism is selected.

### Authorization scope

The set of effects, actions, sources, or state transitions that a future
authorization condition is permitted to authorize.

### Credential multiplicity

The number of credentials associated with one analytical authority relation
under a declared comparison profile.

It is not automatically one.

### Evidence reuse

A possible ability for some authorization evidence or authorization result to
serve more than one candidate-relevant semantic purpose under explicitly
permitted future rules.

No form of evidence reuse is selected.

### Verification operation

An analytical unit representing a cryptographic or authorization verification
action for measurement or comparison.

This document does not define the canonical resource accounting unit associated
with such an operation.

### Algorithm profile

A non-normative comparison profile that freezes relevant properties of a
cryptographic algorithm or algorithm family for an experiment.

Use of an algorithm profile in evidence does not constitute protocol adoption.

### Feature branch

A comparison branch in which an optional capability such as aggregation,
batching, or another algorithm-specific feature is explicitly enabled or
disabled.

### Candidate output

A quantity produced by a candidate mapping or implementation rather than imposed
as a common semantic requirement.

---

## 4. State-Model Labels Do Not Determine Authorization Count

The words Account and UTXO do not determine authorization cardinality.

Minimal Account does not imply:

- one owner;
- one independent authorizer;
- one credential;
- one signature;
- one proof;
- one verification;
- one authorization object; or
- one authorization artifact.

Minimal UTXO does not imply:

- one authorizer per input;
- one credential per input;
- one signature per input;
- one proof per input;
- one verification per input;
- one ownership condition per input; or
- one independently transmitted artifact per input.

Those relationships may become candidate outputs or architecture-specific rules
only after sufficient future specification and evidence.

---

## 5. Three-Layer Cost Separation

Future PQ comparison must keep at least three categories separate.

### Algorithm properties

Examples include:

- public or verification-key size;
- secret-key size where relevant to a non-consensus operational claim;
- evidence size;
- signature size;
- proof size;
- verification work;
- parsing behavior;
- malformed-input behavior;
- scratch-memory requirements;
- aggregation capability;
- batching capability;
- randomized or variable evidence behavior;
- implementation portability; and
- algorithm/version-specific performance.

### Authorization-architecture properties

Examples include:

- independent-authorizer count;
- credential multiplicity;
- authorization grouping;
- authorization scope;
- evidence reuse;
- ownership-condition reuse;
- cryptographic-version coexistence;
- threshold behavior if ever supported;
- aggregation use if available;
- alternate authority if ever adopted; and
- migration assumptions.

### State-model and transaction-shape properties

Examples include:

- number of logical value or state sources;
- candidate-specific internal source count;
- ownership-metadata placement;
- ownership-metadata duplication;
- state discovery requirements;
- transaction fan-in;
- transaction fan-out;
- candidate fragmentation;
- representative transaction shapes; and
- adversarial transaction shapes.

A result attributed to Account or UTXO must not silently contain an unreported
difference from the other two categories.

---

## 6. Authority-Relation Count `N`

Comparison methodology may use `N` as an analytical count of externally required
authority relations when the external semantic case genuinely distinguishes
them.

`N` is not automatically the number of:

- humans;
- owners;
- organizations;
- companies;
- addresses;
- Accounts;
- UTXOs;
- inputs;
- outputs;
- credentials;
- public keys;
- secret keys;
- signatures;
- proofs;
- threshold shares;
- authorization artifacts; or
- cryptographic verification operations.

A statement that all required authority relations are satisfied defines no
specific evidence structure.

---

## 7. `N > 1` Is an Architecture Branch

Native support for multiple independent authority relations is not currently
selected as a Genesis requirement merely because comparison methodology can
represent `N > 1`.

Where `N > 1` is relevant, the comparison must state why.

Possible reasons include:

- an externally required semantic case genuinely has several independent
  authorities;
- a candidate architecture being evaluated explicitly supports such authority;
  or
- an adversarial or future-capability branch is being tested.

A test case must not introduce extra authorizers only to make one candidate look
better or worse.

---

## 8. Owner Count and Authorizer Count Are Distinct

External ownership concepts and consensus-visible authorization requirements are
not automatically identical.

One external owner could, under future rules, correspond to:

- one credential;
- several credentials;
- an indirect authorization structure;
- a threshold condition;
- another authorization structure; or
- none of those examples.

Several external parties could, under future rules, correspond to:

- several independent authorization requirements;
- one jointly evaluated condition;
- a threshold condition;
- a delegated condition; or
- another architecture.

This document selects none of them.

Consensus cannot infer external human identity or moral entitlement merely from
cryptographic evidence.

---

## 9. Credential Multiplicity

Credential count per authority relation is a separate comparison dimension.

It must not be inferred from state-model labels.

A comparison profile that depends materially on credential multiplicity must
state that multiplicity explicitly.

Credential multiplicity may affect:

- transmitted bytes;
- persistent metadata;
- parsing;
- verification;
- migration;
- historical interpretation;
- wallet behavior;
- failure paths; and
- hostile-work exposure.

This document defines no preferred credential multiplicity.

---

## 10. Logical Source Count Is Separate From Authority Count

The number of logical value or state sources involved in an effect does not
automatically equal the number of independent authorizers.

A future candidate may have:

- several sources controlled by one authority relation;
- one source requiring several authority relations;
- several sources with several authority relations;
- grouped authorization across sources; or
- another structure.

The semantic source count and authority-relation count must therefore remain
separate comparison variables.

---

## 11. Authorization Grouping

Authorization grouping is a material comparison assumption.

Questions include:

- Can one authorization condition cover several logical sources?
- Can one evidence artifact cover several permitted effects?
- Can one authorization result cover several candidate-internal references?
- Is grouping defined by ownership, transaction shape, an authorization
  descriptor, or another future mechanism?
- Does grouping preserve exact effect scope?
- Can grouping omit, add, or mutate an effect?
- Does grouping interact with replay identity?
- Does grouping interact with failure atomicity?
- Does grouping interact with canonical ordering?

No grouping mechanism is selected here.

---

## 12. Authorization Scope

Authorization count cannot be interpreted without authorization scope.

A future rule must eventually determine what an accepted authorization permits.

Possible analytical scope dimensions include:

- one effect;
- several effects;
- one logical source;
- several logical sources;
- one transaction;
- a subset of a transaction;
- a state change;
- a credential-management action; or
- another future authoritative scope.

These are examples only.

This document does not select a scope model.

---

## 13. Evidence Reuse

Evidence reuse must not be assumed merely to reduce apparent PQ cost.

A comparison that permits evidence reuse must declare:

- what semantic requirement is reused;
- over which effects or sources reuse applies;
- whether replay rules permit that reuse;
- whether historical interpretation changes reuse;
- whether version changes invalidate reuse;
- whether the same artifact is actually transmitted again;
- whether verification work is actually repeated;
- whether implementation caching is involved; and
- whether the claimed reuse is a protocol property or an implementation
  optimization.

Implementation-local caching is not automatically a consensus-visible
authorization feature.

---

## 14. Key, Credential, Condition, and Evidence Reuse Are Distinct

The phrase "key reuse" is insufficiently precise for a decision-ready
comparison.

The following possibilities are analytically distinct:

- public-key reuse;
- credential reuse;
- ownership-condition reuse;
- authorization-condition reuse;
- authorization-evidence reuse;
- signature reuse;
- proof reuse;
- verification-result reuse; and
- implementation cache reuse.

A future comparison must name the actual kind of reuse being assumed.

This document neither requires nor prohibits cryptographic key reuse.

Whether a particular reuse pattern is safe depends on the future algorithm,
authorization rules, signing semantics, replay rules, and security analysis.

---

## 15. Authorization Artifact Classes

A comparison may need to distinguish candidate artifact classes such as:

- public verification material;
- credential descriptors;
- authorization conditions;
- signatures;
- proofs;
- certificates;
- version markers;
- algorithm identifiers;
- grouping metadata;
- threshold-related material if applicable;
- aggregation-related material if applicable; and
- other candidate authorization evidence.

The list is analytical and non-exhaustive.

No listed artifact is required merely because it appears here.

---

## 16. Artifact Count and Artifact Size Are Different

Artifact count and byte size must not be conflated.

A candidate may have:

- fewer but larger artifacts;
- more but smaller artifacts;
- shared verification material;
- duplicated verification material;
- variable-size evidence;
- fixed-size evidence;
- aggregated evidence;
- non-aggregated evidence; or
- another future structure.

Any bandwidth or storage comparison must report enough information to distinguish
those cases.

---

## 17. Exact Byte Sizes Require a Frozen Experimental Profile

This document defines no exact key, signature, proof, credential, or
authorization-artifact size.

Any future claim involving exact bytes must be bound to a reviewed experimental
profile that identifies, where material:

- the cryptographic algorithm;
- relevant parameters;
- algorithm version;
- artifact class;
- experimental representation or encoding;
- candidate mapping;
- authorization assumptions;
- transaction or workload shape;
- feature branches;
- implementation version where implementation measurements are used; and
- applicable provenance.

An experimental encoding does not become the protocol encoding merely because it
was benchmarked.

---

## 18. Verification-Operation Count

Cryptographic verification-operation count is not automatically equal to:

- authorizer count;
- credential count;
- source count;
- input count;
- signature count;
- proof count; or
- artifact count.

A future algorithm or architecture may permit one artifact to require several
logical verification operations, several artifacts to share work, or another
relationship.

The comparison must therefore record verification cardinality separately when
material.

This document does not define a protocol cryptographic-work unit.

---

## 19. Cryptographic Algorithm Profiles

A non-normative algorithm profile may be used for comparison evidence without
selecting that algorithm for Dilithia.

A material profile may need to record:

- algorithm or family;
- parameter set;
- key sizes;
- evidence sizes;
- verification behavior;
- signing behavior where relevant to a non-consensus claim;
- malformed-input behavior;
- scratch-memory exposure;
- aggregation support;
- batching support;
- deterministic or randomized evidence properties;
- version coexistence assumptions; and
- implementation version for measured claims.

If a material algorithm dimension is unbound, a claim depending on that
dimension is not decision-ready.

---

## 20. No Algorithm Is Selected by This Gate

This document selects no:

- ML-DSA parameter set;
- SLH-DSA parameter set;
- FN-DSA parameter set;
- hash-based signature parameter;
- lattice-based signature parameter;
- signature primitive;
- KEM primitive;
- proof primitive;
- algorithm identifier;
- registry structure;
- algorithm coexistence rule;
- algorithm activation rule; or
- algorithm retirement rule.

The appearance of a named algorithm in a later experimental profile must not be
interpreted as protocol adoption.

---

## 21. Aggregation and Batching Are Distinct

Aggregation and batching must not be used as interchangeable terms.

For analytical purposes:

- aggregation may refer to a capability that changes how several cryptographic
  artifacts or statements are represented or combined; while
- batching may refer to a capability that changes how several verification tasks
  are processed.

A future algorithm may support:

- neither;
- aggregation only;
- batching only;
- both; or
- another relevant optimization.

This document selects no aggregation or batching mechanism.

---

## 22. Optional Cryptographic Features Require Explicit Branches

Aggregation, batching, proof reuse, threshold features, or other optional
cryptographic capabilities may be evaluated only when the declared algorithm or
architecture actually supports them.

Feature-enabled and feature-disabled cases are separate comparison branches when
the distinction materially affects results.

A feature must not be forced onto an algorithm that lacks it.

Both state-model candidates receive symmetric permission to use a feature under
the same external branch assumptions.

Symmetric permission does not require equal internal benefit.

If one candidate obtains greater benefit from the same legitimately available
feature, that difference may be valid evidence.

---

## 23. Multisignature and Threshold Authorization Boundary

Native multisignature is not currently established as a Genesis requirement.

Threshold authorization is also not selected.

This document does not select:

- k-of-m rules;
- native multisig;
- threshold signatures;
- threshold proofs;
- distributed key generation;
- signer ordering;
- signer registries;
- participant identifiers;
- alternate authority;
- backup credentials;
- recovery credentials; or
- delegation.

If a future external requirement genuinely requires several independent
authorities, the comparison must first express that semantic requirement without
silently choosing one of these mechanisms.

---

## 24. Mixed Cryptographic Versions

Crypto Agility may eventually create states or transitions in which more than
one cryptographic or authorization version is relevant.

Comparison assumptions may therefore need to distinguish:

- one current version;
- several current accepted versions;
- historical-only versions;
- deprecated versions;
- unknown versions;
- unsupported versions;
- migration-period coexistence; and
- candidate-specific metadata implications.

No coexistence mechanism or version field is selected here.

---

## 25. Migration and Deprecation

Key rotation, algorithm migration, emergency deprecation, and lost-credential
recovery are distinct analytical cases.

This document does not collapse them into one generic "key change."

A comparison claim depending on migration must state relevant assumptions such
as:

- whether current credentials remain trustworthy;
- whether current credentials remain available;
- whether owner participation is assumed;
- whether multiple versions coexist;
- whether old evidence remains historically interpretable;
- whether current acceptance of an old version continues; and
- whether an alternate pre-authorized authority exists.

The existence of these variables does not select a migration or recovery
mechanism.

---

## 26. Historical Interpretation and Current Acceptance

Historical interpretation of previously canonical authorization evidence is
distinct from current acceptance of that evidence for a new transition.

A cryptographic algorithm may later become:

- historically interpretable;
- no longer accepted for new transitions;
- deprecated;
- migrated away from; or
- otherwise governed by future authoritative rules.

This document selects none of those policies.

PQ comparison must not assume perpetual current acceptance of an old primitive
merely because old canonical history remains interpretable.

---

## 27. Domain and Signing-Context Boundary

Authorization-artifact comparison cannot silently invent signing bytes.

Existing authoritative and decision-readiness boundaries include
purpose-specific domain separation and the current NetworkId requirement for
signed structures.

However, the following remain unresolved:

- complete signing bytes;
- transaction or effect identity;
- ChainId binding;
- authorization-version binding;
- referenced-state binding;
- ownership-condition binding;
- resource or economic declaration binding;
- exact domain-tag registry; and
- any additional validity-affecting context.

This document therefore defines no signing-message format.

---

## 28. Transaction Shape Remains a Separate Dimension

PQ cost may vary materially with transaction shape.

Relevant analytical variables may include:

- external semantic effect count;
- number of logical funding sources;
- number of destination effects;
- number of independent authority relations;
- candidate-internal source count;
- grouping opportunities;
- credential multiplicity;
- version diversity;
- artifact count;
- artifact size; and
- verification operations.

A state-model comparison must not attribute transaction-shape differences to the
state-model label unless the mapping itself causes those differences under the
same external semantic case.

No transaction format is selected.

---

## 29. Minimal Account Candidate Questions

Minimal Account remains a candidate only.

Questions include:

- What externally required authority relations apply to the case?
- Does one potential account-related effect require one or several independent
  authorizers?
- Can several logical sources exist within the relevant comparison case without
  changing external authority requirements?
- Could one authorization cover several account-related effects if future rules
  permit it?
- Could several credentials satisfy one authority relation?
- Could several independent authorizers govern one account-related effect if
  such capability is ever required?
- Where, if anywhere, would authorization-related metadata reside?
- Could authorization metadata be shared or duplicated?
- How would mixed authorization versions affect candidate bytes or verification
  work?
- How would migration affect dormant state?
- How would invalid authorization fail?
- What candidate artifact counts result under each frozen profile?
- What candidate verification-operation counts result?
- Which counts are mapping outputs rather than imposed common assumptions?

These questions do not select an Account authorization architecture.

---

## 30. Minimal UTXO Candidate Questions

Minimal UTXO remains a candidate only.

Questions include:

- What externally required authority relations apply to the case?
- Is independent-authorizer count related to input count, and if so, under what
  future rule?
- Could one authority relation cover several candidate inputs?
- Could one authorization artifact cover several candidate inputs if future
  transaction and signing rules permit it?
- Could several credentials satisfy one ownership condition?
- Could several independent authority relations govern one candidate effect if
  required?
- Is authorization metadata duplicated across candidate value sources, shared
  indirectly, or represented another way?
- How would mixed authorization versions affect candidate bytes or verification
  work?
- How would old ownership conditions remain historically interpretable?
- How would invalid authorization fail?
- What candidate artifact counts result under each frozen profile?
- What candidate verification-operation counts result?
- Which counts are mapping outputs rather than imposed common assumptions?

These questions do not select input structure, output structure, output identity,
consumption semantics, or a UTXO authorization architecture.

---

## 31. Account and UTXO Neutrality

Minimal Account and Minimal UTXO remain co-equal candidates.

The comparison must not assume:

- Account requires fewer PQ signatures;
- UTXO requires more PQ signatures;
- Account has one authorization domain;
- UTXO has one authorization domain per input;
- Account naturally enables key reuse;
- UTXO naturally prevents key reuse;
- Account naturally enables aggregation;
- UTXO naturally benefits more from batching;
- Account necessarily duplicates less credential metadata;
- UTXO necessarily duplicates more credential metadata;
- Account is inherently cheaper to migrate;
- UTXO is inherently easier to migrate;
- either candidate has lower PQ bandwidth;
- either candidate has lower verification work; or
- either candidate is inherently more quantum-resistant.

Those are comparison hypotheses until supported by frozen assumptions and
evidence.

---

## 32. Common Parameters, Profile Parameters, and Candidate Outputs

Comparison must distinguish at least three roles.

### Common semantic parameters

Possible examples include:

- externally justified authority-relation count `N`;
- semantic effect count;
- repetition count; and
- externally defined adversarial population.

### Profile parameters

Possible examples include:

- credential multiplicity;
- algorithm profile;
- cryptographic-version distribution;
- feature-enabled or feature-disabled state;
- migration assumptions;
- authority-participation assumptions; and
- authorization-scope assumptions.

### Candidate outputs

Possible examples include:

- internal source count;
- credential placement;
- signature count;
- proof count;
- authorization-artifact count;
- exact bytes under a frozen experimental encoding;
- verification-operation count;
- state reads;
- state writes;
- persistent metadata;
- failure-stage position;
- temporary memory;
- measured performance; and
- other mapping-specific consequences.

A candidate output must not be silently converted into a common input merely to
equalize candidates.

---

## 33. Equivalent External Requirements

A fair paired comparison applies the same external semantic requirement to both
candidate mappings.

Equivalent external requirements do not require identical internal
representation.

For example, both candidates may be required to satisfy the same `N`
independent authority relations while legitimately producing different:

- credential counts;
- signature counts;
- proof counts;
- artifact bytes;
- verification-operation counts;
- state accesses; or
- metadata placement.

Those differences may be exactly what the comparison is intended to measure.

---

## 34. Symmetric Feature Availability

Where an optional cryptographic feature is part of a comparison branch:

- both candidates must be allowed to use it when semantically applicable;
- neither candidate is required to use it if doing so is not applicable;
- the feature must actually be available under the declared algorithm profile;
- feature-enabled and feature-disabled branches must not be mixed silently;
- branch-specific results must remain separately identifiable; and
- unequal benefit is not evidence of unfairness when external availability was
  symmetric.

Symmetric opportunity is required.

Identical internal use is not.

---

## 35. Adversarial Scenario Matrix

| Scenario | Required analytical property |
|---|---|
| One ordinary native-DLTH authorization | Same external authority requirement is applied to both candidates |
| One authority relation over several logical sources | Source count is not silently converted into authorizer or signature count |
| Several independent authority relations | Every required authority is represented without selecting a multisig mechanism |
| One authority relation with several credentials | Credential multiplicity remains distinct from authority count |
| Several candidate sources share one future authorization condition | Grouping remains an explicit branch rather than a state-model assumption |
| One candidate internally needs more artifacts | Difference remains a candidate output under identical external assumptions |
| One candidate internally needs fewer artifacts | Difference remains a candidate output rather than being normalized away |
| Algorithm feature disabled | Neither candidate receives undeclared aggregation or batching |
| Algorithm feature enabled | Both candidates receive symmetric opportunity to use the feature |
| Mixed cryptographic versions | Version diversity is explicit and cannot cause silent downgrade or reinterpretation |
| Migration between cryptographic eras | Historical meaning and current acceptance remain distinct |
| Unknown authorization version | Failure is deterministic and cannot escalate authority |
| Unsupported algorithm identifier | Failure classification is deterministic under a future specified grammar |
| Malformed authorization artifact | Rejection is bounded and failure-atomic |
| Cryptographically invalid but syntactically valid evidence | Semantic cryptographic failure is not conflated with byte malformation |
| Many expensive invalid authorizations | Hostile attempted work remains bounded under later resource rules |
| Repeated authorization evidence | Reuse, replay, and repeated verification are not conflated |
| Late authorization failure | No partial canonical effect remains |
| State lookup before authorization failure | Candidate work exposure is measured without inventing a resource unit |
| Large PQ evidence profile | Byte claims are tied to the declared algorithm and encoding profile |
| Variable or randomized valid evidence | Candidate identity and replay analysis do not assume byte uniqueness |
| Malformed aggregate or batch | Fallback or failure behavior remains explicit if the selected experimental feature supports such a case |
| Dormant state during algorithm migration | Migration assumptions are stated rather than inferred |
| One potential Account effect with several authorizers | Multi-authorizer behavior remains conditional rather than selected |
| One potential authorization covering several UTXO inputs | Grouping remains conditional rather than selected |
| Cross-network authorization reuse attempt | Existing NetworkId and domain requirements are preserved |
| Cross-purpose artifact reuse attempt | Purpose separation is preserved |
| Very large logical source count with `N = 1` | Source cardinality is not confused with authority cardinality |
| Small source count with `N > 1` | Authority cardinality is not inferred from source cardinality |
| Persistent authorization metadata growth | Later resource/state analysis receives an explicit dependency without selecting economics |

The matrix creates analytical cases only.

It does not establish that any candidate mechanism exists.

---

## 36. Malformed and Invalid Evidence Boundary

The following conditions must not be silently conflated when future experimental
profiles are sufficiently defined to distinguish them:

- malformed bytes;
- non-canonical bytes;
- unsupported algorithm version;
- unknown authorization version;
- structurally valid but cryptographically invalid evidence;
- valid cryptographic evidence with insufficient authorization scope;
- valid evidence for the wrong purpose;
- valid evidence for the wrong network;
- historical evidence not currently accepted; and
- other future authoritative failure classes.

This document defines no concrete grammar.

Until an appropriate experimental or authoritative grammar exists, exact
malformed-byte experiments remain deferred.

---

## 37. Hostile Verification Boundary

PQ authorization comparison must account for hostile verification exposure.

Relevant cases include:

- many invalid signatures;
- many invalid proofs;
- malformed authorization evidence;
- unknown versions;
- unsupported algorithms;
- mixed algorithms;
- repeated evidence;
- late cryptographic failure;
- state lookup before authorization failure;
- large artifacts;
- pathological feature combinations; and
- malformed aggregation or batching paths where such features exist.

This document defines no resource meter or numeric limit.

It only requires that a comparison not treat hostile work as free or derive
candidate superiority from an unbounded validation path.

---

## 38. Cryptographic Resource Accounting Is a Later Boundary

Future cryptographic resource accounting must eventually be typed and
algorithm/version-aware.

It must not assume all signatures, proofs, hashes, parsing operations, or
authorization checks have equal cost.

However, this sixth-gate document selects no:

- cryptographic resource unit;
- operation taxonomy;
- counter;
- numeric bound;
- transaction resource limit;
- block resource limit;
- malformed-input charge;
- batch discount;
- aggregation discount;
- fee weight;
- fee conversion; or
- hardware profile.

Those belong to later cryptographic, transaction, block, and resource design.

---

## 39. Persistent Authorization Metadata Boundary

PQ and authorization architecture may affect persistent state through:

- credential material;
- ownership conditions;
- version markers;
- migration metadata;
- rotation metadata if ever adopted;
- indirect authorization references;
- historical interpretation requirements; or
- other future state.

Those are exposure questions, not selected state fields.

This document does not select:

- persistent credential storage;
- account-scoped authorization state;
- output-scoped authorization state;
- a credential registry;
- a rotation registry;
- tombstones;
- metadata retention;
- pruning;
- state rent;
- cleanup incentives; or
- storage fees.

Persistent-growth evaluation remains part of the later resource gate.

---

## 40. Conflict and Ordering Boundary

Authorization assumptions may expose later conflict questions.

Examples include:

- simultaneous credential-management operations;
- concurrent authorization changes;
- several candidates using the same authority relation;
- competing migration actions;
- repeated use of an authorization artifact;
- conflicts involving grouped authorization; and
- ordering effects during version transitions.

The sixth gate does not select:

- conflict keys;
- canonical winner rules;
- proposer ordering;
- first-seen behavior;
- rejection priority;
- scheduling graphs;
- parallel execution rules; or
- serial ordering.

Those belong to the seventh state-model gate.

---

## 41. Commitment and Snapshot Boundary

Authorization artifacts or ownership metadata may eventually become subjects of
authenticated-state proofs, snapshots, or synchronization.

The sixth gate does not select:

- a state tree;
- trie;
- accumulator;
- hash construction;
- state root;
- membership proof;
- absence proof;
- witness format;
- snapshot format;
- checkpoint;
- light-client construction; or
- trust model.

Those belong to the authenticated-state gate.

---

## 42. Evidence Required Before PQ Comparison Claims

A PQ-sensitive Account/UTXO comparison claim must freeze enough assumptions to
make the claim reproducible.

Depending on the claim, this may include:

- external semantic case;
- authority-relation count;
- authorization scope;
- logical source count;
- credential multiplicity;
- authorization grouping assumptions;
- evidence-reuse assumptions;
- key or condition reuse assumptions where material;
- algorithm profile;
- algorithm version;
- feature branches;
- migration or coexistence profile;
- candidate mapping;
- transaction/workload shape;
- artifact classes measured;
- experimental encoding where exact bytes are claimed;
- verification-operation interpretation;
- malformed/invalid evidence profile where relevant;
- implementation version for measured results;
- resource-accounting branch where attempted work is claimed;
- provenance;
- failures and deviations; and
- independent reproduction appropriate to the evidence claim.

A claim lacking a material assumption is not made valid by averaging over the
missing dimension.

---

## 43. Benchmark and Measurement Boundary

Benchmark results are evidence, not consensus semantics.

Measurements may include:

- encoded bytes under a frozen experimental representation;
- parse cost;
- verification cost;
- temporary memory;
- network transfer;
- state access;
- metadata duplication;
- aggregate transaction cost;
- invalid-candidate cost; and
- implementation performance.

Results must remain attributable to:

- candidate mapping;
- workload;
- algorithm profile;
- authorization profile;
- feature branch;
- implementation; and
- run identity where applicable.

Hardware-specific timing must not become a consensus authorization rule.

An observed implementation speedup from SIMD, threads, caching, batching,
allocator behavior, or database layout is not automatically a protocol property.

---

## 44. Formal-Verification Obligations

Future formal or mechanized evidence may need to prove model-independent
properties such as:

- every required authority relation is satisfied;
- no unauthorized effect becomes canonical;
- authorization scope cannot silently expand;
- grouping does not omit or add protected effects;
- failed authorization leaves no partial canonical effect;
- later failure leaves no partial canonical effect;
- replay protections remain valid under the adopted authorization semantics;
- network and purpose domains cannot be bypassed;
- unknown versions fail deterministically;
- migration does not silently reinterpret authority;
- historical interpretation remains deterministic;
- candidate artifact accounting matches the declared experimental profile; and
- optional cryptographic features do not alter protected semantic outcomes.

One possible non-normative analytical form is:

`Authorize(profile, authority_requirements, candidate_context, evidence) -> result`

This notation is illustrative only.

It does not define:

- a protocol function;
- transaction API;
- Account API;
- UTXO API;
- credential representation;
- state representation;
- signing format;
- verification interface; or
- implementation architecture.

---

## 45. Premature-Commitment Matrix

| Classification | Item | Reason |
|---|---|---|
| SAFE TO RECORD NOW | Account does not imply one authorizer | Prevents label-derived PQ assumptions |
| SAFE TO RECORD NOW | UTXO input count does not imply signature count | Prevents label-derived PQ assumptions |
| SAFE TO RECORD NOW | Owner, credential, artifact, and verification counts are distinct | Required for fair comparison |
| SAFE TO RECORD NOW | Algorithm, authorization architecture, and state-model properties must be separated | Prevents category errors |
| SAFE TO RECORD NOW | Exact-byte claims require a declared experimental profile | Makes evidence attributable |
| SAFE TO RECORD NOW | Symmetric feature availability does not require identical candidate benefit | Preserves fair comparison |
| KEEP AS PROFILE BRANCH | Credential multiplicity | Architecture dependent |
| KEEP AS PROFILE BRANCH | Native support for `N > 1` | Not currently selected |
| KEEP AS PROFILE BRANCH | Authorization grouping | Transaction and ownership architecture dependent |
| KEEP AS PROFILE BRANCH | Evidence reuse | Replay and authorization semantics dependent |
| KEEP AS PROFILE BRANCH | Cryptographic-version coexistence | Crypto Agility dependent |
| KEEP AS PROFILE BRANCH | Aggregation | Algorithm and architecture dependent |
| KEEP AS PROFILE BRANCH | Batching | Algorithm and implementation/protocol composition dependent |
| KEEP AS PROFILE BRANCH | Threshold behavior | Not a current Genesis requirement |
| BLOCKED | Concrete credential format | Authorization architecture unresolved |
| BLOCKED | Concrete signing bytes | Transaction/effect context unresolved |
| BLOCKED | Concrete algorithm registry | Crypto Agility design unresolved |
| BLOCKED | Concrete resource units and limits | Later resource design |
| BLOCKED | Concrete persistent authorization schema | State model and authorization architecture unresolved |
| DO NOT ADOPT | Account = one PQ signature as a model rule | Unsupported state-model stereotype |
| DO NOT ADOPT | UTXO = one PQ signature per input as a model rule | Unsupported state-model stereotype |
| DO NOT ADOPT | Owner = credential = signature = verification operation | Collapses distinct analytical dimensions |
| DO NOT ADOPT | Benchmark timing as consensus authorization semantics | Implementation dependent |
| DO NOT ADOPT | Privileged emergency authorization key | Violates constitutional authority boundary |
| DO NOT ADOPT | Candidate ranking based on an undeclared algorithm or authorization profile | Evidence is not decision-ready |

"DO NOT ADOPT" in this table applies to the listed proposition as stated.

It does not forbid a future authoritative mechanism merely because related words
appear in the proposition.

---

## 46. Sixth-Gate Decision Gates

Before the sixth state-model gate can be treated as sufficiently clarified for a
specific Account/UTXO comparison claim, that claim must have reviewed answers or
explicitly frozen assumptions for every material item below.

1. What externally meaningful authority-relation count applies?
2. Is `N` distinguished from human, owner, credential, signature, proof, and
   verification-operation counts?
3. What logical source count applies?
4. What credential multiplicity applies?
5. What authorization scope applies?
6. What grouping capability is assumed or prohibited in the comparison profile?
7. What evidence-reuse or verification-result-reuse assumption applies?
8. What key, credential, ownership-condition, authorization-condition, or
   implementation-cache reuse assumption is material, and where implementation
   caching is involved, is it explicitly distinguished from protocol-visible
   reuse?
9. What cryptographic algorithm profile is used for the evidence claim?
10. What algorithm/version coexistence assumptions apply?
11. Which artifact classes are being counted or measured?
12. Which artifact counts are external requirements and which are candidate
    outputs?
13. If byte size is claimed, what experimental encoding and parameter profile are
    frozen?
14. How is cryptographic verification-operation count distinguished from artifact
    count?
15. Are aggregation, batching, threshold behavior, or other optional features
    relevant, and if so, which explicit branch is being evaluated?
16. Are both candidates given symmetric opportunity to use applicable optional
    features?
17. What representative transaction or workload shapes are used?
18. What adversarial authorization shapes are used?
19. How are malformed, unsupported, unknown-version, and cryptographically invalid
    evidence distinguished where the experimental grammar allows?
20. What migration, deprecation, and historical-interpretation assumptions affect
    the claim?
21. Which candidate-specific authorization metadata or state consequences remain
    outputs?
22. Which hostile-work questions are deferred to the resource gate?
23. Which conflict/order questions are deferred to the seventh gate?
24. Which commitment/snapshot questions are deferred to the eighth gate?
25. Is the resulting evidence sufficient to prevent state-model labels from
    substituting for actual authorization and PQ assumptions?

The existence, completion, or independent review of this document automatically
satisfies none of these items.

The required answers may differ by comparison campaign or branch where the
external semantic contract genuinely differs.

A full protocol implementation is not required to clarify the gate.

Enough reviewed abstract semantics and frozen comparison assumptions are
required to avoid unstated PQ or authorization assumptions.

---

## 47. Formal Specification Boundary

Future authoritative specification may eventually need to define, depending on
the selected architecture:

- accepted authorization evidence;
- ownership or control semantics;
- authorization scope;
- authorization version interpretation;
- canonical authorization encoding;
- signing context;
- domain and replay binding;
- current and historical cryptographic-version interpretation;
- cryptographic algorithm registry;
- activation and deprecation behavior;
- migration behavior;
- algorithm coexistence;
- failure semantics;
- conflict behavior;
- validity-affecting structural bounds;
- cryptographic resource accounting; and
- other architecture-dependent rules.

This document does not itself require the Formal Specification to adopt:

- native multisig;
- threshold authorization;
- aggregation;
- batching;
- key reuse;
- credential rotation;
- recovery;
- delegation;
- alternate authority;
- stable logical identity;
- persistent authorization state;
- one-authorizer-per-account semantics;
- one-authorizer-per-input semantics; or
- a particular PQ primitive.

Formal Specification changes require the appropriate authoritative process.

---

## 48. Threat Model Boundary

Existing threat analysis already includes relevant generic concerns such as:

- future cryptographic primitive breakage;
- quantum-era cryptographic threats;
- key compromise;
- dormant-state or dormant-value exposure;
- cryptographic resource exhaustion;
- malformed or adversarial verification work;
- persistent-state growth;
- failure atomicity;
- version drift;
- migration ambiguity; and
- implementation-dependent behavior where it could affect consensus.

This document may expose candidate-specific questions involving:

- artifact amplification;
- credential duplication;
- verification amplification;
- malformed aggregation or batching paths;
- mixed-version downgrade or confusion;
- authorization-scope expansion;
- grouping ambiguity;
- stale credential acceptance;
- migration ambiguity;
- dormant authorization metadata;
- repeated evidence; and
- candidate-specific late-failure exposure.

The existence of this document does not itself justify a Threat Model update.

A Threat Model update should follow if:

- a selected authorization or cryptographic architecture creates a concrete new
  model-specific threat; or
- analysis identifies a genuinely new generic threat class not already covered.

---

## 49. Complete TBD Register

The following remain unresolved unless separately established by higher
authority.

### State-model and transaction

- Account versus UTXO selection;
- generalized object model;
- active multiple-native-value-model Hybrid;
- transaction format;
- input structure;
- output structure;
- transaction identity;
- effect identity;
- candidate source representation; and
- state representation.

### Ownership and authorization

- ownership representation;
- authorization architecture;
- independent-authorizer requirements;
- native support for `N > 1`;
- authorization grouping;
- authorization scope;
- credential representation;
- credential multiplicity policy;
- credential placement;
- credential lifecycle;
- evidence grouping;
- evidence reuse;
- authorization descriptor;
- stable identity;
- address role;
- recovery;
- delegation;
- alternate authority;
- key rotation; and
- persistent authorization state.

### Cryptography

- exact PQ algorithm;
- parameter sets;
- signature primitive;
- proof primitive;
- KEM primitive where relevant;
- key format;
- signature format;
- proof format;
- algorithm identifiers;
- registry structure;
- version fields;
- coexistence rules;
- aggregation;
- batching;
- threshold cryptography;
- multisignature mechanism;
- key-reuse policy;
- algorithm activation;
- algorithm deprecation;
- emergency cryptographic response; and
- migration mechanism.

### Signing and replay

- complete signing bytes;
- ChainId representation and binding;
- exact NetworkId discriminants;
- exact domain-tag registry;
- authorization-version binding;
- referenced-state binding;
- transaction/effect identity binding;
- replay mechanism; and
- candidate-specific malleability rules.

### Artifacts and evidence

- exact artifact classes used by the eventual protocol;
- exact artifact counts;
- exact protocol byte sizes;
- protocol serialization of authorization material;
- verification-operation taxonomy;
- whether artifact counts are protocol-declared;
- whether verification work is protocol-declared;
- batch fallback semantics;
- aggregate failure semantics; and
- exact malformed-input grammar.

### Resources and economics

- cryptographic work units;
- resource counters;
- numeric limits;
- candidate-attempt accounting;
- transaction limits;
- block limits;
- batch discounts;
- aggregation discounts;
- state-growth units;
- fee weights;
- fee conversion;
- state rent;
- refunds; and
- cleanup economics.

### Later state-model gates

- canonical conflict rules;
- canonical ordering;
- scheduling equivalence;
- parallel-execution semantics;
- authenticated-state commitment;
- membership proofs;
- absence proofs;
- snapshot semantics;
- light-client requirements;
- logical state-access units;
- mutation units;
- persistent-growth accounting; and
- invalid-candidate resource requirements.

No item in this register is resolved merely by appearing in this document.

---

## 50. Current Project Impact

Creation and review of this document alone has the following effect:

- State model selected: **NO**
- Account selected: **NO**
- UTXO selected: **NO**
- Generalized Object selected: **NO**
- Hybrid selected: **NO**
- Ownership representation selected: **NO**
- Authorization architecture selected: **NO**
- Independent-authorizer count fixed globally: **NO**
- `N = 1` fixed globally: **NO**
- Native `N > 1` support selected: **NO**
- One-authorizer-per-account semantics selected: **NO**
- One-authorizer-per-input semantics selected: **NO**
- Credential multiplicity selected: **NO**
- Credential format selected: **NO**
- Credential placement selected: **NO**
- Authorization grouping selected: **NO**
- Authorization scope mechanism selected: **NO**
- Evidence-reuse mechanism selected: **NO**
- Key-reuse policy selected: **NO**
- Stable logical identity selected: **NO**
- Recovery selected: **NO**
- Delegation selected: **NO**
- Alternate authority selected: **NO**
- Native multisig selected: **NO**
- Threshold authorization selected: **NO**
- PQ algorithm selected: **NO**
- PQ parameter set selected: **NO**
- Signature format selected: **NO**
- Proof format selected: **NO**
- Algorithm registry selected: **NO**
- Crypto-version coexistence mechanism selected: **NO**
- Aggregation selected: **NO**
- Batching selected: **NO**
- Signing bytes selected: **NO**
- Transaction format selected: **NO**
- Replay mechanism selected: **NO**
- State commitment selected: **NO**
- Snapshot mechanism selected: **NO**
- Conflict rule selected: **NO**
- Canonical ordering selected: **NO**
- Parallel scheduling selected: **NO**
- Cryptographic resource unit selected: **NO**
- Resource meter selected: **NO**
- Numeric resource limit selected: **NO**
- Fee mechanism selected: **NO**
- State rent selected: **NO**
- Candidate ranking justified merely by this document: **NO**
- Formal Specification update justified merely by this document: **NO**
- Threat Model update justified merely by this document: **NO**
- Consensus implementation change caused by this document: **NONE**

State-model decision remains **NOT MADE**.

---

## 51. Next Decision Boundary

Completion and independent review of this document may clarify the sixth
state-model decision gate:

> PQ authorization-count and artifact assumptions sufficient for comparison.

The existence, completion, or review of this document does not by itself satisfy
the sixth state-model decision gate.

It also does not satisfy the other remaining state-model gates.

The next model-neutral analytical area is:

> Canonical conflict, ordering, and scheduling-equivalence requirements.

That analysis must not use this document to silently select:

- Account;
- UTXO;
- an authorization architecture;
- a conflict-key representation;
- proposer-first ordering;
- first-seen ordering;
- a canonical sorting rule;
- a transaction format;
- a parallel execution engine;
- a scheduling graph;
- a rollback mechanism;
- a state commitment;
- a resource meter; or
- a state-model ranking.

---

## 52. Conclusion

- This document is non-normative.
- The Formal Specification remains authoritative for protocol behavior.
- Dilithia's post-quantum Genesis direction does not select a specific
  cryptographic primitive in this document.
- No PQ algorithm or parameter set is selected.
- No authorization architecture is selected.
- Account does not imply one authorizer.
- UTXO input count does not imply signature count.
- Owner count, authority-relation count, credential count, source count,
  signature count, proof count, artifact count, and verification-operation count
  are distinct analytical dimensions.
- `N` is an authority-relation comparison variable where externally justified,
  not a synonym for credentials, signatures, or inputs.
- Native support for `N > 1` remains an architecture branch.
- Credential multiplicity remains a profile variable.
- Authorization grouping remains unselected.
- Authorization scope remains unselected.
- Evidence reuse remains unselected.
- Key reuse remains neither required nor prohibited by this document.
- Artifact count and artifact size remain distinct.
- Exact byte claims require a declared experimental algorithm and representation
  profile.
- Algorithm properties, authorization-architecture properties, and state-model
  properties must remain distinguishable.
- Aggregation remains optional and unselected.
- Batching remains optional and unselected.
- Native multisig is not established as a Genesis requirement.
- Threshold authorization remains unselected.
- Feature-enabled and feature-disabled comparison branches must remain explicit
  when the feature materially affects results.
- Symmetric feature availability does not require identical internal candidate
  use or identical benefit.
- Candidate-specific credentials, signatures, proofs, bytes, and verification
  counts may remain outputs under equivalent external semantic requirements.
- Mixed cryptographic-version assumptions must be explicit where material.
- Historical interpretation remains distinct from current cryptographic
  acceptance.
- Migration, deprecation, recovery, and rotation remain distinct analytical
  cases.
- Complete signing bytes remain unresolved.
- Replay mechanism remains unselected.
- Transaction format remains unselected.
- Cryptographic resource accounting remains a later design boundary.
- Conflict and ordering semantics remain unresolved.
- Parallel scheduling remains unresolved.
- Commitment, snapshot, and light-client requirements remain unresolved.
- Minimal Account and Minimal UTXO remain co-equal candidates.
- No PQ bandwidth winner is established.
- No PQ verification-cost winner is established.
- Candidate ranking remains blocked.
- State-model decision remains **NOT MADE**.
- The sixth state-model decision gate is not satisfied merely by this document
  existing, being completed, or being reviewed.
- This document defines no consensus rule.