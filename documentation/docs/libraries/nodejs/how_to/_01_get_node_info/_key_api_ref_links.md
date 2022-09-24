### Build a `Client`

You can build [`Client`](./../libraries/nodejs/references/classes/Client) by instantiating it and passing
your [IClientOptions](./../libraries/nodejs/references/interfaces/IClientOptions).

#### IClientOptions

##### Specify your node's URL

The example uses the node defined in your `.env` file, but you could send any node URL in the `nodes` property.

##### Enable local proof of work

The code example enables local proof of work by
sending [`localPow`](./../libraries/nodejs/references/interfaces/IClientOptions#localpow)
as `true`.

### Get the Node's Info

Once you have built the `Client`, you can retrieve the node's information by
running [`Client.getInfo()`](./../libraries/nodejs/references/classes/Client#getinfo). This function will target
the `/api/v2/info` endpoint and return the node's information as well as its URL if successful.