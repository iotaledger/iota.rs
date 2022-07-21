# Interface: IPreparedTransactionData

Helper struct for offline signing

## Table of contents

### Properties

- [essence](IPreparedTransactionData.md#essence)
- [inputsData](IPreparedTransactionData.md#inputsdata)
- [remainder](IPreparedTransactionData.md#remainder)

## Properties

### essence

• **essence**: `ITransactionEssence`

Transaction essence

___

### inputsData

• **inputsData**: [`IInputSigningData`](IInputSigningData.md)[]

Required address information for signing

___

### remainder

• `Optional` **remainder**: `IRemainder`

Optional remainder output information
