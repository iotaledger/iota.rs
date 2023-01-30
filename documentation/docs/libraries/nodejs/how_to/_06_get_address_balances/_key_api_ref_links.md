### `Client.basicOutputIds(queryParameters)`

You can retrieve outputs from any given address using the [`Client.basicOutputIds(queryParameters)`](./../nodejs/references/classes/Client#basicoutputids)
function.

This function will query the `api/indexer/v1/outputs/basic` endpoint.

#### `QueryParameters`

You should pass the [QueryParameters](./../libraries/nodejs/references/api_ref#queryparameter) as an array of objects.
The following parameters are used in the example:

##### address

The Bech32-encoded address that should be searched for.

##### hasExpiration

Indicates if the outputs should have an expiration unlock condition or not.

##### hasTimeLock

Indicates if the outputs should have a time-locked unlock condition or not.

##### hasStorageDepositReturn

Indicates if the outputs should have storage deposit return unlock condition or not.

### `Client.getOutputs(outputIds)`

The [`Client.getOutputs(outputIds)`](./../libraries/nodejs/references/classes/Client#getoutputs) will query the node for a list
of `outputIds` in parallel. 
