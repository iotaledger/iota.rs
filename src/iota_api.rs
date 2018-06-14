use errors::*;
use reqwest::header::{ContentType, Headers};
use reqwest::{self, Response};
use serde_json::Value;

const REQUEST_FAILED: &'static str = "API request failed";
const PARSE_FAILED: &'static str = "Failed to parse json response";

pub fn get_node_info(uri: &str) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNodeInfo",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn get_neighbors(uri: &str) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNeighbors",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn add_neighbors(uri: &str, uris: &[String]) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "addNeighbors",
        "uris": uris,
    });

   Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn remove_neighbors(uri: &str, uris: &[String]) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "removeNeighbors",
        "uris": uris,
    });

  Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn get_tips(uri: &str) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTips",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn find_transactions(
    uri: &str,
    bundles: Option<&[String]>,
    addresses: Option<&[String]>,
    tags: Option<&[String]>,
    approvees: Option<&[String]>,
) -> Result<Value> {
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
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn get_trytes(uri: &str, hashes: &[String]) -> Result<Value> {
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
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn get_inclusion_states(uri: &str, transactions: &[String], tips: &[String]) -> Result<Value> {
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
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn get_balances(uri: &str, addresses: &[String], threshold: i32) -> Result<Value> {
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
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn get_transactions_to_approve(uri: &str, depth: i32) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTransactionsToApprove",
        "depth": depth,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn attach_to_tangle(
    uri: &str,
    trunk_transaction: &str,
    branch_transaction: &str,
    min_weight_magnitude: i32,
    trytes: &[String],
) -> Result<Value> {
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
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn interrupt_attaching_to_tangle(uri: &str) -> Result<Response> {
    let client = reqwest::Client::new();
    Ok(client.post(uri).send().chain_err(|| REQUEST_FAILED)?)
}

pub fn broadcast_transactions(uri: &str, trytes: &[String]) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "broadcastTransactions",
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}

pub fn store_transactions(uri: &str, trytes: &[String]) -> Result<Value> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "storeTransactions",
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()
        .chain_err(|| REQUEST_FAILED)?
        .json()
        .chain_err(|| PARSE_FAILED)?)
}
