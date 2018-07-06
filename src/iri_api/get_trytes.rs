use super::responses::GetTrytesResponse;
use crate::utils::input_validator;
use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Returns the raw transaction data (trytes) of a specific
/// transaction. These trytes can then be easily converted
/// into the actual transaction object. See utility functions
/// for more details.
pub fn get_trytes(uri: &str, hashes: &[String]) -> Result<GetTrytesResponse> {
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
