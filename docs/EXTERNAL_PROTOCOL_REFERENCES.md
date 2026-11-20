# External protocol references for Dilithia

These references inform Pre-Genesis research. The Dilithia Constitution and
Formal Specification govern protocol decisions. Minimal Account and Minimal
UTXO remain candidates under the [comparison framework](ACCOUNT_UTXO_COMPARISON_FRAMEWORK.md).

| Reference | Mechanism to study | Research question for Dilithia |
| --- | --- | --- |
| [Bitcoin transactions](https://developer.bitcoin.org/devguide/transactions.html) | Inputs reference specific previous outputs, and a spend consumes the referenced output. | Can both candidates express the same conflict when two otherwise valid transactions try to consume the same spendable value? |
| [Ethereum accounts](https://ethereum.org/developers/docs/accounts/) | Account state includes a balance and a nonce. | How does each candidate bind an authorization to the intended state and reject a repeated application of the same value effect? |
| [Cardano extended UTXO](https://docs.cardano.org/about-cardano/learn/eutxo-explainer) | Outputs can carry data that a validator inspects together with the transaction and the spending arguments. | What is the smallest validation context each candidate needs, and what historical data must remain interpretable? |
| [Solana instruction structure](https://solana.com/docs/core/instructions/instruction-structure) | Instructions identify participating accounts and declare signer and writable roles. | Can the same read, write, and conflict sets be described for both candidates before implementation-specific scheduling? |
| [CometBFT application requirements](https://github.com/cometbft/cometbft/blob/main/spec/abci/abci%2B%2B_app_requirements.md) | The application and consensus engine exchange deterministic state commitments. | Which result must two independent implementations reproduce from the same prior state and ordered input? |
| [Tezos amendment process](https://octez.tezos.com/docs/active/voting.html) | Protocol amendments pass through proposal, voting, review time, and activation stages. | What evidence would a Dilithia upgrade need before activation under its own HIP or Super HIP process? |
| [ERC-20 token interface](https://eips.ethereum.org/EIPS/eip-20) | Contracts can expose a common token interface, including supply and balance queries. | How should tests distinguish application-level asset accounting from the native DLTH supply tracked by the protocol? |

The [value-effect pilot](experiments/account-utxo/pilot/VALUE_EFFECT_BASELINE_CASE.md)
has an existing result for the two reference mappings at A = 1.
Reuse that result when designing paired follow-ups for failed transactions and
conflicting spends. Start with the same initial value and transfer intent for
both candidates, then compare the observable state and supply. [Native DLTH lifecycle requirements](NATIVE_DLTH_LIFECYCLE_CONSERVATION_REQUIREMENTS.md)
record the related monetary questions.

## Asset identity

A public catalog can assign distinct IDs to projects with the same symbol.
The CoinPaprika catalog on 2026-11-20 listed the unrelated entry DTALTH under
the symbol `DLTH`. For issued assets, verify the native network or, for a
contract token, the network and contract address. Dilithia's
[project state](PROJECT_STATE.md) records its Pre-Genesis status.
[CoinPaprika catalog fields](https://docs.coinpaprika.com/api-reference/coins/list-coins)
