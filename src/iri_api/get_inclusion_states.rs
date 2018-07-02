use crate::utils::input_validator;
use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn get_inclusion_states(
    uri: &str,
    transactions: &[String],
    tips: &[String],
) -> Result<GetInclusionStatesResponse, Error> {
    ensure!(
        input_validator::is_array_of_hashes(transactions),
        "Provided transactions are not valid: {:?}",
        transactions
    );
    ensure!(
        input_validator::is_array_of_hashes(tips),
        "Provided tips are not valid: {:?}",
        tips
    );

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getInclusionStates",
        "transactions": transactions,
        "tips": tips,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct GetInclusionStatesResponse {
    duration: i64,
    error: Option<String>,
    states: Option<Vec<bool>>,
}
