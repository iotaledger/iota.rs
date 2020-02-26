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
    pub(crate) trunk_transaction: &'a str,
    pub(crate) branch_transaction: &'a str,
    pub(crate) min_weight_magnitude: usize,
    pub(crate) trytes: &'a [&'a str],
}

#[derive(Debug, Serialize)]
pub(crate) struct FindTransactionsRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) bundles: Option<&'a [&'a str]>,
    pub(crate) addresses: Option<&'a [&'a str]>,
    pub(crate) tags: Option<&'a [&'a str]>,
    pub(crate) approvees: Option<&'a [&'a str]>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetBalancesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) addresses: &'a [&'a str],
    pub(crate) threshold: u8,
    pub(crate) tips: Option<&'a [&'a str]>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetInclusionStatesRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) transactions: &'a [&'a str],
    pub(crate) tips: Option<&'a [&'a str]>,
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
