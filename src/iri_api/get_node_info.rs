use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn get_node_info(uri: &str) -> Result<GetNodeInfoResponse, Error> {
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
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct GetNodeInfoResponse {
    #[serde(rename = "appName")]
    app_name: String,
    #[serde(rename = "appVersion")]
    app_version: String,
    duration: i64,
    #[serde(rename = "jreAvailableProcessors")]
    jre_available_processors: usize,
    #[serde(rename = "jreFreeMemory")]
    jre_free_memory: usize,
    #[serde(rename = "jreMaxMemory")]
    jre_max_memory: usize,
    #[serde(rename = "jreTotalMemory")]
    jre_total_memory: usize,
    #[serde(rename = "jreVersion")]
    jre_version: String,
    #[serde(rename = "latestMilestone")]
    latest_milestone: String,
    #[serde(rename = "latestMilestoneIndex")]
    latest_milestone_index: usize,
    #[serde(rename = "latestSolidSubtangleMilestone")]
    latest_solid_subtangle_milestone: String,
    #[serde(rename = "latestSolidSubtangleMilestoneIndex")]
    latest_solid_subtangle_milestone_index: usize,
    #[serde(rename = "milestoneStartIndex")]
    milestone_start_index: usize,
    neighbors: usize,
    #[serde(rename = "packetsQueueSize")]
    packets_queue_size: usize,
    time: usize,
    tips: usize,
    #[serde(rename = "transactionsToRequest")]
    transactions_to_request: usize,
}
