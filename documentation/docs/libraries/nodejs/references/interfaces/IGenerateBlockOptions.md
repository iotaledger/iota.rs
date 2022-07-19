# Interface: IGenerateBlockOptions

Options to generate a new block, possibly with payloads

## Table of contents

### Properties

- [output](IGenerateBlockOptions.md#output)
- [outputHex](IGenerateBlockOptions.md#outputhex)
- [parents](IGenerateBlockOptions.md#parents)
- [allowBurning](IGenerateBlockOptions.md#allowburning)

## Properties

### output

• `Optional` **output**: [`IClientBlockBuilderOutputAddress`](IClientBlockBuilderOutputAddress.md)

Bech32 encoded output address and amount

___

### outputHex

• `Optional` **outputHex**: [`IClientBlockBuilderOutputAddress`](IClientBlockBuilderOutputAddress.md)

Hex encoded output address and amount

___

### parents

• `Optional` **parents**: `string`[]

Parent block IDs

___

### allowBurning

• `Optional` **allowBurning**: `boolean`

Allow burning of native tokens
