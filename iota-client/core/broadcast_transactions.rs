use anyhow::Result;
use iota_bundle_preview::Transaction;

use crate::response::ErrorResponseBuilder;
use crate::util::tx_trytes;
use crate::Client;

/// Builder to construct storeTransactions API
#[derive(Debug)]
pub struct BroadcastTransactionsBuilder<'a> {
    client: &'a Client<'a>,
    trytes: Vec<String>,
}

impl<'a> BroadcastTransactionsBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
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
            "command": "broadcastTransactions",
            "trytes": self.trytes,
        });

        let res: ErrorResponseBuilder = response!(client, body);
        res.build().await
    }
}
