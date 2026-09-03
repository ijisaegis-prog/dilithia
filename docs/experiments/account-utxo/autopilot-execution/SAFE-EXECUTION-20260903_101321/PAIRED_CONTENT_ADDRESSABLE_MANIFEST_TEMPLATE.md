# Paired Content-Addressable Manifest Template

> **UNINSTANTIATED NEUTRAL SCHEMA.** Blank or `UNRESOLVED` fields are not
> evidence, identities, defaults, or protocol choices. Account and UTXO have
> co-equal obligations. This template performs no comparison or selection.

## Manifest envelope

| Field | Required value |
|---|---|
| manifest schema version | `UNRESOLVED` |
| manifest identity algorithm | `SHA-256` for exact manifest bytes, excluding only the identity field under a declared canonicalization rule |
| canonicalization rule | `UNRESOLVED`; must be stated before identity computation |
| source commit | `UNRESOLVED` |
| governing artifact path and SHA-256, one row each | `UNRESOLVED` |
| shared case path and SHA-256 | `UNRESOLVED` |
| frozen profile-set path and SHA-256 | `UNRESOLVED` |
| shared oracle path and SHA-256 | `UNRESOLVED` |
| evidence class and claim boundary | `UNRESOLVED` |
| registration status and receipt identity | `PENDING`; `UNRESOLVED` |

## Co-equal candidate bindings

| Required binding | Account | UTXO |
|---|---|---|
| mapping path | `UNRESOLVED` | `UNRESOLVED` |
| mapping SHA-256 | `UNRESOLVED` | `UNRESOLVED` |
| implementation or model path | `UNRESOLVED` | `UNRESOLVED` |
| implementation or model SHA-256 | `UNRESOLVED` | `UNRESOLVED` |
| harness path | `UNRESOLVED` | `UNRESOLVED` |
| harness SHA-256 | `UNRESOLVED` | `UNRESOLVED` |
| oracle binding identity | `UNRESOLVED` | `UNRESOLVED` |
| input/corpus identity | `UNRESOLVED` | `UNRESOLVED` |
| execution status | `BLOCKED_MISSING_INPUT` | `BLOCKED_MISSING_INPUT` |
| result/output identity | `NONE` | `NONE` |
| review identity | `NONE` | `NONE` |

## Execution provenance fields

Exact working directory, executable identity, command/argument vector,
environment allowlist, toolchain identities, start/end timestamps, stdout
identity, stderr identity, exit status, raw-output locations and identities,
reviewer identity, and exclusions are all required and currently `UNRESOLVED`.

## Completeness rule

An instantiated manifest remains `INCOMPLETE` if any required field is absent,
if a referenced path lacks a recorded digest, if either candidate lacks the
same class of binding, if a status conflicts with the presence or absence of a
result, or if the manifest identity cannot be recomputed under its declared
canonicalization rule. Completeness establishes metadata shape only; it does
not establish truth, candidate behavior, symmetry, or Gate credit.
