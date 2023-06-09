```json
{
   "nodeinfo":{
      "name":"HORNET",
      "version":"0.6.0-alpha",
      "isHealthy":true,
      "networkId":"migration",
      "bech32HRP":"atoi",
      "minPoWScore":100,
      "messagesPerSecond":4.2,
      "referencedMessagesPerSecond":4.1,
      "referencedRate":97.61904761904762,
      "latestMilestoneTimestamp":1618139001,
      "latestMilestoneIndex":7092,
      "confirmedMilestoneIndex":7092,
      "pruningIndex":0,
      "features":[
         "PoW"
      ]
   },
   "url":"https://api.lb-0.h.chrysalis-devnet.iota.cafe"
}
```

The most important properties are:
* `isHealthy`: Indicates whether the given node is in sync with the network and therefore safe to use. Even if a node is
  up and running, it may not be fully prepared to process your API calls properly. The node should be "synced", meaning
  that it should be aware of all transactions in the Tangle. It is better to avoid interacting with nodes which are not
  fully synced.
* `bech32HRP`: Indicates whether the given node is a part of [devnet](https://wiki.iota.org/introduction/reference/networks/devnet)
  (`atoi`) or [mainnet](https://wiki.iota.org/introduction/reference/networks/mainnet) (`iota`). You can find more info regarding the
  [IOTA address format](https://wiki.iota.org/introduction/reference/details/#iota-15-address-anatomy) in the official
  [Chrysalis documentation](https://wiki.iota.org/introduction/welcome).
