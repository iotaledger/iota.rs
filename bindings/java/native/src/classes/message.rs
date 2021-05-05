// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
// use std::{cell::RefCell, rc::Rc};
//
// use iota_wallet::{
// address::AddressWrapper,
// message::{
// Message as MessageRust, MessageId, RemainderValueStrategy as RemainderValueStrategyRust,
// Transfer as TransferRust, TransferBuilder as TransferBuilderRust,
// },
// };
//
// use crate::bee_types::{IndexationPayload, MessagePayload};
//
// use chrono::prelude::{DateTime, Utc};
// use std::num::NonZeroU64;
//
// pub enum RemainderValueStrategy {
// ReuseAddress = 1,
// ChangeAddress = 2,
// }
//
// pub fn remainder_type_enum_to_type(strategy: RemainderValueStrategy) -> RemainderValueStrategyRust {
// match strategy {
// RemainderValueStrategy::ReuseAddress => RemainderValueStrategyRust::ReuseAddress,
// RemainderValueStrategy::ChangeAddress => RemainderValueStrategyRust::ChangeAddress,
// }
// }
//
// pub struct Transfer {
// transfer: TransferRust,
// }
//
// impl Transfer {
// pub fn to_inner(self) -> TransferRust {
// self.transfer
// }
//
// pub fn builder(address: AddressWrapper, amount: u64) -> TransferBuilder {
// TransferBuilder::new(address, amount)
// }
// }
//
// pub struct TransferBuilder {
// builder: Rc<RefCell<Option<TransferBuilderRust>>>,
// }
//
// impl TransferBuilder {
// pub fn new(address: AddressWrapper, amount: u64) -> Self {
// Self {
// builder: Rc::new(RefCell::new(Option::from(TransferBuilderRust::new(
// address,
// NonZeroU64::new(amount).unwrap(),
// )))),
// }
// }
//
// pub fn new_with_builder(builder: TransferBuilderRust) -> Self {
// Self {
// builder: Rc::new(RefCell::new(Option::from(builder))),
// }
// }
//
// pub fn with_remainder_value_strategy(&mut self, strategy: RemainderValueStrategy) -> Self {
// let new_builder = self
// .builder
// .borrow_mut()
// .take()
// .unwrap()
// .with_remainder_value_strategy(remainder_type_enum_to_type(strategy));
// TransferBuilder::new_with_builder(new_builder)
// }
//
// pub fn with_remainder_to_account_with_address(&mut self, address: AddressWrapper) -> Self {
// let new_builder = self
// .builder
// .borrow_mut()
// .take()
// .unwrap()
// .with_remainder_value_strategy(RemainderValueStrategyRust::AccountAddress(address));
// TransferBuilder::new_with_builder(new_builder)
// }
//
// pub fn with_indexation(&mut self, indexation: IndexationPayload) -> Self {
// let new_builder = self
// .builder
// .borrow_mut()
// .take()
// .unwrap()
// .with_indexation(indexation.to_inner());
// TransferBuilder::new_with_builder(new_builder)
// }
//
// Builds the transfer.
// pub fn finish(&self) -> Transfer {
// Transfer {
// transfer: self.builder.borrow_mut().take().unwrap().finish(),
// }
// }
// }
//
// #[derive(PartialEq)]
// pub struct Message {
// message: MessageRust,
// }
//
// impl Clone for Message {
// fn clone(&self) -> Self {
// Message {
// message: self.message.clone(),
// }
// }
// }
//
// impl From<MessageRust> for Message {
// fn from(message: MessageRust) -> Self {
// Self { message }
// }
// }
//
// impl Message {
// pub fn id(&self) -> MessageId {
// self.message.id().clone()
// }
// pub fn version(&self) -> u64 {
// (self.message.version())
// }
// pub fn parents(&self) -> Vec<MessageId> {
// self.message.parents().to_vec()
// }
// pub fn payload_length(&self) -> usize {
// (self.message.payload_length())
// }
//
// pub fn payload(&self) -> Option<MessagePayload> {
// match self.message.payload() {
// None => None,
// Some(e) => Some(e.clone().into()),
// }
// }
// pub fn timestamp(&self) -> DateTime<Utc> {
// (self.message.timestamp())
// }
// pub fn nonce(&self) -> u64 {
// (self.message.nonce())
// }
// pub fn confirmed(&self) -> Option<bool> {
// (self.message.confirmed())
// }
// pub fn broadcasted(&self) -> bool {
// (self.message.broadcasted())
// }
//
// pub fn to_inner(self) -> MessageRust {
// self.message
// }
// }
