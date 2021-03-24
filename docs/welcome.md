# Welcome
This is the documentation for the official IOTA Client Library. It can be used to easily interact with IOTA network (Tangle) via [IOTA node software](https://chrysalis.docs.iota.org/node-software/node-software.html). Official IOTA libraries serve as `one-source-code-of-truth` to IOTA users and providing binding to other programming languages. You can read more about core principles behind IOTA client libraries in the following blog [post](https://blog.iota.org/the-new-iota-client-libraries-harder-better-faster-stronger/).

Example of tasks that `iota.rs` is able to help with:
- Create messages and transactions
- Sign transactions
- Generate addresses
- Interact with an IOTA node

> Please note: there is also available `wallet.rs` library that contains all the logic to safely build wallets or integrations that require value-based IOTA transfers. We strongly recommend to leverage [wallet.rs library](https://wallet-lib.docs.iota.org/) in case one is more focused on IOTA value-based transfers since it integrates the best security practices including our [stronghold enclave](https://blog.iota.org/iota-stronghold-6ce55d311d7c/)`.


## IOTA 1.5 (Chrysalis) in a nutshell
* IOTA network uses a DAG (Directed Acyclic Graph) to store its transactions. Each transaction can reference up to 8 parent transactions
* There is a breaking change moving from IOTA 1.0 to IOTA 1.5 (Chrysalis). IOTA address was originally based on WOTS signature scheme (81 trytes) and it has been replaced by a Ed25519 signature scheme
* In contrast to IOTA 1.0, IOTA 1.5 addresses are perfectly reusable: so even if one spent funds from the given address it can be used again
* There are new client libraries developed in rust, specifically `iota.rs`, `wallet.rs` and `stronghold.rs` that serve as `one-source-code-of-truth` to IOTA users and providing binding to other programming languages 

### IOTA 1.5 address anatomy
IOTA address is based on Ed25519 signature scheme and it is usually represented by Bech32 (checksummed base32) format string of 64 characters:

<table>
    <thead>
        <tr>
            <th colspan=4><center>iota11qykf7rrdjzhgynfkw6z7360avhaaywf5a4vtyvvk6a06gcv5y7sksu7n5cs</center></th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td colspan=4><center>three distinguished parts</center></td>
        </tr>
        <tr>
            <td><center><strong>human-readable id</strong></center></td>
            <td><center><strong>separator</strong></center></td>
            <td><center><strong>data</strong></center></td>
            <td><center><strong>checksum</strong></center></td>
        </tr>
        <tr>
            <td><center>iota | atoi</center></td>
            <td><center>1</center></td>
            <td><center>48 bytes [0..9a..z]</center></td>
            <td><center>6 characters [0..9a..z]</center></td>
        </tr>
        <tr>
            <td><center>iota</center></td>
            <td><center>1</center></td>
            <td><center>1qykf7rrdjzhgynfkw6z7360avhaaywf5a4vtyvvk6a06gcv5y7sks</center></td>
            <td><center>u7n5cs</center></td>
        </tr>
        <tr>
            <td colspan=4>iota = mainnet; atoi = testnet</td>
        </tr>
    </tbody>
</table>

More details: [RFC: Bech32 Address Format](https://github.com/iotaledger/protocol-rfcs/pull/20)


## Warning
This library is in active development. The library targets the Chrysalis testnet and does not work with current IOTA mainnet.


## Testnet
To join the Chrysalis public testnet checkout [this link](https://blog.iota.org/chrysalis-phase-2-testnet-out-now/). More information about Chrysalis components is available at [documentation portal](https://chrysalis.docs.iota.org/).


## Joining the discussion
If you want to get involved in discussions about this library, or you're looking for support, go to the #clients-discussion channel on [Discord](https://discord.iota.org).

## What you will find here
This documentation has five paths. 

1. The Overview, an detailed overview of the client library. 
2. Libraries bindings, all avaiable programming languages and their resources.
3. The Specification, detailed explaination requirements and functionality.
4. Contribute, how you can work on the client software.
5. Get in touch, join the community and become part of the X-Team!
