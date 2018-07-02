use crate::utils::input_validator;
use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn get_trytes(uri: &str, hashes: &[String]) -> Result<GetTrytesResponse, Error> {
    ensure!(
        input_validator::is_array_of_hashes(hashes),
        "Provided hashes are not valid: {:?}",
        hashes
    );

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTrytes",
        "hashes": hashes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct GetTrytesResponse {
    duration: i64,
    trytes: Vec<String>,
}
