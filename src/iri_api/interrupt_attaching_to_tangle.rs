use crate::Result;
use reqwest::Client;
use reqwest::Response;

/// Interupts an existing PoW request if you made one
pub fn interrupt_attaching_to_tangle(client: &Client, uri: &str) -> Result<Response> {
    Ok(client.post(uri).send()?)
}
