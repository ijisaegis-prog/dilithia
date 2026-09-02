# Dilithia G1-AUTH-001 Mapping Identity Binding

**Status:** PROCESS / EVIDENCE-PROVENANCE IDENTITY BINDING
**Protocol adoption effect:** NONE
**Candidate evidence:** NO
**Comparison scoring:** NOT STARTED
**State-model ranking:** NONE
**State-model decision:** NOT MADE
**Account selected:** NO
**UTXO selected:** NO

---

## 1. Purpose and Boundary

This artifact binds the already frozen G1-AUTH-001 paired-manifest identity to
the already frozen identities of the Minimal Account and Minimal UTXO candidate
mappings. It records provenance only.

The dependency is acyclic and one-way:

`frozen manifest -> candidate mappings -> identity binding artifact`

This binding artifact is downstream of the frozen manifest and the candidate
mappings. It is not, and must not become, part of the frozen manifest identity.
The manifest contains no mutable mapping-identity slots, and this artifact does
not claim otherwise. No bytes, SHA-256 value, or raw Git blob identity of the
manifest or either mapping are changed by this binding.

This artifact does not modify, supplement, narrow, widen, or reinterpret
`P0-001`, `V-001`, `E1-001`, `A1-001`, `P1-001`, or `M1-001`.

## 2. Frozen Paired Manifest Identity

| Field | Frozen value |
|---|---|
| Path | `docs/experiments/account-utxo/gate1/G1_AUTH_001_PAIRED_MANIFEST.md` |
| SHA-256 | `3456D4AB164DA7C6B4CB05282E2A14EE884187C374DA0996805AA44940E4555C` |
| Raw Git blob | `6bd8a59e640839d3a4402f6f67a5aa5cf45c6409` |

## 3. Frozen Candidate Mapping Identities

The two mappings are identified symmetrically. Their listing order conveys no
preference, score, rank, selection, or state-model decision.

| Candidate mapping | Path | SHA-256 | Raw Git blob |
|---|---|---|---|
| Minimal Account | `docs/experiments/account-utxo/gate1/G1_AUTH_001_MINIMAL_ACCOUNT_MAPPING.md` | `C33C308A841A820312826C0A0C937E725D00589631382A04D6008DB7A376A17D` | `83073e5ff31d7cc042621827b1e08f8b3c46e231` |
| Minimal UTXO | `docs/experiments/account-utxo/gate1/G1_AUTH_001_MINIMAL_UTXO_MAPPING.md` | `BD6FCFC9FDE0B84A3046F0E862287159013F3AA2FB2E4649B1237C6E7A01475B` | `55106e28f35a2868b1e993e33f2c0180095dc876` |

Both identities are bound to the same frozen paired-manifest identity in
Section 2. Neither binding is stronger, earlier, preferred, or more complete
than the other.

## 4. Freeze and Review Provenance

Mapping freeze commit:

`ed51717583844eab4dcf2a2cc99d2a08c95ca42d`

Focused review record:

| Field | Frozen value |
|---|---|
| Path | `docs/experiments/account-utxo/gate1/G1_AUTH_001_CANDIDATE_MAPPINGS_FOCUSED_REVIEW.md` |
| SHA-256 | `DC00F318817A0687012D8EF9470C319550F16500D4A81F8D62416945038C6C0F` |
| Raw Git blob | `acb7d36a307fcc7d4309c46e56223b719c24a488` |

The review record supplies process provenance for the frozen mapping pair. Its
identity does not alter the manifest or mapping identities.

## 5. Evidence-Provenance Rule

This artifact generates no candidate evidence. Later evidence may reference
this binding artifact to establish which frozen manifest and candidate mapping
identities it concerns. Evidence generation, collection, validation, and use
remain later-stage activities and require their own expressly scoped records.

A later reference to this artifact does not turn this artifact, either mapping,
or the focused review record into candidate evidence. It also does not
authorize scoring, ranking, selection, or a state-model decision.

## 6. Explicit Non-Selections

This binding does not:

- score or rank Account against UTXO;
- select Account or UTXO;
- make a state-model decision;
- generate candidate evidence;
- select any deferred protocol mechanism;
- adopt a state schema, transaction model, authorization mechanism,
  cryptographic mechanism, replay mechanism, lifecycle mechanism, fee or
  economic mechanism, consensus mechanism, commitment mechanism, migration
  mechanism, recovery mechanism, or other deferred protocol mechanism; or
- change the Formal Specification or consensus implementation.

## 7. Process Status

- Candidate evidence: `NO`.
- Scoring: `NOT STARTED`.
- Ranking: `NONE`.
- State-model decision: `NOT MADE`.
- Account selected: `NO`.
- UTXO selected: `NO`.
