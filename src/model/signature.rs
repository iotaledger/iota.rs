#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct Signature {
    address: String,
    signature_fragments: Vec<String>,
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
