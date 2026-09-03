# Gate 1 Executed Structural Cases

## `G1-AUTH-001-PAIR-001`

Evidence class: **structural paper trace only**. The exact reviewed shared case,
manifest, Account mapping, UTXO mapping, and binding were reported checked by
the earlier executor at the then-current base. Their reported SHA-256 values
match the identities published by the existing
evidence package: manifest `3456d4ab...e4555c`, Account mapping
`c33c308a...6a17d`, UTXO mapping `bd6fcfc9...01475b`, and binding
`8b686c36...c4745`. Git blobs also match `6bd8a59e...`, `83073e5f...`,
`55106e28...`, and `ded0d236...` respectively. The executor reported freeze
commits `ed517175...` and `3984d8a9...` in history. These historical command and
hash observations are not independently durable here.

### Shared external trace

| Stage | Frozen fact | Executed structural check | Result |
|---|---|---|---|
| P0 | `X=2u`, `Y=1u`, total `3u`; one authority relation; frozen positive profiles | Same facts supplied to both mappings | PASS |
| V | External reassignment quantity `q=1u` | No candidate-native unit substituted | PASS |
| E1 | Indivisible reassignment from X to Y | Both mappings declare conditional atomic realization | PASS structurally |
| A1 | Evidence covers the current X relation and exact E1 only | Neither mapping adds a second external relation | PASS structurally |
| P1 | `X=1u`, `Y=2u`, total `3u`; issuance/burn `0u` | `2u-1u=1u`; `1u+1u=2u`; `1u+2u=3u` | PASS arithmetically |
| Failure | Rejection has zero canonical effect | Declared obligation only; no executable failure input | NOT EXECUTED |

Account realizes two current logical value facts and a conditional paired
mutation. UTXO realizes current spendable facts and conditional
consumption/creation. These are disclosed internal mappings, not neutral costs.
Both preserve the same external projection by declaration. No validator was
run and no cryptographic validity was established.

### Authorization and reuse trace

Both mappings expose one independent external authority relation. Evidence is
case-local; credential cardinality and verification operations remain
unresolved. Authorization-evidence reuse is absent for this case;
authorization-condition reuse is merely mapping-local; verification-result and
implementation-cache reuse are not claimed. The excluded economic profile does
not assert zero fees or absence of economic effects.

### Disposition

This executor-reported repeated paper check succeeds within the declared frozen paper
contract. It does not repair the existing package's durable-provenance gap, does
not execute negative cases, and is **not direct Gate-1 evidence credit**.
