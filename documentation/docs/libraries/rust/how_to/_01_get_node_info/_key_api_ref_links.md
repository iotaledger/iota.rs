### Build a `Client`

You can build a client using
the [`Client::builder`](https://docs.rs/iota-client/latest/iota_client/builder/struct.ClientBuilder.html)
structure.

#### Chaining calls

The code example chains the following calls to the `Client::builder`:

* [`ClientBuilder.with_node(self, url)`](https://docs.rs/iota-client/latest/iota_client/builder/struct.ClientBuilder.html#method.with_node):
  This call will add an IOTA node by URL.
* [`ClientBuilder.with_ignore_node_health(self)`](https://docs.rs/iota-client/latest/iota_client/builder/struct.ClientBuilder.html#method.with_ignore_node_health):
  This call will ignore the node health status. As every node will be considered healthy, you should not use this call
  in a production setting.
* [`ClientBuilder.finish(self)`](https://docs.rs/iota-client/latest/iota_client/builder/struct.ClientBuilder.html#method.finish):
  This call will tell the `ClientBuilder` that you have already chained all the calls you need, and it should return
  the [Client](https://docs.rs/iota-client/latest/iota_client/client/index.html) instance.

### Use the `Client`

After you have [built the client](#build-a-client), you can use it to get the node's information. The code example uses
the [`Client.get_info(&self)`](https://docs.rs/iota-client/latest/iota_client/client/struct.Client.html#method.get_info)
function which will target the `/api/v2/info` endpoint and return the node's information if successful.