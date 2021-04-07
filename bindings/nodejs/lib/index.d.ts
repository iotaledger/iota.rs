import {
  NodeInfo,
  MessageMetadata,
  OutputMetadata,
  MilestoneMetadata,
  BrokerOptions,
  Address,
  AddressBalance,
  MessageDto,
  MessageWrapper
} from './types'

export declare type Api = 'GetHealth' | 'GetInfo' | 'GetTips' | 'PostMessage' | 'PostMessageWithRemotePoW' | 'GetOutput' | 'GetMilestone'

export declare class ClientBuilder {
  node(url: string): ClientBuilder
  nodeAuth(url: string, name: string, password: string): ClientBuilder
  nodes(urls: string[]): ClientBuilder
  nodePoolUrls(urls: string[]): ClientBuilder
  network(network_name: string): ClientBuilder
  quorumSize(size: number): ClientBuilder
  quorumThreshold(threshold: number): ClientBuilder
  brokerOptions(options: BrokerOptions): ClientBuilder
  nodeSyncInterval(interval: number): ClientBuilder
  disableNodeSync(): ClientBuilder
  requestTimeout(timeoutMs: number): ClientBuilder
  apiTimeout(api: Api, timeoutMs: number): ClientBuilder
  localPow(local: boolean): ClientBuilder
  build(): Client
}

export declare class MessageSender {
  seed(seed: string): MessageSender
  index(index: string | number[] | Uint8Array): MessageSender
  data(data: Uint8Array): MessageSender
  parents(messageIds: string[]): MessageSender
  accountIndex(index: number): MessageSender
  initialAddressIndex(index: number): MessageSender
  input(transactionId: string, index: number): MessageSender
  inputRange(start: number, end: number): MessageSender
  output(address: string, value: number): MessageSender
  dustAllowanceOutput(address: string, value: number): MessageSender
  submit(): Promise<string>
}

export declare class UnspentAddressGetter {
  accountIndex(index: number): UnspentAddressGetter
  initialAddressIndex(index: number): UnspentAddressGetter
  get(): Promise<[Address, number]>
}

export declare class AddressGetter {
  accountIndex(index: number): AddressGetter
  range(start: number, end: number): AddressGetter
  bech32_hrp(bech32_hrp: string): AddressGetter
  get(): [Address, boolean][]
}

export declare class BalanceGetter {
  accountIndex(index: number): BalanceGetter
  initialAddressIndex(index: number): BalanceGetter
  gapLimit(gapLimit: number): BalanceGetter
  get(): Promise<number>
}

export declare interface NetworkInfo {
  network: { type: 'Mainnet' | 'Testnet' }
  networkId: string
  bech32HRP: string
  minPowScore: number
  localPow: boolean
}

export declare interface AddressOutputsOptions {
  includeSpent?: boolean
  outputType?: { type: 'SignatureLockedSingle' | 'SignatureLockedDustAllowance' }
}

export declare class Client {
  networkInfo(): Promise<NetworkInfo>
  subscriber(): TopicSubscriber
  message(): MessageSender
  getUnspentAddress(seed: string): UnspentAddressGetter
  getAddresses(seed: string): Promise<AddressGetter>
  findMessages(indexationKeys: string[], messageIds: string[]): Promise<MessageWrapper[]>
  getBalance(seed: string): BalanceGetter
  getAddressBalances(addresses: string[]): Promise<AddressBalance[]>
  retry(messageId: string): Promise<MessageWrapper>
  retryUntilIncluded(messageId: string, interval?: number, maxAttempts?: number): Promise<MessageWrapper[]>

  getInfo(): Promise<NodeInfo>
  getTips(): Promise<string[]>
  postMessage(message: MessageDto): Promise<string>
  postMessageWithRemotePow(message: MessageDto): Promise<string>
  getMessage(): MessageFinder
  getOutput(outputId: string): Promise<OutputMetadata>
  findOutputs(outputIds: string[], addresses: string[]): Promise<OutputMetadata[]>
  getAddressOutputs(address: string, options?: AddressOutputsOptions): Promise<string[]>
  getAddressBalance(address: string): Promise<AddressBalance>
  bech32ToHex(address: string): string
  hexToBech32(address: string, bech32_hrp?: string): Promise<string>
  isAddressValid(address: string): boolean
  getMilestone(index: number): Promise<MilestoneMetadata>
  getMilestoneUtxoChanges(index: number): Promise<MilestoneUTXOChanges>
  getReceipts(): Promise<Receipts[]>
  getReceiptsMigratedAt(index: number): Promise<Receipts[]>
  getTreasury(): Promise<Treasury>
  getIncludedMessage(): Promise<MessageWrapper>
  reattach(messageId: string): Promise<MessageWrapper>
  promote(messageId: string): Promise<MessageWrapper>
}

export declare class MessageFinder {
  index(index: string | number[] | Uint8Array): Promise<string[]>
  data(messageId: string): Promise<MessageWrapper>
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
