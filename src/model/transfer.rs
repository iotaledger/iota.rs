use serde_json;
use std::fmt;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Transfer {
    timestamp: Option<String>,
    address: String,
    hash: Option<String>,
    persistence: Option<bool>,
    value: u64,
    message: Option<String>,
    tag: Option<String>,
}

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Transfer {
    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_address_mut(&mut self) -> &mut str {
        &mut self.address
    }

    pub fn get_hash(&self) -> &Option<String> {
        &self.hash
    }

    pub fn get_hash_mut(&mut self) -> &mut Option<String> {
        &mut self.hash
    }

    pub fn get_persistence(&self) -> &Option<bool> {
        &self.persistence
    }

    pub fn get_persistence_mut(&mut self) -> &mut Option<bool> {
        &mut self.persistence
    }

    pub fn get_timestamp(&self) -> &Option<String> {
        &self.timestamp
    }

    pub fn get_timestamp_mut(&mut self) -> &mut Option<String> {
        &mut self.timestamp
    }

    pub fn get_value(&self) -> &u64 {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut u64 {
        &mut self.value
    }

    pub fn get_message(&self) -> &Option<String> {
        &self.message
    }

    pub fn get_message_mut(&mut self) -> &mut Option<String> {
        &mut self.message
    }

    pub fn get_tag(&self) -> &Option<String> {
        &self.tag
    }

    pub fn get_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }
}
