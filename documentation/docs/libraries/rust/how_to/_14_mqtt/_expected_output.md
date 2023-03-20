```json
Topic: blocksBlock{
  protocol_version: 2,
  parents: Parents(BoxedSlicePrefix([
    BlockId(0x2c6217376977980929b5bd9f4fd33ee06a13dc5ed41a53ffa0d20eae77a0d3d9),
    BlockId(0x3971dc26622ffef0d1cf5b6d2cbfa0732014dd3271d675158bf854100087475c),
    BlockId(0x5bb64f36721d02b5ef314fd0a012ef1adf779592474dac88dd24d82550bf5242),
    BlockId(0xc95cfe61e184ef1885a29f7832019840d999fa4baf452f6dbb127e2dc4c1d20f)
  ])),
  payload: OptionalPayload(Some(Transaction(TransactionPayload{
    essence: Regular(RegularTransactionEssence{
      network_id: 1856588631910923207,
      inputs: BoxedSlicePrefix([
        Utxo(UtxoInput(0x8e7b59a567f0f439ef7d3b01de094ce2f81aa8e68e09f26705dfd1d39ddf471b0000)),
        Utxo(UtxoInput(0x60e8afa096f3705b6f1a694f5496d51721ea4b4e9876b219dfecdf99586949fd0100)),
        Utxo(UtxoInput(0xf12c189790488ec61ce8c4d9d28ebbb7e82d322b25ec293e40fd4ef2dfe15e0d0000)),
        Utxo(UtxoInput(0xd14bd0777e8ea7266a2af955126dde56ed692d9ad6b19508cfd614d4b3612a950000)),
        Utxo(UtxoInput(0xf31d1bf2c93ff0578d72951471c0fda2be4efd4cfbb7a2bd03e04d64362ef6401b00)),
        Utxo(UtxoInput(0xf34edd2df152ef473679ad468d6f942fadeca7004499576bba9a1a17927da3af7700))
      ]),
      inputs_commitment: InputsCommitment(0x43448b345d2c0bc38ea6e4abd79726852a2880528a753a986deb13bf3f93af5b),
      outputs: BoxedSlicePrefix([
        Nft(NftOutput{
          amount: BoundedU64(51000),
          native_tokens: NativeTokens(BoxedSlicePrefix([
            
          ])),
          nft_id: NftId(0x0000000000000000000000000000000000000000000000000000000000000000),
          unlock_conditions: UnlockConditions(BoxedSlicePrefix([
            Address(AddressUnlockCondition(Ed25519(Ed25519Address(0x56b1638c1bb2566e2150d4667f78a59ee0d9b9a6c1e6af4499c3d55c8fd77e01))))
          ])),
          features: Features(BoxedSlicePrefix([
            
          ])),
          immutable_features: Features(BoxedSlicePrefix([
            Issuer(IssuerFeature(Ed25519(Ed25519Address(0x56b1638c1bb2566e2150d4667f78a59ee0d9b9a6c1e6af4499c3d55c8fd77e01)))),
            Metadata(MetadataFeature(0x736f6d652d697066732d6c696e6b))
          ]))
        }),
        Basic(BasicOutput{
          amount: BoundedU64(1009003400),
          native_tokens: NativeTokens(BoxedSlicePrefix([
            
          ])),
          unlock_conditions: UnlockConditions(BoxedSlicePrefix([
            Address(AddressUnlockCondition(Ed25519(Ed25519Address(0x56b1638c1bb2566e2150d4667f78a59ee0d9b9a6c1e6af4499c3d55c8fd77e01))))
          ])),
          features: Features(BoxedSlicePrefix([
            
          ]))
        })
      ]),
      payload: OptionalPayload(Some(TaggedData(TaggedDataPayload{
        tag: "0x484f524e4554205370616d6d65722053656d692d4c617a79",
        data: "0x57652061726520616c6c206d616465206f662073746172647573742e0a436f756e743a203032353636380a54696d657374616d703a20323032322d30392d30325431343a30353a33355a0a54697073656c656374696f6e3a20313836c2b573"
      })))
    }),
    unlocks: Unlocks(BoxedSlicePrefix([
      Signature(SignatureUnlock(Ed25519(Ed25519Signature{
        public_key: 0x1eea893bc9fc06787b7e5d273cdb02e02f9a5cce9eb4e1dc513eec14e8d95047,
        signature: 0xbaab2fd45c038a0661f896184aecd249c8169b4b08dda8965d7d99e3778828cbe69c95a6893115d83705887d20f85eef4d3f9d2d39e89d009649da66fdfffa0a
      }))),
      Reference(ReferenceUnlock(BoundedU16(0))),
      Reference(ReferenceUnlock(BoundedU16(0))),
      Reference(ReferenceUnlock(BoundedU16(0))),
      Reference(ReferenceUnlock(BoundedU16(0))),
      Reference(ReferenceUnlock(BoundedU16(0)))
    ]))
  }))),
  nonce: 1785168781326745297
}
Topic: milestone-info/latest{
  "index": 798690,
  "milestoneId": "0xc405fa4839b32d78b1a7f27e297d07842aa83a8ced1e5b556e446317c7348be6",
  "timestamp": 1662127539
}
[...]
```