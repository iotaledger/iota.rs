export declare interface Input {
  type: number // 0 == 'UtxoInput', 1 == 'TreasuryInput'
  // UtxoInput
  transactionId?: string
  transactionOutputIndex?: number
  // TreasuryInput
  milestoneId?: string
}

export declare interface Output {
  type: number // 0 == 'SignatureLockedSingle', 1 == 'DustAllowance' 2 == 'TreasuryOutput'
  address: Ed25519Address
  amount: number
}

export declare interface TransactionPayloadEssence {
  type: number,
  inputs: Input[]
  outputs: Output[]
  payload?: Payload
}

export declare interface Ed25519SignatureUnlockBlock {
  type: number //0 'Ed25519'
  data: {
    publicKey: number[]
    signature: number[]
  }
}

export declare interface SignatureUnlockBlock {
  type: number //0
  data: Ed25519SignatureUnlockBlock
}

export declare interface ReferenceUnlockBlock {
  type: number //1 'Reference'
  data: number
}

export declare type UnlockBlock = SignatureUnlockBlock | ReferenceUnlockBlock

export declare interface TransactionPayload {
  type: number // 0
  essence: TransactionPayloadEssence
  unlockBlocks: UnlockBlock[]
}

export declare interface IndexationPayload {
  type: number // 2
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
  receipt?: ReceiptPayload
}

export declare interface MilestonePayload {
  type: number // 1
  essence: MilestoneEssence
  signatures: number[][]
}

export declare type Payload = TransactionPayload | MilestonePayload | IndexationPayload

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

export declare interface ReceiptPayload {
  type: number // 3
  migratedAt: number
  funds: MigratedFundsEntry[]
  transaction: TreasuryTransactionPayload
  final: boolean
}

export declare interface TreasuryTransactionPayload {
  type: number // 4
  input: Input
  output: Output
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
