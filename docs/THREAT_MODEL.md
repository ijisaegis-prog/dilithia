# Dilithia Protocol — Threat Model

**Status:** DRAFT v0.1 — Pre-Genesis design review. Not yet ratified.
**Companion documents:** `CONSTITUTION.md` (what MUST NOT change), `SPECIFICATION.md` (how the protocol behaves).

## 1. Purpose and Scope

This document defines:

- the adversaries Dilithia is designed to resist,
- the adversaries and attack classes that are explicitly out of scope,
- the trust assumptions the protocol relies on,
- the boundaries between components that do not trust each other.

It exists to answer *why* a consensus-critical invariant exists. A HIP that touches consensus-critical behavior SHOULD cite the relevant section here when justifying its security rationale. Numeric thresholds are intentionally **not** defined in this document — they belong in `SPECIFICATION.md`'s Governance section, per the constitution/specification separation already agreed on.

## 2. Trust Assumptions

| ID | Assumption |
|----|------------|
| TA-1 | A node trusts no other node's claims about state without independently verifying them against consensus rules. |
| TA-2 | A node treats its own verified local chain state as canonical until a valid alternative satisfying consensus rules is observed. |
| TA-3 | Cryptographic primitives hold their standardized security properties (e.g. EUF-CMA for signatures, IND-CCA2 for KEMs) until formally broken or deprecated via HIP. |
| TA-4 | Governance participants are non-collusive below the threshold defined for each HIP tier. |
| TA-5 | The Rust toolchain and pinned dependencies are trusted unless a supply-chain compromise is demonstrated (§7). |

## 3. Adversary Classes

| ID | Adversary | Capabilities | Constraints |
|----|-----------|---------------|-------------|
| A-NET | Network attacker | Observe, delay, drop, reorder, inject P2P messages | Cannot forge valid signatures |
| A-SYBIL | Sybil attacker | Create unbounded P2P identities | Bounded by peer-scoring cost in `dilithia-p2p` (design pending) |
| A-BFT | Byzantine consensus participant | Arbitrary behavior by up to *f* nodes/stake-weight | Exact *f* depends on the finalized consensus algorithm — **not yet fixed** |
| A-QC-FUTURE | Future quantum attacker (CRQC exists) | Breaks classical discrete-log/factoring schemes (ECDSA, RSA) | Does not break well-parameterized lattice-based PQC under current cryptanalysis — a standing assumption, not a permanent guarantee |
| A-CHAIN | Malicious validator/proposer | Censor transactions; reorder within protocol limits | Cannot produce an invalid state transition without other honest nodes rejecting the block |
| A-SUPPLY | Supply-chain attacker | Compromise a dependency, the build pipeline, or a release artifact | Detectable via reproducible builds, SBOM, signed releases (§7) |
| A-GOV | Governance attacker | Sybil voting identities, bribery, coercion | Bounded by the Sybil-resistance of the governance weighting mechanism (TBD) |

## 4. In-Scope Threats

- **Sybil attack (P2P layer):** mitigated via peer diversity requirements and connection scoring (`dilithia-p2p`, design pending).
- **Eclipse attack:** mitigated via seed/anchor diversity and out-of-band peer discovery.
- **Long-range attack:** relevant only once the consensus mechanism assigns weight to historical stake; mitigation direction is checkpointing / weak subjectivity, to be finalized with the consensus algorithm.
- **Consensus divergence via non-determinism:** floating-point arithmetic, platform-dependent integer width, unordered iteration, or any other source of cross-platform non-determinism. Mitigated structurally by the Reserved Types / determinism rules in `SPECIFICATION.md` §3.2 (Primitive Types) — this belongs here, not just in the spec, because a crafted input that triggers platform-dependent behavior can be used deliberately to fork honest nodes from one another.
- **Harvest-Now-Decrypt-Later (HNDL):** see §6.1.
- **Future Signature Forgery:** see §6.2.
- **Supply-chain compromise:** reproducible builds, dependency pinning, signed releases (§7).
- **Governance capture:** Super HIP thresholds exist specifically to raise the cost of capturing constitutional-level changes.

## 5. Explicit Non-Goals

Stating what this protocol does *not* defend against is as important as the in-scope list.

- **Majority collusion of the consensus-securing resource** (hash power or stake, depending on the final algorithm). This is a foundational limit of any permissionless consensus protocol, not a Dilithia-specific gap.
- **Physical coercion or seizure of an individual key holder.** The protocol cannot distinguish a coerced-but-valid signature from a voluntary one; key custody is the holder's responsibility.
- **Legal or regulatory action** against node operators, contributors, or exchanges.
- **Hardware-level side-channel attacks** (timing, power analysis) against a specific implementation — belongs in implementation security guidelines, not this document.
- **A cryptographic break of the deployed PQC scheme itself.** No protocol-level design prevents this. It is mitigated only by Crypto Agility (the ability to migrate), never by this document's existence.

## 6. Quantum-Era Threats: Two Distinct Classes

These are often conflated. Dilithia treats them separately because their target data, attack timing, and mitigations differ.

### 6.1 Harvest-Now-Decrypt-Later (HNDL)

**Definition:** an adversary records confidentiality-dependent ciphertext today and decrypts it once a cryptographically relevant quantum computer (CRQC) exists.

**In scope:**
- Encrypted P2P transport — MUST use a PQC KEM (e.g. ML-KEM) or a hybrid classical+PQC construction from Genesis; MUST NOT rely on classical-only ECDH.
- Wallet backup encryption.
- Any metadata whose confidentiality (not authenticity) depends on a classical primitive.

### 6.2 Future Signature Forgery

**Definition:** an adversary with a future CRQC forges a signature for an exposed public key.

Because Dilithia's baseline signature scheme is post-quantum (ML-DSA family) from Genesis, it does not inherit the "classical ECDSA legacy exposure" problem that retrofit-PQC chains face (e.g. any chain where a revealed classical public key becomes forgeable once a CRQC exists). For a PQC-native chain, this threat class instead reduces to:

- **(a) Any legacy or bridge signature path** introduced later for interoperability with non-PQC-native chains — MUST be treated as carrying full forgery exposure and scoped accordingly.
- **(b) A future break of the deployed PQC scheme itself** — quantum or classical cryptanalysis of the specific lattice parameters. This is the primary reason Crypto Agility and the HIP/Super HIP migration path are constitutional-level guarantees rather than optional features.
- **(c) Weak key derivation or entropy failures** at the wallet/implementation level — a distinct, non-quantum threat with the same outcome (forgeable signatures); belongs in implementation security guidelines.

Dormant addresses and old unspent outputs are the *exposure surface* to track if (a) or (b) occurs, not a separate root cause.

## 7. Supply-Chain and Build Integrity

- `Cargo.lock` SHOULD be committed for the workspace, including the `dilithia-node` binary, for reproducible builds.
- CI SHOULD produce reproducible build artifacts where feasible.
- Release binaries SHOULD be signed, with signing keys published out-of-band from the repository.
- An SBOM SHOULD be generated per release once CI exists.
- Fuzzing and formal verification (cargo-fuzz, proptest, Kani) reduce, but do not eliminate, the risk of an introduced defect being consensus-breaking.

## 8. Trust Boundaries

| Component | Trusts | Does NOT trust |
|-----------|--------|-----------------|
| `dilithia-node` | Its own verified local state | Any unverified peer input or claim |
| `dilithia-p2p` | A message once signature and DCS encoding are verified | Peer identity or delivery ordering prior to verification |
| `dilithia-serialization` (DCS) | Nothing implicitly — canonicalizes and rejects malformed input by construction | Any input byte stream; ambiguous encodings are rejected, never repaired |
| `dilithia-consensus` | `dilithia-crypto`'s primitive correctness; DCS's canonical-encoding guarantees | Any external state prior to full state-transition validation |
| `dilithia-crypto` | The stated hardness assumptions of its primitives (TA-3) | Caller-supplied key material without explicit validation |
| `dilithia-guard` | Nothing implicitly — it is the last invariant check before an upgrade activates | Everything upstream of it, by design |

## 9. Key Compromise Scenarios

For each class of key material, this section states what is lost if that key is compromised — the *blast radius* — so key-management design (HSM usage, threshold signatures, rotation policy) can be prioritized by actual consequence rather than by convenience.

| Key | Compromise Enables | Blast Radius | Mitigation Direction |
|---|---|---|---|
| End-user wallet key | Spend from that address only | Single account | User-side custody hygiene — not a protocol failure |
| Validator / block-producer key | Equivocation, censorship, invalid proposals from that validator | Bounded by that validator's stake/voting weight, as long as the A-BFT threshold (§3) holds | Key rotation; slashing if the finalized consensus design supports it; HSM for validator operators |
| Governance / voting key | Cast fraudulent votes as that identity | Bounded by that identity's voting weight, unless it alone crosses a HIP/Super HIP threshold | Multi-sig or threshold-signature identities for high-weight voters |
| Bridge / interoperability key (if ever introduced) | Forge cross-chain messages; mint or release wrapped assets without authorization | Up to the full value locked in that bridge — historically the highest-blast-radius key class in other ecosystems | Threshold custody, time-locked withdrawals, independent monitoring; MUST NOT be a single key under any circumstances |
| `dilithia-guard` check-bypass (if the Guard ever exposed a checkable override rather than pure logic) | Bypass the last invariant check before an upgrade activates | The entire protocol — the maximum possible blast radius | This is exactly why the Guard's design goal is "nothing implicitly trusted" (§8), not a single verifiable signer |

A rule falls out of this table: **blast radius should be inversely proportional to how easy a key is to compromise.** The easiest keys to compromise (individual wallets, individual validators) already have the smallest blast radius by construction. Any design that gives a single, easily-compromised key an outsized blast radius — a lone admin key, a lone bridge key, a lone Guard override — is a Constitutional-level red flag, not just an implementation bug. This is the same reasoning behind "No Admin Key" in `CONSTITUTION.md`, stated here in blast-radius terms.

This section also sharpens the Non-Goal in §5: physical coercion of an individual key holder is out of scope for *that individual's* funds specifically because their blast radius is contained — the table above is what decides whether a coerced key stays a personal problem or becomes a systemic one.

## 10. Revisit Triggers

This document MUST be revisited at minimum when:

- the consensus algorithm is finalized (affects A-BFT and long-range-attack relevance),
- a HIP or Super HIP proposes a cryptographic primitive change,
- NIST or another relevant SDO updates guidance on a primitive Dilithia uses,
- a disclosed cryptanalytic advance materially changes the assumed security margin of the deployed PQC scheme,
- and, mandatorily, once before Mainnet Genesis as a pre-Genesis review gate.

---
*Draft skeleton for design review. Numeric thresholds (f, Sybil cost parameters, HIP thresholds) are deliberately left to `SPECIFICATION.md`.*
