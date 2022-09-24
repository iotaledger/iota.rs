### `Client.get_block(blockId)`

You can retrieve a block's data using its block ID using
the [`Client.get_block(&self, block_id: &BlockId)`](./../libraries/nodejs/references/classes/Client#getblock)
function.

If successful, the function will return the block as JSON.

This function queries the `GET /api/core/v2/blocks/{BlockId}` endpoint.

### `Client.get_block(&self, block_id: &BlockId)`

You can retrieve a block's metadata using its block ID using
the [``Client. get_block_metadata(&self, block_id: &BlockId)`](iota_client/client/struct.Client.html#method.get_block_metadata)
function.

If successful, the function will return the block's metadata as JSON.

This function queries the `GET /api/core/v2/blocks/{BlockId}/metadata` endpoint.
