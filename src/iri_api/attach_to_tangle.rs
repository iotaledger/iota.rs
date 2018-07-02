use crate::utils::input_validator;
use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn attach_to_tangle(
    uri: &str,
    trunk_transaction: &str,
    branch_transaction: &str,
    min_weight_magnitude: i32,
    trytes: &[String],
) -> Result<AttachToTangleResponse, Error> {
    ensure!(input_validator::is_hash(trunk_transaction), "Provided trunk transaction is not valid: {:?}", trunk_transaction);
    ensure!(input_validator::is_hash(branch_transaction), "Provided branch transaction is not valid: {:?}", branch_transaction);
    ensure!(input_validator::is_array_of_trytes(trytes), "Provided trytes are not valid: {:?}", trytes);

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "attachToTangle",
        "trunkTransaction": trunk_transaction,
        "branchTransaction": branch_transaction,
        "minWeightMagnitude": min_weight_magnitude,
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct AttachToTangleResponse {
    duration: i64,
    exception: Option<String>,
    trytes: Option<Vec<String>>,
}
