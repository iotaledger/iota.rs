use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn find_transactions(
    uri: &str,
    bundles: Option<&[String]>,
    addresses: Option<&[String]>,
    tags: Option<&[String]>,
    approvees: Option<&[String]>,
) -> Result<FindTransactionsResponse, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let mut body = json!({
        "command": "findTransactions",
    });

    if let Some(b) = bundles {
        body["bundles"] = json!(b);
    }
    if let Some(a) = addresses {
        body["addresses"] = json!(a);
    }
    if let Some(t) = tags {
        body["tags"] = json!(t);
    }
    if let Some(a) = approvees {
        body["approvees"] = json!(a);
    }

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct FindTransactionsResponse {
    duration: i64,
    error: Option<String>,
    hashes: Option<Vec<String>>,
}

impl FindTransactionsResponse {
    pub fn duration(&self) -> i64 {
        self.duration
    }
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    pub fn hashes(&self) -> Option<Vec<String>> {
        self.hashes.clone()
    }
}
