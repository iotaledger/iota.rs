export declare interface Input {
  type: 'UTXO'
  data: string
}

export declare interface Output {
  type: 'SignatureLockedSingle' | 'DustAllowance'
  data: {
    address: string
    amount: number
  }
}

export declare interface TransactionPayloadEssence {
  inputs: Input[]
  outputs: Output[]
  payload?: Payload
}

export declare interface Ed25519SignatureUnlockBlock {
  type: 'Ed25519'
  data: {
    publicKey: number[]
    signature: number[]
  }
}

export declare interface SignatureUnlockBlock {
  type: 'Signature'
  data: Ed25519SignatureUnlockBlock
}

export declare interface ReferenceUnlockBlock {
  type: 'Reference'
  data: number
}

export declare type UnlockBlock = SignatureUnlockBlock | ReferenceUnlockBlock

export declare interface TransactionPayload {
  essence: TransactionPayloadEssence
  unlockBlocks: UnlockBlock[]
}

export declare interface IndexationPayload {
  index: Uint8Array
  data: number[]
}

export declare interface MilestoneEssence {
  index: string
  timestamp: string
  parents: string[]
  inclusionMerkleProof: number[]
  nextPoWScore: number
  nextPoWScoreMilestoneIndex: number
  publicKeys: number[]
}

export declare interface MilestonePayload {
  essence: MilestoneEssence
  signatures: number[][]
}

export declare type Payload = { type: 'Indexation', data: IndexationPayload } |
{ type: 'Milestone', data: MilestonePayload } |
{ type: 'Transaction', data: TransactionPayload }

export declare interface Message {
  networkId: number
  parents: string[]
  payload?: Payload
  nonce: number
}

export declare interface MessageWrapper {
  messageId: string
  message: Message
}

export declare interface Receipts {
  type: number
  migratedAt: number
  funds: MigratedFundsEntry[]
  transaction: TreasuryTransactionPayload
  final: boolean
}

export declare interface TreasuryTransactionPayload {
  type: number
  input: Input
  output: Output
}

export declare interface Input {
  kind: number
  transactionId: string
  transactionOutputIndex: number
}

export declare interface Output {
  kind: number
  address: string
  amount: number
}

export declare interface MigratedFundsEntry {
  tailTransactionHash: string
  address: Ed25519Address
  deposit: number
}

export declare interface Ed25519Address {
  type: number
  address: string
}

export declare interface Treasury {
  type: number
  amount: number
}
