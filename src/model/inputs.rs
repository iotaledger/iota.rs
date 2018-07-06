use super::input::Input;
use serde_json;
use std::fmt;

/// Represents a grouping of inputs and their cumulative balance
#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    inputs_list: Vec<Input>,
    total_balance: i64,
}

impl fmt::Display for Inputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Inputs {
    /// Provides a view of the inputs_list
    pub fn inputs_list(&self) -> &[Input] {
        &self.inputs_list
    }

    /// Provides a mutable view of the inputs_list
    pub fn inputs_list_mut(&mut self) -> &mut [Input] {
        &mut self.inputs_list
    }

    /// Provides a view of the inputs address
    pub fn add(&mut self, input: Input) {
        self.inputs_list.push(input);
    }

    /// Provides a view of the total_balance
    pub fn total_balance(&self) -> i64 {
        self.total_balance
    }

    /// Provides a mutable view of the total_balance
    pub fn total_balance_mut(&mut self) -> &mut i64 {
        &mut self.total_balance
    }
}
