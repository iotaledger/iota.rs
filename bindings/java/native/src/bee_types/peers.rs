// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use getset::{CopyGetters, Getters};
use iota_client::bee_rest_api::types::dtos::{PeerDto as RustPeerDto, RelationDto as RustRelationDto};

use crate::bee_types::GossipDto;

#[derive(Eq, PartialEq, Getters, CopyGetters, Debug)]
pub struct PeerDto {
    #[getset(get = "pub")]
    pub id: String,
    pub multi_addresses: Vec<String>,
    pub alias: Option<String>,
    #[getset(get_copy = "pub")]
    pub relation: Relation,
    #[getset(get_copy = "pub")]
    pub connected: bool,
    #[getset(get_copy = "pub")]
    pub gossip: Option<GossipDto>,
}

impl PeerDto {
    pub fn multi_addresses(&self) -> Vec<String> {
        self.multi_addresses.to_vec()
    }
    pub fn alias(&self) -> Option<String> {
        self.alias.clone()
    }
}

impl core::fmt::Display for PeerDto {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "id={}, multi_addresses=({:?}), alias={:?}, relation={:?}, connected={}, gossip={:?}",
            self.id, self.multi_addresses, self.alias, self.relation, self.connected, self.gossip
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Relation {
    KNOWN,
    UNKNOWN,
    AUTOPEERED,
}

impl From<RustPeerDto> for PeerDto {
    fn from(peer: RustPeerDto) -> Self {
        let gossip = peer.gossip.map(GossipDto::from);
        Self {
            id: peer.id,
            multi_addresses: peer.multi_addresses,
            alias: peer.alias,
            relation: Relation::from(peer.relation),
            connected: peer.connected,
            gossip,
        }
    }
}

impl From<RustRelationDto> for Relation {
    fn from(relation: RustRelationDto) -> Relation {
        match relation {
            RustRelationDto::Known => Relation::KNOWN,
            RustRelationDto::Unknown => Relation::UNKNOWN,
            RustRelationDto::Autopeered => Relation::AUTOPEERED,
        }
    }
}
