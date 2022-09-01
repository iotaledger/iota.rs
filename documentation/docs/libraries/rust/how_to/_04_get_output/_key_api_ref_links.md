## Get an Output by ID

You can retrieve any output using its `outputId` (transaction id + output index) by calling
the [`Client.get_output(output_id)`](.iota_client/client/struct.Client.html#method.get_output) function and providing a
valid output ID. 

If the function is successful, it will return an `OutputResponse` with the block's metadata and the
output object.

This function call will target the `GET /api/core/v2/outputs/{outputId}` endpoint.