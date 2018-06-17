use model::input::Input;
use serde_json;
use std::fmt;

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct Inputs {
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
    fn inputs_list(&self) -> &[Input] {
        &self.inputs_list
    }

    fn inputs_list_mut(&mut self) -> &mut [Input] {
        &mut self.inputs_list
    }

    fn total_balance(&self) -> i64 {
        self.total_balance
    }

    fn total_balance_mut(&mut self) -> &mut i64 {
        &mut self.total_balance
    }
}
