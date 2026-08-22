# Dilithia Transaction and Block Resource Architecture

> **NON-NORMATIVE ARCHITECTURE DIRECTION — DOES NOT DEFINE CONSENSUS RULES**
>
> This document records a reviewed design direction only. It cannot resolve a
> Formal Specification TBD, establish protocol validity, or activate a protocol
> change. The Dilithia Constitution and Formal Specification remain
> authoritative. Benchmark and design documents provide evidence and rationale
> only.

## 1. Status, Authority, and Scope

This document is subordinate to the following authority order:

1. Constitution
2. Formal Specification
3. Ratified HIP / Super HIP
4. Normatively adopted conformance vectors, where applicable
5. Implementation
6. `PROJECT_STATE.md`
7. Non-normative design and benchmark documents, including this document

The architecture can be reviewed and recorded before transaction, block, state,
cryptographic, economic, and consensus designs are complete. Recording it does
not make any part of it a protocol rule.

Terms such as "must" and "should" express safety criteria for this design
direction only; they are not normative protocol keywords in this document.

This document defines no transaction, block, or state format; no resource-vector
layout or identifier; no counter width or numeric limit; no collection or depth
rule; no work unit; no cryptographic algorithm, operation taxonomy, registry, or
batch rule; no state-access or persistent-growth unit; no memory proxy; no fee,
rent, refund, or pricing rule; no hardware or architecture requirement; and no
activation, governance, timing, or block-interval rule. All such decisions remain
**TBD** unless an authoritative protocol document defines them later.

## 2. Core Architecture Direction

The leading safety direction is independently enforceable, componentwise
resource envelopes rather than a single scalar gas value. A multidimensional
resource vector is a useful organizational abstraction, but its exact dimensions
and representation are not finalized here.

The architecture distinguishes:

- value and collection protection;
- transaction aggregation;
- block aggregation;
- candidate-validation attempt metering;
- canonical state and resource accounting;
- local ingress and abuse accounting; and
- economic pricing.

No economic willingness to pay may relax a hard safety envelope. Future pricing,
whatever its eventual design, must remain subordinate to every applicable hard
resource bound. This document does not select scalar fee conversion or any other
pricing model.

## 3. Three Accounting Domains

The following are separate conceptual accounting domains. They are not three
on-chain objects and do not imply a new persisted protocol data structure.

### A. Canonical State and Resource Context

This context is provisional during validation. It is committed only after the
candidate has completed every applicable canonical and semantic validity check.
On failure, caller-visible canonical state and successful resource effects remain
unchanged.

If future resource accounting affects successful state transition or validity,
its exact behavior belongs in the authoritative protocol specification. This
document defines only the failure-atomic boundary.

### B. Candidate-Validation Attempt Meter

This meter exists while a node validates one untrusted candidate transaction or
block. It bounds work that a hostile candidate can cause before its validity is
known. Capacity reserved or consumed for an attempted action is monotonic for
that candidate and is not restored merely because the candidate later fails.

The meter need not be stored on-chain and need not be a canonical persisted data
structure. Its exact units, representation, and relationship to future resource
counters remain **TBD**.

### C. Local Ingress and Abuse Meter

This is local operational policy for exposure such as repeated invalid traffic,
duplicate messages, peer behavior, or work already incurred by one node. It may
be stricter than consensus admission where safe.

Local ingress and abuse decisions cannot redefine canonical transaction or block
validity. A transaction rejected by one node's local policy does not thereby
become invalid in a block.

## 4. Invalid-Candidate Safety Invariant

Every potentially expensive action performed while validating an untrusted
candidate must be preceded by a deterministic bound check or conservative
reservation against the candidate-validation attempt envelope. Attempted
capacity remains monotonic for that candidate.

Conceptual examples include parsing descent, cryptographic verification, logical
state access, proof processing, and deterministic validation work. This document
does not assign units to those actions.

The eventual validation architecture must ensure that:

- framing and declared sizes are bounded before an unsafe allocation;
- parsing descent cannot bypass the containing attempt envelope;
- required cryptographic capacity is reserved before verification;
- applicable state-access capacity is reserved before a logical access;
- proof processing and other expensive work are checked before execution;
- a conservative upper bound may be reserved when exact cost is not yet known;
- arithmetic overflow or envelope exhaustion causes immediate deterministic
  rejection;
- failed canonical effects roll back;
- attempted validation capacity for that candidate does not roll back; and
- validation stops once the candidate is known invalid.

Valid-block limits alone are insufficient. A hostile block can contain a long
valid prefix followed by a malformed field, an invalid proof, an expensive state
condition, or another late failure. If only successfully committed transaction
usage counts, the failing work can occur outside the apparent block total. The
attempt envelope must therefore bound work before successful canonical commit is
known.

## 5. Layered Safety and Composition

Protection is layered across values, collections, transactions, and blocks:

- A value boundary prevents one field from being unbounded.
- A collection boundary prevents many individually acceptable values from
  amplifying a container.
- A transaction boundary aggregates all fields, collections, and validation
  effects belonging to one transaction.
- A block boundary aggregates transactions and block-level artifacts.

A lower-layer check never resets its containing higher-layer accounting.
Splitting a value across fields must not evade a transaction envelope; splitting
fields across collections must not evade their transaction; splitting work
across transactions must not evade the block; and many small transactions must
not evade aggregate handling or structural protection.

Persistent growth across blocks is a longer-lived problem that per-value,
per-transaction, or per-block byte limits cannot solve alone. Its eventual
protection depends on future state and economic design. This document introduces
no account-level quota.

## 6. Minimal Resource Risk Classes

The following may be recorded now as architectural risk classes:

- canonical encoded data volume;
- aggregate structural complexity;
- deterministic validation effort;
- cryptographic effort;
- logical state access and mutation;
- persistent-state exposure; and
- transaction and block aggregation.

These risk classes are not finalized vector fields, counters, or identifiers.
Their purpose is to prevent unlike safety risks from disappearing behind one
opaque scalar.

Possible future dimensions remain candidates only. Examples include decoded
logical bytes, element count, nesting or depth, validation-work events,
hash-input bytes, algorithm/version-aware cryptographic units, logical state
reads or writes, gross mutation, positive persistent growth, transaction or base
handling count, temporary-memory proxies, and future execution units. None is
made a protocol counter by appearing in this list.

## 7. Conservative Resource Aggregation

Transaction and block resource accounting should default to conservative,
independent charging. It must not assume that block usage is always a simple sum
of context-free transaction vectors.

Where cost depends on transaction order, shared state, or another block context,
the future rule must account in the canonical transaction and block context.
Shared work, deduplication, caching, batching, common subexpressions, or other
aggregation receives no accounting discount merely because an implementation
optimizes it.

A future authoritative protocol rule may define a discount only after it defines:

- the shared object or work;
- every participating operation;
- deterministic accounting independent of implementation strategy;
- failure and partial-failure behavior; and
- protocol-version and historical interpretation.

Block-level proofs, commitments, certificates, or other artifacts must receive
explicit accounting if later introduced. Required candidate-attempt capacity is
reserved before the associated expensive work. State-dependent and
neighbor-dependent effects remain conceptual until the state and transaction
models exist.

## 8. Cryptographic Accounting Boundary

Future cryptographic resource accounting must be typed and
algorithm/version-aware. It must not assume that all signatures, proofs, hashes,
or parsing operations have equal cost.

A future cryptographic resource contract may need to define canonical artifacts,
parse bounds, logical operations, input-dependent components, batch semantics,
conservative attempted-work charges, activation interpretation, and historical
interpretation.

This document defines no algorithm, parameter set, identifier, representation,
operation taxonomy, weight, registry structure, or batch rule. Malformed-input
cost must never be derived from measured failure time. A Crypto Agility change
requires renewed resource-accounting review before the new algorithm becomes
active, without changing the interpretation of historical data.

## 9. State and Persistent-State Boundary

State accounting must be logical and independent of the chosen storage backend.
State access, mutation effort, and long-lived growth are distinct risks. Net
change alone cannot represent all state-related work: large write-and-delete
churn can have a small net result while still creating substantial validation and
mutation exposure.

Physical database pages, cache behavior, indexes, journals, compaction, and write
amplification remain implementation and benchmark evidence. They do not directly
determine consensus validity.

Persistent-state exposure is a separate long-term risk because it affects future
storage, synchronization, backup, proof, and migration burdens. It cannot be
derived only from temporary transaction byte size.

Logical key and value semantics, reads and writes, repeated-read behavior,
deduplication, replacement, write/delete ordering, proofs, current versus
historical state, and exact gross or net counters remain **TBD**. This document
defines no state format, storage fee, rent, refund, pruning, or cleanup policy.

## 10. Temporary-Memory Boundary

Resident-set size, allocation counts, language object size, allocator success,
language layout, and thread count must not become consensus inputs.

Formats and validation workflows should admit at least one bounded-memory
implementation under every applicable protocol bound. Potential deterministic
proxies remain candidates only. They must eventually be evaluated against risks
such as recursive structures, multiple simultaneous representations,
cryptographic scratch space, proof expansion, decompression, state witness
material, and pathological data structures.

Benchmark evidence across independent implementations should demonstrate that
the selected deterministic limits imply an acceptable memory envelope. Benchmark
RSS, allocation, and object-layout measurements remain evidence, not resource
counters.

## 11. Counter Invariants

The following counter principles are safe to record before exact counters exist:

- protocol-defined fixed-width domains, with exact widths **TBD**;
- checked arithmetic only;
- no wraparound or saturation;
- explicit transaction, block, nested, and candidate-attempt reset boundaries;
- shared containing accounting across nested modules;
- deterministic accumulation where context affects meaning;
- protocol-version-specific semantics; and
- a bound check or reservation before the charged expensive action.

Consensus accounting must not depend on `usize`, floating point, host CPU cycles,
or host timing. Hard safety capacity should not be replenished by a deletion,
refund, or negative adjustment unless a future authoritative rule defines that
behavior without introducing underflow, order dependence, or resource-evasion
risk. No counter representation is selected here.

## 12. Simple DLTH Transfer Preservation

Simple DLTH transfers should retain a narrow validation path. The resource
architecture should not require a generic execution virtual machine, dynamic
resource maps, or module dispatch unrelated to the transfer.

Unused resource categories should add no unrelated work or economic charge.
Future modules should not add processing to the simple-transfer path unless the
protocol genuinely requires it, and expensive modules should not be
cross-subsidized by simple transfers. No transfer format or fee is defined here.

## 13. Parallelism and Implementation Independence

Resource semantics must not depend on threads, scheduling, batching speedup,
SIMD, caching, allocator behavior, Rust object layout, `usize`, database backend,
compiler optimization, or CPU architecture.

Future serial and parallel implementations must agree on validity, deterministic
resource results, canonical state transition, and protocol-version behavior.
Parallelism remains an implementation optimization. A serial validator may be
useful as a non-normative audit oracle, but this architecture does not require a
serial implementation.

## 14. Formal Specification and Hardware Boundary

Future validity-affecting rules may eventually require the Formal Specification
to define canonical formats, deterministic resource units, counter arithmetic and
overflow, validity-affecting limits, aggregation and failure behavior,
version-specific interpretation, cryptographic and state accounting once those
subsystems are designed, fee rules where they affect canonical validity or state
transition, and activation or historical-validation behavior.

The Formal Specification should not define benchmark machine profiles, CPU or
RAM requirements, hardware product identifiers, throughput targets, or benchmark
timing. Minimum-node capability profiles remain non-normative benchmark and
operational evidence.

A node that cannot process a consensus-valid input has an implementation or
operational capability problem. Its local failure must not reinterpret the
canonical input as invalid. Exact capability targets and supported architecture
policy remain **TBD**, and no architecture may observe different consensus
semantics because of its host properties.

## 15. Economic Boundary

Hard safety envelopes, deterministic resource usage, and economic pricing are
separate layers. Future pricing cannot purchase permission to exceed a hard
bound.

Fees, pricing units, coefficients, scalar conversion, rent, refunds, subsidies,
storage economics, and fee-market mechanics remain **TBD**. This document defines
no economic formula.

## 16. Versioning and Evolution

Validity-affecting resource semantics must be explicitly versioned. A node cannot
silently ignore unknown activated semantics, and historical data retains the
interpretation applicable to its protocol history.

Future features require reviewed resource contracts before activation. A new
dimension is justified only by a genuinely distinct safety risk that existing
dimensions cannot bound without unsafe substitution or ambiguity. Exact
activation, migration, old-node, Evolution Engine, and governance mechanics
remain **TBD**.

## 17. Candidate Threat-Model Additions

The following are non-normative candidates for a future, independently reviewed
update to `THREAT_MODEL.md`:

- invalid-block late failure;
- value, collection, transaction, and block resource amplification;
- quadratic parsing and algorithmic-complexity attacks;
- hash-table or pathological lookup behavior;
- decompression bombs;
- proof amplification;
- malformed batch fallback;
- resource-counter overflow or underflow;
- counter reset across nested modules;
- shared-state order dependence;
- deletion or refund exploits against hard budgets;
- cross-block persistent-state growth;
- proposer-selected worst-case cache behavior;
- differential accounting across implementations; and
- neighbor-dependent transaction cost.

This list does not modify the current threat model or declare mitigation details
normative.

## 18. Premature-Commitment Matrix

| Concept | Classification | Boundary |
|---|---|---|
| Explicitly non-normative architecture direction | SAFE TO RECORD NOW | Creates no protocol rule |
| Layered value, collection, transaction, and block protection | SAFE TO RECORD NOW | Exact limits and formats remain TBD |
| Componentwise hard safety envelopes | SAFE TO RECORD NOW | Exact vector dimensions remain TBD |
| Separation of hard limits, pricing, and local policy | SAFE TO RECORD NOW | Economic rules remain TBD |
| Fixed-width checked-counter principle | SAFE TO RECORD NOW | Widths and representation remain TBD |
| Logical rather than host-physical accounting | SAFE TO RECORD NOW | Exact logical units remain TBD |
| Candidate-validation attempt envelope | SAFE TO RECORD NOW | Exact units and mechanics remain TBD |
| Failure-atomic canonical effects | SAFE TO RECORD NOW | Exact transaction/state semantics remain TBD |
| Local ingress and abuse separation | SAFE TO RECORD NOW | Cannot change canonical validity |
| Explicitly versioned validity semantics | SAFE TO RECORD NOW | Activation mechanics remain TBD |
| Persistent state as a distinct risk | SAFE TO RECORD NOW | Attribution and economics remain TBD |
| Exact resource-vector layout and identifiers | BLOCKED BY FORMAT DESIGN | Do not define here |
| Decoded-byte, element, and depth counters | BLOCKED BY FORMAT DESIGN | Keep as candidates |
| Validation-work units | BLOCKED BY FORMAT DESIGN | Also depends on validation workflow |
| Concrete crypto registry or taxonomy | BLOCKED BY CRYPTO DESIGN | Typed/version-aware principle only |
| Batch accounting | BLOCKED BY CRYPTO DESIGN | Also depends on consuming formats |
| State read/write taxonomy | BLOCKED BY STATE DESIGN | Logical/backend-independent principle only |
| Gross, net, deletion, and replacement accounting | BLOCKED BY STATE DESIGN | Do not define here |
| Temporary-memory proxy set | KEEP AS CANDIDATE | Depends on formats, crypto, and state |
| Scalar fee conversion | KEEP AS CANDIDATE | Economic design remains separate |
| Block deduplication or batching discounts | KEEP AS CANDIDATE | Requires authoritative composition rules |
| Exact activation and governance rules | BLOCKED BY CONSENSUS/GOVERNANCE DESIGN | Do not infer them |
| Hardware profile as a protocol validity rule | DO NOT ADOPT | Hardware remains non-normative evidence |
| Mandatory serial validator architecture | DO NOT ADOPT | Serial validation may be an audit oracle only |

## 19. Complete TBD Matrix

| Unresolved decision | Dependency or reason |
|---|---|
| Exact resource-vector dimensions | Transaction, block, state, and crypto design |
| Vector identifiers and representation | Format and versioning design |
| Counter widths and representation | Unit ranges and composition analysis |
| All numeric resource limits | Formats, threat review, conformance, and benchmark evidence |
| Transaction and block formats | Future protocol design |
| State format and model | Future state design |
| Collection, element, and nesting rules | Consuming format design |
| Decoded-work and validation-work units | Validation workflow and adversarial analysis |
| Cryptographic algorithms and parameters | Crypto Agility design |
| Crypto resource taxonomy and representation | Selected algorithms and consuming formats |
| Batch and shared-work accounting | Crypto, transaction, and block composition |
| Logical state-access and mutation units | State transition design |
| Persistent-growth attribution | State model and long-term economic design |
| Temporary-memory proxies | Format, crypto, state, and independent implementation evidence |
| Candidate-attempt exact units and mechanics | Validation workflows and worst-case proofs |
| Fees and economic conversion | Economic and state-transition design |
| Rent, refunds, subsidies, and storage economics | Economic and persistent-state design |
| Minimum-node capability targets | Node roles, workloads, and benchmark evidence |
| Supported architecture policy | Capability and cross-platform review |
| Activation, migration, and governance mechanics | Consensus, HIP, and Evolution Engine design |
| Consensus timing and block interval | Consensus and liveness design |

No entry in this matrix is resolved by this document.

## 20. Recommended Design Sequence

The following is a non-normative work sequence:

1. Review and record the abstract resource architecture and its authority boundary.
2. Independently review proposed resource-related additions to the threat model.
3. Design transaction, block, and state abstractions with bounded validation paths.
4. Define deterministic units and counter semantics alongside those formats.
5. Define cryptographic and state resource contracts when those subsystems exist.
6. Create cross-implementation conformance vectors.
7. Prototype and benchmark independent implementations.
8. Run formal parameter-selection campaigns under the benchmark methodology.
9. Select numeric limits and economic rules using threat and benchmark evidence.
10. Adopt resulting consensus rules through the authoritative protocol process.
11. Implement after the applicable normative decisions are settled.

Implementation must not resolve a normative TBD by precedent.

## 21. Concluding Design Direction

The leading safety direction is multidimensional, componentwise hard resource
envelopes. Exact dimensions remain **TBD**, and economic pricing remains a
separate **TBD**. Candidate-validation attempt metering is required to bound
hostile late-failure validation independently of canonical commit and local abuse
policy. Minimum-node hardware profiles remain non-normative. This document
defines no protocol value, format, algorithm, fee, or activation rule.
