use reqwest::{Client, Error};

/// Gets information about the specified node
pub(crate) async fn get_node_info(
    client: &Client,
    uri: &str,
) -> Result<GetNodeInfoResponse, Error> {
    let body = json!({
        "command": "getNodeInfo",
    });

    client
        .post(uri)
        .header("Content-Type", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
        .await?
        .json()
        .await
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
    jre_available_processors: u16,
    /// Amount of free memory on IRI node
    #[serde(rename = "jreFreeMemory")]
    jre_free_memory: u64,
    /// Max amount of memory on IRI node
    #[serde(rename = "jreMaxMemory")]
    jre_max_memory: u64,
    /// Total amount of memory on IRI node
    #[serde(rename = "jreTotalMemory")]
    jre_total_memory: u64,
    /// JRE version of IRI node
    #[serde(rename = "jreVersion")]
    jre_version: String,
    /// Latest milestone on IRI node
    #[serde(rename = "latestMilestone")]
    latest_milestone: String,
    /// Latest milestone index on IRI node
    #[serde(rename = "latestMilestoneIndex")]
    latest_milestone_index: u32,
    /// Latest solid subtangle milestone on IRI node
    #[serde(rename = "latestSolidSubtangleMilestone")]
    latest_solid_subtangle_milestone: String,
    /// Latest solid subtangle milestone index on IRI node
    #[serde(rename = "latestSolidSubtangleMilestoneIndex")]
    latest_solid_subtangle_milestone_index: u32,
    /// Milestone start index on IRI node
    #[serde(rename = "milestoneStartIndex")]
    milestone_start_index: u32,
    /// Amount of neighbors connected to IRI node
    neighbors: u16,
    /// Packet queue size on IRI node
    #[serde(rename = "packetsQueueSize")]
    packets_queue_size: u16,
    /// Current time on IRI node (UNIX Seconds),
    time: u64,
    /// Amount of tips on IRI node
    tips: u32,
    /// Transactions to request on IRI node
    #[serde(rename = "transactionsToRequest")]
    transactions_to_request: u32,
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
    pub fn jre_available_processors(&self) -> u16 {
        self.jre_available_processors
    }
    /// Returns the jre_free_memory attribute
    pub fn jre_free_memory(&self) -> u64 {
        self.jre_free_memory
    }
    /// Returns the jre_max_memory attribute
    pub fn jre_max_memory(&self) -> u64 {
        self.jre_max_memory
    }
    /// Returns the jre_total_memory attribute
    pub fn jre_total_memory(&self) -> u64 {
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
    pub fn latest_milestone_index(&self) -> u32 {
        self.latest_milestone_index
    }
    /// Returns the latest_solid_subtangle_milestone attribute
    pub fn latest_solid_subtangle_milestone(&self) -> &str {
        &self.latest_solid_subtangle_milestone
    }
    /// Returns the latest_solid_subtangle_milestone_index attribute
    pub fn latest_solid_subtangle_milestone_index(&self) -> u32 {
        self.latest_solid_subtangle_milestone_index
    }
    /// Returns the milestone_start_index attribute
    pub fn milestone_start_index(&self) -> u32 {
        self.milestone_start_index
    }
    /// Returns the neighbors attribute
    pub fn neighbors(&self) -> u16 {
        self.neighbors
    }
    /// Returns the packets_queue_size attribute
    pub fn packets_queue_size(&self) -> u16 {
        self.packets_queue_size
    }
    /// Returns the time attribute
    pub fn time(&self) -> u64 {
        self.time
    }
    /// Returns the tips attribute
    pub fn tips(&self) -> u32 {
        self.tips
    }
    /// Returns the transactions_to_request attribute
    pub fn transactions_to_request(&self) -> u32 {
        self.transactions_to_request
    }
}
