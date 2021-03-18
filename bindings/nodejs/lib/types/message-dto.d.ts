export declare interface InputDto {
  type: 'UTXO'
  data: string
}

export declare interface OutputDto {
  address: string
  amount: number
}

export declare interface TransactionPayloadEssenceDto {
  inputs: InputDto[]
  outputs: OutputDto[]
  payload?: IndexationPayloadDto
}

export declare interface Ed25519SignatureUnlockBlockDto {
  publicKey: number[]
  signature: number[]
}

export declare interface SignatureUnlockBlockDto {
  data: Ed25519SignatureUnlockBlockDto
}

export declare type ReferenceUnlockBlockDto = number

export declare type UnlockBlockDto = SignatureUnlockBlockDto | ReferenceUnlockBlockDto

export declare interface TransactionPayloadDto {
  essence: TransactionPayloadEssenceDto
  unlockBlocks: UnlockBlockDto[]
}

export declare interface IndexationPayloadDto {
  index: string | Uint8Array | number[]
  data?: Uint8Array | number[]
}

export declare type PayloadDto = TransactionPayloadDto | IndexationPayloadDto

export declare interface MessageDto {
  parents?: string[]
  payload?: PayloadDto
}
