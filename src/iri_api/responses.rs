use crate::model::Neighbor;

/// This is a typed representation of the JSON response
#[derive(Clone, Debug, Deserialize)]
pub struct RemoveNeighborsResponse {
    #[serde(rename = "removedNeighbors")]
    removed_neighbors: usize,
    error: Option<String>,
    exception: Option<String>,
}

impl RemoveNeighborsResponse {
    /// Returns the duration attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the duration attribute
    fn exception(&self) -> &Option<String> {
        &self.exception
    }
}

/// This is a typed representation of the JSON response
#[derive(Clone, Debug, Deserialize)]
pub struct StoreTransactionsResponse {
    duration: i64,
    error: Option<String>,
    exception: Option<String>,
}

impl StoreTransactionsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the duration attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the duration attribute
    fn exception(&self) -> &Option<String> {
        &self.exception
    }
}

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct WereAddressesSpentFromResponse {
    duration: i64,
    error: Option<String>,
    states: Option<Vec<bool>>,
}

impl WereAddressesSpentFromResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
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
#[derive(Deserialize, Debug)]
pub struct GetTrytesResponse {
    duration: i64,
    trytes: Vec<String>,
}

impl GetTrytesResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the trytes attribute
    pub fn trytes(&self) -> &[String] {
        &self.trytes
    }
    /// Takes ownership the trytes attribute
    pub fn take_trytes(self) -> Vec<String> {
        self.trytes
    }
}

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct GetTransactionsToApprove {
    duration: i64,
    error: Option<String>,
    #[serde(rename = "trunkTransaction")]
    trunk_transaction: Option<String>,
    #[serde(rename = "branchTransaction")]
    branch_transaction: Option<String>,
}

impl GetTransactionsToApprove {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
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
#[derive(Deserialize, Debug)]
pub struct GetTipsResponse {
    duration: i64,
    hashes: Vec<String>,
}

impl GetTipsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
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

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct GetNeighborsResponse {
    duration: i64,
    error: Option<String>,
    neighbors: Option<Vec<Neighbor>>,
}

impl GetNeighborsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
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
#[derive(Deserialize, Debug)]
pub struct GetInclusionStatesResponse {
    duration: i64,
    error: Option<String>,
    states: Option<Vec<bool>>,
}

impl GetInclusionStatesResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
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
#[derive(Clone, Deserialize, Debug)]
pub struct GetBalancesResponse {
    duration: i64,
    error: Option<String>,
    balances: Option<Vec<String>>,
    #[serde(rename = "milestoneIndex")]
    milestone_index: Option<i64>,
    references: Option<Vec<String>>,
}

impl GetBalancesResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the duration attribute
    pub fn balances(&self) -> &Option<Vec<String>> {
        &self.balances
    }
    /// Returns the duration attribute
    pub fn take_balances(self) -> Option<Vec<String>> {
        self.balances
    }
    /// Returns the duration attribute
    pub fn milestone_index(&self) -> Option<i64> {
        self.milestone_index
    }
    /// Returns the duration attribute
    pub fn references(&self) -> &Option<Vec<String>> {
        &self.references
    }
    /// Returns the duration attribute
    pub fn take_references(self) -> Option<Vec<String>> {
        self.references
    }
}

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct FindTransactionsResponse {
    duration: i64,
    error: Option<String>,
    hashes: Option<Vec<String>>,
}

impl FindTransactionsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
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
#[derive(Deserialize, Debug)]
pub struct BroadcastTransactionsResponse {
    duration: i64,
    error: Option<String>,
    exception: Option<String>,
}

impl BroadcastTransactionsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    pub fn exception(&self) -> Option<String> {
        self.exception.clone()
    }
}

/// This is a typed representation of the JSON response
/// `duration` will be zero if local PoW is selected.
#[derive(Deserialize, Debug)]
pub struct AttachToTangleResponse {
    duration: i64,
    id: Option<String>,
    error: Option<String>,
    exception: Option<String>,
    trytes: Option<Vec<String>>,
}

impl AttachToTangleResponse {
    pub fn new(
        duration: i64,
        id: Option<String>,
        error: Option<String>,
        exception: Option<String>,
        trytes: Option<Vec<String>>,
    ) -> AttachToTangleResponse {
        AttachToTangleResponse {
            duration,
            id,
            error,
            exception,
            trytes,
        }
    }
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the id attribute
    pub fn id(&self) -> Option<String> {
        self.id.clone()
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
#[derive(Copy, Clone, Deserialize, Debug)]
pub struct AddNeighborsResponse {
    #[serde(rename = "addedNeighbors")]
    added_neighbors: usize,
}

impl AddNeighborsResponse {}
