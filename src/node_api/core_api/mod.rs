// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node code API

// todo

// https://github.com/gohornet/hornet/blob/stardust-utxo/plugins/restapi/v2/plugin.go

// // RouteInfo is the route for getting the node info.
// // GET returns the node info.
// RouteInfo = "/info"

// // RouteTips is the route for getting tips.
// // GET returns the tips.
// RouteTips = "/tips"

// // RouteMessageData is the route for getting message data by its messageID.
// // GET returns message data (json).
// RouteMessageData = "/messages/:" + restapipkg.ParameterMessageID

// // RouteMessageMetadata is the route for getting message metadata by its messageID.
// // GET returns message metadata (including info about "promotion/reattachment needed").
// RouteMessageMetadata = "/messages/:" + restapipkg.ParameterMessageID + "/metadata"

// // RouteMessageBytes is the route for getting message raw data by it's messageID.
// // GET returns raw message data (bytes).
// RouteMessageBytes = "/messages/:" + restapipkg.ParameterMessageID + "/raw"

// // RouteMessageChildren is the route for getting message IDs of the children of a message, identified by its
// messageID. // GET returns the message IDs of all children.
// RouteMessageChildren = "/messages/:" + restapipkg.ParameterMessageID + "/children"

// // RouteMessages is the route for getting message IDs or creating new messages.
// // POST creates a single new message and returns the new message ID.
// RouteMessages = "/messages"

// // RouteTransactionsIncludedMessage is the route for getting the message that was included in the ledger for a given
// transaction ID. // GET returns message data (json).
// RouteTransactionsIncludedMessage = "/transactions/:" + restapipkg.ParameterTransactionID + "/included-message"

// // RouteMilestone is the route for getting a milestone by it's milestoneIndex.
// // GET returns the milestone.
// RouteMilestone = "/milestones/:" + restapipkg.ParameterMilestoneIndex

// // RouteMilestoneUTXOChanges is the route for getting all UTXO changes of a milestone by its milestoneIndex.
// // GET returns the output IDs of all UTXO changes.
// RouteMilestoneUTXOChanges = "/milestones/:" + restapipkg.ParameterMilestoneIndex + "/utxo-changes"

// // RouteOutput is the route for getting outputs by their outputID (transactionHash + outputIndex).
// // GET returns the output.
// RouteOutput = "/outputs/:" + restapipkg.ParameterOutputID

// // RouteTreasury is the route for getting the current treasury output.
// RouteTreasury = "/treasury"

// // RouteReceipts is the route for getting all stored receipts.
// RouteReceipts = "/receipts"

// // RouteReceiptsMigratedAtIndex is the route for getting all receipts for a given migrated at index.
// RouteReceiptsMigratedAtIndex = "/receipts/:" + restapipkg.ParameterMilestoneIndex

// // RoutePeer is the route for getting peers by their peerID.
// // GET returns the peer
// // DELETE deletes the peer.
// RoutePeer = "/peers/:" + restapipkg.ParameterPeerID

// // RoutePeers is the route for getting all peers of the node.
// // GET returns a list of all peers.
// // POST adds a new peer.
// RoutePeers = "/peers"

// // RouteControlDatabasePrune is the control route to manually prune the database.
// // POST prunes the database.
// RouteControlDatabasePrune = "/control/database/prune"

// // RouteControlSnapshotsCreate is the control route to manually create a snapshot files.
// // POST creates a snapshot (full, delta or both).
// RouteControlSnapshotsCreate = "/control/snapshots/create"
