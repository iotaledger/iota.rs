```plaintext
Alias(
    AliasOutput {
        amount: 59200,
        native_tokens: NativeTokens(
            BoxedSlicePrefix([]),
        ),
        alias_id: AliasId(0x0000000000000000000000000000000000000000000000000000000000000000),
        state_index: 0,
        state_metadata: BoxedSlicePrefix([
            104,
            101,
            108,
            108,
            111,
        ]),
        foundry_counter: 0,
        unlock_conditions: UnlockConditions(
            BoxedSlicePrefix([
                StateControllerAddress(
                    StateControllerAddressUnlockCondition(
                        Ed25519(
                            Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                        ),
                    ),
                ),
                GovernorAddress(
                    GovernorAddressUnlockCondition(
                        Ed25519(
                            Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                        ),
                    ),
                ),
            ]),
        ),
        features: Features(
            BoxedSlicePrefix([
                Sender(
                    SenderFeature(
                        Ed25519(
                            Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                        ),
                    ),
                ),
                Metadata(
                    MetadataFeature(0x68656c6c6f),
                ),
            ]),
        ),
        immutable_features: Features(
            BoxedSlicePrefix([
                Issuer(
                    IssuerFeature(
                        Ed25519(
                            Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                        ),
                    ),
                ),
                Metadata(
                    MetadataFeature(0x68656c6c6f),
                ),
            ]),
        ),
    },
)
```