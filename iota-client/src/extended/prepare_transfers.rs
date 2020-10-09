use std::cmp::Ordering;

use bee_crypto::ternary::Hash;
use bee_signing::ternary::{seed::Seed, wots::WotsSecurityLevel};
use bee_ternary::{T1B1Buf, TritBuf};
use bee_transaction::bundled::{
    Address, Bundle, BundledTransactionBuilder as TransactionBuilder, BundledTransactionField,
    Index, Nonce, OutgoingBundleBuilder, Payload, Tag, Timestamp, Value, PAYLOAD_TRIT_LEN,
};

use crate::error::*;
use crate::response::{Input, Transfer};
use crate::util::str_to_trytes;
use crate::Client;

/// Builder to construct PrepareTransfers API
//#[derive(Debug)]
pub struct PrepareTransfersBuilder<'a> {
    client: &'a Client,
    seed: Option<&'a Seed>,
    transfers: Vec<Transfer>,
    security: u8,
    inputs: Option<Vec<Input>>,
    remainder: Option<Address>,
}

impl<'a> PrepareTransfersBuilder<'a> {
    pub(crate) fn new(client: &'a Client, seed: Option<&'a Seed>) -> Self {
        Self {
            client,
            seed,
            transfers: Default::default(),
            security: 2,
            inputs: None,
            remainder: None,
        }
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
        let total_output = self.transfers.iter().fold(0, |acc, tx| acc + tx.value);
        let inputs = if total_output > 0 {
            match self.inputs {
                Some(i) => i,
                None => {
                    self.client
                        .get_inputs(self.seed.ok_or(Error::MissingSeed)?)
                        .index(0)
                        .security(self.security)
                        .threshold(total_output)
                        .generate()
                        .await?
                        .1
                }
            }
        } else {
            Vec::new()
        };
        let total_input = inputs.iter().fold(0, |acc, tx| acc + tx.balance);

        let need_remainder = match total_input.cmp(&total_output) {
            Ordering::Less => return Err(Error::ThresholdNotEnough),
            Ordering::Greater => true,
            Ordering::Equal => false,
        };

        let timestamp = chrono::Utc::now().timestamp();

        let mut bundle = OutgoingBundleBuilder::default();
        // add transfers
        for transfer in self.transfers {
            if let Some(message) = transfer.message {
                let message: TritBuf<T1B1Buf> = str_to_trytes(&message).as_trits().encode();
                let mut value = transfer.value as i64;
                let tag = match transfer.tag {
                    Some(t) => t,
                    None => Tag::zeros(),
                };

                for i in message.chunks(PAYLOAD_TRIT_LEN) {
                    let mut trits = TritBuf::<T1B1Buf>::zeros(PAYLOAD_TRIT_LEN);
                    trits.subslice_mut(0..i.len()).copy_from(i);
                    let payload = Payload::from_inner_unchecked(trits);

                    bundle.push(
                        TransactionBuilder::new()
                            .with_payload(payload)
                            .with_address(transfer.address.clone())
                            .with_value(Value::from_inner_unchecked(value))
                            .with_obsolete_tag(Tag::zeros())
                            .with_timestamp(Timestamp::from_inner_unchecked(timestamp as u64))
                            .with_index(Index::from_inner_unchecked(0))
                            .with_last_index(Index::from_inner_unchecked(0))
                            // TODO add tag (but probably better to left as is)
                            .with_tag(tag.clone())
                            .with_attachment_ts(Timestamp::from_inner_unchecked(0))
                            .with_bundle(Hash::zeros())
                            .with_trunk(Hash::zeros())
                            .with_branch(Hash::zeros())
                            .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                            .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                            .with_nonce(Nonce::zeros()),
                    );
                    value = 0;
                }
            } else {
                bundle.push(
                    TransactionBuilder::new()
                        .with_payload(Payload::zeros())
                        .with_address(transfer.address.clone())
                        .with_value(Value::from_inner_unchecked(transfer.value as i64))
                        .with_obsolete_tag(Tag::zeros())
                        .with_timestamp(Timestamp::from_inner_unchecked(timestamp as u64))
                        .with_index(Index::from_inner_unchecked(0))
                        .with_last_index(Index::from_inner_unchecked(0))
                        // TODO add tag (but probably better to left as is)
                        .with_tag(Tag::zeros())
                        .with_attachment_ts(Timestamp::from_inner_unchecked(0))
                        .with_bundle(Hash::zeros())
                        .with_trunk(Hash::zeros())
                        .with_branch(Hash::zeros())
                        .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                        .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                        .with_nonce(Nonce::zeros()),
                );
            }
        }

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
                    .with_attachment_ts(Timestamp::from_inner_unchecked(0))
                    .with_bundle(Hash::zeros())
                    .with_trunk(Hash::zeros())
                    .with_branch(Hash::zeros())
                    .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                    .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                    .with_nonce(Nonce::zeros()),
            );

            for _ in 1..self.security {
                bundle.push(
                    TransactionBuilder::new()
                        // TODO add message
                        .with_payload(Payload::zeros())
                        .with_address(input.address.clone())
                        .with_value(Value::from_inner_unchecked(0))
                        .with_obsolete_tag(Tag::zeros())
                        .with_timestamp(Timestamp::from_inner_unchecked(timestamp as u64))
                        .with_index(Index::from_inner_unchecked(0))
                        .with_last_index(Index::from_inner_unchecked(0))
                        // TODO add tag (but probably better to left as is)
                        .with_tag(Tag::zeros())
                        .with_attachment_ts(Timestamp::from_inner_unchecked(0))
                        .with_bundle(Hash::zeros())
                        .with_trunk(Hash::zeros())
                        .with_branch(Hash::zeros())
                        .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                        .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                        .with_nonce(Nonce::zeros()),
                );
            }
        }

        // add remainder
        if need_remainder {
            let remainder = match self.remainder {
                Some(r) => r,
                None => {
                    self.client
                        .generate_new_address(self.seed.ok_or(Error::MissingSeed)?)
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
                    .with_attachment_ts(Timestamp::from_inner_unchecked(0))
                    .with_bundle(Hash::zeros())
                    .with_trunk(Hash::zeros())
                    .with_branch(Hash::zeros())
                    .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                    .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                    .with_nonce(Nonce::zeros()),
            );
        }

        // TODO bundle crate uses tuple for convinience atm. We should sync the type.
        let security = match self.security {
            1 => WotsSecurityLevel::Low,
            2 => WotsSecurityLevel::Medium,
            3 => WotsSecurityLevel::High,
            _ => panic!("Invalid scurity level"),
        };

        if total_output > 0 {
            let inputs: Vec<(usize, Address, WotsSecurityLevel)> = inputs
                .into_iter()
                .map(|i| (i.index as usize, i.address, security))
                .collect();

            Ok(bundle
                .seal()
                .expect("Fail to seal bundle")
                .sign(self.seed.ok_or(Error::MissingSeed)?, &inputs[..])
                .expect("Fail to sign bundle")
                .attach_local(Hash::zeros(), Hash::zeros())
                .expect("Fail to attach bundle")
                .build()
                .expect("Fail to build bundle"))
        } else {
            Ok(bundle
                .seal()
                .expect("Fail to seal bundle")
                .attach_local(Hash::zeros(), Hash::zeros())
                .expect("Fail to attach bundle")
                .build()
                .expect("Fail to build bundle"))
        }
    }
}
