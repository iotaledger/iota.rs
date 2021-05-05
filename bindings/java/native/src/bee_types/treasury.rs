// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
// use iota_wallet::message::TreasuryTransactionPayload as TreasuryTransactionPayloadRust;
//
// use iota::Output;
//
// pub struct TreasuryTransactionPayload {
// payload: TreasuryTransactionPayloadRust,
// }
//
// impl From<TreasuryTransactionPayloadRust> for TreasuryTransactionPayload {
// fn from(payload: TreasuryTransactionPayloadRust) -> Self {
// Self { payload }
// }
// }
//
// impl TreasuryTransactionPayload {
// pub fn output(&self) -> u64 {
// if let Output::Treasury(payload) = self.payload.output() {
// return payload.amount();
// }
// unreachable!()
// }
// }
