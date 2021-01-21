use crate::error::*;
use bee_signing::ternary::seed::Seed;

use crate::core::AddressBuilder;
use crate::response::Input;
use crate::Client;
use bee_transaction::bundled::Address;

/// Builder to get inputs (spent addresses included)
pub struct GetAllInputsBuilder<'a> {
    client: &'a Client,
    seed: Option<&'a Seed>,
    start_index: u64,
    security: u8,
    threshold: Option<u64>,
    interval: u64,
}

impl<'a> GetAllInputsBuilder<'a> {
    /// Create a new GetAllInputsBuilder
    pub fn builder(client: &'a Client) -> Self {
        Self {
            client,
            seed: None,
            start_index: 0,
            security: 2,
            threshold: None,
            interval: 30,
        }
    }

    /// Set the seed
    pub fn with_seed(mut self, seed: &'a Seed) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set address index from which to start searching for balance
    pub fn with_start_index(mut self, start_index: u64) -> Self {
        self.start_index = start_index;
        self
    }

    /// Set the amount of addresses which will be generated in each round to find one with balance
    pub fn with_interval(mut self, interval: u64) -> Self {
        self.interval = interval;
        self
    }

    /// Set security level
    pub fn with_security(mut self, security: u8) -> Self {
        self.security = match security {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => panic!("Invalid security level"),
        };
        self
    }

    /// Set minimum amount of balance required
    pub fn with_balance_threshold(mut self, threshold: u64) -> Self {
        self.threshold = Some(threshold);
        self
    }

    /// Send GetInputs request and returns the inputs with their total balance
    pub async fn finish(self) -> Result<(u64, Vec<Input>)> {
        // If threshold is None generate addresses until 30 addresses without balance in a row
        let seed = match self.seed {
            Some(s) => s,
            None => return Err(Error::MissingSeed),
        };

        let mut total_balance = 0;
        let mut inputs = Vec::new();
        let mut loop_index = 0;
        loop {
            let range = self.start_index + (loop_index * self.interval)
                ..self.start_index + (loop_index * self.interval) + self.interval;
            let addresses_with_index = AddressBuilder::builder()
                .with_seed(&seed)
                .with_range(range)
                .finish()
                .unwrap();

            let addresses: Vec<Address> = addresses_with_index
                .clone()
                .into_iter()
                .map(|(_, address)| address)
                .collect();

            let balance_response = self
                .client
                .get_balances()
                .addresses(&addresses[..])
                .send()
                .await?;
            let aggregated_balance: u64 = balance_response.balances.iter().sum::<u64>();

            // If the next couple of addresses don't have any balance, we determine it fails to prevent from infinite searching.
            if aggregated_balance == 0 {
                break;
            }

            //Iterate over each balance address output here
            for (index, balance) in balance_response.balances.into_iter().enumerate() {
                if balance == 0 {
                    continue;
                }
                inputs.push(Input {
                    address: addresses_with_index[index].1.clone(),
                    balance,
                    index: addresses_with_index[index].0,
                });
                total_balance += balance;
                if let Some(balance_treshold) = self.threshold {
                    if total_balance >= balance_treshold {
                        return Ok((total_balance, inputs));
                    }
                }
            }
            loop_index += 1;
        }

        if total_balance == 0 {
            return Err(Error::NoBalance);
        }

        if let Some(balance_treshold) = self.threshold {
            if total_balance < balance_treshold {
                return Err(Error::ThresholdNotEnough);
            }
        }

        Ok((total_balance, inputs))
    }
}
