# Dilithia Formal Specification

**Status:** DRAFT v0.1 — Pre-Genesis. Sections 1–4 drafted below; remaining sections are stubs pending design.
**Companion documents:** `CONSTITUTION.md` (what MUST NOT change), `THREAT_MODEL.md` (why these rules exist).

Per `CONSTITUTION.md` Article 8 (Formal Specification Primacy), this document — not any single implementation — is the authoritative definition of protocol behavior.

## 1. Protocol Identity and Scope

Dilithia is an independent Layer-1 blockchain protocol with a native coin, DLTH, built on post-quantum cryptography (PQC) from Genesis. Its identity is defined by `CONSTITUTION.md`, not by any single cryptographic algorithm — see Crypto Agility (§7, pending).

This document defines *how* the protocol behaves. `CONSTITUTION.md` defines what MUST NOT change; this document may evolve through the ordinary HIP process, except where a specific clause is itself protected by a Constitutional Article, in which case that clause may only change via Super HIP.

## 2. Determinism

Per `CONSTITUTION.md` Article 13, every consensus-critical operation SHALL produce identical results on all compliant implementations, regardless of local clock, floating-point arithmetic, undefined behavior, OS, hardware architecture, or implementation-specific optimization.

This is enforced structurally, not just by convention: §3.2 reserves and prohibits the primitive types (floating point, platform-dependent integers, etc.) that are the most common real-world source of cross-implementation divergence. A type that is not in the canonical type system MUST NOT appear in any consensus-critical structure.

## 3. Dilithia Canonical Serialization (DCS)

### 3.1 Serialization Invariants

- For each protocol version and data type, **exactly one valid byte representation SHALL exist** (per `CONSTITUTION.md` Article 6).
- Encoding SHALL be deterministic: the same logical value SHALL always produce the same bytes.
- Decoding SHALL be strict: any byte sequence that does not match the canonical form for its type SHALL be rejected, never repaired or reinterpreted (see §3.5).
- No type in this specification may be encoded with redundant information (e.g. no length prefix where length is already fixed; see `Bytes<N>` below).

### 3.2 Primitive Types

| Type | Encoding |
|---|---|
| `u8`, `u16`, `u32`, `u64`, `u128`, `u256` | Fixed-width, little-endian. |
| `Bool` | Single byte: `0x00` = false, `0x01` = true. Any other byte value SHALL be rejected on decode. |
| `Bytes<N>` | Exactly N raw bytes. No length prefix. No padding. No truncation. |
| `String` | ULEB128 length prefix, followed by that many UTF-8 bytes. Invalid UTF-8 SHALL be rejected. |
| `Option<T>` | One tag byte (`0x00` = None, `0x01` = Some) followed by the encoded `T` if Some. |
| `UnixTimestamp` | `u64`, milliseconds since Unix epoch, UTC. |
| `NetworkId` | Fixed enumerated discriminant — `Mainnet`, `Testnet`, `Devnet` (exact byte values TBD in §3.3) — included in every signed structure to prevent cross-network replay. |
| `ChainId` | TBD — defined alongside `NetworkId` in §3.3 once the consensus algorithm's chain-identification needs are finalized. |

**ULEB128** (Unsigned Little Endian Base 128): each byte encodes 7 bits of the value in its low bits; the high bit is a continuation flag (1 = more bytes follow, 0 = last byte). Canonical ULEB128 SHALL use the minimal number of bytes — a non-minimal encoding (e.g. a trailing zero continuation byte) is invalid and SHALL be rejected on decode.

#### Reserved Types

The following are reserved for future protocol versions and MUST NOT appear in any consensus-critical structure unless explicitly standardized through the HIP process:

- Decimal
- Floating Point (`f32`, `f64`)
- BigInt (arbitrary-precision integer)
- Platform-dependent integer (e.g. `usize`, `isize`)

Floating-point types specifically are prohibited in consensus-critical structures under all circumstances — this is not merely reserved-for-later, it is a permanent exclusion, since IEEE 754 arithmetic is not guaranteed bit-identical across platforms and compiler optimization levels.

### 3.3 Composite Types

Structs and enums are composed from the primitives in §3.2, encoded field-by-field in the order they are declared in this specification — not the order declared in any implementation's source code.

Enum variants SHALL be identified by a **fixed discriminant** assigned in this specification, not derived from declaration order in any implementation. Adding a new variant to an existing enum in a future version is an ordinary HIP; renumbering an existing discriminant is a breaking change regardless of tier.

*(Exact discriminant values for `NetworkId`, `ChainId`, and other enums are TBD — to be assigned when `dilithia-serialization` implementation begins.)*

### 3.4 Domain Separation

Every hash or signature computed over DCS-encoded bytes SHALL be computed over `domain_tag || canonical_bytes`, where `domain_tag` is a fixed, purpose-specific byte string unique to that hash or signature's context (e.g. distinct tags for "transaction signature" vs. "block hash" vs. "constitution hash"). This prevents a value canonically encoded for one purpose from being replayed as if it were canonically encoded for another.

*(Exact domain tag registry is TBD — to be defined alongside each consuming subsystem, e.g. `dilithia-consensus`, `dilithia-crypto`.)*

### 3.5 Canonical Decoding

Decoding is not the inverse of encoding plus error tolerance — it is a strict validity check. A decoder SHALL reject, not repair or best-effort-interpret:

- Non-minimal ULEB128 encodings (§3.2).
- Trailing bytes after a fully-decoded structure, when the structure's length is otherwise implied by context.
- Any primitive value outside its type's valid range (e.g. a `Bool` byte other than `0x00`/`0x01`).
- Invalid UTF-8 in a `String`.

A structure that decodes successfully SHALL, when re-encoded, produce byte-identical output to the original input. This round-trip property is the practical test for canonical-form compliance and SHOULD be enforced by property-based tests (`proptest`) in `dilithia-serialization`.

### 3.6 Resource Limits

To bound worst-case resource consumption during decoding (see `THREAT_MODEL.md` §4, Sybil/spam-adjacent concerns), DCS decoders SHALL enforce maximum sizes before allocating memory for a value — e.g. a maximum `String` byte length and a maximum element count for any length-prefixed collection type. *(Exact numeric limits are TBD — to be set alongside the transaction/block size limits defined elsewhere in this specification.)*

## 4. Genesis and Constitution Hash Anchoring

To make the constitutional text itself verifiable rather than merely asserted (see the WARNING banner in `CONSTITUTION.md`), Genesis and subsequent state SHALL track:

| Field | Meaning |
|---|---|
| `GenesisConstitutionHash` | `SHA3-256` of the canonical `CONSTITUTION.md` bytes (§3.1 rules apply) ratified at Genesis. Permanent, immutable historical anchor — never updates. |
| `CurrentConstitutionHash` | The hash of the presently-ratified constitution text. Updates only when a Super HIP amendment activates. |
| `ConstitutionVersion` | Human-readable version string of the current text (e.g. `1.0.0-rc1`). |
| `PreviousConstitutionHash` | `CurrentConstitutionHash` of the immediately prior ratified version, forming a hash-chain of the full amendment history. Absent/null for the Genesis-original version. |
| `SuperHipId` | Identifier of the Super HIP that ratified the current version. Absent/null for the Genesis-original version. |

Any party can verify a local copy of `CONSTITUTION.md` against either anchor by computing `SHA3-256` over its canonical bytes and comparing. A mismatch against `CurrentConstitutionHash` means the copy is stale or altered; a copy that matches `GenesisConstitutionHash` but not `CurrentConstitutionHash` is historically genuine but superseded by a since-ratified amendment.

## 5. Governance

*(Pending. Will define exact HIP and Super HIP thresholds — approval percentage, review period length, activation delay — per `CONSTITUTION.md` Article 12's requirement that these numbers live here, not in the Constitution, since they may depend on the finalized consensus algorithm.)*

## 6. HIP / Super HIP Process

*(Pending. Will define the mechanical process: proposal format, review period, automated/fuzz testing gate, formal verification gate for consensus-critical changes, Constitutional Guard check, activation delay, deterministic upgrade activation.)*

## 7. Crypto Agility

*(Pending. Will define the primitive-registry mechanism that lets `dilithia-crypto` support multiple signature/KEM algorithms simultaneously and migrate between them via HIP/Super HIP without changing protocol identity.)*

## 8. Transactions

*(Pending.)*

## 9. State

*(Pending.)*

## 10. Consensus

*(Pending — algorithm not yet finalized; see `THREAT_MODEL.md` §3, A-BFT.)*

---
*Sections 1–4 are a draft for review, not yet ratified. Sections 5–10 are placeholders reflecting the outline only.*
