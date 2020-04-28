use anyhow::Result;
use bee_bundle::{Address, Hash, Tag, TransactionField};
use iota_conversion::Trinary;

use crate::response::{FindTransactionsResponse, FindTransactionsResponseBuilder};
use crate::Client;

/// Builder to construct findTransactions API
#[derive(Debug)]
pub struct FindTransactionsBuilder<'a> {
    client: &'a Client<'a>,
    bundles: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    approvees: Option<Vec<String>>,
}

impl<'a> FindTransactionsBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            bundles: Default::default(),
            addresses: Default::default(),
            tags: Default::default(),
            approvees: Default::default(),
        }
    }

    /// Add bundle hashes to search for
    pub fn bundles(mut self, bundles: &[Hash]) -> Self {
        self.bundles = Some(
            bundles
                .iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Add tags to search for
    pub fn tags(mut self, tags: &[Tag]) -> Self {
        self.tags = Some(
            tags.iter()
                .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Add child transactions to search for
    pub fn approvees(mut self, approvees: &[Hash]) -> Self {
        self.approvees = Some(
            approvees
                .iter()
                .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Add addresses to search for (do not include the checksum)
    pub fn addresses(mut self, addresses: &[Address]) -> Self {
        self.addresses = Some(
            addresses
                .iter()
                .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Send findTransactions request
    pub async fn send(self) -> Result<FindTransactionsResponse> {
        let client = self.client;
        let mut body = json!({
            "command": "findTransactions",
        });

        if let Some(bundles) = self.bundles {
            body["bundles"] = json!(bundles);
        }

        if let Some(addresses) = self.addresses {
            body["addresses"] = json!(addresses);
        }

        if let Some(tags) = self.tags {
            body["tags"] = json!(tags);
        }

        if let Some(approvees) = self.approvees {
            body["approvees"] = json!(approvees);
        }

        let res: FindTransactionsResponseBuilder = response!(client, body);
        res.build().await
    }
}
