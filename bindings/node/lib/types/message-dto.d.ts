export declare interface InputDto {
  type: 'UTXO'
  data: string
}

export declare interface OutputDto {
  address: string
  amount: number
}

export declare interface TransactionEssenceDto {
  inputs: InputDto[]
  outputs: OutputDto[]
  payload?: IndexationPayload
}

export declare type WotsSignatureUnlockBlockDto = number[]

export declare interface Ed25519SignatureUnlockBlockDto {
  publicKey: number[]
  signature: number[]
}

export declare interface SignatureUnlockBlockDto {
  data: WotsSignatureUnlockBlockDto | Ed25519SignatureUnlockBlockDto
}

export declare type ReferenceUnlockBlockDto = number

export declare type UnlockBlockDto = SignatureUnlockBlockDto | ReferenceUnlockBlockDto

export declare interface TransactionPayloadDto {
  essence: TransactionEssenceDto
  unlockBlocks: UnlockBlockDto[]
}

export declare interface IndexationPayloadDto {
  index: string
  data: string
}

export declare type PayloadDto = TransactionPayloadDto | IndexationPayloadDto

export declare interface MessageDto {
  parent1: string
  parent2: string
  payload?: Payload
  nonce?: number
}
