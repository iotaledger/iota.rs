/// SendTrytesOptions
#[derive(Clone, Debug, PartialEq)]
pub struct SendTrytesOptions<'a> {
    /// The depth for getting transactions to approve
    pub depth: usize,
    /// The minimum weight magnitude for doing proof of work
    pub min_weight_magnitude: usize,
    /// Perform PoW locally
    pub local_pow: bool,
    /// Optionally specify how many threads to use, defaults to max available
    pub threads: usize,
    /// Optionally used as the reference to start searching for transactions to approve
    pub reference: Option<&'a str>,
}

impl<'a> Default for SendTrytesOptions<'a> {
    fn default() -> Self {
        SendTrytesOptions {
            depth: 3,
            min_weight_magnitude: 14,
            local_pow: true,
            threads: num_cpus::get(),
            reference: None,
        }
    }
}
