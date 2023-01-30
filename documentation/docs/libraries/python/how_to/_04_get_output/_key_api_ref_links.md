## Get an Output by ID

You can retrieve any output using its `outputId` (transaction id + output index) by calling
the [`Client.get_output(output_id)`](./../libraries/python/api_reference#get_output) function and providing a
valid output ID.

If the function is successful, it will return an object with the block's metadata and the
output object.

This function call will target the `GET /api/core/v2/outputs/{outputId}` endpoint.