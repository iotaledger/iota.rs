use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Gets information about the specified node
pub(crate) fn get_node_info(
    client: &Client,
    uri: &str,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "getNodeInfo",
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}

/// This is a typed representation of the JSON response
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct GetNodeInfoResponse {
    /// Name of IRI node
    #[serde(rename = "appName")]
    app_name: String,
    /// IRI version
    #[serde(rename = "appVersion")]
    app_version: String,
    /// Number of threads IRI is using
    #[serde(rename = "jreAvailableProcessors")]
    jre_available_processors: usize,
    /// Amount of free memory on IRI node
    #[serde(rename = "jreFreeMemory")]
    jre_free_memory: usize,
    /// Max amount of memory on IRI node
    #[serde(rename = "jreMaxMemory")]
    jre_max_memory: usize,
    /// Total amount of memory on IRI node
    #[serde(rename = "jreTotalMemory")]
    jre_total_memory: usize,
    /// JRE version of IRI node
    #[serde(rename = "jreVersion")]
    jre_version: String,
    /// Latest milestone on IRI node
    #[serde(rename = "latestMilestone")]
    latest_milestone: String,
    /// Latest milestone index on IRI node
    #[serde(rename = "latestMilestoneIndex")]
    latest_milestone_index: usize,
    /// Latest solid subtangle milestone on IRI node
    #[serde(rename = "latestSolidSubtangleMilestone")]
    latest_solid_subtangle_milestone: String,
    /// Latest solid subtangle milestone index on IRI node
    #[serde(rename = "latestSolidSubtangleMilestoneIndex")]
    latest_solid_subtangle_milestone_index: usize,
    /// Milestone start index on IRI node
    #[serde(rename = "milestoneStartIndex")]
    milestone_start_index: usize,
    /// Amount of neighbors connected to IRI node
    neighbors: usize,
    /// Packet queue size on IRI node
    #[serde(rename = "packetsQueueSize")]
    packets_queue_size: usize,
    /// Current time on IRI node (UNIX Seconds),
    time: usize,
    /// Amount of tips on IRI node
    tips: usize,
    /// Transactions to request on IRI node
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
