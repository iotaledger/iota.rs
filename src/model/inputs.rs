use serde_json;
use std::fmt;

use super::Input;

/// Represents a grouping of inputs and their cumulative balance
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_inputs_list<T>(&mut self, new_value: T)
    where
        T: Into<Vec<Input>>,
    {
        self.inputs_list = new_value.into();
    }

    /// Provides a view of the inputs address
    pub fn add<T>(&mut self, new_value: T)
    where
        T: Into<Input>,
    {
        self.inputs_list.push(new_value.into());
    }

    /// Provides a view of the total_balance
    pub fn total_balance(&self) -> i64 {
        self.total_balance
    }

    /// Provides a mutable view of the total_balance
    pub fn total_balance_mut(&mut self) -> &mut i64 {
        &mut self.total_balance
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_total_balance<T>(&mut self, new_value: T)
    where
        T: Into<i64>,
    {
        self.total_balance = new_value.into();
    }
}
