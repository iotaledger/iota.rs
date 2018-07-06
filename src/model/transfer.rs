use serde_json;
use std::fmt;

/// Represents a transfer in IOTA
#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Transfer {
    timestamp: Option<String>,
    address: String,
    hash: Option<String>,
    persistence: Option<bool>,
    value: i64,
    message: String,
    tag: Option<String>,
    obsolete_tag: Option<String>,
}

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Transfer {
    /// Provides a view of the address
    pub fn address(&self) -> &str {
        &self.address
    }
    /// Provides a mutable view of the address
    pub fn address_mut(&mut self) -> &mut String {
        &mut self.address
    }
    /// Provides a view of the hash
    pub fn hash(&self) -> &Option<String> {
        &self.hash
    }
    /// Provides a mutable view of the hash
    pub fn hash_mut(&mut self) -> &mut Option<String> {
        &mut self.hash
    }
    /// Provides a view of the persistence
    pub fn persistence(&self) -> &Option<bool> {
        &self.persistence
    }
    /// Provides a mutable view of the persistence
    pub fn persistence_mut(&mut self) -> &mut Option<bool> {
        &mut self.persistence
    }
    /// Provides a view of the timestamp
    pub fn timestamp(&self) -> &Option<String> {
        &self.timestamp
    }
    /// Provides a mutable view of the timestamp
    pub fn timestamp_mut(&mut self) -> &mut Option<String> {
        &mut self.timestamp
    }
    /// Provides a view of the value
    pub fn value(&self) -> &i64 {
        &self.value
    }
    /// Provides a mutable view of the value
    pub fn value_mut(&mut self) -> &mut i64 {
        &mut self.value
    }
    /// Provides a view of the message
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Provides a mutable view of the message
    pub fn message_mut(&mut self) -> &mut String {
        &mut self.message
    }
    /// Provides a view of the tag
    pub fn tag(&self) -> Option<String> {
        self.tag.clone()
    }
    /// Provides a mutable view of the tag
    pub fn tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }
    /// Provides a view of the obsolete_tag
    pub fn obsolete_tag(&self) -> Option<String> {
        self.obsolete_tag.clone()
    }
    /// Provides a mutable view of the obsolete_tag
    pub fn obsolete_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.obsolete_tag
    }
}
