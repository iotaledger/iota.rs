# @iota/client

## Table of contents

### Classes

- [Client](classes/Client.md)
- [MessageHandler](classes/MessageHandler.md)

### Enumerations

- [CoinType](enums/CoinType.md)
- [LedgerDeviceType](enums/LedgerDeviceType.md)
- [Network](enums/Network.md)

### Type Aliases

- [BlockId](api_ref.md#blockid)
- [QueryParameter](api_ref.md#queryparameter)
- [AliasQueryParameter](api_ref.md#aliasqueryparameter)
- [FoundryQueryParameter](api_ref.md#foundryqueryparameter)
- [NftQueryParameter](api_ref.md#nftqueryparameter)
- [SecretManager](api_ref.md#secretmanager)

### Interfaces

- [IBuildBlockOptions](interfaces/IBuildBlockOptions.md)
- [IClientBlockBuilderOutputAddress](interfaces/IClientBlockBuilderOutputAddress.md)
- [IClientOptions](interfaces/IClientOptions.md)
- [IDuration](interfaces/IDuration.md)
- [IGenerateAddressesOptions](interfaces/IGenerateAddressesOptions.md)
- [IGenerateAddressOptions](interfaces/IGenerateAddressOptions.md)
- [LedgerNanoStatus](interfaces/LedgerNanoStatus.md)
- [LedgerApp](interfaces/LedgerApp.md)
- [IAuth](interfaces/IAuth.md)
- [IMqttBrokerOptions](interfaces/IMqttBrokerOptions.md)
- [INode](interfaces/INode.md)
- [INetworkInfo](interfaces/INetworkInfo.md)
- [INodeInfoWrapper](interfaces/INodeInfoWrapper.md)
- [IAliasOutputBuilderOptions](interfaces/IAliasOutputBuilderOptions.md)
- [IBasicOutputBuilderOptions](interfaces/IBasicOutputBuilderOptions.md)
- [IFoundryOutputBuilderOptions](interfaces/IFoundryOutputBuilderOptions.md)
- [INftOutputBuilderOptions](interfaces/INftOutputBuilderOptions.md)
- [IPreparedTransactionData](interfaces/IPreparedTransactionData.md)
- [IInputSigningData](interfaces/IInputSigningData.md)
- [IRange](interfaces/IRange.md)
- [LedgerNanoSecretManager](interfaces/LedgerNanoSecretManager.md)
- [MnemonicSecretManager](interfaces/MnemonicSecretManager.md)
- [StrongholdSecretManager](interfaces/StrongholdSecretManager.md)

## Type Aliases

### BlockId

Ƭ **BlockId**: `string`

A block identifier, the BLAKE2b-256 hash of the block bytes.
See <https://www.blake2.net/> for more information.

___

### QueryParameter

Ƭ **QueryParameter**: `Address` \| `AliasAddress` \| `HasStorageDepositReturn` \| `StorageDepositReturnAddress` \| `HasTimelock` \| `TimelockedBefore` \| `TimelockedAfter` \| `HasExpiration` \| `ExpiresBefore` \| `ExpiresAfter` \| `ExpirationReturnAddress` \| `Sender` \| `Tag` \| `Issuer` \| `StateController` \| `Governor` \| `CommonQueryParameters`

Query parameter for filtering output requests

___

### AliasQueryParameter

Ƭ **AliasQueryParameter**: `StateController` \| `Governor` \| `Issuer` \| `Sender` \| `CommonQueryParameters`

Query parameters for filtering Alias Outputs

___

### FoundryQueryParameter

Ƭ **FoundryQueryParameter**: `AliasAddress` \| `CommonQueryParameters`

Query parameters for filtering Foundry Outputs

___

### NftQueryParameter

Ƭ **NftQueryParameter**: `Address` \| `AliasAddress` \| `HasStorageDepositReturn` \| `StorageDepositReturnAddress` \| `HasTimelock` \| `TimelockedBefore` \| `TimelockedAfter` \| `HasExpiration` \| `ExpiresBefore` \| `ExpiresAfter` \| `ExpirationReturnAddress` \| `Sender` \| `Tag` \| `CommonQueryParameters`

Query parameters for filtering Nft Outputs

___

### SecretManager

Ƭ **SecretManager**: [`LedgerNanoSecretManager`](interfaces/LedgerNanoSecretManager.md) \| [`MnemonicSecretManager`](interfaces/MnemonicSecretManager.md) \| [`StrongholdSecretManager`](interfaces/StrongholdSecretManager.md)

Supported secret managers
