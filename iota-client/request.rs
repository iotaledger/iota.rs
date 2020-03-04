//! Request types

#[derive(Debug, Serialize)]
pub(crate) struct SingleRequest<'a> {
    pub(crate) command: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct AddressesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) addresses: &'a [&'a str],
}

#[derive(Debug, Serialize)]
pub(crate) struct AttachToTangleRequest<'a> {
    pub(crate) command: &'a str,
    #[serde(rename = "trunkTransaction")]
    pub(crate) trunk_transaction: &'a str,
    #[serde(rename = "branchTransaction")]
    pub(crate) branch_transaction: &'a str,
    #[serde(rename = "minWeightMagnitude")]
    pub(crate) min_weight_magnitude: usize,
    pub(crate) trytes: &'a [&'a str],
}

#[derive(Debug, Serialize)]
pub(crate) struct FindTransactionsRequest<'a> {
    pub(crate) command: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) bundles: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) addresses: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) tags: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) approvees: Vec<&'a str>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetBalancesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) addresses: &'a [&'a str],
    pub(crate) threshold: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) tips: Vec<&'a str>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetInclusionStatesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) transactions: &'a [&'a str],
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) tips: Vec<&'a str>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GTTARequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) depth: usize,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub(crate) reference: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct HashesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) hashes: &'a [&'a str],
}

#[derive(Debug, Serialize)]
pub(crate) struct TailsRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) tails: &'a [&'a str],
}

#[derive(Debug, Serialize)]
pub(crate) struct TrytesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) trytes: &'a [&'a str],
}

#[derive(Debug, Serialize)]
pub(crate) struct UrisRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) uris: &'a [&'a str],
}
