use super::responses::FindTransactionsResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;
/// Finds transactions the match any of the provided parameters
pub fn find_transactions(
    client: &Client,
    uri: &str,
    bundles: Option<&[String]>,
    addresses: Option<&[String]>,
    tags: Option<&[String]>,
    approvees: Option<&[String]>,
) -> Result<FindTransactionsResponse> {
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

    let resp: FindTransactionsResponse = client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = resp.error() {
        return Err(format_err!("{}", error));
    }

    Ok(resp)
}
