use anyhow::Result;
use bee_crypto::Kerl;
use bee_signing::IotaSeed;

use crate::response::Input;
use crate::Client;

/// Builder to construct GetInputs API
//#[derive(Debug)]
pub struct GetInputsBuilder<'a> {
    client: &'a Client<'a>,
    seed: Option<&'a IotaSeed<Kerl>>,
    index: u64,
    security: u8,
    threshold: u64,
}

impl<'a> GetInputsBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            seed: None,
            index: 0,
            security: 2,
            threshold: 0,
        }
    }

    /// Add iota seed
    pub fn seed(mut self, seed: &'a IotaSeed<Kerl>) -> Self {
        self.seed = Some(seed);
        self
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
        let seed = match self.seed {
            Some(s) => s,
            None => return Err(anyhow!("Seed is not provided")),
        };

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
                .get_new_address()
                .seed(seed)
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

            // If next couple of addresses dont have any balance, we determine it fails to prevent from infinite searching.
            if balance == 0 {
                zero_balance_warning -= 1;
            } else {
                zero_balance_warning = 5;
            }

            inputs.push(Input {
                address,
                balance,
                index,
            });
            total += balance;
            index = next_index + 1;

            if total >= self.threshold {
                return Ok((total, inputs));
            }
        }

        Err(anyhow!("Cannot find enough inputs to satisify threshold"))
    }
}
