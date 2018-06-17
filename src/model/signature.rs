use std::fmt;
use serde_json;

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct Signature {
    address: String,
    signature_fragments: Vec<String>,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

impl Signature {
    fn address(&self) -> &str {
        &self.address
    }

    fn address_mut(&mut self) -> &mut String {
        &mut self.address
    }

    fn signature_fragments(&self) -> &[String] {
        &self.signature_fragments
    }

    fn signature_fragments_mut(&mut self) -> &mut [String] {
        &mut self.signature_fragments
    }
}
