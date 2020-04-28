use anyhow::Result;
use reqwest::Url;

use crate::response::RemoveNeighborsResponse;
use crate::Client;

/// Builder to construct removeNeighbors API
#[derive(Debug)]
pub struct RemoveNeighborsBuilder<'a> {
    client: &'a Client<'a>,
    uris: &'a [&'a str],
}

impl<'a> RemoveNeighborsBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            uris: Default::default(),
        }
    }

    /// Slice of neighbor URIs(`&str`) to remove
    pub fn uris(mut self, uris: &'a [&str]) -> Result<Self> {
        for uri in uris {
            match Url::parse(uri)?.scheme() {
                "tcp" | "udp" => (),
                _ => return Err(anyhow!("Uri scheme should be either tcp or udp")),
            }
        }

        self.uris = uris;

        Ok(self)
    }

    /// Send removeNeighbors request
    pub async fn send(self) -> Result<RemoveNeighborsResponse> {
        let client = self.client;
        let body = json!({
            "command": "removeNeighbors",
            "uris": self.uris,
        });

        Ok(response!(client, body))
    }
}
