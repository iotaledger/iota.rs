import {
  NodeInfo,
  MessageMetadata,
  OutputMetadata,
  MilestoneMetadata,
  BrokerOptions,
  Address,
  AddressBalance,
  Message,
  MessageDto
} from './types'

export declare class ClientBuilder {
  node(url: string): ClientBuilder
  nodes(urls: string[]): ClientBuilder
  quorumSize(size: number): ClientBuilder
  quorumThreshold(threshold: number): ClientBuilder
  brokerOptions(options: BrokerOptions): ClientBuilder
  nodeSyncInterval(interval: number): ClientBuilder
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
