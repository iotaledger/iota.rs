// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::mqtt::{Error, Topic};

#[test]
fn valid_topics() {
    assert!(Topic::try_new("milestone-info/latest").is_ok());
    assert!(Topic::try_new("milestone-info/confirmed").is_ok());
    assert!(Topic::try_new("milestones").is_ok());
    assert!(Topic::try_new("blocks").is_ok());
    assert!(Topic::try_new("blocks/transaction").is_ok());
    assert!(Topic::try_new("blocks/transaction/tagged-data").is_ok());
    assert!(Topic::try_new("blocks/transaction/tagged-data/0x0123456789abcdef").is_ok());
    assert!(Topic::try_new("blocks/tagged-data").is_ok());
    assert!(Topic::try_new("blocks/tagged-data/0x0123456789abcdef").is_ok());
    assert!(
        Topic::try_new("block-metadata/0x36845227a59864ac12d3d2389fcb4ea0bdd1a5d1d4ed464bde3154216c3246c4").is_ok()
    );
    assert!(Topic::try_new("block-metadata/referenced").is_ok());
    assert!(
        Topic::try_new(
            "transactions/0x36845227a59864ac12d3d2389fcb4ea0bdd1a5d1d4ed464bde3154216c3246c4/included-block"
        )
        .is_ok()
    );
    assert!(Topic::try_new("outputs/0x36845227a59864ac12d3d2389fcb4ea0bdd1a5d1d4ed464bde3154216c3246c40000").is_ok());
    assert!(Topic::try_new("outputs/alias/0xb21517992e96865d5fd90b403fe05fe25c6d4acfb6cdd6e7c9bbfb4266d05151").is_ok());
    assert!(Topic::try_new("outputs/nft/0x38500750eb788bfb89b4589634a82b0cee9c6a9724bafde505ffa1bb875ab0b5").is_ok());
    assert!(
        Topic::try_new(
            "outputs/foundry/0x08e10a5c7bcfdce48ff500156040f7548ca511d79a6e253a22759116c2ae8c818d0100000000"
        )
        .is_ok()
    );
    assert!(
        Topic::try_new("outputs/unlock/address/iota1qrwfnskm4f7utdrxqnkfntfqxehtpj8s0kf68zkcwm0yrhuemzjp5sjfw5v")
            .is_ok()
    );
    assert!(
        Topic::try_new("outputs/unlock/address/iota1qrwfnskm4f7utdrxqnkfntfqxehtpj8s0kf68zkcwm0yrhuemzjp5sjfw5v/spent")
            .is_ok()
    );
    assert!(Topic::try_new("receipts").is_ok());
}

#[test]
fn invalid_tags() {
    // Empty.
    assert!(matches!(
        Topic::try_new("blocks/transaction/tagged-data/0x"),
        Err(Error::InvalidTopic(_))
    ));
    assert!(matches!(
        Topic::try_new("blocks/tagged-data/0x"),
        Err(Error::InvalidTopic(_))
    ));
    // Uneven.
    assert!(matches!(
        Topic::try_new("blocks/transaction/tagged-data/0x0123456789abcde"),
        Err(Error::InvalidTopic(_))
    ));
    assert!(matches!(
        Topic::try_new("blocks/tagged-data/0x0123456789abcde"),
        Err(Error::InvalidTopic(_))
    ));
    // Too large.
    assert!(matches!(
        Topic::try_new(
            "blocks/transaction/tagged-data/0xb21517992e96865d5fd90b403fe05fe25c6d4acfb6cdd6e7c9bbfb4266d05151b21517992e96865d5fd90b403fe05fe25c6d4acfb6cdd6e7c9bbfb4266d05151ff"
        ),
        Err(Error::InvalidTopic(_))
    ));
    assert!(matches!(
        Topic::try_new(
            "blocks/tagged-data/0xb21517992e96865d5fd90b403fe05fe25c6d4acfb6cdd6e7c9bbfb4266d05151b21517992e96865d5fd90b403fe05fe25c6d4acfb6cdd6e7c9bbfb4266d05151ff"
        ),
        Err(Error::InvalidTopic(_))
    ));
    // Invalid chars.
    assert!(matches!(
        Topic::try_new("blocks/transaction/tagged-data/0x012345@789abcde"),
        Err(Error::InvalidTopic(_))
    ));
    assert!(matches!(
        Topic::try_new("blocks/tagged-data/0x012345@789abcde"),
        Err(Error::InvalidTopic(_))
    ));
}
