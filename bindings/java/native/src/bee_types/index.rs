 // Copyright 2020 IOTA Stiftung
 // SPDX-License-Identifier: Apache-2.0
 use crate::Result;
 use anyhow::anyhow;

 use iota_client::bee_message::prelude::{
    IndexationPayload as RustIndexationPayload,
 };

 pub struct IndexationPayload {
    payload: RustIndexationPayload,
 }

 impl IndexationPayload {
    pub fn to_inner(self) -> RustIndexationPayload {
        self.payload
    }

    pub fn new(index: &[u8], data: &[u8]) -> Result<IndexationPayload> {
        let index = RustIndexationPayload::new(&index, &data);
        match index {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(i) => Ok(IndexationPayload { payload: i }),
        }
    }

    pub fn index(&self) -> &[u8] {
        self.payload.index()
    }

    pub fn data(&self) -> &[u8] {
        self.payload.data()
    }
 }


 /*
 bee_message::prelude::{
        Address as RustAddress, Ed25519Address as RustEd25519Address, Ed25519Signature as RustEd25519Signature,
        Essence as RustEssence, IndexationPayload as RustIndexationPayload, Input as RustInput, Message as RustMessage,
        MigratedFundsEntry as RustMigratedFundsEntry, MilestonePayloadEssence as RustMilestonePayloadEssence,
        Output as RustOutput, Payload as RustPayload, ReferenceUnlock as RustReferenceUnlock,
        RegularEssence as RustRegularEssence,
        SignatureLockedDustAllowanceOutput as RustSignatureLockedDustAllowanceOutput,
        SignatureLockedSingleOutput as RustSignatureLockedSingleOutput, SignatureUnlock as RustSignatureUnlock,
        TransactionId as RustTransactionId, TransactionPayload as RustTransactionPayload,
        UnlockBlock as RustUnlockBlock, UnlockBlocks as RustUnlockBlocks, UtxoInput as RustUtxoInput,
    },
    bee_rest_api::types::{
        dtos::{
            AddressDto as RustAddressDto, Ed25519AddressDto as RustEd25519AddressDto, GossipDto as RustgossipDto,
            HeartbeatDto as RustheartbeatDto, InputDto as RustInputDto,
            LedgerInclusionStateDto as RustLedgerInclusionStateDto, MetricsDto as RustMetricsDto,
            MigratedFundsEntryDto as RustMigratedFundsEntryDto, OutputDto as RustOutputDto,
            PayloadDto as RustPayloadDto, PeerDto as RustPeerDto, ReceiptDto as RustReceiptDto,
            ReceiptPayloadDto as RustReceiptPayloadDto, RelationDto as RustRelationDto,
            SignatureLockedDustAllowanceOutputDto as RustSignatureLockedDustAllowanceOutputDto,
            SignatureLockedSingleOutputDto as RustSignatureLockedSingleOutputDto,
            TreasuryOutputDto as RustTreasuryOutputDto,
            TreasuryTransactionPayloadDto as RustTreasuryTransactionPayloadDto,
        },
        responses::{
            BalanceAddressResponse as RustBalanceAddressResponse,
            MessageMetadataResponse as RustMessageMetadataResponse, OutputResponse as RustOutputResponse,
            TreasuryResponse as RustTreasuryResponse, UtxoChangesResponse as RustUtxoChangesResponse,
        },
    },
    */