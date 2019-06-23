use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// Represents an address and a grouping of signature fragments
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub address: String,
    pub signature_fragments: Vec<String>,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Signature {
    /// Inserts a fragment into the signature
    pub fn add_fragment(&mut self, fragment: impl Into<String>) {
        self.signature_fragments.push(fragment.into());
    }
}
