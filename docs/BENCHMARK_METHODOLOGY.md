# Dilithia Benchmark Methodology

> **NON-NORMATIVE BENCHMARK METHODOLOGY — DOES NOT DEFINE CONSENSUS RULES**
>
> Benchmark results are evidence, not protocol rules. Runtime consensus must
> never depend on wall-clock timing, measured CPU cycles, allocator behavior,
> operating-system scheduling or load, thermal state, cache state, optional
> hardware acceleration, or benchmark results. Any future consensus limit
> informed by benchmark evidence must still be adopted through the authoritative
> protocol process.
>
> This document defines no consensus limit, performance target, hardware
> requirement, fee constant, transaction format, block format, state format,
> cryptographic algorithm, minimum node hardware, or maximum `String` length.
> All such decisions remain **TBD**.

## Authority Boundary

This document is subordinate to the repository's authority hierarchy:

1. Constitution
2. Formal Specification
3. Ratified HIP / Super HIP
4. Normatively adopted conformance vectors, where applicable
5. Implementation
6. `PROJECT_STATE.md`
7. Non-normative design and benchmark documents, including this document

Neither this methodology nor any report produced under it can override an
authoritative protocol document. A benchmark corpus is not a conformance corpus
unless it is separately adopted as such through the authoritative process.
Campaign registration, content hashes, signatures, and attestations provide
auditability and evidence provenance only. They create no protocol signing key,
privileged identity, governance authority, validator authority, or consensus
authority, and they cannot activate or alter a protocol rule.
Pre-run publication receipts establish evidence timing only. The registration
mechanism is not a Dilithia consensus service, and normal consensus validation
does not access or verify campaign manifests, receipts, timestamps, signatures,
or attestations.

## 1. Status and Scope

The repository can define a durable benchmark methodology now for provenance,
reproducibility, corpus construction, statistics, artifact publication, and the
currently implemented serialization primitives. These rules describe how to
collect and assess evidence; they do not determine which inputs are valid.

Complete benchmark campaigns for cryptography, transactions, blocks, networking,
and persistent state remain blocked by future protocol and implementation work.
The minimum compliant node profile also remains open because node roles,
transaction and block validation, state retention, synchronization, consensus
liveness, and networking requirements are not yet defined.

The methodology is intended to support the layered architecture proposed by
`RESOURCE_BUDGET_DESIGN.md`: per-value, per-collection, per-transaction,
per-block, network/message, decoded-work, and persistent-state resources remain
separate. Benchmark evidence should preserve those distinctions rather than
compressing every cost into one opaque score.

## 2. Benchmark Principles

Every benchmark campaign should follow these principles:

- **Reproducibility:** preserve enough source, build, workload, environment, and
  result metadata for an independent party to repeat the run.
- **Deterministic corpora:** fixed inputs must be content-addressed; generated
  inputs must use a versioned generator and recorded seed and output hash.
- **Conservative adversarial analysis:** include valid worst-shape inputs,
  malformed inputs, early and late failures, nesting, and aggregate composition.
- **Vendor neutrality:** no processor, storage product, cloud provider, operating
  system, compiler backend, or hardware accelerator is treated as the protocol.
- **Multiple implementations:** performance evidence should eventually include
  independently structured and cross-language implementations, while
  conformance remains identical across them.
- **Raw-data preservation:** publish raw samples and failures, not only summary
  charts or selected averages.
- **Explicit versioning:** methodology, harness, corpora, environment recipes,
  and reports must have traceable immutable versions.
- **Separation from consensus:** measured performance informs offline review;
  runtime validity uses only deterministic protocol-defined rules.

Correctness verification precedes performance interpretation. A fast benchmark
whose result is not checked is not evidence. A slower conforming implementation
does not change the validity of a protocol input.

## 3. Benchmark Purpose Categories

Benchmark campaigns must identify their purpose because the following uses have
different controls and interpretation:

| Purpose | Intended use | Not sufficient for |
|---|---|---|
| Protocol-limit evidence | Conservative offline evidence for future budget proposals | Automatic or runtime consensus changes |
| Performance regression analysis | Detect implementation changes against a controlled baseline | Declaring a protocol-invalid input |
| Implementation optimization | Compare behaviorally equivalent code paths | Defining canonical behavior by fastest implementation |
| Node monitoring | Diagnose deployment health and capacity | Consensus validation decisions |
| Fee/resource calibration evidence | Inform future relative economic-cost review | Creating fee constants or replacing hard safety limits |
| Network propagation studies | Evaluate relay behavior under controlled and real conditions | Defining network or block limits by timing alone |

Results collected for one purpose must not be silently reused for another. In
particular, a microbenchmark regression threshold is not a protocol safety
margin, and production telemetry is not a consensus oracle.

### Exploratory and Formal Campaigns

This methodology distinguishes two campaign classes:

- **Exploratory or development benchmarks** may be iterated freely for
  engineering, debugging, optimization, and methodology development. They must
  remain identifiable as exploratory and are not sufficient evidence for
  consensus-parameter selection.
- **Formal parameter-selection campaigns** use a frozen methodology version and
  a content-addressed manifest set rooted in a frozen pre-run campaign-definition
  manifest. Before evidence measurements begin, that definition predeclares the
  corpus, seeds and generation parameters, planned machines or reference classes,
  builds, sampling and stopping plan, exclusions, and analysis procedure.
  The exact content-addressed definition must also receive an independently
  verifiable external publication or registration receipt before those
  measurements begin.

Every run started under a formally registered campaign must receive a discoverable
run identity and remain in its evidence record. Its retained status must identify
whether it completed, failed, was aborted, was superseded, or deviated, with the
reason where applicable. This requirement applies only after an operator
explicitly registers a formal parameter-selection campaign; it does not require
registration or publication of informal developer experiments.

If a registered definition changes, the changed campaign receives a new
content-addressed identity and a new independently verifiable pre-run receipt.
The prior registered identity and its started runs remain discoverable and must
not be silently replaced or deleted. Registration controls evidence auditability
only and is not protocol versioning or protocol activation.

## 4. Prohibited Consensus Inputs

The following host observations must never determine runtime protocol validity or
deterministic resource charges:

- wall-clock elapsed time;
- measured runtime CPU cycles or instruction counts tied to a host;
- allocator timing or allocator success thresholds;
- thread scheduling, interleaving, or worker availability;
- operating-system load or process contention;
- thermal state, throttling, or current clock frequency;
- cold or warm cache state;
- optional SIMD, cryptographic, GPU, or other hardware acceleration;
- physical database page layout, compaction state, or write amplification; and
- host-specific allocation counts or allocation sizes.

These observations remain valuable benchmark diagnostics. They cannot be
protocol inputs because independent compliant nodes may observe different values
for the same canonical data.

## 5. Methodology Versioning

Each published methodology revision should receive a unique immutable version
identifier. A report records exactly one methodology version and must not claim
compatibility with another version unless the relationship is documented.

- Published reports remain immutable historical evidence.
- Corrections create a new report that identifies and supersedes the old report;
  the original remains available.
- Methodology changes create a new methodology version rather than silently
  changing old instructions.
- Corpus identifiers and hashes remain traceable across methodology versions.
- When practical, old and new methodologies should be run over an overlapping
  corpus and environment set to show how the change affects results.
- A change log should distinguish editorial clarification from changes to
  workload, measurement, statistics, or interpretation.
- A change to a registered campaign's methodology, corpus, seed, machines,
  build, sampling/stopping plan, exclusions, or analysis procedure creates a new
  campaign identity, requires a new pre-run receipt, and preserves the prior
  discoverable campaign record.

Methodology versions are non-normative evidence versions. They do not create or
replace protocol versions, activation rules, or Evolution Engine mechanics.

## 6. Required Reproducibility Metadata

Every report must include the following universal metadata.

### Formal Campaign Manifest and Provenance

A formal parameter-selection campaign must have a top-level, content-addressed
manifest set rooted in a campaign-definition manifest that is frozen before any
measurement intended as campaign evidence. The pre-run definition binds directly,
or by immutable hash or reference:

- methodology version;
- Dilithia source commit and repository dirty/clean state;
- the complete patch or diff when a dirty source tree is permitted;
- benchmark harness source and version;
- corpus manifest;
- corpus-generator source and version;
- deterministic seeds and generation parameters;
- analysis source and version;
- toolchain and build metadata;
- produced binary hashes;
- machine and environment profiles;
- sampling and stopping plan; and
- planned exclusion and deviation handling.

Before measurement begins, the exact campaign-definition identity must receive
an independently verifiable publication or registration receipt demonstrating
that it was externally committed before the campaign started. Mechanism-neutral
examples include an append-only public transparency record, a verifiable
publication receipt, an independently witnessed timestamped publication, or
another independently verifiable method. No provider, identity system,
timestamping system, blockchain, or technical mechanism is mandated.

After measurement begins, immutable run records and a final evidence manifest
must reference the frozen campaign-definition identity and bind:

- the independently verifiable pre-run receipt;
- raw result artifacts;
- exclusions, deviations, failed runs, and aborted runs; and
- summary and report artifacts.

Adding run and result records does not mutate the pre-run definition or reuse its
identity for a changed plan. This two-phase manifest structure binds the complete
evidence chain while preserving proof of what was registered before results were
known.

The registration record must keep every registered formal campaign identity
discoverable. Each run identity created under it must likewise remain discoverable
with its completed, failed, aborted, superseded, or deviated status. A changed
campaign definition cannot overwrite or delete the earlier identity or its run
history.

Formal campaigns should normally use a clean source tree. If a dirty tree is
necessary, the full diff must be preserved and bound into the campaign manifest;
recording only that the tree was dirty is insufficient. This is an evidence rule,
not a Git-based consensus requirement.

Formal parameter-selection evidence must have attributable provenance through a
cryptographic signature, verifiable build/run attestation, or another
independently verifiable mechanism. No particular vendor, identity system, or
package ecosystem is selected here. A signature or attestation authenticates
evidence provenance only; it has no protocol authority and creates no privileged
protocol key. Independent reproduction remains necessary evidence where
practical and must not be replaced by an attestation.

Registration establishes auditability only, receipts and timestamps establish
provenance timing only, and signatures or attestations establish evidence
provenance only. None creates a validator, governance, consensus, or activation
key; none can activate or modify protocol rules. Dilithia consensus nodes are not
required to retrieve or verify these artifacts during normal validation. Any
parameter change still requires the authoritative protocol-adoption process.

### Source and Build

- Dilithia source or revision identity and repository dirty/clean state;
- the complete dirty patch when applicable;
- benchmark harness identity and immutable version;
- dependency-lock or immutable dependency-manifest identity and hash where the
  implementation uses one;
- compiler, interpreter, virtual machine, runtime, or toolchain identity and
  version as applicable;
- build-system identity and version;
- target platform, triple, ABI, or equivalent as applicable;
- enabled implementation features and configuration;
- build-affecting environment flags and options;
- optimization and build profile or equivalent; and
- hashes of produced executables, libraries, runtime images, or equivalent
  benchmark artifacts.

If generated source, vendored code, patches, submodules, external toolchains, or
similar inputs participate, their exact provenance, versions, and hashes must be
recorded where applicable.
Compiler, linker, build-tool, and relevant system-library provenance must be
recorded to the extent that they can affect the produced binary. A binary hash
identifies an artifact but does not by itself prove how that artifact was built.

### Current Rust and Cargo Implementation

For reports benchmarking the current Rust implementation, the applicable
implementation-specific metadata must additionally include:

- `Cargo.lock` hash;
- full `rustc -Vv` output;
- Cargo version;
- Rust target triple and enabled target features;
- Cargo profile and relevant profile settings;
- enabled workspace and crate feature flags;
- `RUSTFLAGS` and other Rust build-affecting environment variables; and
- hashes of the produced Rust benchmark binaries or libraries.

These are evidence requirements for Rust/Cargo benchmark campaigns only. They
are not protocol rules and do not apply to an independent implementation written
in another language or using another build ecosystem.

### Corpus

- corpus version or immutable identifier;
- generator version when generation is used;
- deterministic seed;
- corpus manifest hash;
- case identifiers;
- hash or derivation record for each case; and
- expected validity, decoded result, or expected error classification.

### Execution

- warmup method and stopping rule;
- repetition and sampling method;
- process, thread, and concurrency configuration;
- CPU affinity or an explicit statement that affinity was not controlled;
- cache-state policy;
- batching method and operations represented by each sample; and
- instrumentation and timing source used for measurement.

### Hardware and Environment

- CPU model and physical/logical core topology;
- simultaneous-multithreading state;
- enabled instruction features;
- microcode version where available;
- installed RAM capacity;
- operating system and kernel version;
- virtualization or container state;
- power, frequency, and boost policy;
- observed thermal-throttling state; and
- cryptographic or other optional acceleration status.

### Results

- every raw sample with its unit and case identifier;
- failed, interrupted, or invalid runs;
- correctness-verification result;
- exclusions, the reason for each exclusion, and the responsible reviewer; and
- the exact analysis procedure and software version used to produce summaries.

The campaign must preserve the analysis source or immutable artifact hash,
configuration, exclusions, transformations, and statistical procedure needed to
reconstruct every published summary from raw samples. A summary is never the
only acceptable evidence artifact.

Universal metadata is required for every performance report. Workload-specific
metadata below becomes mandatory whenever that resource is measured.

## 7. Workload-Specific Metadata

| Workload | Additional required metadata |
|---|---|
| Memory | Allocator and version, instrumentation, memory topology, memory-speed class where available, page policy, process isolation, and concurrency |
| Storage | Device and interface, firmware where available, filesystem and mount settings, free space, cache policy, database version/configuration, fixture state, and compaction status |
| Network | NIC and link properties, topology, peer placement, transport/framing version, shaping profile, bandwidth, latency, loss, jitter, and geography where relevant |
| Cryptography | Algorithm and parameter-set identifier, implementation/backend, portable or accelerated mode, key/signature/proof sizes, preprocessing state, and corpus identity |

Background processes, thermal conditions, and environmental changes should be
recorded at enough detail to diagnose interference. Exact hardware serial numbers
or other privacy-sensitive identifiers are not required.

### Evidence-Tool and Supply-Chain Integrity

For a formal parameter-selection campaign, the benchmark harness, corpus
generators, report-schema tooling, and analysis scripts must have:

- pinned source and versions;
- pinned dependencies or an immutable record of every dependency;
- review appropriate to the evidence criticality;
- validated input and output schemas;
- deterministic inputs or complete records of nondeterministic inputs;
- checked arithmetic for byte, sample, operation, and count aggregation;
- explicit overflow detection that fails the campaign rather than wrapping;
- self-consistency checks between manifests, cases, samples, and summaries; and
- correctness checks for benchmark outputs.

Where practical, separately reviewed tooling should independently reconstruct
the published summaries from the preserved raw samples. Independent correctness
oracles should validate decoded values, errors, cursor behavior, and future
deterministic accounting where such oracles exist. A second protocol
implementation is not required before one exists, but shared corpora should be
used for differential parsing and accounting once independent implementations
are available.

Harness, generator, analysis, compiler, and build dependencies are part of the
benchmark evidence supply chain. Their provenance and review protect evidence
integrity only; they create no protocol authority. Allocation counts, tool
timings, and other implementation observations remain non-consensus evidence.

## 8. Minimum Compliant Node Evaluation Framework

This methodology selects no minimum node. The preferred future evaluation model
is a hybrid with three evidence layers:

- an abstract capability envelope describing the resources a node role must
  sustain;
- multiple non-normative reference hardware classes used to test that envelope; and
- exact records for every benchmark machine used to produce evidence.

Reference classes should span multiple vendors and architectures where practical.
An exact processor, computer, drive, cloud instance, or other hardware SKU must
never become a consensus rule.

The abstract capability envelope must preserve separately auditable resource
dimensions. It must not be defined solely by one aggregate score produced by the
same benchmark suite used to justify the envelope. Strong performance in one
resource must not silently compensate for an unsafe deficiency in another unless
a future authoritative protocol design explicitly permits that tradeoff.

The eventual evaluation must keep these capabilities separate:

- consensus-correct processing of every valid input;
- worst-case peak memory;
- sustained transaction and block validation throughput;
- cryptographic throughput;
- storage capacity and sustained storage behavior;
- initial and ongoing synchronization;
- sustained network bandwidth and constrained-peer behavior;
- latency or liveness-related capability required by future consensus design; and
- operational reliability under memory, storage, thermal, and restart pressure.

Correctness capability does not mean that a host is operationally able to keep up
with an active network. Conversely, a high-performance host does not gain a
different interpretation of consensus rules. The profile remains **TBD** until
node roles and protocol workloads exist.

A formal minimum-profile evaluation must preserve evidence from planned qualified
reference systems, the least-capable qualifying systems identified by the
campaign, and systems that fail or cannot complete required cases. Excluding a
failed or slow system from a published conclusion without retaining its evidence
is not permitted within that formal campaign.

## 9. Architecture and Platform Evaluation

This methodology does not choose 32-bit support, 64-bit support, or any other
target architecture policy.

Architecture-related parameter campaigns should proceed only after the applicable
authoritative protocol design defines, or explicitly leaves for later decision,
the following matters:

- the fixed-width domains and checked arithmetic for consensus-critical lengths,
  charges, and counters;
- checked conversion to `usize`, allocation sizes, or other host API types;
- identical validity, decoded values, and deterministic accounting across every
  supported target; and
- an architecture-support decision based on protocol requirements and capability
  evidence rather than implementation convenience.

The benchmark methodology evaluates those authoritative definitions; it does not
create them or infer missing protocol semantics from benchmark results.

Architecture width is not a substitute for a node profile: it does not establish
CPU, memory, storage, network, or cryptographic capacity. If multiple widths are
later supported, cross-target conformance and worst-case capability evidence will
be required. If a width is excluded, that decision still must not create
architecture-dependent consensus semantics.

## 10. Corpus Architecture

Every corpus should have an immutable, content-addressed manifest. The manifest
records:

- corpus identifier and version;
- methodology compatibility;
- literal bytes for small consensus-sensitive vectors;
- generator identifier and version for generated cases;
- deterministic seed and generation parameters;
- resulting case and corpus hashes;
- validity expectation;
- decoded-value or error expectation; and
- expected cursor and deterministic resource-counter state when those semantics
  eventually exist.

A deterministic seed is insufficient by itself because a changed generator can
produce different output. Both the generator version and output hash are needed.
Small boundary and malformed cases should be stored literally when practical.

Corpus changes create a new identity; old reports retain their original corpus.
Adversarial cases should receive independent review. Public corpora should be
supplemented by independently designed audit cases to reduce benchmark overfitting,
then published after the evaluated implementation is frozen so findings remain
reproducible.

For formal campaigns, the corpus manifest, generator source, dependencies,
seeds, generation parameters, and expected outcomes are frozen before campaign
measurement. Independent correctness oracles should be used where practical.
When multiple implementations exist, shared corpora should be used for
differential parsing, error, cursor, and deterministic-accounting comparison.

## 11. Generic Case Classes

Benchmark suites should compose cases from these conceptual classes:

- minimum valid input;
- representative valid input;
- candidate-boundary cases;
- malformed canonical encoding;
- truncated input;
- failure detected early;
- failure detected late;
- adversarial nesting;
- many individually valid values;
- repeated rejected inputs;
- multiple fields that amplify aggregate resource use;
- byte-heavy input;
- decoded-work-heavy input;
- memory-heavy input;
- cryptographic-work-heavy input;
- state-heavy input; and
- mixed adversarial input chosen to stress interacting resources.

Any experimental candidate size must be labeled **NON-NORMATIVE BENCHMARK
PARAMETER** in its manifest and report. Candidate sizes are experiment inputs,
not proposed or adopted protocol limits.

Average-case, representative, boundary, and adversarial cases must remain
distinguishable in raw data and summaries.

## 12. Current Serialization Benchmark Matrix

The following scenarios can be defined from the current implementation without
adding a new DCS rule or claiming an absent resource limit.

| Codec | Success and boundary scenarios | Failure and adversarial scenarios |
|---|---|---|
| Canonical ULEB128 `u64` | Values at every encoded-width transition, representative values, largest representable value, encode and decode separately | Non-minimal forms, every meaningful truncation, overflow forms, early versus late failure, cursor preservation |
| Fixed-width integers | Every implemented width, zero/one/maximum/representative values, exact byte throughput, encode and decode separately | Every truncated length, failure atomicity, trailing-input preservation |
| `U256` | Fixed literal bit patterns, encode/decode copying, exact-width success | Every truncation, cursor preservation, trailing-input preservation |
| Bool | Both canonical values, encode/decode, trailing input | Empty input, every non-canonical byte, cursor preservation |
| `Bytes<N>` | Empty and positive lengths, multiple experimental `N` classes, exact copy throughput, trailing input | Every truncation for each selected `N`, cursor preservation |
| UnixTimestamp | Representative values and comparison with the delegated fixed-width `u64` codec | Every truncation, cursor preservation |
| `Option<T>` | `None`, `Some` over several current codecs, nested options, encode/decode composition | Invalid tag, nested early/late failure, nested cursor rollback, fallible encoder propagation |

The timed region must exclude corpus construction and expected-result creation.
Every benchmark must verify the returned value, error, cursor, and produced bytes
so compiler elimination or a broken fast path cannot appear successful.

For each codec, collect encode/decode throughput, operations, bytes processed,
peak memory, allocations as an implementation diagnostic, successful and failed
path cost, and failure-atomicity overhead. Do not convert these timings into
consensus work units under this methodology.

When an independent implementation or oracle exists, the same literal and
generated corpora should be run through it and compared for encoded bytes,
decoded values, error classifications, cursor behavior, and any future
authoritative deterministic accounting. The absence of a second implementation
today does not block current exploratory serialization benchmarks.

## 13. Future String and UTF-8 Benchmark Matrix

`MAX_STRING_BYTES` and the `String` ULEB128 length domain remain **TBD**. Future
String experiments should include:

- ASCII;
- valid two-byte UTF-8 sequences;
- valid three-byte UTF-8 sequences;
- valid four-byte UTF-8 sequences;
- mixed-width UTF-8;
- valid inputs across explicitly non-normative candidate size classes;
- invalid UTF-8 near the beginning;
- invalid UTF-8 near the end;
- incomplete UTF-8 sequences;
- malformed or non-canonical ULEB128 prefixes;
- truncated payloads;
- a huge declared length with a tiny payload;
- multiple Strings stressing future aggregate budgets;
- borrowed UTF-8 validation without allocation;
- owned decoding and allocation; and
- complete cursor and resource-budget rollback on failure.

UTF-8 byte length is the relevant resource dimension because wire bandwidth,
input scanning, slicing, and allocation operate on bytes. Character, scalar, or
grapheme counts are not substitutes.

Unicode normalization is outside DCS unless a future semantic rule explicitly
introduces it. This methodology therefore requires no normalization benchmark for
canonical String decoding.

## 14. Future Cryptographic Benchmark Framework

No algorithm is selected here. A future algorithm-neutral suite should support:

- hashing over varied message shapes;
- signature generation where operationally relevant;
- valid signature verification;
- malformed or invalid signature verification;
- key, signature, and proof parsing;
- post-quantum algorithms and their larger artifacts;
- proof verification if introduced; and
- batch verification with valid, invalid, and mixed batches.

Every report must identify algorithm, parameter-set version, corpus, key and
artifact sizes, implementation backend, preprocessing, and portable versus
accelerated mode. Cold and warm states should be measured separately.

Algorithm-independent resource categories may describe hashing, verification,
or parsing conceptually. Evidence remains algorithm-specific. Crypto Agility
requires re-evaluation before a new or changed algorithm informs any future
budget or fee proposal. Optional acceleration must not be hidden or assumed.

## 15. Future Transaction Benchmark Framework

No transaction format is proposed. Future scenario categories should include:

- minimal transfer-like operation;
- byte-heavy transaction;
- cryptographic-work-heavy transaction;
- state-read-heavy transaction;
- state-write-heavy transaction;
- persistent-state-growth-heavy transaction;
- nested variable-length data;
- multiple individually valid maximum-cost fields;
- malformed transaction rejected early;
- malformed transaction rejected late; and
- mixed adversarial transaction.

Each case should eventually report a resource vector covering encoded and decoded
bytes, validation work, cryptographic work, state access, temporary memory, and
persistent-state delta.

One transactions-per-second number is insufficient because it hides transaction
composition, batching, concurrency, rejection cost, storage state, and which
resource is saturated. Throughput must remain associated with the exact workload.

## 16. Future Block Benchmark Framework

No block format is proposed. Future evaluation should cover:

- byte-heavy blocks;
- many small transactions;
- fewer expensive transactions;
- cryptographic-work-heavy blocks;
- state-heavy and persistent-growth-heavy blocks;
- mixed adversarial composition;
- peak temporary memory;
- full validation latency;
- propagation behavior;
- a serial deterministic validation baseline; and
- the parallel production implementation.

Serial and parallel paths must produce identical validity, decoded results, and
deterministic accounting. Parallel speedup and scheduling remain implementation
measurements.

Average and adversarial block cases must be reported separately. A representative
average mix cannot establish safety under a proposer-selected worst-case mix.

## 17. Network Methodology

Network studies should combine controlled simulation or traffic shaping with
real distributed-node testing.

Controlled tests provide reproducible variation in:

- bandwidth;
- latency;
- packet loss;
- jitter;
- slow peers;
- duplicate or repeated traffic;
- concurrent transaction and block relay; and
- serialization and framing overhead.

Distributed tests provide evidence across geographic placement, providers,
routing paths, last-mile conditions, and heterogeneous peers. Reports should
record topology, peer placement, shaping, payload bytes, on-wire bytes,
retransmissions, duplication, resource use, and propagation distributions.

LAN-only results or a single cloud environment are not sufficient evidence for
network-wide assumptions. This methodology defines no network limit, propagation
target, or liveness threshold.

Local relay, admission, peer, or mempool policy may be stricter than consensus
rules where safe, but such local policy must not redefine canonical transaction
or block validity. This sentence defines no current transaction or mempool
semantics.

## 18. Memory Methodology

Implementation memory measurements should include:

- peak RSS or the platform-equivalent process footprint;
- total heap bytes allocated;
- peak live heap;
- allocation count;
- temporary input, output, and validation buffers;
- retained decoded objects;
- per-transaction temporary memory when transactions exist;
- per-block temporary memory when blocks exist;
- nested-decoder memory;
- successful and failure-path memory; and
- memory retained after rejection or allocator caching.

High-water measurements should use isolated processes where practical so one
case does not contaminate another. Reports must record allocator, instrumentation,
OS, page policy, concurrency, and cache state.

RSS and allocations differ across allocators, languages, operating systems, and
optimization strategies. They are implementation measurements, not direct
consensus-accounting units.

## 19. Persistent-State Methodology

Once a state model and storage implementations exist, benchmark:

- state creation;
- modification;
- deletion;
- logical state delta;
- physical database growth;
- write amplification;
- indexing;
- compaction;
- synchronization;
- snapshot and restore;
- crash recovery;
- migration; and
- historical retention or pruning.

Use versioned, hashed fixtures representing fresh, populated, aged, fragmented,
and growth-heavy states. Record database version/configuration, cache state,
compaction state, filesystem, storage device, and fixture identity.

For a state-resource parameter campaign to be ready, the authoritative protocol
specification must first define any consensus-relevant state accounting in
storage-engine-independent logical terms. The methodology may then compare
physical pages, indexes, journals, compaction, and write amplification as
implementation evidence. It must not infer a consensus rule from those physical
measurements. Permanent and transient costs should remain separately visible in
benchmark reports without this document defining their protocol treatment.

## 20. Statistics Policy

Every campaign should predeclare its warmup, repetition, sampling, grouping,
analysis, and exclusion procedures. It should include independent runs across
multiple machines and times where appropriate.

For a formal parameter-selection campaign, these procedures, the stopping rule,
the corpus and seeds, the planned machine set, and the analysis configuration are
frozen in the content-addressed campaign-definition manifest and covered by its
independently verifiable pre-run receipt before evidence measurements. Every run
started under that identity remains discoverable and disclosed, including
failed, aborted, superseded, or deviating runs. A changed plan creates a new
campaign identity and pre-run receipt rather than silently replacing the original.

Reports should retain and present:

- all raw samples;
- median and mean;
- upper-tail percentiles;
- variance or another declared dispersion measure;
- confidence intervals;
- minimum and maximum observed results;
- failures and interrupted runs; and
- results grouped by machine, implementation, environment, and workload class.

No outlier may be silently removed. Any exclusion must be justified, retained in
raw data, and shown in analysis with enough information to reproduce both the
included and excluded views.

Published summaries must be reconstructable from the raw samples using the
recorded analysis version or hash, configuration, exclusions, transformations,
and statistical procedure. Independent re-analysis with separately reviewed
tooling should be included where practical.

Average-case and adversarial worst-case results must be reported separately. A
mean alone hides tails; a single worst-observed result is not a proven upper
bound. Reports must avoid pseudo-precision and match displayed precision to
measurement uncertainty.

This methodology does not define fixed sample counts, selected percentile
targets, confidence levels, safety margins, or pass/fail performance thresholds.

## 21. Benchmark-Gaming Defenses

| Bias or gaming risk | Required defense |
|---|---|
| Favorable-only workloads | Mandatory representative, boundary, rejection, and adversarial corpus classes |
| Hardware cherry-picking | Multiple vendors, architectures, machines, and independent operators where practical |
| Hidden acceleration | Disclose target features and publish portable and accelerated modes separately |
| Warm-cache-only testing | Distinct cold, warm, and steady-state policies |
| Ignored rejection paths | Mandatory early, late, repeated, truncated, and nested failure cases |
| LAN-only networking | Controlled impaired links and real distributed tests |
| Compiler-specific tuning | Record compiler/build artifacts and compare relevant portable configurations |
| Algorithm-specific crypto tuning | Algorithm/version-specific reports and Crypto Agility re-evaluation |
| Mean-only reporting | Raw distributions, tails, dispersion, and failures |
| Harness-specific shortcuts | Verify outputs and review benchmark-only code paths |
| Compromised or erroneous evidence tooling | Pin and review harness/generator/analysis source and dependencies; validate schemas; use checked arithmetic and independent re-analysis |
| Selective campaign publication or seed shopping | Obtain an independently verifiable pre-run receipt for the frozen formal campaign definition and preserve discoverable identities and statuses for every run started under it |
| Post-result methodology changes | Create a new campaign identity and preserve the prior registered campaign and results |
| Corpus overfitting | Independent adversarial review and post-freeze audit cases |
| Concurrency or oversubscription bias | Disclose process/thread counts, affinity, scheduler-sensitive settings, and oversubscription; retain a serial baseline where meaningful |

Result-selection criteria and exclusions should be declared before results are
examined. Published raw artifacts allow independent reviewers to detect bias that
summary reports miss.

Concurrent reports must disclose process and thread counts, CPU affinity policy,
oversubscription, and scheduler-sensitive configuration. Where meaningful, a
serial or single-thread baseline should be retained to interpret parallel
results. Parallel performance remains non-normative implementation evidence.

## 22. Deterministic Resource Accounting Relationship

Benchmark evidence may eventually inform categories such as:

- encoded bytes;
- decoded logical bytes;
- element count;
- hash operations or inputs under a future definition;
- algorithm-versioned cryptographic operations;
- logical state reads;
- logical state writes;
- persistent logical bytes; and
- future execution units if an execution model is adopted.

This document defines no weights, conversion ratios, counters, or budgets for
those categories.

For a parameter-selection campaign to be considered ready, the authoritative
protocol specification must already define how the relevant deterministic unit
is computed, including its arithmetic, version context, and composition behavior,
or identify those matters as unresolved. Benchmark methodology should evaluate
only the resulting protocol-defined unit and must not invent missing weights,
budgets, or semantics. Runtime consensus counts only authoritative
protocol-defined units, never benchmark timing or host observations.

## 23. Fee Relationship

No fee formula, schedule, constant, or adjustment mechanism is defined here.
Benchmark evidence may later inform relative resource-pricing review while
preserving these architectural distinctions:

- hard safety limits remain separate from economic charges;
- simple DLTH transfers retain a low-cost path;
- resource-heavy features pay for their own use; and
- persistent-state cost remains distinct from transient encoded bytes.

The ability to pay must never permit an otherwise unsafe resource load. Any fee
rule that affects state transition or validity requires the authoritative
protocol process independently of this methodology.

## 24. CI Versus Dedicated Benchmarking

Normal CI should remain fast and reliable. If benchmark infrastructure is later
added, normal CI may cover:

- correctness, unit, and conformance tests;
- compilation of benchmark targets without a full campaign;
- corpus manifest and hash integrity;
- deterministic generator checks; and
- tiny functional smoke cases that verify benchmark outputs.

Dedicated infrastructure should cover:

- stable-machine performance regression runs;
- long adversarial suites;
- memory and storage instrumentation;
- scheduled historical reports; and
- multi-machine protocol-limit evaluation campaigns.

Manual release or protocol-parameter campaigns should add independent machines,
implementations, reviewers, and distributed network environments.

Timing from a hosted CI runner must never become a consensus input or a hard
protocol-performance gate. Shared-runner performance may be diagnostic only.

## 25. Benchmark Artifacts

Future campaigns should publish or commit, as appropriate:

- top-level content-addressed campaign manifests;
- independently verifiable pre-run publication or registration receipts;
- discoverable campaign and run-identity records with retained statuses;
- methodology versions;
- machine-readable report schemas;
- benchmark source;
- literal corpora;
- deterministic corpus generators;
- manifests and content hashes;
- environment and build recipes;
- execution and analysis scripts;
- raw data;
- summarized reports;
- benchmark-machine profiles;
- binary hashes;
- attributable signatures or attestations for evidence provenance; and
- independent reproduction reports.

These artifacts are **NON-NORMATIVE EVIDENCE** unless an authoritative protocol
document separately adopts a particular artifact for a normative purpose.
Benchmark source, reports, machine profiles, and measured timing do not define
protocol validity.

The top-level manifest should bind the evidence chain from source and tooling to
the pre-run receipt, raw samples, and summaries. The summary cannot replace raw
evidence. Signatures, attestations, registrations, timestamps, receipts, and
independent reproduction establish provenance, provenance timing, and
auditability only; none is a consensus authority or can activate a protocol
change. Normal consensus validation does not depend on these artifacts.

## 26. Preconditions for Numeric Resource Limits

All listed values remain **TBD**. Before they can responsibly be selected, the
following dependencies must exist:

| Future decision | Required preceding work |
|---|---|
| `MAX_STRING_BYTES` | String length domain, boundary semantics, failure behavior, aggregate contexts, candidate corpus, minimum-profile evidence, and transaction/block containment model |
| Transaction encoded-byte limit | Transaction format, validation workflow, cryptographic/state semantics, rejection behavior, and relay assumptions |
| Block encoded-byte limit | Block format, transaction composition, consensus/liveness design, validation workflow, propagation evidence, and peak-memory evidence |
| Collection limits | Canonical collection format, element and nesting semantics, aggregate accounting, and consuming contexts |
| Decoded-work budget | Deterministic unit taxonomy, checked counter behavior, adversarial corpus, and conformance vectors |
| Cryptographic-work budget | Selected/versioned algorithms and parameters, portable implementation evidence, malformed-input behavior, and batch semantics |
| Minimum compliant node profile | Node roles, architecture policy, transaction/block workload, state retention, synchronization, networking, liveness, and reliability expectations |

Every future selection also requires a frozen methodology version, reproducible
corpora, multiple machines and implementations where practical, raw results,
uncertainty analysis, threat review, explicit conservative reasoning, and the
authoritative protocol process.

Transaction and block protocol design remains necessary before many numeric
limits can be meaningfully evaluated. Benchmark methodology alone is not enough.

## 27. Next Design Stage

After this methodology framework is reviewed, the next design focus should be
transaction and block resource architecture. That work should identify resource
boundaries and validation workflows without prematurely selecting formats or
numeric limits.

The methodology can then be extended with protocol-specific corpora and workload
definitions as those designs become authoritative enough to benchmark. Actual
parameter-selection campaigns begin only after the readiness checklist below is
satisfied for the parameter under review.

## 28. Open Questions

The following remain unresolved:

- the methodology version identifier and publication process;
- campaign, report, and corpus manifest schemas;
- the mechanism-neutral pre-run registration, receipt, provenance, and
  attestation process;
- benchmark harness and measurement tools;
- evidence-tool dependency and supply-chain review process;
- independent-review and reproduction process;
- acceptable qualification criteria for independent implementations;
- minimum node roles and abstract capability categories;
- supported target architectures;
- reference hardware-class selection and refresh process;
- workload-specific instrumentation choices;
- statistical sample design and uncertainty policy;
- adversarial corpus review and post-freeze audit procedure;
- deterministic decoded-work units;
- algorithm-versioned cryptographic-work units;
- storage-engine-independent state accounting;
- transaction and block resource architecture;
- network and liveness evaluation requirements;
- `String` length domain and `MAX_STRING_BYTES`;
- collection, transaction, block, network, and persistent-state limits;
- fee/resource-pricing architecture; and
- protocol activation and historical-validation behavior for future limits.

No item in this list is resolved by this document.

## 29. Non-Normative Parameter-Campaign Readiness Checklist

This checklist governs readiness to collect parameter-selection evidence. It
does not define protocol validity or approve any parameter.

- [ ] The candidate parameter's protocol resource and accounting boundary are defined for review.
- [ ] All prerequisite formats and validation workflows needed to construct representative workloads exist.
- [ ] The methodology version and report schema are frozen for the campaign.
- [ ] A top-level content-addressed manifest set is rooted in a campaign-definition manifest frozen before evidence measurement.
- [ ] The exact frozen definition identity has an independently verifiable pre-run publication or registration receipt.
- [ ] Sampling, stopping, exclusion, seed, machine, build, corpus, and analysis plans are pre-registered in that definition manifest.
- [ ] The final evidence manifest references and binds the pre-run receipt.
- [ ] Registered formal campaign identities remain discoverable and cannot be silently replaced or deleted.
- [ ] Every run started under the formal campaign has a discoverable identity and retained completed, failed, aborted, superseded, or deviated status.
- [ ] Immutable run records and the final evidence manifest reference the frozen campaign identity and bind every raw and summary artifact.
- [ ] Corpus manifests, literal vectors, generators, seeds, and hashes are reviewed.
- [ ] Correctness, expected errors, cursor behavior, and deterministic accounting expectations are verified independently.
- [ ] Representative, boundary, malformed, early-failure, late-failure, aggregate, and mixed adversarial cases are included.
- [ ] Every experimental size is labeled `NON-NORMATIVE BENCHMARK PARAMETER`.
- [ ] The applicable universal and implementation-specific source, toolchain, build, artifact, execution, and environment metadata are complete.
- [ ] Clean source is used, or the complete dirty diff is preserved and bound into the campaign manifest.
- [ ] Harness, generator, schema, analysis, compiler, build, and dependency provenance is pinned or immutably recorded and reviewed according to campaign criticality.
- [ ] Evidence-tool aggregation uses checked arithmetic and treats overflow as campaign failure.
- [ ] The minimum-node evaluation framework and relevant abstract capability categories are defined.
- [ ] Multiple benchmark machines, vendors, architectures, and implementations are included where applicable.
- [ ] Portable and accelerated modes are separated and disclosed.
- [ ] Memory, storage, network, or cryptographic instrumentation required by the campaign is validated.
- [ ] Warmup, sampling, repetition, exclusion, and statistical analysis procedures are predeclared.
- [ ] Every run started under the registered campaign, including aborts, failures, superseded runs, and deviations, will be disclosed with reasons.
- [ ] Raw samples, failures, exclusions, scripts, transformations, and environment recipes will be preserved.
- [ ] Published summaries can be reconstructed independently from raw evidence using recorded analysis tooling and configuration.
- [ ] Attributable evidence provenance is provided by a signature, attestation, or another independently verifiable mechanism with no protocol authority.
- [ ] Evidence-tool and dependency supply-chain review is complete at a level appropriate to the campaign.
- [ ] Independent adversarial review and reproduction are planned.
- [ ] Benchmark evidence is explicitly separated from deterministic runtime accounting.
- [ ] The process for converting evidence into a proposal includes threat review, conservative uncertainty treatment, conformance vectors, and authoritative protocol adoption.
- [ ] No result will silently modify an existing protocol version or historical validity rule.

Until all applicable readiness items are satisfied, benchmark results may support
exploration and implementation regression work but are not sufficient evidence
for selecting a consensus-critical resource parameter.
