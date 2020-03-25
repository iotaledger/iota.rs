//! Response types
use anyhow::Result;
use bee_bundle::Hash;

/// addNeighbors Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct AddNeighborsResponse {
    #[serde(rename = "addedNeighbors")]
    /// Total number of added neighbors
    pub added_neighbors: Option<usize>,
}

/// checkConsistency Response Type
#[derive(Clone, Debug)]
pub struct ConsistencyResponse {
    /// State of the given transactions in the `tails` parameter. A `true` value means
    /// that all given transactions are consistent. A `false` value means that one
    /// or more of the given transactions are inconsistent.
    pub state: bool,
    /// If the `state` field is false, this field contains information about why the transaction is inconsistent.
    pub info: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConsistencyResponseBuilder {
    state: Option<bool>,
    info: Option<String>,
    exception: Option<String>,
    error: Option<String>,
}

impl ConsistencyResponseBuilder {
    pub(crate) async fn build(self) -> Result<ConsistencyResponse> {
        let mut state = false;
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        } else if let Some(s) = self.state {
            state = s;
        }

        Ok(ConsistencyResponse {
            state: state,
            info: self.info,
        })
    }
}

/// attachToTangle Response Type
#[derive(Clone, Debug)]
pub struct AttachToTangleResponse {
    /// Transaction trytes that include a valid `nonce` field
    pub trytes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct AttachToTangleResponseBuilder {
    trytes: Option<Vec<String>>,
    error: Option<String>,
    exception: Option<String>,
}

impl AttachToTangleResponseBuilder {
    pub(crate) async fn build(self) -> Result<AttachToTangleResponse> {
        let mut trytes: Vec<String> = Vec::new();
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        } else if let Some(s) = self.trytes {
            trytes = s;
        }

        Ok(AttachToTangleResponse { trytes })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ErrorResponseBuilder {
    error: Option<String>,
    exception: Option<String>,
}

impl ErrorResponseBuilder {
    pub(crate) async fn build(self) -> Result<()> {
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        }

        Ok(())
    }
}

/// findTransactions Response Type
#[derive(Clone, Debug)]
pub struct FindTransactionsResponse {
    /// The transaction hashes which are returned depend on your input.
    /// * bundles: returns an array of transaction hashes that contain the given bundle hash.
    /// * addresses: returns an array of transaction hashes that contain the given address in the address field.
    /// * tags: returns an array of transaction hashes that contain the given value in the tag field.
    /// * approvees: returns an array of transaction hashes that contain the given transactions in their branchTransaction or trunkTransaction fields.
    pub hashes: Vec<Hash>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct FindTransactionsResponseBuilder {
    hashes: Option<Vec<String>>,
    error: Option<String>,
    exception: Option<String>,
}

impl FindTransactionsResponseBuilder {
    pub(crate) async fn build(self) -> Result<FindTransactionsResponse> {
        let mut hashes: Vec<Hash> = Vec::new();
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        } else if let Some(s) = self.hashes {
            hashes = s.iter().map(|s| Hash::from_str(&s)).collect::<Vec<Hash>>();
        }

        Ok(FindTransactionsResponse { hashes })
    }
}

/// getBalances Response Type
#[derive(Clone, Debug)]
pub struct GetBalancesResponse {
    /// Array of balances in the same order as the `addresses` parameters were passed to the endpoint
    pub balances: Vec<String>,
    /// The index of the milestone that confirmed the most recent balance
    pub milestone_index: i64,
    /// The referencing tips. If no `tips` parameter was passed to the endpoint,
    /// this field contains the hash of the latest milestone that confirmed the balance
    pub references: Vec<Hash>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct GetBalancesResponseBuilder {
    balances: Option<Vec<String>>,
    #[serde(rename = "milestoneIndex")]
    milestone_index: Option<i64>,
    references: Option<Vec<String>>,
    error: Option<String>,
    exception: Option<String>,
}

impl GetBalancesResponseBuilder {
    pub(crate) async fn build(self) -> Result<GetBalancesResponse> {
        let mut res = GetBalancesResponse {
            balances: Vec::new(),
            milestone_index: 0,
            references: Vec::new(),
        };

        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        }

        if let Some(s) = self.balances {
            res.balances = s;
        }

        if let Some(s) = self.milestone_index {
            res.milestone_index = s;
        }

        if let Some(s) = self.references {
            res.references = s.iter().map(|s| Hash::from_str(&s)).collect::<Vec<Hash>>();
        }

        Ok(res)
    }
}

/// getInclusionStatesResponse Response Type
#[derive(Clone, Debug)]
pub struct GetInclusionStatesResponse {
    /// List of boolean values in the same order as the `transactions` parameters.
    /// A `true` value means the transaction was confirmed
    pub states: Vec<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct GetInclusionStatesResponseBuilder {
    states: Option<Vec<bool>>,
    error: Option<String>,
    exception: Option<String>,
}

impl GetInclusionStatesResponseBuilder {
    pub(crate) async fn build(self) -> Result<GetInclusionStatesResponse> {
        let mut states = Vec::new();
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        } else if let Some(s) = self.states {
            states = s;
        }

        Ok(GetInclusionStatesResponse { states })
    }
}

/// getNeighbors Response Type
#[derive(Clone, Debug)]
pub struct GetNeighborsResponse {
    /// Vector of `NeighborResponse`
    pub neighbors: Vec<NeighborResponse>,
}

/// getNeighbors Response Type
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct GetNeighborsResponseBuilder {
    neighbors: Option<Vec<NeighborResponse>>,
    error: Option<String>,
    exception: Option<String>,
}

impl GetNeighborsResponseBuilder {
    pub(crate) async fn build(self) -> Result<GetNeighborsResponse> {
        let mut neighbors = Vec::new();
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        } else if let Some(s) = self.neighbors {
            neighbors = s;
        }

        Ok(GetNeighborsResponse { neighbors })
    }
}

/// getNodeAPIConfiguration Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct GetNodeAPIConfigurationResponse {
    /// Maximum number of transactions that may be returned by the findTransactions endpoint
    #[serde(rename = "maxFindTransactions")]
    pub max_find_transactions: Option<usize>,
    /// Maximum number of parameters in an API call
    #[serde(rename = "maxRequestsList")]
    pub max_requests_list: Option<usize>,
    /// Maximum number of trytes that may be returned by the getTrytes endpoint
    #[serde(rename = "maxGetTrytes")]
    pub max_get_trytes: Option<usize>,
    /// Maximum number of characters that the body of an API call may contain
    #[serde(rename = "maxBodyLength")]
    pub max_body_length: Option<usize>,
    /// See if the node runs on a network other than the Mainnet
    #[serde(rename = "testNet")]
    pub testnet: Option<bool>,
    /// Milestone start index on IRI node
    #[serde(rename = "milestoneStartIndex")]
    pub milestone_start_index: i64,
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
#[derive(Clone, Debug)]
pub struct GTTAResponse {
    /// Valid trunk transaction hash
    pub trunk_transaction: Hash,
    /// Valid branch transaction hash
    pub branch_transaction: Hash,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct GTTAResponseBuilder {
    #[serde(rename = "trunkTransaction")]
    trunk_transaction: Option<String>,
    #[serde(rename = "branchTransaction")]
    branch_transaction: Option<String>,
    error: Option<String>,
    exception: Option<String>,
}

impl GTTAResponseBuilder {
    pub(crate) async fn build(self) -> Result<GTTAResponse> {
        let mut res = GTTAResponse {
            trunk_transaction: Hash::zeros(),
            branch_transaction: Hash::zeros(),
        };

        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        }

        if let Some(s) = self.trunk_transaction {
            res.trunk_transaction = Hash::from_str(&s);
        }

        if let Some(b) = self.branch_transaction {
            res.branch_transaction = Hash::from_str(&b);
        }

        Ok(res)
    }
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
#[derive(Clone, Debug)]
pub struct GetTrytesResponse {
    /// Vector of transaction trytes for the given transaction hashes (in the same order as the parameters)
    pub trytes: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct GetTrytesResponseBuilder {
    trytes: Option<Vec<String>>,
    exception: Option<String>,
    error: Option<String>,
}

impl GetTrytesResponseBuilder {
    pub(crate) async fn build(self) -> Result<GetTrytesResponse> {
        let mut trytes = Vec::new();
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        } else if let Some(s) = self.trytes {
            trytes = s;
        }

        Ok(GetTrytesResponse { trytes })
    }
}

/// removeNeighbors Response Type
#[derive(Clone, Debug, Deserialize)]
pub struct RemoveNeighborsResponse {
    /// Total number of removed neighbors
    #[serde(rename = "removedNeighbors")]
    pub removed_neighbors: Option<usize>,
}

/// wereAddressesSpentFrom Response Type
#[derive(Clone, Debug)]
pub struct WereAddressesSpentFromResponse {
    /// States of the specified addresses in the same order as the values in the `addresses` parameter.
    /// A `true` value means that the address has been spent from.
    pub states: Vec<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct WereAddressesSpentFromResponseBuilder {
    states: Option<Vec<bool>>,
    exception: Option<String>,
    error: Option<String>,
}

impl WereAddressesSpentFromResponseBuilder {
    pub(crate) async fn build(self) -> Result<WereAddressesSpentFromResponse> {
        let mut states = Vec::new();
        if let Some(exception) = self.exception {
            return Err(anyhow!("{}", exception));
        } else if let Some(error) = self.error {
            return Err(anyhow!("{}", error));
        } else if let Some(s) = self.states {
            states = s;
        }

        Ok(WereAddressesSpentFromResponse { states })
    }
}
