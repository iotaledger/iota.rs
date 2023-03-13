```plaintext
[
    Basic(
        BasicOutput {
            amount: 1000000,
            native_tokens: NativeTokens(
                BoxedSlicePrefix([]),
            ),
            unlock_conditions: UnlockConditions(
                BoxedSlicePrefix([
                    Address(
                        AddressUnlockCondition(
                            Ed25519(
                                Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                            ),
                        ),
                    ),
                ]),
            ),
            features: Features(
                BoxedSlicePrefix([]),
            ),
        },
    ),
    Basic(
        BasicOutput {
            amount: 1000000,
            native_tokens: NativeTokens(
                BoxedSlicePrefix([]),
            ),
            unlock_conditions: UnlockConditions(
                BoxedSlicePrefix([
                    Address(
                        AddressUnlockCondition(
                            Ed25519(
                                Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                            ),
                        ),
                    ),
                ]),
            ),
            features: Features(
                BoxedSlicePrefix([
                    Metadata(
                        MetadataFeature(0x48656c6c6f2c20576f726c6421),
                    ),
                ]),
            ),
        },
    ),
    Basic(
        BasicOutput {
            amount: 1000000,
            native_tokens: NativeTokens(
                BoxedSlicePrefix([]),
            ),
            unlock_conditions: UnlockConditions(
                BoxedSlicePrefix([
                    Address(
                        AddressUnlockCondition(
                            Ed25519(
                                Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                            ),
                        ),
                    ),
                    StorageDepositReturn(
                        StorageDepositReturnUnlockCondition {
                            return_address: Ed25519(
                                Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                            ),
                            amount: 1000000,
                        },
                    ),
                ]),
            ),
            features: Features(
                BoxedSlicePrefix([]),
            ),
        },
    ),
    Basic(
        BasicOutput {
            amount: 1000000,
            native_tokens: NativeTokens(
                BoxedSlicePrefix([]),
            ),
            unlock_conditions: UnlockConditions(
                BoxedSlicePrefix([
                    Address(
                        AddressUnlockCondition(
                            Ed25519(
                                Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                            ),
                        ),
                    ),
                    Expiration(
                        ExpirationUnlockCondition {
                            return_address: Ed25519(
                                Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                            ),
                            timestamp: 1,
                        },
                    ),
                ]),
            ),
            features: Features(
                BoxedSlicePrefix([]),
            ),
        },
    ),
    Basic(
        BasicOutput {
            amount: 1000000,
            native_tokens: NativeTokens(
                BoxedSlicePrefix([]),
            ),
            unlock_conditions: UnlockConditions(
                BoxedSlicePrefix([
                    Address(
                        AddressUnlockCondition(
                            Ed25519(
                                Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                            ),
                        ),
                    ),
                    Timelock(
                        TimelockUnlockCondition(
                            1,
                        ),
                    ),
                ]),
            ),
            features: Features(
                BoxedSlicePrefix([]),
            ),
        },
    ),
]
```