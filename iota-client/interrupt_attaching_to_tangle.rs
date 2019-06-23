use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Interupts an existing PoW request if you made one
pub fn interrupt_attaching_to_tangle(
    client: &Client,
    uri: &str,
) -> impl Future<Item = Response, Error = Error> {
    client.post(uri).send()
}
