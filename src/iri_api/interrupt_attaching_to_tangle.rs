use crate::Result;
use reqwest::{self, Response};

/// Interupts an existing PoW request if you made one
pub fn interrupt_attaching_to_tangle(uri: &str) -> Result<Response> {
    let client = reqwest::Client::new();
    Ok(client.post(uri).send()?)
}
