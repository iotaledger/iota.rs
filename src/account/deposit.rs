use std::time::Instant;

pub struct Conditions {
    timeout_at: Instant,
    multi_use: bool,
    expected_amount: u64,
}

pub struct CDA {
    conditions: Conditions,
    address: String,
}

impl CDA {}
