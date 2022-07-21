# Interface: IInputSigningData

Data for transaction inputs for signing and ordering of unlock blocks

## Table of contents

### Properties

- [output](IInputSigningData.md#output)
- [outputMetaData](IInputSigningData.md#outputmetadata)
- [chain](IInputSigningData.md#chain)
- [bech32Address](IInputSigningData.md#bech32address)

## Properties

### output

• **output**: `OutputTypes`

The output

___

### outputMetaData

• **outputMetaData**: `IOutputMetadataResponse`

The output metadata

___

### chain

• `Optional` **chain**: `ISegment`[]

The chain derived from seed, only for ed25519 addresses

___

### bech32Address

• **bech32Address**: `string`

The bech32 encoded address, required because of alias outputs where we have multiple possible unlock
conditions, because we otherwise don't know which one we need
