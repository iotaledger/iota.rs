use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Gets information about the specified node
pub fn get_node_info(uri: &str) -> Result<GetNodeInfoResponse> {
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

/// This is a typed representation of the JSON response
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

impl GetNodeInfoResponse {
    /// Returns the app_name attribute
    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    /// Returns the app_version attribute
    pub fn app_version(&self) -> &str {
        &self.app_version
    }
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the jre_available_processors attribute
    pub fn jre_available_processors(&self) -> usize {
        self.jre_available_processors
    }
    /// Returns the jre_free_memory attribute
    pub fn jre_free_memory(&self) -> usize {
        self.jre_free_memory
    }
    /// Returns the jre_max_memory attribute
    pub fn jre_max_memory(&self) -> usize {
        self.jre_max_memory
    }
    /// Returns the jre_total_memory attribute
    pub fn jre_total_memory(&self) -> usize {
        self.jre_total_memory
    }
    /// Returns the jre_version attribute
    pub fn jre_version(&self) -> &str {
        &self.jre_version
    }
    /// Returns the latest_milestone attribute
    pub fn latest_milestone(&self) -> &str {
        &self.latest_milestone
    }
    /// Returns the latest_milestone_index attribute
    pub fn latest_milestone_index(&self) -> usize {
        self.latest_milestone_index
    }
    /// Returns the latest_solid_subtangle_milestone attribute
    pub fn latest_solid_subtangle_milestone(&self) -> &str {
        &self.latest_solid_subtangle_milestone
    }
    /// Returns the latest_solid_subtangle_milestone_index attribute
    pub fn latest_solid_subtangle_milestone_index(&self) -> usize {
        self.latest_solid_subtangle_milestone_index
    }
    /// Returns the milestone_start_index attribute
    pub fn milestone_start_index(&self) -> usize {
        self.milestone_start_index
    }
    /// Returns the neighbors attribute
    pub fn neighbors(&self) -> usize {
        self.neighbors
    }
    /// Returns the packets_queue_size attribute
    pub fn packets_queue_size(&self) -> usize {
        self.packets_queue_size
    }
    /// Returns the time attribute
    pub fn time(&self) -> usize {
        self.time
    }
    /// Returns the tips attribute
    pub fn tips(&self) -> usize {
        self.tips
    }
    /// Returns the transactions_to_request attribute
    pub fn transactions_to_request(&self) -> usize {
        self.transactions_to_request
    }
}
