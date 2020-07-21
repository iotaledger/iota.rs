use crate::error::*;
use bee_crypto::ternary::Kerl;
use bee_signing::ternary::TernarySeed as Seed;

use crate::response::Input;
use crate::Client;

/// Builder to construct GetInputs API
//#[derive(Debug)]
pub struct GetInputsBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed<Kerl>,
    index: u64,
    security: u8,
    threshold: u64,
}

impl<'a> GetInputsBuilder<'a> {
    pub(crate) fn new(client: &'a Client, seed: &'a Seed<Kerl>) -> Self {
        Self {
            client,
            seed,
            index: 0,
            security: 2,
            threshold: 0,
        }
    }

    /// Set key index to start search at
    pub fn index(mut self, index: u64) -> Self {
        self.index = index;
        self
    }

    /// Set security level
    pub fn security(mut self, security: u8) -> Self {
        self.security = security;
        self
    }

    /// Set minimum amount of balance required
    pub fn threshold(mut self, threshold: u64) -> Self {
        self.threshold = threshold;
        self
    }

    /// Send GetInputs request
    pub async fn generate(self) -> Result<(u64, Vec<Input>)> {
        if self.threshold == 0 {
            return Ok((0, Vec::default()));
        }

        let mut index = self.index;
        let mut total = 0;
        let mut inputs = Vec::new();
        let mut zero_balance_warning = 5;

        while zero_balance_warning != 0 {
            let (next_index, address) = self
                .client
                .generate_new_address(self.seed)
                .index(index)
                .security(self.security)
                .generate()
                .await?;

            let balance = self
                .client
                .get_balances()
                .addresses(&[address.clone()])
                .send()
                .await?
                .balances[0];

            // If the next couple of addresses don't have any balance, we determine it fails to prevent from infinite searching.
            if balance == 0 {
                zero_balance_warning -= 1;
            } else {
                zero_balance_warning = 5;
            }

            total += balance;
            index = next_index;
            inputs.push(Input {
                address,
                balance,
                index,
            });
            index += 1;

            if total >= self.threshold {
                return Ok((total, inputs));
            }
        }

        Err(Error::ThresholdNotEnough)
    }
}
