use failure::Error;
use reqwest::{self, Response};

pub fn interrupt_attaching_to_tangle(uri: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    Ok(client.post(uri).send()?)
}
