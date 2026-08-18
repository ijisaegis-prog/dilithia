# Dilithia Project State

> **NON-NORMATIVE PROJECT STATUS DOCUMENT**
>
> This document records implementation progress only.
> It is not part of the Dilithia Protocol Specification.
> If this document conflicts with the Constitution, Formal Specification,
> or a ratified HIP/Super HIP, the authoritative protocol documents take precedence.

## Current Status

- Pre-Genesis
- Design & Early Implementation Phase
- Current stable milestone: DCS-1 completed
- Next planned milestone: DCS-2

## Current Stable Main

- `main` merge commit: `d7c1d3d`
- DCS-1 implementation commit: `e604a78`
- PR #1 merged
- Post-merge CI green

## Completed

- Rust workspace scaffold containing the guard, cryptography, serialization, core,
  consensus, P2P, and node crates
- Dilithia Technical Constitution, version 1.0.0-rc1, with status
  Pre-Genesis Ratified Draft (Frozen)
- Formal Specification draft sections 1–4: Protocol Identity and Scope,
  Determinism, Dilithia Canonical Serialization, and Genesis and Constitution
  Hash Anchoring
- Threat Model draft covering adversaries, trust assumptions, threats,
  non-goals, quantum-era threats, supply-chain integrity, trust boundaries, key
  compromise scenarios, and revisit triggers
- GitHub Actions CI with formatting, Clippy, and workspace test checks
- GitHub Actions checkout action updated to `actions/checkout@v6`
- DCS-1 canonical minimal ULEB128 encoding and decoding for `u64`
- Exact ULEB128 conformance vectors for `0`, `1`, `127`, `128`, `255`,
  `16383`, `16384`, `624485`, and `u64::MAX`
- Rejection of non-canonical ULEB128 encodings
- Rejection of empty and truncated ULEB128 input with `UnexpectedEof`
- Rejection of ULEB128 values that overflow `u64`
- Decoder input cursor changes only after successful decoding and remains
  unchanged on decoding errors
- Unsafe Rust forbidden at the `dilithia-serialization` crate root

## Current DCS-1 API

The public API currently exported by `dilithia-serialization` is:

```rust
pub enum SerializationError {
    UnexpectedEof,
    NonCanonicalUleb128,
    Uleb128Overflow,
}

pub fn encode_uleb128_u64(value: u64) -> Vec<u8>;

pub fn decode_uleb128_u64(
    input: &mut &[u8],
) -> Result<u64, SerializationError>;
```

`encode_uleb128_u64` produces the unique minimal ULEB128 representation of a
`u64`. `decode_uleb128_u64` accepts one canonical ULEB128-encoded `u64` and
advances the supplied slice only on success.

## Explicitly Not Implemented Yet

- Fixed-width DCS unsigned primitives (`u8`, `u16`, `u32`, `u64`, `u128`, and
  `u256`), which the Formal Specification defines as little-endian
- `Bool`
- `Bytes<N>`
- `String`, including its resource limit; the exact maximum byte length is
  **TBD**
- `Option<T>`
- `UnixTimestamp`
- `NetworkId` discriminant values are **TBD**
- `ChainId` representation is **TBD**
- Domain-tag registry is **TBD**
- Cryptographic primitives and crypto-agility runtime mechanisms
- Transaction formats and processing
- State model
- Consensus algorithm; the Byzantine threshold is **TBD**
- P2P implementation, including pending peer-scoring and discovery design
- Governance thresholds and the mechanical HIP/Super HIP process are **TBD**
- Evolution Engine runtime implementation
- Exact collection, transaction, and block resource limits are **TBD**

Formal Specification sections 5–10 remain pending or stubbed: Governance,
HIP / Super HIP Process, Crypto Agility, Transactions, State, and Consensus.

## Next Milestone

DCS-2 is the next planned milestone. The current Formal Specification §3.2
defines the unsigned integer primitives `u8`, `u16`, `u32`, `u64`, `u128`, and
`u256` as fixed-width, little-endian encodings. Any DCS-2 implementation must
remain deterministic and canonical under §3.1 and must not use the ULEB128
length-prefix encoding as the normal DCS representation of these integer
primitives.

The Formal Specification does not define a DCS-2 implementation API, detailed
milestone boundary, or conformance-vector set. Those details must be established
without resolving or inventing any specification item currently marked **TBD**.

## Authority Order

1. Constitution
2. Formal Specification
3. Ratified HIP / Super HIP
4. Conformance vectors / tests
5. Implementation
6. This `PROJECT_STATE` document
7. AI conversations
