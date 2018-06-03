#[derive(Default)]
struct Signature {
    address: String,
    signature_fragments: Vec<String>,
}

impl Signature {
    fn get_address(&self) -> &str {
        &self.address
    }

    fn set_address(&mut self, address: String) {
        self.address = address;
    }

    fn get_signature_fragments(&self) -> &Vec<String> {
        &self.signature_fragments
    }
}
