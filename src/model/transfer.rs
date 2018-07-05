use serde_json;
use std::fmt;

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
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn address_mut(&mut self) -> &mut String {
        &mut self.address
    }

    pub fn hash(&self) -> &Option<String> {
        &self.hash
    }

    pub fn hash_mut(&mut self) -> &mut Option<String> {
        &mut self.hash
    }

    pub fn persistence(&self) -> &Option<bool> {
        &self.persistence
    }

    pub fn persistence_mut(&mut self) -> &mut Option<bool> {
        &mut self.persistence
    }

    pub fn timestamp(&self) -> &Option<String> {
        &self.timestamp
    }

    pub fn timestamp_mut(&mut self) -> &mut Option<String> {
        &mut self.timestamp
    }

    pub fn value(&self) -> &i64 {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut i64 {
        &mut self.value
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn message_mut(&mut self) -> &mut String {
        &mut self.message
    }

    pub fn tag(&self) -> Option<String> {
        self.tag.clone()
    }

    pub fn tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }

    pub fn obsolete_tag(&self) -> Option<String> {
        self.obsolete_tag.clone()
    }

    pub fn obsolete_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.obsolete_tag
    }
}
