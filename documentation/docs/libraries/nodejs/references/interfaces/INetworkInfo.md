# Interface: INetworkInfo

Struct containing network and PoW related information

## Table of contents

### Properties

- [minPowScore](INetworkInfo.md#minpowscore)
- [localPow](INetworkInfo.md#localpow)
- [fallbackToLocalPow](INetworkInfo.md#fallbacktolocalpow)
- [tipsInterval](INetworkInfo.md#tipsinterval)
- [rentStructure](INetworkInfo.md#rentstructure)

## Properties

### minPowScore

• **minPowScore**: `number`

Minimum proof of work score

___

### localPow

• **localPow**: `boolean`

Local proof of work

___

### fallbackToLocalPow

• **fallbackToLocalPow**: `boolean`

Fallback to local proof of work if the node doesn't support remote Pow

___

### tipsInterval

• **tipsInterval**: `number`

Tips request interval during PoW in seconds

___

### rentStructure

• **rentStructure**: [`IRentStructureResponse`](IRentStructureResponse.md)

Rent structure of the protocol
