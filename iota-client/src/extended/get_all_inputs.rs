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
    number_of_addresses: u64,
}

impl<'a> GetAllInputsBuilder<'a> {
    /// Create a new GetAllInputsBuilder
    pub fn builder(client: &'a Client) -> Self {
        Self {
            client,
            seed: None,
            start_index: 0,
            security: 2,
            number_of_addresses: 30,
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

    /// Set the amount of addresses which will be generated
    pub fn with_number_of_addresses(mut self, number_of_addresses: u64) -> Self {
        self.number_of_addresses = number_of_addresses;
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

    /// Send GetInputs request and returns the inputs with their total balance
    pub async fn finish(self) -> Result<(u64, Vec<Input>)> {
        let seed = match self.seed {
            Some(s) => s,
            None => return Err(Error::MissingSeed),
        };

        let mut total_balance = 0;
        let mut inputs = Vec::new();

        let range = self.start_index..self.start_index + self.number_of_addresses;
        println!("{:?}", range);
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
        }

        Ok((total_balance, inputs))
    }
}
