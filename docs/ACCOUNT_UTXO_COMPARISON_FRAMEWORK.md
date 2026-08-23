# Dilithia Account/UTXO Comparison Framework

> **NON-NORMATIVE DESIGN AND EVIDENCE FRAMEWORK**
>
> This document defines a method for comparing two analytical native-value-
> accounting hypotheses. It does not define protocol behavior, select a state
> model, or authorize protocol adoption. If it conflicts with the Dilithia
> Constitution, Formal Specification, or a ratified HIP/Super HIP, the
> authoritative protocol documents take precedence.

## 1. Status, Authority, and Scope

This document is non-normative. Terms such as **must**, **required**, and
**prohibited** describe controls for a credible comparison under this framework;
they are not current consensus rules.

The authority order is:

1. `CONSTITUTION.md`
2. `SPECIFICATION.md`
3. Ratified HIP / Super HIP documents
4. Normatively adopted conformance material, where applicable
5. Implementation
6. `PROJECT_STATE.md`
7. This comparison framework and other non-normative design or evidence documents
8. AI or other design discussions

The scope is a future evidence-based comparison of two co-equal analytical
hypotheses for native-value accounting. The framework defines comparison
controls, workload semantics, evidence categories, branch handling, and ranking
readiness. It defines no protocol object, wire format, transition format,
authorization rule, replay rule, commitment, migration procedure, resource
limit, or activation process.

## 2. Purpose and Decision State

The purpose is to make future Account/UTXO comparison work reproducible,
symmetric, resistant to strawman implementations, and explicit about unresolved
dependencies.

The repository does not currently contain enough authoritative protocol detail
to rank the hypotheses. Comparison work may begin using declared experimental
assumptions, but conclusions remain conditional on those assumptions and may not
be represented as protocol direction.

This framework therefore supports:

- construction of equivalent external workload contracts;
- exploration of multiple defensible candidate mappings;
- correctness, security, resource, and implementation-independence evidence;
- sensitivity analysis across unresolved architecture branches; and
- identification of evidence still required before a final ranking.

## 3. Analytical Candidate Hypotheses

The labels in this section are non-normative native-value-accounting hypotheses
only.

**Account hypothesis:** native value is analyzed as quantities associated with
persistent logical value positions, with a value-affecting effect represented
analytically as changes to those quantities or their availability.

**UTXO hypothesis:** native value is analyzed as a collection of discrete
value-bearing logical records, with a value-affecting effect represented
analytically as a relation between a before-set and an after-set of those
records.

These minimal descriptions exist only to identify different value-accounting
shapes for comparison. They do not specify storage structures, protocol fields,
record identity, record lifecycle, or effect syntax. Neither hypothesis is
ranked or preferred.

## 4. Prohibited Inferences from Candidate Labels

The words **Account** and **UTXO** do not, by themselves, define or imply:

- ownership, identity, credentials, authorization coverage, or authority count;
- mutable control state or any relationship between control and value state;
- one-use identity, consumption mechanics, or record lifecycle;
- replay protection, counters, nonces, or uniqueness mechanisms;
- transaction inputs, transaction outputs, transaction identity, or a
  transaction/state-machine format;
- cryptographic algorithms, signature count, proof systems, or domain tags;
- conflict granularity, concurrency, execution ordering, or parallelism;
- database keys, physical layout, deletion, pruning, or historical retention;
- state commitments, authenticated structures, or proof formats; or
- rotation, deprecation, participation, recovery, or migration semantics.

Any experiment that needs one of these properties must declare it separately as
an experimental mapping choice, unresolved dependency, or architecture branch.
The property must not be attributed to a candidate label.

## 5. Shared Comparison Contract

Every paired comparison must start from one shared, candidate-neutral feature
contract. The contract describes externally equivalent native-value effects and
observable validity outcomes without prescribing either candidate's internal
representation.

At minimum, a paired contract records:

- equivalent abstract initial native-value conditions;
- the same intended value-affecting effect;
- the same applicable authorization workload assumptions, without selecting an
  authorization mechanism;
- the same validity or failure condition;
- the same externally defined semantic conflict relation;
- the same applicable ordering, protocol-version, and profile assumptions;
- the same adversarial capability and input-control assumptions;
- the same externally observable success or rejection outcome; and
- the authoritative monetary, security, and preservation constraints applicable
  to the effect.

For this non-normative experimental framework, two workload effects are
semantically conflicting when, under the same declared initial canonical-state
assumptions, ordering assumptions, and version/profile conditions, both cannot
validly coexist according to the shared experimental semantic contract. A
candidate mapping must not redefine this relation to favor itself. Both
candidates receive the identical external conflict workload.

Internal logical record counts, source counts, field counts, logical operations,
byte counts, conflict sets, and storage changes are not part of the shared
outcome unless the experiment explicitly declares one of them as the independent
variable being tested. Otherwise, they are candidate outputs.

## 6. Workload Definition Rules

Workloads are defined by externally equivalent value/effect semantics, not by
candidate-native terminology. A workload manifest must avoid assuming that one
candidate's internal unit has a one-to-one equivalent in the other.

A workload may vary semantic dimensions such as:

- number and distribution of abstract value positions before and after an
  effect;
- number of value-affecting effects in a candidate or block-shaped workload;
- amount and distribution of native value, using only non-normative experiment
  parameters;
- number `N` of independently required consensus authority relations in the
  declared external semantic workload;
- valid, malformed, conflicting, duplicate, early-failure, and late-failure
  cases; and
- repeated, skewed, adversarial, and state-growth-oriented sequences.

`N` is not automatically equal to the number of humans, owners, companies or
organizations, credentials, signatures, proofs, threshold shares, or
cryptographic verification operations. Whether native consensus supports
`N > 1` remains an architecture branch. This framework selects no native
multi-authorizer support or related mechanism.

The manifest must state whether an internal cardinality is an input chosen to
test a specific property or an output observed from a mapping. Fan-in,
fan-out, consolidation, batching, and conflict workloads must be described by
their external effect first. Conflict workloads must also use the shared frozen
semantic conflict relation. Candidate-specific internal cardinalities and
realized conflicts are then reported separately.

No experimental value, count, size, or workload shape in a manifest becomes a
protocol limit or protocol rule.

## 7. Candidate Mapping and Anti-Strawman Controls

Each comparison must satisfy all of these controls:

1. **Same feature contract.** Both mappings implement the same declared external
   effects, validity conditions, security assumptions, and failure outcomes.
2. **Pre-result freeze.** Candidate mappings, allowed optimizations, measurement
   methods, corpora, and exclusion rules are frozen before comparative results
   are examined. Material post-result changes create a new experiment identity.
3. **Comparable engineering effort.** Neither side may be an intentionally
   simplified, unoptimized, incomplete, or operationally unrealistic foil for
   the other.
4. **Independent review.** Reviewers assess semantic equivalence, implementation
   quality, hidden assumptions, and candidate-specific advantages before the
   results are used for conclusions.
5. **Multiple defensible mappings.** Where materially different, credible
   mappings exist, the campaign evaluates more than one or explains why the
   omitted mapping cannot affect the stated conclusion.
6. **Mapping-qualified conclusions.** Every conclusion identifies the exact
   mappings, branches, workload contract, corpus, and implementation versions to
   which it applies.

An experiment that fails these controls may remain exploratory evidence but
cannot support a final candidate-ranking claim.

## 8. Symmetric Optimization Policy

Both candidates receive symmetric permission to use optimizations that preserve
the shared feature contract and declared safety properties. This includes
candidate-appropriate batching, caching, indexing, deduplication, compact
representations, scheduling, proof reuse, and implementation specialization when
those techniques are available under the same branch assumptions.

Symmetry does not require identical internal techniques. It requires equal
opportunity, comparable maturity, equivalent correctness checks, and disclosure
of any optimization available to only one mapping. A candidate-specific
optimization is evidence about the tested mapping, not an inherent property of
the candidate label unless the conclusion survives other defensible mappings.

Results must retain an auditable unoptimized or reference baseline where doing
so is meaningful for correctness and attribution. Benchmark-only shortcuts that
change semantics are prohibited.

## 9. Common Safety and Correctness Gates

Every mapping must satisfy the following comparison gates before its performance
or resource evidence is decision-relevant:

- consistency with the Constitution and all applicable Formal Specification
  requirements;
- deterministic, independently reproducible results for consensus-relevant
  operations;
- implementation-independent logical behavior, without host architecture,
  operating-system, local-clock, or floating-point dependence;
- canonical serialization wherever an authoritative canonical encoding applies;
- explicit success and rejection behavior for the tested mapping;
- no partial canonical state effect after failed validation;
- bounded worst-case resource exposure during validation of untrusted input;
- deterministic, host-independent arithmetic domains and explicit overflow and
  underflow behavior;
- satisfaction of the authoritative monetary and supply invariants applicable
  to each value-affecting transition;
- no structural foreclosure of authoritative cryptographic-rule evolution; and
- preservation of the currently authoritative outcomes protected by
  Constitution Article 7.

These gates do not select mechanisms. In particular, they select no balance or
counter width, counter architecture, authorization structure, resource meter,
commitment, or state representation.

## 10. Hostile-Work and Failure-Atomicity Boundaries

The generic hostile-work requirement is that validation of untrusted input has a
deterministic, bounded worst-case resource exposure. Each experimental mapping
must explain how the tested validation path establishes that bound before its
resource evidence is used.

Candidate-attempt meters, monotonic attempt accounting, pre-action reservation,
and no-refund behavior are resource-architecture branches. They may be evaluated
as explicit branches, but this framework does not require any one of them and
does not import them into the definitions of Account or UTXO.

Canonical failure atomicity is separate from attempted or local resource
accounting: failed validation produces no partial canonical state effect. This
does not decide whether attempted work is charged, restored, reserved, refunded,
or represented by a meter. Local ingress and abuse controls are likewise outside
canonical state semantics unless an authoritative protocol rule later says
otherwise.

Valid-case limits alone must not be offered as evidence that invalid inputs have
bounded cost. Tests and arguments must cover malformed, adversarial, and
late-failure paths under each evaluated resource branch.

## 11. Arithmetic and Monetary Integrity

Arithmetic used by an experimental mapping must have a deterministic,
host-independent domain. Overflow and underflow behavior must be explicit and
must not depend on language defaults, machine word size, undefined behavior, or
build mode.

This framework does not select a balance width, counter width, counter layout,
counter lifetime, or relationship between value arithmetic and resource
accounting. Such choices remain separate experimental parameters or architecture
branches until authorized elsewhere.

Every value-affecting transition must satisfy the authoritative monetary and
supply invariants applicable to that transition. The comparison must identify
which authoritative invariant is being tested and cover success and failure
paths. This framework does not create a universal conservation formula, issuance
rule, destruction rule, fee rule, or other monetary mechanism.

## 12. Crypto Agility and Article 7 Boundaries

Crypto Agility is a hard comparison gate only in the following structural sense:
a candidate mapping must not foreclose future evolution of cryptographic rules
that is authorized by the Constitution, Formal Specification, and applicable HIP
or Super HIP process. The framework selects no algorithm, registry structure,
credential form, version field, coexistence rule, or cryptographic migration
procedure.

Article 7 is a hard comparison gate only for outcomes that are currently
authoritative. This framework does not extend those outcomes into a guarantee of
lost-credential recovery, compromised-credential recovery, dormant-owner
participation, perpetual acceptance of an old primitive, or exclusive-control
continuity after the sole distinguishing evidence becomes forgeable.

Authorization-coverage Level C, Level D, deprecation policy, participation
requirements, migration policy, and recovery remain unresolved policy or
architecture branches. They may be tested conditionally and symmetrically, but
none is required, selected, or defined here.

## 13. Architecture Branch Register

The comparison must keep materially outcome-sensitive choices as named branches
rather than averaging them into a candidate label.

| Branch family | Examples of unresolved choices | Comparison treatment |
|---|---|---|
| Authorization coverage | Evidence grouping, reuse, authorizer count support, mixed-version coexistence | Apply equivalent external requirements and report each branch separately |
| Replay and uniqueness | Required property and its candidate-specific realization | Do not define here; freeze a reviewed experimental assumption before measuring affected outcomes |
| Value-record lifecycle | Creation, replacement, deletion, recreation, and historical interpretation | Treat as mapping assumptions, not label-derived semantics |
| Resource architecture | Candidate-attempt accounting, reservation, monotonicity, refund behavior, local ingress controls | Evaluate as explicit branches; none is universal in this framework |
| State commitment | Abstract commitment properties and candidate-compatible constructions | Keep commitment-free evidence visible and report commitment-dependent evidence by branch |
| Concurrency and ordering | Applicable ordering assumptions and candidate-specific conflict realization | Freeze the same external semantic conflict relation first; report realized conflict breadth as an output; require the authoritative result, not a mandatory validator architecture |
| Crypto evolution | Version coexistence, deprecation, participation, and currently unresolved guarantee levels | Report only conditional evidence under declared assumptions |
| Physical persistence | Database layout, indexing, caching, pruning, snapshots, and history | Separate logical behavior from implementation evidence |
| Economics | Fees, weights, rent, refunds, subsidies, and persistent-state pricing | Exclude unless a later authoritative scope defines the branch |

State commitment remains an architecture branch. A campaign may compare
candidate-specific approaches under the same abstract commitment properties when
one concrete approach would bias the comparison. Commitment-free metrics must
remain visible so a commitment branch cannot silently determine the overall
conclusion.

## 14. Measurement Model

Measurements are meaningful only after the shared workload contract,
experimental mappings, branch assumptions, validation stages, and optimization
permissions are frozen.

Subject to that condition, candidate outputs may include:

- logical reads, writes, replacements, deletions, and validation steps;
- realized logical conflict breadth, conflict-set cardinality, record or unit
  overlap, candidate-representation contention, schema-induced conflict
  amplification, and deterministic scheduling constraints;
- canonical bytes and other explicitly defined logical representation bytes;
- persistent logical-state growth and history or snapshot consequences;
- cryptographic verification work under a declared algorithm/workload branch;
- commitment-independent and commitment-dependent proof or update work;
- temporary memory, allocation, database, synchronization, and network evidence;
- valid, invalid, early-failure, and late-failure resource exposure; and
- proof obligations, implementation complexity, and independently reproduced
  correctness evidence.

Logical reads and writes require a frozen logical-access definition. The external
semantic conflict relation and applicable ordering, version, and profile
assumptions must be frozen before candidate mapping. Candidate mappings must not
redefine them; realized conflict outputs require a frozen candidate conflict
model.

Any experimental encoding or schema used to support an exact-byte or
schema-dependent quantitative claim must be explicitly non-normative, versioned,
content-addressed, bound to a frozen semantic profile, bound to a frozen
assumption profile, bound to the candidate mapping being measured,
feature-symmetric across the compared candidates, and used only as experimental
evidence. An experimental schema or encoding must not resolve a Formal
Specification TBD by precedent, implementation convenience, benchmark success,
or repeated use. An exact byte claim is valid only for the explicitly identified
experimental schema, mapping, semantic-profile, and assumption-profile
combination.

State-growth measurements require a frozen lifecycle and retention assumption.
None of these outputs may be treated as intrinsic to a candidate name before
those prerequisites are fixed.

Physical database operations, cache behavior, allocator behavior, wall-clock
time, CPU cycles, and machine-specific measurements are implementation evidence,
not consensus inputs.

## 15. Benchmark Evidence and Reproducibility

`BENCHMARK_METHODOLOGY.md` is a non-normative evidence methodology. Its campaign
registration, provenance, corpus, statistics, anti-gaming, and reproduction
controls provide auditability; they do not create protocol authority or runtime
validity conditions.

Exploratory experiments must be labeled exploratory. A campaign claiming
conformance to the non-normative benchmark methodology must follow its applicable
formal-campaign controls, including pre-result registration, frozen artifacts,
discoverable run identities, retained failures and deviations, complete
provenance, raw-result preservation, and independent reproduction where
required.

Correctness is checked independently of timing. Reports must disclose failed or
incomplete runs, unsupported cases, harness-specific code, compilation settings,
dependencies, machine profiles, and environmental conditions. Candidate-specific
fast paths must be reviewed for semantic equivalence.

No benchmark timing, CPU-cycle count, allocator result, operating-system effect,
cache state, thermal state, database layout, optional acceleration result, or
benchmark conclusion determines consensus validity.

## 16. Branch-Specific Analysis and Reporting

Results must be reported per material architecture branch, candidate mapping,
workload class, and implementation version. Aggregation across branches is
permitted only when the aggregation rule is predeclared and cannot conceal a
material reversal.

This framework uses no scalar score by default. That is a current methodology
choice, not a constitutional rule or permanent prohibition. A later authorized
decision process may adopt weights or another aggregation method, but the method,
authority, sensitivity, and consequences would need separate review.

Default reporting uses branch-specific metric vectors and Pareto relationships.
A mapping is strongly dominant within a frozen comparison only if it is no worse
on every decision-relevant reported dimension and better on at least one, under
the same declared assumptions. Strong dominance is evidence; it is not the only
possible future selection method.

Every conclusion must state:

- the exact external feature contract and workload domain;
- mappings and optimization permissions;
- architecture branches and unresolved assumptions;
- metrics included and excluded;
- uncertainty, dispersion, failures, and sensitivity results;
- whether the relationship changes across defensible mappings or branches; and
- the limited claim that the evidence actually supports.

No result may be generalized from one experimental mapping to an entire
candidate family without evidence that the result is robust across other
materially defensible mappings.

## 17. Exit Criteria for Final Ranking

The categories below separate ranking necessities from useful or conditional
evidence. Satisfying them would make a future ranking reviewable; it would not by
itself authorize protocol adoption.

### REQUIRED FOR FINAL RANKING

- authoritative constraints and currently applicable TBDs are identified, with
  no higher-authority conflict;
- a candidate-neutral external feature contract and workload domain, including
  the shared semantic conflict relation and applicable ordering, version, and
  profile assumptions, are frozen;
- each mapping is complete enough to satisfy the same feature contract and
  common safety gates;
- anti-strawman controls and symmetric optimization permissions have passed
  independent review;
- all material architecture branches are evaluated or the ranking is explicitly
  limited to a declared branch;
- metric definitions, validation stages, experimental representations, corpora,
  and exclusion rules are frozen before comparative results;
- every experimental representation supporting an exact-byte or
  schema-dependent quantitative claim satisfies the identity, profile-binding,
  feature-symmetry, and evidence-only requirements in Section 14;
- deterministic correctness, failure atomicity, monetary/supply integrity, and
  bounded hostile-work exposure are demonstrated for the tested mappings;
- decision-relevant results are reproducible and reported with provenance,
  uncertainty, failures, and sensitivity analysis;
- conclusions are qualified by mapping, branch, workload, and implementation;
  and
- unresolved assumptions capable of reversing the ranking are either resolved
  authoritatively or preserved as separate conditional outcomes.

### STRONGLY DESIRABLE

- multiple independent implementations or independent reimplementations of the
  frozen logical mappings;
- independently reproduced formal benchmark campaigns;
- machine-checked or otherwise auditable proofs for key conservation,
  determinism, replay-safety assumptions, and failure properties once those
  properties are defined by the experimental branch;
- broad representative and adversarial corpora, including early- and
  late-failure paths;
- several defensible mappings for every material design family; and
- evidence across implementation strategies, databases, supported platforms,
  and relevant state scales.

### CONDITIONAL

- commitment-dependent measurements, if a ranking claim includes proof,
  authenticated-state, or synchronization consequences;
- Section 14-compliant experimental encodings, if a claim includes byte size,
  decoding, or bandwidth;
- authorization-coverage, deprecation, participation, or crypto-evolution
  branches, if a claim depends on those policies;
- an explicit resource-accounting branch, if a claim concerns candidate-attempt
  capacity, fees, refunds, or hard-limit eligibility;
- persistence and history assumptions, if a claim concerns state growth,
  snapshots, pruning, or synchronization; and
- concurrency and conflict models, if a claim concerns parallel execution or
  scheduling.

### NOT NECESSARY MERELY TO RANK

- a complete production implementation;
- a production database selection or physical storage layout;
- final fee constants, economic conversion, or fee-market rules;
- final network topology, peer-to-peer protocol, or deployment operations;
- final governance, activation, or release mechanics;
- selection of every future cryptographic algorithm;
- a finalized user-interface or wallet architecture; or
- implementation of unrelated protocol subsystems, unless the ranking claim
  relies on them.

## 18. Evidence Deliverables and Non-Deliverables

A comparison package should contain:

- a versioned external feature contract and workload manifest;
- frozen mapping descriptions kept outside normative protocol documents;
- a branch matrix and assumption register;
- correctness oracles and adversarial cases;
- benchmark manifests, source, corpora, provenance, raw results, and analysis;
- independent mapping, security, and anti-strawman reviews;
- branch-specific metric vectors and sensitivity analysis; and
- a limitations statement identifying every unsupported generalization.

This framework itself supplies none of the following: a selected candidate,
candidate data layout, transaction or state format, ownership or replay rule,
concrete state commitment, migration procedure, mandatory attempt-accounting
mechanism, numeric resource limit, measured candidate result, or protocol
adoption decision.

## 19. Unresolved Dependency Register

The following remain unresolved unless a higher-authority repository document is
updated separately:

- transaction structure, value-effect semantics, validation order, and canonical
  transition rules;
- state representation, lifecycle, history, synchronization, and commitment;
- ownership, authorization coverage, evidence grouping, and replay protection;
- stable identity, credential evolution, deprecation, participation, migration,
  and recovery policy;
- cryptographic algorithms, registry mechanics, coexistence, and domain tags;
- consensus algorithm, ordering, conflict, finality, and block-production rules;
- resource units, counting rules, limits, aggregate budgets, and hostile-attempt
  accounting;
- collection, transaction, block, network, memory, cryptographic-work, and
  persistent-state limits;
- fees, weights, rent, refunds, subsidies, and economic conversion;
- state-growth population models, minimum-node profiles, and supported
  architecture policy; and
- governance thresholds, HIP/Super HIP mechanics, and Evolution Engine runtime
  behavior.

No item is resolved by its appearance in this document. Experiments must label
assumed values or mechanisms as non-normative and must not silently promote them
into project direction.

## 20. Conclusion

Account and UTXO remain co-equal analytical native-value-accounting hypotheses.
Evidence collection can begin under frozen, symmetric, independently reviewed
experimental mappings, but current authoritative gaps prevent an unconditional
final ranking.

The safe next analytical step is to prepare candidate-neutral workload contracts,
mapping-review templates, branch manifests, and measurement definitions. Any
future ranking must remain branch-specific, mapping-qualified, reproducible, and
subordinate to the Constitution and Formal Specification.
