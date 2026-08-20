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
- Current stable implementation milestone: DCS-3A completed
- Next planned milestone: not yet designated

## DCS-1 Stable Baseline

- DCS-1 merge commit: `d7c1d3d`
- DCS-1 implementation commit: `e604a78`
- PR #1 merged
- Post-merge CI green

This section records the immutable DCS-1 milestone baseline, not the latest `main` branch HEAD.

## DCS-2 Stable Baseline

### DCS-2A

- DCS-2A implementation commit: `0f36be7`
- DCS-2A merge commit: `63f7468`
- PR #4 merged

### DCS-2B

- DCS-2B implementation commit: `996f11e`
- DCS-2B merge commit: `2164b53`
- PR #5 merged

This section records the immutable DCS-2 milestone baseline, not the latest `main` branch HEAD.

## DCS-3A Stable Baseline

- DCS-3A implementation commit: `2e03620`
- DCS-3A merge commit: `bc32660`
- PR #7 merged: `feat(serialization): implement Bool and Bytes<N> DCS-3A`

`DCS-3A` is an implementation/project-tracking label, not a Formal
Specification section or normative protocol designation. This section records
the immutable DCS-3A implementation baseline, not the latest `main` branch
HEAD.

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
- DCS-2A canonical fixed-width little-endian serialization for `u8`, `u16`,
  `u32`, `u64`, and `u128`
- Exact-width encoding of 1, 2, 4, 8, and 16 bytes for the DCS-2A unsigned
  integer primitives
- Strict rejection of truncated DCS-2A integer input with `UnexpectedEof`
- DCS-2A decoder cursors advance only after successful decoding and remain
  unchanged on decoding failure
- DCS-2B project-owned `U256` type with private canonical little-endian
  `[u8; 32]` storage
- `U256::from_le_bytes`, `U256::to_le_bytes`, `encode_u256`, and `decode_u256`
- Exact canonical 32-byte `U256` encoding, with every 32-byte bit pattern valid
- Truncated `U256` input rejected with `UnexpectedEof`, with the decoder cursor
  preserved on failure
- No `U256` arithmetic or external integer dependency added
- DCS-3A canonical `Bool` serialization: `false` encodes as `0x00`, `true`
  encodes as `0x01`, and bytes `0x02..=0xFF` are rejected
- Empty `Bool` input returns `UnexpectedEof`, while an invalid `Bool` encoding
  returns `InvalidBool`
- Successful `Bool` decoding consumes exactly one byte and leaves trailing
  input untouched; the decoder input remains unchanged on failure
- DCS-3A canonical `Bytes<N>` serialization as exactly N raw bytes, without a
  length prefix, ULEB128 prefix, or padding; every `[u8; N]` value is valid
- Successful `Bytes<N>` decoding consumes exactly N bytes and leaves trailing
  input untouched; insufficient input returns `UnexpectedEof` without changing
  the decoder cursor
- `Bytes<0>` encoding and decoding supported because the current Formal
  Specification does not prohibit N = 0
- The `dilithia-serialization` crate currently has 32 passing unit tests; this
  count records current implementation status and is not a protocol invariant
- Unsafe Rust forbidden at the `dilithia-serialization` crate root

## Current Serialization API

The public API below is derived from the current `dilithia-serialization`
source. `error` and `uleb128` are public modules; the fixed-width, `U256`,
`Bool`, and `Bytes<N>` items are re-exported at the crate root from private
modules.

### Shared error API

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationError {
    UnexpectedEof,
    InvalidBool,
    NonCanonicalUleb128,
    Uleb128Overflow,
}
```

`SerializationError` implements `core::fmt::Display` and `std::error::Error`.

### DCS-1 API

```rust
pub fn encode_uleb128_u64(value: u64) -> Vec<u8>;

pub fn decode_uleb128_u64(
    input: &mut &[u8],
) -> Result<u64, SerializationError>;
```

`encode_uleb128_u64` produces the unique minimal ULEB128 representation of a
`u64`. `decode_uleb128_u64` accepts one canonical ULEB128-encoded `u64` and
advances the supplied slice only on success.

### DCS-2A API

```rust
pub fn encode_u8(value: u8) -> [u8; 1];
pub fn encode_u16(value: u16) -> [u8; 2];
pub fn encode_u32(value: u32) -> [u8; 4];
pub fn encode_u64(value: u64) -> [u8; 8];
pub fn encode_u128(value: u128) -> [u8; 16];

pub fn decode_u8(input: &mut &[u8]) -> Result<u8, SerializationError>;
pub fn decode_u16(input: &mut &[u8]) -> Result<u16, SerializationError>;
pub fn decode_u32(input: &mut &[u8]) -> Result<u32, SerializationError>;
pub fn decode_u64(input: &mut &[u8]) -> Result<u64, SerializationError>;
pub fn decode_u128(input: &mut &[u8]) -> Result<u128, SerializationError>;
```

### DCS-2B API

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct U256 {
    bytes_le: [u8; 32], // private
}

impl U256 {
    pub const fn from_le_bytes(bytes: [u8; 32]) -> Self;
    pub const fn to_le_bytes(&self) -> [u8; 32];
}

pub fn encode_u256(value: &U256) -> [u8; 32];

pub fn decode_u256(
    input: &mut &[u8],
) -> Result<U256, SerializationError>;
```

### DCS-3A API

```rust
pub fn encode_bool(value: bool) -> [u8; 1];

pub fn decode_bool(
    input: &mut &[u8],
) -> Result<bool, SerializationError>;

pub fn encode_bytes<const N: usize>(
    value: &[u8; N],
) -> [u8; N];

pub fn decode_bytes<const N: usize>(
    input: &mut &[u8],
) -> Result<[u8; N], SerializationError>;
```

## Explicitly Not Implemented Yet

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

The next project milestone has not yet been designated by the authoritative
repository documents. This document does not assign a name, API, or scope to a
future milestone.

## Authority Order

1. Constitution
2. Formal Specification
3. Ratified HIP / Super HIP
4. Conformance vectors / tests
5. Implementation
6. This `PROJECT_STATE` document
7. AI conversations
