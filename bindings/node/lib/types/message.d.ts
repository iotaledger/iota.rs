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

export declare interface TransactionEssence {
  inputs: Input[]
  outputs: Output[]
  payload?: Payload
}

export declare interface WotsSignatureUnlockBlock {
  type: 'Wots'
  data: number[]
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
  data: WotsSignatureUnlockBlock | Ed25519SignatureUnlockBlock
}

export declare interface ReferenceUnlockBlock {
  type: 'Reference'
  data: number
}

export declare type UnlockBlock = SignatureUnlockBlock | ReferenceUnlockBlock

export declare interface TransactionPayload {
  essence: TransactionEssence
  unlock_blocks: UnlockBlock[]
}

export declare interface IndexationPayload {
  index: string
  data: number[]
}

export declare interface MilestoneEssence {
  index: string
  timestamp: string
  parent1: string
  parent2: string
  merkle_proof: number[]
  public_keys: number[]
}

export declare interface MilestonePayload {
  essence: MilestoneEssence
  signatures: number[][]
}

export declare type Payload = { Indexation: IndexationPayload } |
{ Milestone: MilestonePayload } |
{ Transaction: TransactionPayload }

export declare interface Message {
  network_id: number
  parent1: string
  parent2: string
  payload?: Payload
  nonce: number
}