export declare interface NodeInfo {
  name: string
  version: string
  isHealthy: boolean
  networkId: string
  latestMilestoneIndex: number
  solidMilestoneIndex: number
  pruningIndex: number
  features: string[]
}

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

export declare interface MessageMetadata {
  messageId: string
  parent1MessageId: string
  parent2MessageId: string
  isSolid: boolean
  shouldPromote?: boolean
  shouldReattach?: boolean
  referencedByMilestoneIndex?: number
  ledgerInclusionState?: string
}

export declare interface OutputMetadata {
  messageId: string
  transactionId: string
  outputIndex: number
  isSpent: boolean
  address: Address
  amount: number
}

export declare interface MilestoneMetadata {
  milestoneIndex: number
  messageId: string
  timestamp: number
}

export declare interface BrokerOptions {
  automaticDisconnect: boolean
  // timeout in milliseconds
  timeout: number
}

export declare type Address = 'string'

export declare interface AddressBalance {
  address: Address
  balance: number
}

export declare class ClientBuilder {
  node(url: string): ClientBuilder
  nodes(urls: string[]): ClientBuilder
  quorumSize(size: number): ClientBuilder
  quorumThreshold(threshold: number): ClientBuilder
  brokerOptions(options: BrokerOptions): ClientBuilder
  build(): Client
}

export declare class ValueTransactionSender {
  path(bip32path: string): ValueTransactionSender
  index(index: number): ValueTransactionSender
  output(address: string, value: number): ValueTransactionSender
  submit(): Promise<string>
}

export declare class UnspentAddressGetter {
  path(bip32path: string): UnspentAddressGetter
  index(index: number): UnspentAddressGetter
  get(): Promise<[Address, number]>
}

export declare class AddressFinder {
  path(bip32path: string): AddressFinder
  range(start: number, end: number): AddressFinder
  get(): Address[]
}

export declare class BalanceGetter {
  path(bip32path: string): BalanceGetter
  index(index: number): BalanceGetter
  get(): Promise<number>
}

export declare class Client {
  subscriber(): TopicSubscriber
  send(seed: string): ValueTransactionSender
  getUnspentAddress(seed: string): UnspentAddressGetter
  findAddresses(seed: string): AddressFinder
  findMessages(indexationKeys: string[], messageIds: string[]): Promise<Message[]>
  getBalance(seed: string): BalanceGetter
  getAddressBalances(addresses: string[]): Promise<AddressBalance[]>
  retry(messageId: string): Promise<Message>

  getInfo(): Promise<NodeInfo>
  getTips(): Promise<[string, string]>
  postMessage(message: MessageDto): Promise<string>
  getMessage(): MessageFinder
  getOutput(outputId: string): Promise<OutputMetadata>
  findOutputs(outputIds: string[], addresses: string[]): Promise<OutputMetadata[]>
  getAddressOutputs(address: string): Promise<string[]>
  getAddressBalance(address: string): Promise<number>
  getMilestone(index: number): Promise<MilestoneMetadata>
  reattach(messageId: string): Promise<Message>
  promote(messageId: string): Promise<Message>
}

export declare class MessageFinder {
  index(index: string): Promise<string[]>
  data(messageId: string): Promise<Message>
  raw(messageId: string): Promise<string>
  children(messageId: string): Promise<string[]>
  metadata(messageId: string): Promise<MessageMetadata>
}

export declare type Callback = (err: any, data: any) => void

export declare class TopicSubscriber {
  topic(topic: string): TopicSubscriber
  topics(topic: string[]): TopicSubscriber
  subscribe(cb: Callback): TopicSubscriber
  unsubscribe(cb: Callback): TopicSubscriber
}
