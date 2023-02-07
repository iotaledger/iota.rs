# Interface: IBuildBlockOptions

Options to build a new block, possibly with payloads

## Table of contents

### Properties

- [output](IBuildBlockOptions.md#output)
- [outputHex](IBuildBlockOptions.md#outputhex)
- [parents](IBuildBlockOptions.md#parents)
- [burn](IBuildBlockOptions.md#burn)

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

### burn

• `Optional` **burn**: [`Burn`](Burn.md)

Explicit burning of aliases, nfts, foundries and native tokens
