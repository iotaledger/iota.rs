use reqwest::{Client, Error, Response};

/// Interupts an existing PoW request if you made one
pub(crate) async fn interrupt_attaching_to_tangle(
    client: &Client,
    uri: &str,
) -> Result<Response, Error> {
    client.post(uri).send().await
}
