### `Client.get_block_data(block_id)`

You can retrieve a block's metadata using its block ID using
the [`Client.get_block_data(block_id)`](./../libraries/python/api_reference#get_block_data)
function.

If successful, the function will return the block's as JSON.

This function queries the `GET /api/core/v2/blocks/{BlockId}` endpoint.

### `Client.get_block_metadata(block_id)`

You can retrieve a block's metadata using its block ID using
the [`Client.get_block_metadata(block_id)`](./../libraries/python/api_reference#get_block_metadata)
function.

If successful, the function will return the block's metadata as JSON.

This function queries the `GET /api/core/v2/blocks/{BlockId}/metadata` endpoint.