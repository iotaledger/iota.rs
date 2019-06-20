use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

/// Represents an address and a grouping of signature fragments
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_address<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.address = new_value.into();
    }

    /// Provides a view of the signature_fragments
    pub fn signature_fragments(&self) -> &[String] {
        &self.signature_fragments
    }

    /// Provides a mutable view of the address
    pub fn signature_fragments_mut(&mut self) -> &mut [String] {
        &mut self.signature_fragments
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_signature_fragments<T>(&mut self, new_value: T)
    where
        T: Into<Vec<String>>,
    {
        self.signature_fragments = new_value.into();
    }

    /// Inserts a fragment into the signature
    pub fn add_fragment<S>(&mut self, fragment: S)
    where
        S: Into<String>,
    {
        self.signature_fragments.push(fragment.into());
    }
}
