use serde_json;
use std::fmt;

/// Represents an address and a grouping of signature fragments
#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    address: String,
    signature_fragments: Vec<String>,
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
    /// Provides a view of the address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Provides a mutable view of the address
    pub fn address_mut(&mut self) -> &mut String {
        &mut self.address
    }

    /// Provides a view of the signature_fragments
    pub fn signature_fragments(&self) -> &[String] {
        &self.signature_fragments
    }

    /// Provides a mutable view of the address
    pub fn signature_fragments_mut(&mut self) -> &mut [String] {
        &mut self.signature_fragments
    }

    /// Inserts a fragment into the signature
    pub fn add_fragment(&mut self, fragment: &str) {
        self.signature_fragments.push(fragment.to_string());
    }
}
