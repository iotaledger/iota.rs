/// GetInputsOptions
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetInputsOptions {
    /// The start index for addresses to search
    pub start: Option<usize>,
    /// The end index for addresses to search
    pub end: Option<usize>,
    /// The amount of Iota you're trying to find in the wallet
    pub threshold: Option<i64>,
    /// The security to use for address generation
    pub security: Option<usize>,
}
