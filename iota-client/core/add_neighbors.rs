use anyhow::Result;
use reqwest::Url;

use crate::response::AddNeighborsResponse;
use crate::Client;

/// Builder to construct removeNeighbors API
#[derive(Debug)]
pub struct AddNeighborsBuilder<'a> {
    client: &'a Client,
    uris: Vec<String>,
}

impl<'a> AddNeighborsBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            uris: Default::default(),
        }
    }

    /// Slice of neighbor URIs(`&str`) to add
    pub fn uris(mut self, uris: Vec<String>) -> Result<Self> {
        for uri in &uris {
            match Url::parse(&uri)?.scheme() {
                "tcp" | "udp" => (),
                _ => return Err(anyhow!("Uri scheme should be either tcp or udp")),
            }
        }

        self.uris = uris;

        Ok(self)
    }

    /// Send removeNeighbors request
    pub async fn send(self) -> Result<AddNeighborsResponse> {
        let client = self.client;
        let body = json!({
            "command": "addNeighbors",
            "uris": self.uris,
        });

        Ok(response!(client, body))
    }
}
