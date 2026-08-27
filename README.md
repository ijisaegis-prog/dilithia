# Dilithia

**Status:** Pre-Genesis · Design & Early Implementation Phase

Dilithia is an independent Layer-1 blockchain protocol with a native coin, **$DLTH**, built around post-quantum security goals and **Crypto Agility** — the ability to migrate to new cryptographic primitives through a formal governance process without changing the network's constitutional identity.

This repository is not yet a working blockchain. There is no Testnet, no Mainnet, and no DLTH has been issued. See [Project Status](#project-status) below for exactly what exists today.

## Core Principles

Full detail lives in [`CONSTITUTION.md`](./docs/CONSTITUTION.md); in short:

- **No pre-mine.** No coins exist before Genesis.
- **No admin key.** No party can unilaterally alter balances, mint supply, seize funds, or exercise privileged cryptographic authority.
- **Formal Specification Primacy.** Behavior is defined by [`SPECIFICATION.md`](./docs/SPECIFICATION.md); implementations conform to the specification, not the reverse.
- **Protocol Determinism.** All compliant implementations MUST derive identical consensus-critical results from identical canonical inputs.
- **Crypto Agility.** The protocol is designed so cryptographic rules can evolve through the authoritative protocol process without changing Dilithia's constitutional identity.
- **Evolution Engine.** The protocol is intended to evolve through the HIP / Super HIP process, but no upgrade mechanism may override the Constitution.

## Repository Structure

```text
crates/
├─ dilithia-guard/          # Constitutional invariant checks
├─ dilithia-crypto/         # Cryptographic workspace and future agility mechanisms
├─ dilithia-serialization/  # DCS — Dilithia Canonical Serialization
├─ dilithia-core/           # Shared core types
├─ dilithia-consensus/      # Consensus-related implementation workspace
├─ dilithia-p2p/            # Peer-to-peer networking workspace
└─ dilithia-node/           # Node binary

docs/
├─ CONSTITUTION.md          # Supreme constitutional rules
├─ SPECIFICATION.md         # Formal protocol behavior
├─ THREAT_MODEL.md          # Security threats, assumptions, and trust boundaries
└─ hips/                    # HIP / Super HIP material
```

## Governance: HIP / Super HIP

Protocol changes are intended to proceed through the **HIP** (Dilithia Improvement Proposal) process.

Constitutional-level changes require the stricter **Super HIP** process.

The Constitution requires a Super HIP to include, at minimum:

- a substantially higher approval threshold than an ordinary HIP,
- an extended public review period,
- mandatory independent security review,
- formal verification of all affected consensus-critical logic,
- and a mandatory activation delay.

The exact mechanical governance thresholds and activation rules remain **TBD** in the Formal Specification and must not be inferred from implementation or discussion.

Articles 1–5 and Article 13 of the current Constitution cannot be directly or indirectly removed, weakened, bypassed, reinterpreted, or rendered unenforceable through Super HIP.

## Building

Requires Rust (stable) and Cargo.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
cargo test --workspace
```

## Project Status

| Item | Status |
|---|---|
| Brand / ticker (Dilithia / DLTH) | ✅ Decided |
| Rust workspace (7 crates) | ✅ Created; `cargo test --workspace` passing |
| `CONSTITUTION.md` | ✅ v1.0.0-rc1 · Pre-Genesis Ratified Draft (Frozen) |
| `SPECIFICATION.md` | 🔶 Formal Specification draft Sections 1–4 established; Sections 5–10 pending/stubbed |
| `THREAT_MODEL.md` | 🔶 Draft v0.1 with adversaries, trust assumptions, resource threats, quantum-era threats, supply-chain integrity, trust boundaries, key-compromise scenarios, and revisit triggers |
| DCS-1 | ✅ Canonical minimal ULEB128 `u64` implemented |
| DCS-2A | ✅ Fixed-width little-endian `u8/u16/u32/u64/u128` implemented |
| DCS-2B | ✅ Project-owned canonical little-endian `U256` implemented |
| DCS-3A | ✅ Canonical `Bool` and `Bytes<N>` implemented |
| `UnixTimestamp` serialization | ✅ Implemented |
| `Option<T>` serialization | ✅ Implemented |
| Serialization unit tests | ✅ 45 passing |
| Workspace unit tests | ✅ 50 passing, 0 failed |
| Crypto primitives / runtime Crypto Agility | ⬜ Not yet implemented; authoritative algorithm choices remain pending |
| Transaction format / processing | ⬜ Not yet implemented |
| State model | ⬜ Not selected |
| Consensus algorithm | ⬜ Not selected |
| Governance thresholds / mechanical HIP-Super HIP process | ⬜ TBD |
| P2P implementation | ⬜ Pending |
| Evolution Engine runtime | ⬜ Not yet implemented |
| Devnet / Testnet / Mainnet | ⬜ Not started |
| DLTH issuance | ⬜ None — Pre-Genesis |
| Exchange listing | ⬜ Not applicable — no process has begun |

## Project Philosophy

Dilithia is engineered around one simple discipline:

- **Move slowly where decisions are hard to reverse.**
- **Specify first.**
- **Verify before trust.**
- **Security before features.**
- **Correctness before performance.**
- **Evidence before irreversible protocol decisions.**
- **Evolution without sacrificing determinism.**

The long-term design principle is:

> **Identity is permanent. Technology evolves.**

Dilithia aims to preserve its constitutional identity while allowing its technical mechanisms to improve as cryptography, distributed systems, and security engineering evolve.

## License

MIT OR Apache-2.0
