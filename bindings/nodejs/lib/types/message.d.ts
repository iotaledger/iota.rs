export declare interface Input {
  type: 'UTXO'
  data: string
}

export declare interface Output {
  type: 'SignatureLockedSingle'
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
    public_key: number[]
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
  unlock_blocks: UnlockBlock[]
}

export declare interface IndexationPayload {
  index: string
  data: number[]
}

export declare interface MilestoneEssence {
  index: string
  timestamp: string
  parents: string[]
  merkle_proof: number[]
  next_pow_score: number
  next_pow_score_milestone_index: number
  public_keys: number[]
}

export declare interface MilestonePayload {
  essence: MilestoneEssence
  signatures: number[][]
}

export declare type Payload = { type: 'Indexation', data: IndexationPayload } |
{ type: 'Milestone', data: MilestonePayload } |
{ type: 'Transaction', data: TransactionPayload }

export declare interface Message {
  network_id: number
  parents: string[]
  payload?: Payload
  nonce: number
}

export declare interface MessageWrapper {
  messageId: string
  message: Message
}