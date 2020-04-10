use std::cmp::Ordering;

use anyhow::Result;
use bee_bundle::{
    Address, Bundle, Hash, Index, Nonce, OutgoingBundleBuilder, Payload, Tag, Timestamp,
    TransactionBuilder, TransactionField, Value,
};
use bee_crypto::Kerl;
use bee_signing::IotaSeed;

use crate::response::{Input, Transfer};
use crate::Client;

/// Builder to construct PrepareTransfers API
//#[derive(Debug)]
pub struct PrepareTransfersBuilder<'a> {
    client: &'a Client<'a>,
    seed: Option<&'a IotaSeed<Kerl>>,
    transfers: Vec<Transfer>,
    security: u8,
    inputs: Option<Vec<Input>>,
    remainder: Option<Address>,
}

impl<'a> PrepareTransfersBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            seed: None,
            transfers: Default::default(),
            security: 2,
            inputs: None,
            remainder: None,
        }
    }

    /// Add iota seed
    pub fn seed(mut self, seed: &'a IotaSeed<Kerl>) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Add transfers
    pub fn transfers(mut self, transfers: Vec<Transfer>) -> Self {
        self.transfers = transfers;
        self
    }

    /// Set security level
    pub fn security(mut self, security: u8) -> Self {
        self.security = security;
        self
    }

    /// Add custom inputs. It is always better to provide inputs yourself
    /// since it will have to seaching valid inputs from the beginning.
    pub fn inputs(mut self, inputs: Vec<Input>) -> Self {
        self.inputs = Some(inputs);
        self
    }

    /// Add custom remainder
    pub fn remainder(mut self, remainder: Address) -> Self {
        self.remainder = Some(remainder);
        self
    }

    /// Send PrepareTransfers request
    pub async fn build(self) -> Result<Bundle> {
        let seed = match self.seed {
            Some(s) => s,
            None => return Err(anyhow!("Seed is not provided")),
        };

        let total_output = self.transfers.iter().fold(0, |acc, tx| acc + tx.value);
        let inputs = match self.inputs {
            Some(i) => i,
            None => {
                self.client
                    .get_inputs()
                    .seed(seed)
                    .index(0)
                    .security(self.security)
                    .threshold(total_output)
                    .generate()
                    .await?
                    .1
            }
        };
        let total_input = inputs.iter().fold(0, |acc, tx| acc + tx.balance);

        let need_remainder = match total_input.cmp(&total_output) {
            Ordering::Less => return Err(anyhow!("Inputs balance is insufficient.")),
            Ordering::Greater => true,
            Ordering::Equal => false,
        };

        let timestamp = chrono::Utc::now().timestamp();

        let mut bundle = OutgoingBundleBuilder::new();
        // add inputs
        for input in &inputs {
            bundle.push(
                TransactionBuilder::new()
                    .with_payload(Payload::zeros())
                    .with_address(input.address.clone())
                    .with_value(Value::from_inner_unchecked(-(input.balance as i64)))
                    .with_obsolete_tag(Tag::zeros())
                    .with_timestamp(Timestamp::from_inner_unchecked(timestamp as u64))
                    .with_index(Index::from_inner_unchecked(0))
                    .with_last_index(Index::from_inner_unchecked(0))
                    .with_tag(Tag::zeros())
                    .with_attachment_ts(Timestamp::from_inner_unchecked(timestamp as u64))
                    .with_bundle(Hash::zeros())
                    .with_trunk(Hash::zeros())
                    .with_branch(Hash::zeros())
                    .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                    .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                    .with_nonce(Nonce::zeros()),
            );
        }

        // add transfers
        for transfer in self.transfers {
            bundle.push(
                TransactionBuilder::new()
                    // TODO add message
                    .with_payload(Payload::zeros())
                    .with_address(transfer.address)
                    .with_value(Value::from_inner_unchecked(transfer.value as i64))
                    .with_obsolete_tag(Tag::zeros())
                    .with_timestamp(Timestamp::from_inner_unchecked(timestamp as u64))
                    .with_index(Index::from_inner_unchecked(0))
                    .with_last_index(Index::from_inner_unchecked(0))
                    // TODO add tag (but probably better to left as is)
                    .with_tag(Tag::zeros())
                    .with_attachment_ts(Timestamp::from_inner_unchecked(timestamp as u64))
                    .with_bundle(Hash::zeros())
                    .with_trunk(Hash::zeros())
                    .with_branch(Hash::zeros())
                    .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                    .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                    .with_nonce(Nonce::zeros()),
            );
        }

        // add remainder
        if need_remainder {
            let remainder = match self.remainder {
                Some(r) => r,
                None => {
                    self.client
                        .get_new_address()
                        .seed(seed)
                        .security(self.security)
                        .index(inputs.last().unwrap().index + 1)
                        .generate()
                        .await?
                        .1
                }
            };

            bundle.push(
                TransactionBuilder::new()
                    .with_payload(Payload::zeros())
                    .with_address(remainder)
                    .with_value(Value::from_inner_unchecked(
                        (total_input - total_output) as i64,
                    ))
                    .with_obsolete_tag(Tag::zeros())
                    .with_timestamp(Timestamp::from_inner_unchecked(timestamp as u64))
                    .with_index(Index::from_inner_unchecked(0))
                    .with_last_index(Index::from_inner_unchecked(0))
                    .with_tag(Tag::zeros())
                    .with_attachment_ts(Timestamp::from_inner_unchecked(timestamp as u64))
                    .with_bundle(Hash::zeros())
                    .with_trunk(Hash::zeros())
                    .with_branch(Hash::zeros())
                    .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                    .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                    .with_nonce(Nonce::zeros()),
            );
        }

        // TODO attach to tangle & validate before build
        Ok(bundle
            .seal()
            .expect("Fail to seal bundle")
            .sign()
            .expect("Fail to sign bundle")
            .attach_local(Hash::zeros(), Hash::zeros())
            .expect("Fail to attach bundle")
            .build()
            .expect("Fail to build bundle"))
    }
}
