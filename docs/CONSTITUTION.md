# Dilithia Technical Constitution

Version: 1.0.0-rc1
Status: Pre-Genesis Ratified Draft (Frozen)

> **WARNING**
>
> This document defines the constitutional identity of the Dilithia Protocol.
> Unauthorized modification of this repository copy does not modify
> the Dilithia Protocol Constitution.
>
> Only amendments ratified through the Super HIP procedure (Article 12)
> are recognized as valid constitutional changes.

This document defines the supreme constitutional rules of the Dilithia protocol.
No code, no HIP, no Evolution Engine action, no community vote, and no emergency measure
may violate these Articles except through a valid Super HIP where amendment is
constitutionally permitted (Article 12).

## Article 1 — Immutability of Genesis
The Genesis block, its parameters, and the initial state defined at network launch
are permanently immutable. No mechanism may alter, rewrite, or reinterpret the Genesis state.

## Article 2 — Absolute Prohibition of Privileged Keys
No admin key, master key, emergency key, foundation key, or any form of privileged
cryptographic authority may ever exist in the protocol.
Any proposal that introduces such a key is automatically invalid.

## Article 3 — Fair Issuance and Monetary Integrity

There shall be zero pre-mine, zero foundation allocation, zero team allocation,
and zero reserved coins. This guarantee is permanent; per Article 12, it is not
subject to amendment through Super HIP or any other procedure.

Detailed monetary mechanics SHALL be defined by the Formal Specification.

Such mechanics may evolve only through procedures explicitly authorized by the
Formal Specification, and no change may introduce retroactive issuance,
privileged allocation, hidden issuance, or preferential monetary treatment.

## Article 4 — Supremacy of the Constitution over Upgrades
The Evolution Engine and all protocol upgrade mechanisms are strictly subordinate to this Constitution.
No ordinary protocol upgrade may modify, weaken, or circumvent any Article of this Constitution.

## Article 5 — Security Non-Degradation Principle
Any proposed change or implementation practice that reduces cryptographic strength, weakens consensus security,
or lowers the minimum security assumptions is automatically rejected, regardless of voting outcome.

## Article 6 — Canonical Serialization
All consensus-critical data shall use a canonical, versioned, and domain-separated serialization specification.
For each protocol version and data type, exactly one valid byte representation shall exist.
Previously valid canonical encodings shall remain unambiguously decodable or safely migratable under the Formal Specification.

## Article 7 — Long-term Ownership and State Preservation
Address formats and state representations must remain permanently migratable without loss of ownership.
No upgrade may render previously valid assets unreachable, unspendable, or un-migratable.

## Article 8 — Formal Specification Primacy
The Formal Specification is the authoritative definition of protocol behavior.
Any implementation (including the reference implementation) that diverges from
the Formal Specification is considered incorrect.

## Article 9 — Deterministic and Verifiable Upgrades
All protocol changes must follow the Dilithia Improvement Proposal (HIP) process.
Every upgrade must be:
- Fully deterministic
- Reproducible byte-for-byte by every node
- Subject to time-lock and multi-stage verification
- Rejectable by the Constitutional Guard if it violates any Article

## Article 10 — Independence of Implementations
The protocol must support multiple independent implementations in different languages.
Consensus correctness is defined solely by adherence to the Formal Specification
and this Constitution, not by any single codebase.

## Article 11 — Economic Safety Constraints
The protocol must maintain economic attack resistance.
Mechanisms that allow unbounded state growth, free spam, or permanent storage
without cost are prohibited unless compensated by explicit economic rules
(Storage Rent, Adaptive Fees, Dust Cleanup, etc.) defined in the Formal Specification.

## Article 12 — Amendment via Super HIP

This Constitution may only be amended through **Super HIP** — the special, higher-threshold protocol-upgrade track reserved for constitutional-level changes (exact numeric thresholds are defined in `SPECIFICATION.md`'s Governance section, not here). A Super HIP SHALL require, at minimum:
- a substantially higher approval threshold than an ordinary HIP,
- an extended public review period,
- mandatory independent security review,
- formal verification of all affected consensus-critical logic,
- a mandatory activation delay (time-lock).

The Super HIP process shall not directly or indirectly remove, weaken, bypass,
reinterpret, or render unenforceable Articles 1–5 or Article 13.

## Article 13 — Protocol Determinism
Every consensus-critical operation shall produce identical results on all compliant implementations.
Consensus correctness shall never depend on local clocks, floating-point arithmetic,
undefined behavior, operating system behavior, hardware architecture,
or implementation-specific optimizations.

End of Constitution
