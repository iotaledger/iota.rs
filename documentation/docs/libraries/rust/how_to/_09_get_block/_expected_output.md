```json
Block {
    protocol_version: 2,
    parents: Parents(
        BoxedSlicePrefix([
            BlockId(0x2ec007718ebd1f8dd10be36c76820321fd24fd1739a52a7176f3314f1158ea44),
            BlockId(0x536f8b2beba143278bda5074a311c827ef37c5756d80ea90372b2688b92cd5ae),
            BlockId(0x6b6bcad97ba4f8d6f75aeea1dfc945425d2d6ead793af302dd5040590f5e409d),
            BlockId(0xcff2436f2d65438c7351946fe793af3a8f059b485250d0ee2c5875d0aed71884),
        ]),
    ),
    payload: OptionalPayload(
        Some(
            Transaction(
                TransactionPayload {
                    essence: Regular(
                        RegularTransactionEssence {
                            network_id: 1856588631910923207,
                            inputs: BoxedSlicePrefix([
                                Utxo(
                                    UtxoInput(0xcb664cb88dcb4182df622fb82661e2d82e81ff95f3f04d5f3b071484a10fdc490100),
                                ),
                                Utxo(
                                    UtxoInput(0xeede6413d25f1cb1fc68187cda339ec22e66e4fb945b8397c7251d46164a0c495900),
                                ),
                                Utxo(
                                    UtxoInput(0x45a6247d6af3214828fcab1206f6b6c8e61a359fec821d48cb578b3f70a291a40000),
                                ),
                                Utxo(
                                    UtxoInput(0x71638c38cf0be640f082edf9b1a9dc1ea898932762ede49b164f0894826009703b00),
                                ),
                            ]),
                            inputs_commitment: InputsCommitment(0x10cb15a61c30e10e569f10513834d5fd0ee53049a77223ccf43a921ea22e1675),
                            outputs: BoxedSlicePrefix([
                                Alias(
                                    AliasOutput {
                                        amount: BoundedU64(
                                            53700,
                                        ),
                                        native_tokens: NativeTokens(
                                            BoxedSlicePrefix([]),
                                        ),
                                        alias_id: AliasId(0x0000000000000000000000000000000000000000000000000000000000000000),
                                        state_index: 0,
                                        state_metadata: BoxedSlicePrefix([]),
                                        foundry_counter: 0,
                                        unlock_conditions: UnlockConditions(
                                            BoxedSlicePrefix([
                                                StateControllerAddress(
                                                    StateControllerAddressUnlockCondition(
                                                        Ed25519(
                                                            Ed25519Address(0x4f56285a1876d31b62f085c1bb35a85c0edc29889074631b81172991c3e4d6ff),
                                                        ),
                                                    ),
                                                ),
                                                GovernorAddress(
                                                    GovernorAddressUnlockCondition(
                                                        Ed25519(
                                                            Ed25519Address(0x4f56285a1876d31b62f085c1bb35a85c0edc29889074631b81172991c3e4d6ff),
                                                        ),
                                                    ),
                                                ),
                                            ]),
                                        ),
                                        features: Features(
                                            BoxedSlicePrefix([]),
                                        ),
                                        immutable_features: Features(
                                            BoxedSlicePrefix([
                                                Issuer(
                                                    IssuerFeature(
                                                        Ed25519(
                                                            Ed25519Address(0x4f56285a1876d31b62f085c1bb35a85c0edc29889074631b81172991c3e4d6ff),
                                                        ),
                                                    ),
                                                ),
                                            ]),
                                        ),
                                    },
                                ),
                                Basic(
                                    BasicOutput {
                                        amount: BoundedU64(
                                            1000047500,
                                        ),
                                        native_tokens: NativeTokens(
                                            BoxedSlicePrefix([]),
                                        ),
                                        unlock_conditions: UnlockConditions(
                                            BoxedSlicePrefix([
                                                Address(
                                                    AddressUnlockCondition(
                                                        Ed25519(
                                                            Ed25519Address(0x4f56285a1876d31b62f085c1bb35a85c0edc29889074631b81172991c3e4d6ff),
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
                            ]),
                            payload: OptionalPayload(
                                Some(
                                    TaggedData(
                                        TaggedDataPayload {
                                            tag: "0x484f524e4554205370616d6d65722053656d692d4c617a79",
                                            data: "0x57652061726520616c6c206d616465206f662073746172647573742e0a436f756e743a203030353539320a54696d657374616d703a20323032322d30392d30315431343a32323a33345a0a54697073656c656374696f6e3a2031312e3832376d73",
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                    unlocks: Unlocks(
                        BoxedSlicePrefix([
                            Signature(
                                SignatureUnlock(
                                    Ed25519(
                                        Ed25519Signature {
                                            public_key: 0x229172a3883abbeb5ff7e680a6a0a0f5c31bf222d48fd1747d9d0ead24155a6b,
                                            signature: 0x354174e8f7981aa05eca6d936cd2c7bf0eb9e2954b1f550e683959eb412194f8edab0f8536c370e576eb812da434496749221c3b96b40d6d2c9de4c06d97030f,
                                        },
                                    ),
                                ),
                            ),
                            Reference(
                                ReferenceUnlock(
                                    BoundedU16(
                                        0,
                                    ),
                                ),
                            ),
                            Reference(
                                ReferenceUnlock(
                                    BoundedU16(
                                        0,
                                    ),
                                ),
                            ),
                            Reference(
                                ReferenceUnlock(
                                    BoundedU16(
                                        0,
                                    ),
                                ),
                            ),
                        ]),
                    ),
                },
            ),
        ),
    ),
    nonce: 1785168781326821022,
}
```