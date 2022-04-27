:::note Dust Protection
There is a [dust protection](https://wiki.iota.org/chrysalis-docs/faq#what-is-dust-protection-and-how-does-it-work)
mechanism implemented in the network protocol to prevent malicious actors from spamming the network to decrease node
performance while keeping track of unspent amount (`UTXO`).


"... microtransactions below 1Mi of IOTA tokens can be sent to another address if there is already at least 1Mi on that address"


That's why the code in the example sent 1Mi, to comply with the protection.
:::

