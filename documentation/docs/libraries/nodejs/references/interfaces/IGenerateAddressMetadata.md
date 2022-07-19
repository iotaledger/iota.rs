# Interface: IGenerateAddressMetadata

Metadata provided to Generate Address

## Table of contents

### Properties

- [syncing](IGenerateAddressMetadata.md#syncing)
- [network](IGenerateAddressMetadata.md#network)

## Properties

### syncing

• **syncing**: `boolean`

Indicates that the address is being generated as part of the account syncing process.
This means that the account might not be saved.
If it is false, the prompt will be displayed on ledger devices.

___

### network

• **network**: [`Network`](../enums/Network.md)

The network which is used so the correct BIP32 path is used for the ledger. Debug mode starts with 44'/1' and
in mainnet-mode it's 44'/4218'
