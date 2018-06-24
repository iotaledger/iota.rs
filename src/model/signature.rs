use serde_json;
use std::fmt;

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
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn address_mut(&mut self) -> &mut String {
        &mut self.address
    }

    pub fn signature_fragments(&self) -> &[String] {
        &self.signature_fragments
    }

    pub fn signature_fragments_mut(&mut self) -> &mut [String] {
        &mut self.signature_fragments
    }
}
