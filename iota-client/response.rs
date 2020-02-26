//! Response types

/// addNeighbors Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct AddNeighborsResponse {
    #[serde(rename = "addedNeighbors")]
    /// Total number of added neighbors
    pub added_neighbors: Option<usize>,
}

/// checkConsistency Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct ConsistencyResponse {
    /// State of the given transactions in the `tails` parameter. A `true` value means
    /// that all given transactions are consistent. A `false` value means that one
    /// or more of the given transactions are inconsistent.
    pub state: Option<bool>,
    /// If the `state` field is false, this field contains information about why the transaction is inconsistent.
    pub info: Option<String>,
    /// Any exception that occurred
    pub exception: Option<String>,
    /// Any errors that occurred
    pub error: Option<String>,
}

/// attachToTangle Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct AttachToTangleResponse {
    /// Transaction trytes that include a valid `nonce` field
    pub trytes: Option<Vec<String>>,
    /// Any errors that occurred
    pub error: Option<String>,
    /// Any exceptions that occurred
    pub exception: Option<String>,
}

/// storeTransactions/broadcastTransaction Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    /// Any error that occurred
    pub error: Option<String>,
    /// Any exceptions that occurred
    pub exception: Option<String>,
}

/// findTransactions Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct FindTransactionsResponse {
    /// The transaction hashes which are returned depend on your input.
    /// * bundles: returns an array of transaction hashes that contain the given bundle hash.
    /// * addresses: returns an array of transaction hashes that contain the given address in the address field.
    /// * tags: returns an array of transaction hashes that contain the given value in the tag field.
    /// * approvees: returns an array of transaction hashes that contain the given transactions in their branchTransaction or trunkTransaction fields.
    pub hashes: Option<Vec<String>>,
    /// Any errors that occurred
    pub error: Option<String>,
}

/// getBalances Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct GetBalancesResponse {
    /// Array of balances in the same order as the `addresses` parameters were passed to the endpoint
    pub balances: Option<Vec<String>>,
    /// The index of the milestone that confirmed the most recent balance
    #[serde(rename = "milestoneIndex")]
    pub milestone_index: Option<i64>,
    /// The referencing tips. If no `tips` parameter was passed to the endpoint,
    /// this field contains the hash of the latest milestone that confirmed the balance
    pub references: Option<Vec<String>>,
    /// Any error that occurred
    pub error: Option<String>,
}

/// getInclusionStatesResponse Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct GetInclusionStatesResponse {
    /// List of boolean values in the same order as the `transactions` parameters.
    /// A `true` value means the transaction was confirmed
    pub states: Option<Vec<bool>>,
    /// Any errors that occurred
    pub error: Option<String>,
    /// Any exceptions that occurred
    pub exception: Option<String>,
}

/// getNeighbors Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct GetNeighborsResponse {
    /// Vector of `NeighborResponse`
    pub neighbors: Vec<NeighborResponse>,
}

/// getNodeInfo Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct GetNodeInfoResponse {
    /// Name of IRI node
    #[serde(rename = "appName")]
    pub app_name: String,
    /// IRI version
    #[serde(rename = "appVersion")]
    pub app_version: String,
    /// Number of threads IRI is using
    #[serde(rename = "jreAvailableProcessors")]
    pub jre_available_processors: Option<u16>,
    /// Amount of free memory on IRI node
    #[serde(rename = "jreFreeMemory")]
    pub jre_free_memory: Option<u64>,
    /// Max amount of memory on IRI node
    #[serde(rename = "jreMaxMemory")]
    pub jre_max_memory: Option<u64>,
    /// Total amount of memory on IRI node
    #[serde(rename = "jreTotalMemory")]
    pub jre_total_memory: Option<u64>,
    /// JRE version of IRI node
    #[serde(rename = "jreVersion")]
    pub jre_version: Option<String>,
    /// Latest milestone on IRI node
    #[serde(rename = "latestMilestone")]
    pub latest_milestone: String,
    /// Latest milestone index on IRI node
    #[serde(rename = "latestMilestoneIndex")]
    pub latest_milestone_index: u32,
    /// Latest solid subtangle milestone on IRI node
    #[serde(rename = "latestSolidSubtangleMilestone")]
    pub latest_solid_subtangle_milestone: String,
    /// Latest solid subtangle milestone index on IRI node
    #[serde(rename = "latestSolidSubtangleMilestoneIndex")]
    pub latest_solid_subtangle_milestone_index: u32,
    /// Milestone start index on IRI node
    #[serde(rename = "milestoneStartIndex")]
    pub milestone_start_index: i64,
    /// Amount of neighbors connected to IRI node
    pub neighbors: u16,
    /// Packet queue size on IRI node
    #[serde(rename = "packetsQueueSize")]
    pub packets_queue_size: u16,
    /// Current time on IRI node (UNIX Seconds),
    pub time: u64,
    /// Amount of tips on IRI node
    pub tips: u32,
    /// Transactions to request on IRI node
    #[serde(rename = "transactionsToRequest")]
    pub transactions_to_request: u32,
}

/// getTips Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct GetTipsResponse {
    /// Vector of tip transaction hashes
    pub hashes: Vec<String>,
}

/// getTransactionsToApprove Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct GTTAResponse {
    /// Valid trunk transaction hash
    #[serde(rename = "trunkTransaction")]
    pub trunk_transaction: Option<String>,
    /// Valid branch transaction hash
    #[serde(rename = "branchTransaction")]
    pub branch_transaction: Option<String>,
    /// Any errors that occurred
    pub error: Option<String>,
}

/// Representation of neighbor node
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct NeighborResponse {
    /// IP address of neighbors
    pub address: String,
    /// Domain of neighbors
    pub domain: String,
    /// Number of all transactions
    #[serde(rename = "numberOfAllTransactions")]
    pub number_of_all_transactions: usize,
    /// Number of invalid transactions
    #[serde(rename = "numberOfInvalidTransactions")]
    pub number_of_invalid_transactions: usize,
    /// Number of new transactions
    #[serde(rename = "numberOfNewTransactions")]
    pub number_of_new_transactions: usize,
    /// Number of random transaction requests
    #[serde(rename = "numberOfRandomTransactionRequests")]
    pub number_of_random_transactions: usize,
    /// Number of sent transactions
    #[serde(rename = "numberOfSentTransactions")]
    pub number_of_sent_transactions: usize,
    /// Number of sent transactions
    #[serde(rename = "numberOfStaleTransactions")]
    pub number_of_stale_transactions: usize,
    /// Number of sent transactions
    #[serde(rename = "numberOfDroppedSentPackets")]
    pub number_of_dropped_sent_packets: usize,
    /// Type of connection, either tcp or udp
    #[serde(rename = "connectionType")]
    pub connection_type: String,
    /// Status of Neighbor connection
    pub connected: bool,
}

/// getTrytes Response Type
#[derive(Clone, Deserialize, Debug)]
pub struct GetTrytesResponse {
    /// Vector of transaction trytes for the given transaction hashes (in the same order as the parameters)
    pub trytes: Option<Vec<String>>,
    /// Any exception that occurred
    pub exception: Option<String>,
    /// Any errors that occurred
    pub error: Option<String>,
}

/// removeNeighbors Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct RemoveNeighborsResponse {
    /// Total number of removed neighbors
    #[serde(rename = "removedNeighbors")]
    pub removed_neighbors: Option<usize>,
}

/// wereAddressesSpentFrom Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct WereAddressesSpentFromResponse {
    /// States of the specified addresses in the same order as the values in the `addresses` parameter.
    /// A `true` value means that the address has been spent from.
    pub states: Option<Vec<bool>>,
    /// Any exception that occurred
    pub exception: Option<String>,
}
