### `Client.basic_output_ids(query_parameters)`

You can retrieve outputs from any given address using the [`Client.basic_output_ids(query_parameters)`](iota_client/client/struct.Client.html#method.basic_output_ids)
function.

This function will query the `api/indexer/v1/outputs/basic` endpoint.

#### `query_parameters`

You should pass the [QueryParameters](ota_client/node_api/indexer/query_parameters/enum.QueryParameter.html) as an array
of objects. The following objects are used in the example:

##### address

The Bech32-encoded address that should be searched for.

##### hasExpiration

Indicates if the outputs should have an expiration unlock condition or not.

##### hasTimeLock

Indicates if the outputs should have a time-locked unlock condition or not.

##### hasStorageDepositReturn

Indicates if the outputs should have storage deposit return unlock condition or not.

### `Client.get_outputs(output_ids)`

The [`Client.get_outputs(output_ids)`](iota_client/client/struct.Client.html#method.get_outputs) will query the node for a list
of `output_ids` in parallel. 
