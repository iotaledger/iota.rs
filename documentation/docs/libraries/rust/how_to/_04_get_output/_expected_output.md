```json
OutputResponse {
    metadata: OutputMetadataResponse {
        block_id: "0xa9f11aba1e9965173dc21a47ec4fbfe5b953a6e60277857a3f7a5c1499e7c454",
        transaction_id: "0x1e857d380f813d8035e487b6dfd2ff4740b6775273ba1b576f01381ba2a1a44c",
        output_index: 0,
        is_spent: true,
        milestone_index_spent: Some(
            249147,
        ),
        milestone_timestamp_spent: Some(
            1659379781,
        ),
        transaction_id_spent: Some(
            "0xa927ea21d3c4a49d5c73169c80302ddb223894c182a4b7cfef0af89568262749",
        ),
        milestone_index_booked: 249145,
        milestone_timestamp_booked: 1659379771,
        ledger_index: 918846,
    },
    output: Alias(
        AliasOutputDto {
            kind: 4,
            amount: "2000000",
            native_tokens: [],
            alias_id: AliasIdDto(
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            ),
            state_index: 0,
            state_metadata: "",
            foundry_counter: 0,
            unlock_conditions: [
                StateControllerAddress(
                    StateControllerAddressUnlockConditionDto {
                        kind: 4,
                        address: Ed25519(
                            Ed25519AddressDto {
                                kind: 0,
                                pub_key_hash: "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3",
                            },
                        ),
                    },
                ),
                GovernorAddress(
                    GovernorAddressUnlockConditionDto {
                        kind: 5,
                        address: Ed25519(
                            Ed25519AddressDto {
                                kind: 0,
                                pub_key_hash: "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3",
                            },
                        ),
                    },
                ),
            ],
            features: [
                Sender(
                    SenderFeatureDto {
                        kind: 0,
                        address: Ed25519(
                            Ed25519AddressDto {
                                kind: 0,
                                pub_key_hash: "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3",
                            },
                        ),
                    },
                ),
                Metadata(
                    MetadataFeatureDto {
                        kind: 2,
                        data: "0x010203",
                    },
                ),
            ],
            immutable_features: [
                Issuer(
                    IssuerFeatureDto {
                        kind: 1,
                        address: Ed25519(
                            Ed25519AddressDto {
                                kind: 0,
                                pub_key_hash: "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3",
                            },
                        ),
                    },
                ),
            ],
        },
    ),
}

```