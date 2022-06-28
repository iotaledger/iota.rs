---
description: The IOTA network is based on messages and payloads. A message is a data structure broadcast in the IOTA network and represents a node (vertex) in the Tangle graph, while a payload represents a layer of concern.
image: /img/libraries/messages_in_tangle.svg
keywords:
- explanation
- message
- payload
- ledger state
- outputs
- unspent transaction output
- UTXO
---

# Messages, Payloads, and Transactions

The IOTA network is based on messages and payloads. This section will explain the relationship between messages and
payloads and how you can create value transactions.

## Messages

A message is a data structure broadcast in the IOTA network and represents a node (vertex) in
the [Tangle graph](https://explorer.iota.org/mainnet/visualizer/). It can refer to up to 8 previous messages, and once a
message is attached to the Tangle and approved by a milestone, the Tangle structure ensures the content of the message
will remain unaltered. Each message has a unique `message_id`, which is based on a hash algorithm of the binary content
of the message. A message is an atomic unit that is confirmed by the network as a whole.

A message is broadcast using a binary format, has an arbitrary size of up to 35 kB, and can hold variable sets of
information called [payloads](#payloads). A message can encapsulate any number of payloads, and even a message without a
payload is perfectly valid.

## Payloads

A payload represents a layer of concern. Some payloads may change a state of the ledger (ex. `transactions`), and some
may provide extra features to some specific applications and business use cases (ex. `indexed data`).

There are already implemented [core payloads](#core-payloads), such as `SignedTransaction`, `MilestonePayload`
and `IndexationPayload`. However, the message and payload definitions are generic enough to incorporate any future
payload(s) the community agrees upon.

The IOTA network ensures the outer structure of any message is valid and aligned with a network consensus protocol.
However, the message’s inner structure is very flexible, future-proof, and offers an unmatched network extensibility.

![messages_in_tangle](/img/libraries/messages_in_tangle.svg)

### Core Payloads

The current IOTA network incorporates the following core payloads:

#### SignedTransaction

A payload that describes [`UTXO` transactions](#unspent-transaction-output-utxo) that are the cornerstone of value-based
transfers in the IOTA network. You can use this payload to sign a message cryptographically. This payload changes the
ledger state as old `outputs` are spent (replaced) and new `outputs` are created.

Example of a message with a `SignedTransaction` payload:

```json
{
	"message": {
		"networkId": "14379272398717627559",
		"parentMessageIds": [
			"a59a5d11da0944c88b58f9f9c095c11ee4b8b7fd9da47bd25412d39f815bb804",
			"c3d42c42eccd25bc3374a0552e3a4b21180facece14f31c36e5ac580e5496ccc",
			"dae4a36cef9a3fd03caff5ddbc5c90bc5523477f4e4937837202bfe4bd5b98aa",
			"fe188a4f57ecd6a135b05b31913d86617550d9397476ab5bb7445138f782ec34"
		],
		"payload": {
			"type": 0,
			"essence": {
				"type": 0,
				"inputs": [
					{
						"type": 0,
						"transactionId": "b2b9723c9119f4fb49084472e72821e842ba4779df02e1038f03dd8b25d96730",
						"transactionOutputIndex": 1
					}
				],
				"outputs": [
					{
						"type": 0,
						"address": {
							"type": 0,
							"address": "43e80947ebd17637569ba7f90fd2541f50038de731467c45aa5c92d4429c9446"
						},
						"amount": 1000
					},
					{
						"type": 0,
						"address": {
							"type": 0,
							"address": "b4d1e9abfbcf4d2d2f0e042f23301a7d23f6ac1bde0a1fed508de5afec884ba8"
						},
						"amount": 8995995
					}
				],
				"payload": null
			},
			"unlockBlocks": [
				{
					"type": 0,
					"signature": {
						"type": 0,
						"publicKey": "27177dd41cc479ed379b8ad2535d66fa58480c119a8a15a7a296f055401ab958",
						"signature": "8403dc1fb949365e960f14cdc19b6b3abb6b0a6bce83f1082a33e3857a30ddd2be1098074b6c261f442db8e59eb640002d24d9a577262fd8152c6fee2d076c0b"
					}
				}
			]
		},
		"nonce": "156106"
	},
	"messageId": "92f427d68c7008a81fde290b9cb99071373d9893d65718bfc22928273877e041"
}
```

Each `transaction` includes the following information:

* `inputs`: A list of valid `outputs` that should be used to fund the given message. Those `outputs` will be spent and
  once the message is confirmed, those outputs are not valid anymore. Outputs are uniquely referenced via `transaction_id`
  and inner `index`. At least one output has to be given with enough balance to source all `outputs` of the given message.
* `outputs`: A list of IOTA address(es) and related amount(s) that the input `outputs` should be split among. Based on
  this information, new `UTXO` entities (outputs) will be being created.
* `unlockBlocks`: Includes a transaction signature(s) (currently based on `Ed25519` scheme) that prooves token
  ownership based on a valid seed. Only the valid seed owner is able to correctly sign the given transaction and proove
  ownership of the tokens under the given output(s). Each input `output` has to have a corresponding `unblockBlocks` entry
  in case more `outputs` are used to fund the operation, either using the given signature or as a reference to the
  existing signature
* `payload`: Each `SignedTransaction`(payload type 0) can include additional payload(s) such as an `IndexationPayload`
  (payload type 1) for example. This means any value-based messages can also contain arbitrary data and its key index. It
  is also an example how individual payloads can be encapsulated on different levels of concern

#### MilestonePayload

A payload that is emitted by the Coordinator.

#### IndexationPayload

A payload that enables adding an index to the encapsulating message, as well as arbitrary data. You can use the
index to search for the message(s).

### Unspent Transaction Output (UTXO)

IOTA uses an `unspent transaction output` model called `UTXO`. `UTXO` is based on an idea to track the unspent amount of
tokens using a data structure called `output`. This model can be explained using a simple example:

* There are 100 tokens recorded in the ledger as `Output A`, which belongs to Alice. So **initial state of
  ledger**: `Output A` = 100 tokens.
* Alice sends 20 tokens to Paul, 30 tokens to Linda, and keeps 50 tokens at her disposal.
* Her 100 tokens are currently stored as `Output A`. This means she has to divide (spend) her tokens to create three new
  outputs:
    * `Output B` with 20 tokens that go to Paul.
    * `Output C` with 30 tokens that go to Linda.
    * `Output D` with the remaining 50 tokens that she keeps for herself.
* The **original `Output A`** was spent entirely and can not be used anymore. As it is spent, it **becomes irrelevant**
  to the ledger state. The **new state of ledger** is:
    * `Output B` = 20 tokens.
    * `Output C` = 30 tokens.
    * `Output D` = 50 tokens.
* The total supply remains the same. However, the number of outputs differs, and some were replaced by other outputs in
  the process.

![utxo](/img/libraries/utxo.svg)

The key takeaway of the outlined process is that each unique `output` can **only be spent once**. Once an output is
spent, it can not be used anymore and is irrelevant regarding the ledger state.

So even if Alice still wants to keep the remaining tokens at her fingertips, those tokens have to be moved to completely
new `output` that can still be tied to the same IOTA address Alice used before.

Every `output` also stores information about an IOTA address it is coupled with. This means addresses and tokens are
indirectly coupled via outputs. So basically, the sum of outputs and their amounts under a given address is a balance of
the given address, i.e., the number of tokens the address can spend. And the sum of all unspent outputs and their
amounts is equal to the total supply.

Outputs are encapsulated in a message as a part of the `SignedTransaction` payload.

## Related Examples

* [Simple Message](./../examples/simple_message.mdx)
* [Get Outputs](./../examples/get_outputs.mdx)
* [Get Message Data](./../examples/data_message.mdx)
* [Send a Signed Transaction](./../examples/transaction.mdx)
