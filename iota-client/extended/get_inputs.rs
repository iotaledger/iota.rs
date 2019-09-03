use crate::client::Client;
use crate::extended::get_new_address::new_address;
use crate::options::{GetBalancesOptions, GetNewAddressOptions};
use crate::Result;
use iota_model::{Input, Inputs};

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

impl<'a> Client<'a> {
    /// Given a seed, iterates through addresses looking for
    /// enough funds to meet specified threshold
    ///
    /// * `seed` - The wallet seed to use
    /// * `options` - See `GetInputsOptions`
    pub fn get_inputs(&mut self, seed: &str, options: GetInputsOptions) -> Result<Inputs> {
        ensure!(iota_validation::is_trytes(&seed), "Invalid seed.");
        let start = options.start.unwrap_or(0);
        let security = options.security.unwrap_or(2);

        if let Some(end) = options.end {
            ensure!(
                start <= end && end <= start + 500,
                "Invalid inputs provided."
            );
            let mut all_addresses: Vec<String> = vec![];
            for i in start..end {
                all_addresses.push((new_address(&seed, security, i, false))?);
            }
            self.get_balance_and_format(&all_addresses, start, options.threshold, security)
        } else {
            let new_address = self.get_new_address(
                seed,
                false,
                true,
                GetNewAddressOptions {
                    security: Some(security),
                    index: Some(start),
                    total: None,
                },
            )?;
            self.get_balance_and_format(&new_address, start, options.threshold, security)
        }
    }

    fn get_balance_and_format(
        &mut self,
        addresses: &[String],
        start: usize,
        threshold: Option<i64>,
        security: usize,
    ) -> Result<Inputs> {
        let resp = self.get_balances(GetBalancesOptions {
            addresses: addresses.to_owned(),
            ..GetBalancesOptions::default()
        })?;
        let mut inputs = Inputs::default();

        let mut threshold_reached = threshold.is_none();

        let balances = resp.take_balances().unwrap_or_default();
        for (i, address) in addresses.iter().enumerate() {
            let balance: i64 = balances[i].clone().parse()?;
            if balance > 0 {
                let new_entry = Input {
                    address: address.clone(),
                    balance,
                    key_index: start + i,
                    security,
                };
                inputs.add(new_entry);
                *inputs.total_balance_mut() += balance;
                if let Some(threshold) = threshold {
                    if inputs.total_balance() >= threshold {
                        threshold_reached = true;
                    }
                }
            }
        }
        if threshold_reached {
            Ok(inputs)
        } else {
            Err(format_err!("Not enough balance."))
        }
    }
}
