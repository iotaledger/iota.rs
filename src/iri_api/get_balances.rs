use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn get_balances(
    uri: &str,
    addresses: &[String],
    threshold: i32,
) -> Result<GetBalancesResponse, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getBalances",
        "addresses": addresses,
        "threshold": threshold,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct GetBalancesResponse {
    duration: i64,
    error: Option<String>,
    balances: Option<Vec<String>>,
    #[serde(rename = "milestoneIndex")]
    milestone_index: Option<i64>,
    references: Option<Vec<String>>,
}

impl GetBalancesResponse {
    pub fn duration(&self) -> i64 {
        self.duration
    }
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    pub fn balances(&self) -> Option<Vec<String>> {
        self.balances.clone()
    }
    pub fn milestone_index(&self) -> Option<i64> {
        self.milestone_index
    }
    pub fn references(&self) -> Option<Vec<String>> {
        self.references.clone()
    }
}
