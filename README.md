# Dilithia

**Status:** Pre-Genesis · Design & Early Implementation Phase

Dilithia is an independent Layer-1 blockchain protocol with a native coin, **$DLTH**, built around post-quantum cryptography (PQC) and **Crypto Agility** — the ability to migrate to new cryptographic primitives through a formal governance process without changing the network's identity.

This repository is not yet a working blockchain. There is no Testnet, no Mainnet, and no DLTH has been issued. See [Project Status](#project-status) below for exactly what exists today.

## Core Principles

Full detail lives in [`CONSTITUTION.md`](./docs/CONSTITUTION.md); in short:

- **No pre-mine.** No coins exist before Genesis.
- **No admin key.** No party can unilaterally alter balances, mint supply, or seize funds.
- **Formal Specification Primacy.** Behavior is defined by [`SPECIFICATION.md`](./docs/SPECIFICATION.md); implementations conform to the spec, not the reverse.
- **Protocol Determinism.** All nodes MUST derive identical state from identical input — treated as a security property, not just a correctness one (see [`THREAT_MODEL.md`](./docs/THREAT_MODEL.md) §4).
- **Crypto Agility.** Cryptographic primitives can be upgraded via governance; the protocol's identity is not tied to any single algorithm.
- **Evolution Engine.** The protocol can evolve — new signature schemes, consensus improvements, VM features — through the HIP process, but never in ways that violate the Constitution.

## Repository Structure

```text
crates/
├─ dilithia-guard/          # Constitutional Guard — invariant checks for protocol upgrades
├─ dilithia-crypto/         # Cryptographic primitives (PQC signatures, KEMs)
├─ dilithia-serialization/  # DCS — Dilithia Canonical Serialization
├─ dilithia-core/           # Shared core types
├─ dilithia-consensus/      # Consensus state-transition logic
├─ dilithia-p2p/            # Peer-to-peer networking
└─ dilithia-node/           # Node binary

docs/
├─ CONSTITUTION.md          # What MUST NOT change
├─ SPECIFICATION.md         # How the protocol behaves (DCS, primitives, etc.)
├─ THREAT_MODEL.md          # Adversaries, trust assumptions, trust boundaries
└─ hips/                    # Historical and proposed protocol upgrades
```

## Governance: HIP / Super HIP

Ordinary protocol changes go through a **HIP** (Dilithia Improvement Proposal): public review → automated/fuzz testing → formal verification (for consensus-critical changes) → Constitutional Guard check → community approval → activation delay → deterministic upgrade.

Changes to the Constitution itself require the stricter **Super HIP** track: a substantially higher approval threshold, an extended review period, mandatory independent security review, and a longer activation delay. Exact numeric thresholds live in `SPECIFICATION.md`'s Governance section rather than here, since they may depend on the finalized consensus mechanism.

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
| Rust workspace (7 crates) | ✅ Created, `cargo check --workspace` passing |
| `CONSTITUTION.md` | 🔶 Drafted (rc1), being ported into this repo |
| `SPECIFICATION.md` (DCS §3.1–3.3) | 🔶 Drafted, being ported into this repo |
| `THREAT_MODEL.md` | 🔶 Skeleton drafted |
| DCS implementation (`dilithia-serialization`) | ⬜ Not started |
| Devnet / Testnet / Mainnet | ⬜ Not started |
| Exchange listing | ⬜ Not applicable — no process has begun |

## Project Philosophy

Dilithia is engineered around one simple discipline:

- **Move slowly.**
- **Specify first.**
- **Verify before trust.**
- **Security before features.**
- **Correctness before performance.**
- **Evolution without sacrificing determinism.**

## License

MIT OR Apache-2.0
