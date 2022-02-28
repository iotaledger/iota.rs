// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use iota_client::{api::GetBalanceBuilder as RustGetBalanceBuilderApi, Seed as RustSeed};

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{full_node_api::Client, Result};
use anyhow::anyhow;

struct GetBalanceBuilderApiInternal<'a> {
    client: &'a Client,
    seed: RustSeed,
    account_index: usize,
    initial_address_index: usize,
    gap_limit: usize,
}

pub struct GetBalanceBuilderApi<'a> {
    fields: Rc<RefCell<Option<GetBalanceBuilderApiInternal<'a>>>>,
}

impl<'a> GetBalanceBuilderApi<'a> {
    #[allow(dead_code)]
    pub(crate) fn from_old(client: &'a Client, seed: &str) -> Self {
        let internal = GetBalanceBuilderApiInternal {
            client,
            seed: RustSeed::from_bytes(seed.as_bytes()),
            account_index: 0,
            initial_address_index: 0,
            gap_limit: crate::address::ADDRESS_GAP_RANGE,
        };
        Self {
            fields: Rc::new(RefCell::new(Option::from(internal))),
        }
    }

    pub fn new(client: &'a Client, seed: &str) -> Result<Self> {
        match hex::decode(seed) {
            Ok(s) => {
                let internal = GetBalanceBuilderApiInternal {
                    client,
                    seed: RustSeed::from_bytes(&s),
                    account_index: 0,
                    initial_address_index: 0,
                    gap_limit: crate::address::ADDRESS_GAP_RANGE,
                };
                Ok(Self {
                    fields: Rc::new(RefCell::new(Option::from(internal))),
                })
            }
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    fn new_with_fields(fields: GetBalanceBuilderApiInternal<'a>) -> Self {
        Self {
            fields: Rc::new(RefCell::new(Option::from(fields))),
        }
    }

    /// Sets the account index.
    pub fn with_account_index(&self, account_index: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.account_index = account_index;
        GetBalanceBuilderApi::new_with_fields(fields)
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(&self, initial_address_index: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.initial_address_index = initial_address_index;
        GetBalanceBuilderApi::new_with_fields(fields)
    }

    /// Sets the gap limit to specify how many addresses will be checked each round.
    /// If gap_limit amount of addresses in a row have no balance the function will return.
    pub fn with_gap_limit(&self, gap_limit: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.gap_limit = gap_limit;
        GetBalanceBuilderApi::new_with_fields(fields)
    }

    /// Consume the builder and get the API result
    pub fn finish(&self) -> Result<u64> {
        let fields = self.fields.borrow_mut().take().unwrap();
        let ret = crate::block_on(async {
            RustGetBalanceBuilderApi::new(fields.client.borrow(), &fields.seed)
                .with_account_index(fields.account_index)
                .with_initial_address_index(fields.initial_address_index)
                .with_gap_limit(fields.gap_limit)
                .finish()
                .await
        });

        match ret {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
