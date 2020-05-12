use anyhow::Result;
use iota_bundle_preview::Transaction;

use crate::response::ErrorResponseBuilder;
use crate::util::tx_trytes;
use crate::Client;

/// Builder to construct storeTransactions API
#[derive(Debug)]
pub struct StoreTransactionsBuilder<'a> {
    client: &'a Client,
    trytes: Vec<String>,
}

impl<'a> StoreTransactionsBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            trytes: Default::default(),
        }
    }

    /// Add transaction trytes
    pub fn trytes(mut self, trytes: &[Transaction]) -> Self {
        self.trytes = trytes.iter().map(|tx| tx_trytes(tx)).collect();
        self
    }

    /// Send storeTransactions request
    pub async fn send(self) -> Result<()> {
        let client = self.client;
        let body = json!({
            "command": "storeTransactions",
            "trytes": self.trytes,
        });

        let res: ErrorResponseBuilder = response!(client, body);
        res.build().await
    }
}
