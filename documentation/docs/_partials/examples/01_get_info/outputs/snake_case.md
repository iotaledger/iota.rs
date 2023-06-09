```json
{
   "nodeinfo":{
      "name":"HORNET",
      "version":"0.6.0-alpha",
      "is_healthy":true,
      "network_id":"migration",
      "bech32_hrp":"atoi",
      "min_pow_score":100,
      "messages_per_second":4.2,
      "referenced_messages_per_second":4.1,
      "referenced_rate":97.61904761904762,
      "latest_milestone_timestamp":1618139001,
      "latest_milestone_index":7092,
      "confirmed_milestone_index":7092,
      "pruning_index":0,
      "features":[
         "PoW"
      ]
   },
   "url":"https://api.lb-0.h.chrysalis-devnet.iota.cafe"
}
```

The most important properties are:
* `is_healthy`: Indicates whether the given node is in sync with the network and therefore safe to use. Even if a node is
  up and running, it may not be fully prepared to process your API calls properly. The node should be "synced", meaning
  that it should be aware of all transactions in the Tangle. It is better to avoid interacting with nodes which are not
  fully synced.
* `bech32_hrp`: Indicates whether the given node is a part of [devnet](https://wiki.iota.org/introduction/reference/networks/devnet)
  (`atoi`) or [mainnet](https://wiki.iota.org/introduction/reference/networks/mainnet) (`iota`). You can find more info regarding the
  [IOTA address format](https://wiki.iota.org/chrysalis-docs/guides/developer/#iota-15-address-anatom) in the official
  [Chrysalis documentation](https://wiki.iota.org/chrysalis-docs/welcome).
