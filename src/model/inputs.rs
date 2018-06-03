use model::input::Input;
use serde_json;
use std::fmt;

#[derive(Default, Serialize, Deserialize)]
struct Inputs {
    inputs_list: Vec<Input>,
    total_balance: i64,
}

impl fmt::Display for Inputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Inputs {
    fn get_inputs_list(&self) -> &Vec<Input> {
        &self.inputs_list
    }

    fn set_inputs_list(&mut self, inputs_list: Vec<Input>) {
        self.inputs_list = inputs_list;
    }

    fn get_total_balance(&self) -> i64 {
        self.total_balance
    }

    fn set_total_balance(&mut self, total_balance: i64) {
        self.total_balance = total_balance;
    }
}
