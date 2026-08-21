# Dilithia Resource Budget Design

> **NON-NORMATIVE DESIGN PROPOSAL — DOES NOT DEFINE CONSENSUS RULES**
>
> This document proposes an architecture for future resource-budget decisions.
> It does not establish protocol validity, consensus limits, fee constants, or
> implementation requirements. Every numeric value discussed by category below
> remains **TBD** until adopted through the authoritative protocol process. If
> this document conflicts with the Dilithia Constitution, Formal Specification,
> or a ratified HIP/Super HIP, those authoritative documents take precedence.

## Status and Scope

The Formal Specification already requires DCS decoders to enforce maximum sizes
before allocation, but leaves the exact limits TBD and ties them to future
transaction and block limits. The transaction, state, and consensus sections are
also pending. This proposal therefore describes how resource limits could fit
together without selecting values, inventing transaction or block formats, or
changing any current protocol rule.

The design target is a layered system in which a value can be canonical yet
still be rejected when its containing collection, transaction, block, message,
work budget, or state-growth budget is exhausted. No single global limit can
represent all of those costs safely.

## Design Goals

The eventual normative design should support:

- deterministic consensus across independent implementations;
- bounded memory, computation, and bandwidth consumption;
- a low and explicitly documented node-hardware burden;
- predictable worst-case validation cost under adversarial input;
- a minimal stable consensus core with narrowly defined accounting rules;
- modular evolution without silently changing the meaning of old data;
- a pay-for-what-you-use economic model where fees are appropriate;
- explicit control of persistent state growth; and
- deterministic, reviewable protocol upgrades with safe activation boundaries.

These goals are related but not interchangeable. A hard validity limit protects
consensus operation, while a fee influences incentives. Charging for a resource
does not by itself bound worst-case use, and bounding a resource does not by
itself price its long-term cost.

## Layered Resource Architecture

The proposed architecture uses separate, composable layers:

| Layer | Resource bounded | Purpose |
|---|---|---|
| Per value | Encoded or decoded size of one value, such as a `String` | Prevent one field from consuming unbounded resources |
| Per collection | Element count and cumulative encoded or variable-length content | Prevent many individually valid elements from amplifying cost |
| Per transaction | Total bytes, validation work, cryptographic work, state access, and state growth caused by one transaction | Bound the cost of accepting or rejecting one transaction |
| Per block | Aggregate bytes, work, memory pressure, and state growth across all included transactions and block data | Bound worst-case block propagation and validation |
| Network or message | Frame size, buffering, ingress rate, and repeated invalid traffic | Protect peers before or outside full consensus validation |
| Decoded work | Deterministically counted parsing and validation effort | Bound inputs whose byte size understates their processing cost |
| Persistent state | New and retained state attributable to accepted operations | Bound costs carried by nodes beyond the originating transaction or block |

A value would have to fit every applicable layer. Context-specific budgets may
be tighter than a protocol-wide type ceiling, but cannot make a non-canonical
value canonical. Aggregate counters must not reset at a nested field boundary.

Some network and mempool controls may be local policy and may deliberately be
stricter than consensus acceptance. Any limit or accounting rule that determines
whether a transaction, block, or state transition is valid is consensus-critical
and must be specified and versioned identically for every compliant node.

## Proposed String Resource Model

The current Formal Specification defines the `String` wire form as a canonical
ULEB128 length prefix followed by exactly that many UTF-8 bytes, with invalid
UTF-8 rejected. It does not explicitly select the integer domain of that length
prefix. The existing `u64` ULEB128 implementation is implementation evidence,
not authority to settle the `String` length domain. That domain remains **TBD**.

The intended layered model is:

- a canonical ULEB128 length in the future specified length domain;
- a protocol-wide base ceiling on the UTF-8 byte length of one `String`;
- tighter context-specific ceilings where a field has a narrower purpose;
- aggregate budgets on the containing collection, transaction, block, message,
  decoded work, and persistent state as applicable; and
- checked conversion from the protocol length domain to a host allocation or
  slice index only after protocol-level limits have passed.

All length accounting is over encoded UTF-8 bytes, not Unicode scalar values,
graphemes, displayed characters, or normalized text. DCS should validate UTF-8
and preserve its exact bytes. It should not normalize Unicode, because doing so
would repair or reinterpret input and would conflict with byte-identical
canonical round trips. A consuming subsystem may eventually define additional
text semantics for a particular field, but that would be a separate rule.

A future decoder should operate on a shadow cursor and follow a failure-atomic
sequence conceptually like this:

- decode and validate the minimal canonical ULEB128 prefix;
- reject a length outside the specified protocol domain;
- check the per-String ceiling, the context ceiling, and all applicable
  remaining aggregate budgets using fixed-width checked accounting;
- perform a checked conversion to the host indexing type;
- verify that the declared bytes are present;
- validate UTF-8 over the borrowed input bytes without normalization;
- allocate or copy only after every preceding check succeeds; and
- commit the caller-visible cursor and resource charges only on full success.

On any failure, the caller-visible input cursor and caller-visible budget state
should remain unchanged. The exact error taxonomy, inclusive or exclusive limit
boundaries, length domain, counters, and ceilings remain **TBD**.

## Aggregate-Budget Model

Per-value limits prevent one oversized value but do not stop an adversary from
combining many maximum-cost values. Each container therefore needs cumulative
accounting appropriate to its role. Examples include:

- an `Option<String>`, whose present value consumes the nested String budget and
  its containing context's remaining budget;
- a future `Vec<String>`, which needs both an element-count limit and cumulative
  byte and work limits;
- a transaction with several variable-length fields, where each field may pass
  its own ceiling while their sum is excessive;
- governance metadata containing multiple independently bounded text values; and
- future module or application data composed of nested variable-length values.

Nested decoding should receive a shared or explicitly subdivided budget. Checked
addition and subtraction should prevent wraparound, and failed nested decoding
should not consume the outer budget. A container should not be allowed to evade
aggregate limits by splitting the same content across fields or nested values.
The exact accounting units and ownership model remain **TBD**.

## Transaction Resource Model

No transaction format is proposed here. A future design should evaluate at least
these independent categories:

| Category | Architectural role |
|---|---|
| Encoded bytes | Bounds transport, buffering, decoding input, and retained transaction data |
| Decoding and validation work | Accounts for deterministic structural and semantic checks |
| Cryptographic work | Accounts for hashes, signatures, proofs, or other cryptographic verification |
| State reads | Accounts for deterministic access to existing consensus state |
| State writes | Accounts for mutation work distinct from retained size |
| Persistent bytes | Accounts for new or expanded data that nodes must continue to store |
| Optional future execution units | Provides a separate category only if a later execution model requires it |

Hard maxima and any accounting algorithm used to decide transaction validity are
likely consensus-critical. The fee schedule applied to those measured resources
is an economic-policy question, but becomes consensus-critical wherever it
affects transaction validity, debits, credits, issuance, or state transition.
Local admission policy may reject work more conservatively, provided it does not
change block validity.

Encoded size alone is not a complete proxy for cost. A compact transaction may
trigger expensive cryptography or many state accesses, while a larger but simple
transaction may be cheap to validate. Resource categories should remain visible
rather than being hidden behind one opaque score unless a later specification
defines a deterministic conversion model.

## Block Resource Model

Blocks need aggregate limits independent of per-transaction limits. Otherwise a
block can be composed entirely of individually valid, maximum-cost transactions
and still exceed safe propagation, validation, or state-growth bounds.

The future block model should separately consider:

- encoded block bytes and network propagation pressure;
- end-to-end validation latency;
- peak and retained memory during decoding and validation;
- aggregate cryptographic verification workload;
- aggregate reads, writes, and persistent state growth; and
- adversarial mixtures chosen to maximize the most constrained resource rather
  than representative average workloads.

Block accounting must include block-level data as well as its transactions. It
must define deterministic handling of shared work or batching if those features
are later introduced. No block format, batching rule, or limit is selected here.

## Network and Message Resource Model

Network defenses should act before an attacker can force a full allocation or
consensus validation. Candidate controls include bounded framing, bounded
buffering, progressive parsing, peer-level ingress quotas, duplicate suppression,
and rate or reputation controls for repeated invalid traffic.

Transport frame limits and local peer policy need not automatically equal DCS,
transaction, or block limits. A network may carry control messages with different
needs, and block transport may require framing without changing canonical block
bytes. Consensus-valid data must remain processable under the minimum compliant
profile, while nodes may adopt stricter non-consensus relay or mempool policies
only where doing so cannot alter validation results.

## Decoded-Work Model

Byte limits do not bound all parsing cost. Invalid UTF-8 near the end of a large
declared value, deeply composed data, repeated validation checks, and future
cryptographic fields can consume substantial work before rejection.

If decoded work affects validity, it should be measured in deterministic,
specified units and accumulated with checked fixed-width counters. Wall-clock
time, allocator timing, thread scheduling, hardware acceleration, and measured
host CPU cycles cannot determine consensus validity. Benchmark results can guide
the selection of units and budgets, but benchmark time itself must not become a
consensus input. The exact work model remains **TBD**.

## Persistent-State Model

Transient transaction bytes and permanent state growth have different lifetimes.
Transient data primarily consumes bandwidth, short-lived memory, and validation
work. Persistent data imposes repeated storage, synchronization, backup, proof,
indexing, and future migration costs on every relevant node.

Persistent bytes should therefore have distinct and eventually stricter
accounting than temporary bytes. The model should attribute creation, expansion,
replacement, and deletion deterministically and should define how shared or
deduplicated data is counted before relying on it. Constitution Article 11
requires explicit economic protection against unbounded state growth, free spam,
and uncompensated permanent storage. This proposal does not define storage fees,
rent, refunds, cleanup rules, or state formats.

## Fee Architecture

Without selecting units, constants, or a formula for consensus, the intended
relationship can be expressed conceptually as:

```text
fee = base cost
    + encoded byte cost
    + computation cost
    + persistent state cost
```

The categories allow richer features to pay for the resources they consume
without making a simple DLTH transfer expensive merely because unrelated
features exist. A future schedule should preserve a low-cost path for simple
transfers while preventing cross-subsidy of expensive validation or persistent
storage. Hard safety budgets remain necessary even when fees apply.

## Host and Platform Determinism

Protocol validity must not depend on the host's `usize` width, address space,
allocator behavior, operating system, or optimization choices. Resource lengths,
charges, and remaining budgets used for consensus should use specified
fixed-width domains with checked arithmetic. Their exact widths remain **TBD**.

Conversion to `usize` or another host API type should occur only after the value
has passed protocol limits and only through a checked conversion. A host that
cannot represent or allocate a consensus-valid value must not silently reinterpret
it as invalid protocol data; the normative limits and minimum compliant node
profile must instead be chosen together so every supported implementation can
process valid worst-case inputs.

The eventual specification must state a minimum compliant implementation profile
and determine supported architecture expectations. Tests should demonstrate
identical validity and accounting results on supported 32-bit and 64-bit targets.
This document does not decide which architectures are supported.

## Versioning and Evolution

Limits and accounting rules that influence validity should be part of an explicit
protocol-version schedule. A change should identify its activation boundary and
must not take effect through local configuration, software rollout timing, or an
unversioned default.

Increasing a limit may make new data valid that old nodes cannot safely process.
Decreasing a limit may make newly submitted data invalid while historical data
created under an earlier version remains valid. The future specification should
therefore define:

- the version under which each transaction, block, and state transition is
  validated;
- deterministic activation boundaries for increases and decreases;
- historical decoding and validation under the rules active at that history;
- migration requirements if retained state no longer fits a new representation;
- old-node behavior at and after activation; and
- conformance vectors spanning the activation boundary.

Nodes that do not understand an activated consensus version must not silently
continue under old limits and risk divergence. How Dilithia coordinates such an
upgrade belongs to the future upgrade specification; this document does not
design the Evolution Engine.

## Threat-to-Defense Mapping

| Attack pattern | Primary defense layer | Supporting layers |
|---|---|---|
| Huge declared length | Canonical decoder checks the length domain and applicable value limit before allocation | Network framing and transaction input bounds reduce exposure |
| Many maximum-size values | Collection and containing-resource aggregate budgets | Transaction and block aggregate validation |
| Nested amplification | Shared decoded-work, byte, element, and allocation budgets across nested decoders | Depth or structure rules if later specified |
| Invalid UTF-8 near the end | Borrowed validation after pre-allocation bounds, with failure-atomic decoding | Decoded-work and transaction budgets bound repeated cost |
| Allocation pressure | Pre-allocation length and aggregate checks | Peak-memory profile, network buffering limits, and bounded concurrency |
| Block propagation abuse | Block encoded-byte and aggregate-work validation | Network framing, propagation strategy, and peer policy |
| Repeated rejected transactions | Mempool and network rate, duplicate, and peer controls | Cheap canonical prechecks and transaction work limits |
| Persistent state bloat | Consensus state-growth accounting and hard state budgets | Persistent-state fees or other economic rules once formally defined |

The canonical decoder protects representation validity and must fail before an
unsafe allocation. Transaction validation protects one candidate transaction.
Mempool and network rules protect ingress and repeated invalid work. Block
validation protects aggregate consensus processing. State accounting protects
long-lived storage. Moving all defenses into any one layer leaves the others
exposed.

## Benchmark Plan

Benchmarks should establish reproducible methodology and distributions before
numeric budgets are proposed. They should cover both successful and adversarial
inputs and record at least:

- UTF-8 validation throughput, including invalid data detected early and late;
- canonical decoding throughput for fixed and variable-length structures;
- hashing throughput over representative and worst-case payload shapes;
- signature verification throughput for the eventually selected algorithms;
- end-to-end transaction validation work by resource category;
- peak memory during streaming, decoding, validation, and failure rollback;
- block propagation behavior across representative constrained networks; and
- persistent-state growth, synchronization, indexing, and migration effects.

The methodology should define input corpora, cache conditions, concurrency,
measurement boundaries, compiler settings, supported platforms, and reporting of
variance. It should include the proposed minimum node profile and stronger hosts,
plus supported 32-bit and 64-bit targets where applicable. Benchmarks inform
limits but do not replace deterministic accounting or conformance tests. This
document sets no performance target.

## Open Questions

All numeric values and the decisions needed to interpret them remain **TBD**,
including:

- `MAX_STRING_BYTES`, its boundary convention, and its protocol version;
- the canonical ULEB128 integer domain used for `String` lengths;
- maximum transaction encoded bytes;
- maximum block encoded bytes;
- collection element limits;
- cumulative variable-length bytes per collection, transaction, and block;
- decoded-work units and budgets;
- cryptographic-work accounting;
- state-read and state-write accounting;
- persistent-state budgets and attribution rules;
- network frame and buffering limits;
- the minimum compliant node profile and supported architectures;
- the relationship between hard budgets, relay policy, and fee policy;
- error taxonomy for resource-limit and host-capability failures; and
- activation and historical-validation rules for budget changes.

Transaction formats, block formats, state formats, execution semantics, fee
constants, and the Evolution Engine design also remain outside this proposal.

## Recommended Decision Sequence

The work should be finalized in this order:

1. Agree on the layered resource-budget architecture and accounting boundaries.
2. Ratify a reproducible benchmark methodology and minimum-profile evaluation.
3. Design the transaction and block formats and their validation workflows.
4. Select numeric per-layer budgets using threat analysis and benchmark evidence.
5. Publish independent conformance vectors, including boundary and composition cases.
6. Adopt the consensus rules through Formal Specification changes and the required HIP process.
7. Implement only after the normative domains, limits, errors, and version behavior are settled.

Implementation should not be used to resolve a normative TBD by precedent.

## String Implementation Readiness Gate

Implementation of canonical `String` serialization is safe to begin only after
all applicable items below are resolved authoritatively:

- [ ] The `String` ULEB128 length integer domain is specified.
- [ ] The protocol-wide maximum UTF-8 byte length and exact boundary convention are specified.
- [ ] Context-specific String ceilings required by the first consuming structures are specified.
- [ ] Aggregate variable-length byte, collection, transaction, and block budgets needed by those contexts are specified.
- [ ] The interaction between declared length, remaining input, UTF-8 validation, and pre-allocation rejection is specified.
- [ ] Failure atomicity for the input cursor and resource-budget state is specified.
- [ ] Fixed-width resource counter domains, checked arithmetic, and host-conversion behavior are specified.
- [ ] The minimum compliant node profile is compatible with every consensus-valid String.
- [ ] The error taxonomy distinguishes malformed encoding, invalid UTF-8, exhausted budgets, and implementation incapacity as required.
- [ ] Version activation and historical decoding behavior for the limits are specified.
- [ ] Literal, boundary, truncation, invalid UTF-8, aggregate, nesting, and cross-platform conformance vectors are approved.
- [ ] The relevant Formal Specification and required HIP/Super HIP changes are ratified.

Until every applicable gate is closed, `String` limits and their implementation
remain intentionally **TBD**.
