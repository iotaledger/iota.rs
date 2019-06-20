use iota_model::Neighbor;

/// This is a typed representation of the JSON response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoveNeighborsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
    /// Amount of neighbors removed
    #[serde(rename = "removedNeighbors")]
    removed_neighbors: Option<usize>,
}

impl RemoveNeighborsResponse {
    /// Returns the error attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    fn exception(&self) -> &Option<String> {
        &self.exception
    }
    /// Returns a reference to the amount of removed neighbors
    fn removed_neighbors(&self) -> &Option<usize> {
        &self.removed_neighbors
    }
    /// Consumes the response and returns the amount of removed neighbors if any
    fn take_removed_neighbors(self) -> Option<usize> {
        self.removed_neighbors
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreTransactionsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
}

impl StoreTransactionsResponse {
    /// Returns the error attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    fn exception(&self) -> &Option<String> {
        &self.exception
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WereAddressesSpentFromResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// States of addresses if found
    states: Option<Vec<bool>>,
}

impl WereAddressesSpentFromResponse {
    /// Returns the error attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the states attribute
    pub fn states(self) -> Option<Vec<bool>> {
        self.states
    }
    /// Returns a specfic index into the states attribute
    pub fn state(self, index: usize) -> bool {
        self.states.unwrap_or_default()[index]
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetTrytesResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Trytes if found
    trytes: Option<Vec<String>>,
}

impl GetTrytesResponse {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the trytes attribute
    pub fn trytes(&self) -> &Option<Vec<String>> {
        &self.trytes
    }
    /// Takes ownership the trytes attribute
    pub fn take_trytes(self) -> Option<Vec<String>> {
        self.trytes
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetTransactionsToApprove {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
    /// Trunk transaction to approve
    #[serde(rename = "trunkTransaction")]
    trunk_transaction: Option<String>,
    /// Branch transaction to approve
    #[serde(rename = "branchTransaction")]
    branch_transaction: Option<String>,
}

impl GetTransactionsToApprove {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    pub fn exception(&self) -> &Option<String> {
        &self.exception
    }
    /// Returns the trunk_transaction attribute
    pub fn trunk_transaction(&self) -> Option<String> {
        self.trunk_transaction.clone()
    }
    /// Returns the branch_transaction attribute
    pub fn branch_transaction(&self) -> Option<String> {
        self.branch_transaction.clone()
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetTipsResponse {
    /// Hashes of tips
    hashes: Vec<String>,
}

impl GetTipsResponse {
    /// Returns the hashes attribute
    pub fn hashes(&self) -> &[String] {
        &self.hashes
    }
    /// Takes ownership the hashes attribute
    pub fn take_hashes(self) -> Vec<String> {
        self.hashes
    }
}

/// This is a typed representation of the JSON response
#[derive(Serialize, Deserialize, Debug)]
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

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetNeighborsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Neighbors if found
    neighbors: Option<Vec<Neighbor>>,
}

impl GetNeighborsResponse {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the neighbors attribute
    pub fn neighbors(self) -> Option<Vec<Neighbor>> {
        self.neighbors
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetInclusionStatesResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// States if found
    states: Option<Vec<bool>>,
}

impl GetInclusionStatesResponse {
    /// Returns any potential errors
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    /// Returns the states attribute
    pub fn states(self) -> Option<Vec<bool>> {
        self.states
    }
    /// Returns a specfic index into the states attribute
    pub fn state(self, index: usize) -> bool {
        self.states.unwrap_or_default()[index]
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetBalancesResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Balances if found
    balances: Option<Vec<String>>,
    /// Milestone index if found
    #[serde(rename = "milestoneIndex")]
    milestone_index: Option<i64>,
    /// References if found
    references: Option<Vec<String>>,
}

impl GetBalancesResponse {
    /// Returns any potential errors
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the balances attribute
    pub fn balances(&self) -> &Option<Vec<String>> {
        &self.balances
    }
    /// Takes ownership of the balances attribute
    pub fn take_balances(self) -> Option<Vec<String>> {
        self.balances
    }
    /// Returns the milestone_index attribute
    pub fn milestone_index(&self) -> Option<i64> {
        self.milestone_index
    }
    /// Returns the references attribute
    pub fn references(&self) -> &Option<Vec<String>> {
        &self.references
    }
    /// Takes ownership of the references attribute
    pub fn take_references(self) -> Option<Vec<String>> {
        self.references
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FindTransactionsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Hashes of matching transactions
    hashes: Option<Vec<String>>,
}

impl FindTransactionsResponse {
    /// Returns any potential errors
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the hashes attribute
    pub fn hashes(&self) -> &Option<Vec<String>> {
        &self.hashes
    }
    /// Takes ownership of the hashes attribute
    pub fn take_hashes(self) -> Option<Vec<String>> {
        self.hashes
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BroadcastTransactionsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exception that occurred
    exception: Option<String>,
}

impl BroadcastTransactionsResponse {
    /// Returns any potential errors
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    /// Returns any potential exceptions
    pub fn exception(&self) -> Option<String> {
        self.exception.clone()
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttachToTangleResponse {
    /// This is only used when using PoW Box service
    #[serde(rename = "jobId")]
    job_id: Option<String>,
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
    /// Trytes returned by PoW
    trytes: Option<Vec<String>>,
}

impl AttachToTangleResponse {
    /// Creates a new repsonse
    ///
    /// * `job_id` - This is only used when using PoW Box service
    /// * `error` - Any errors that occurred
    /// * `exception` - Any exceptions that occurred
    /// * `trytes` -  trytes returned by PoW
    pub fn new(
        job_id: Option<String>,
        error: Option<String>,
        exception: Option<String>,
        trytes: Option<Vec<String>>,
    ) -> AttachToTangleResponse {
        AttachToTangleResponse {
            job_id,
            error,
            exception,
            trytes,
        }
    }
    /// Returns the id attribute
    pub fn job_id(&self) -> Option<String> {
        self.job_id.clone()
    }
    /// Returns the error attribute
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    /// Returns the exception attribute
    pub fn exception(&self) -> Option<String> {
        self.exception.clone()
    }
    /// Returns the trytes attribute
    pub fn trytes(self) -> Option<Vec<String>> {
        self.trytes
    }
}

/// This is a typed representation of the JSON response
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AddNeighborsResponse {
    #[serde(rename = "addedNeighbors")]
    added_neighbors: usize,
}

impl AddNeighborsResponse {}
