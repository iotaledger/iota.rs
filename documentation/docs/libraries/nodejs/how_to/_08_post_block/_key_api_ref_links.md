### `Client.getBlock(blockId)`

You can retrieve a block's metadata using its block ID using
the [`Client.getBlock(blockId)`](./../libraries/nodejs/references/classes/Client#getblock)
function.

If successful, the function will return the block's as JSON.

This function queries the `GET /api/core/v2/blocks/{BlockId}` endpoint.

### `Client.getBlockMetadata(blockId)`

You can retrieve a block's metadata using its block ID using 
the [`Client.getBlockMetadata(blockId)`](./../libraries/nodejs/references/classes/Client#getblockmetadata)
function.

If successful, the function will return the block's metadata as JSON.

This function queries the `GET /api/core/v2/blocks/{BlockId}/metadata` endpoint.
