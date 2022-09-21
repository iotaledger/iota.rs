### Build a `Client`

You can build an [`IotaClient`](./../libraries/python/api_reference#iotaclient-objects) by instantiating it and
specifying your desired nodes URL in the `client_config`.

### Get the Node's Info

Once you have built the `IotaClient`, you can retrieve the node's information by
running [`IotaClient.get_Info()`](./../libraries/python/api_reference#get_info). This function will target
the `/api/v2/info` endpoint and return the node's information as well as its URL if successful.