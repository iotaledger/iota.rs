//! Request types

#[derive(Debug, Serialize)]
pub(crate) struct SingleRequest<'a> {
    pub(crate) command: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct AddressesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) addresses: &'a [String],
}

#[derive(Debug, Serialize)]
pub(crate) struct AttachToTangleRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) trunk_transaction: &'a str,
    pub(crate) branch_transaction: &'a str,
    pub(crate) min_weight_magnitude: usize,
    pub(crate) trytes: &'a [&'a str],
}

#[derive(Debug, Serialize)]
pub(crate) struct FindTransactionsRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) bundles: Option<Vec<String>>,
    pub(crate) addresses: Option<Vec<String>>,
    pub(crate) tags: Option<Vec<String>>,
    pub(crate) approvees: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetBalancesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) addresses: Vec<String>,
    pub(crate) threshold: u8,
    pub(crate) tips: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetInclusionStatesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) transactions: Vec<String>,
    pub(crate) tips: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GTTARequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) depth: usize,
    pub(crate) reference: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub(crate) struct HashesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) hashes: &'a [String],
}

#[derive(Debug, Serialize)]
pub(crate) struct TrytesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) trytes: &'a [String],
}

#[derive(Debug, Serialize)]
pub(crate) struct UrisRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) uris: &'a [String],
}
