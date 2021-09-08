import {
  TransactionPayloadEssence,
  Input,
  Output
} from './message'

export declare interface NodeInfoWrapper {
  url: string
  nodeinfo: NodeInfo
}
export declare interface NodeInfo {
  name: string
  version: string
  isHealthy: boolean
  networkId: string
  bech32HRP: string
  minPoWScore: number
  messagesPerSecond: number
  referencedMessagesPerSecond: number
  referencedRate: number
  latestMilestoneTimestamp: number
  latestMilestoneIndex: number
  confirmedMilestoneIndex: number
  pruningIndex: number
  features: string[]
}

export declare interface MessageMetadata {
  messageId: string
  parents: string[]
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
  index: number
  messageId: string
  timestamp: number
}

export declare interface MilestoneUTXOChanges {
  index: number
  createdOutputs: string[]
  consumedOutputs: string[]
}

export declare interface BrokerOptions {
  automaticDisconnect?: boolean
  // timeout in seconds
  timeout?: number
  useWs?: boolean
  port?: number
  maxReconnectionAttempts?: number
}

export declare type Address = string

export declare interface AddressBalance {
  address: Address
  balance: number
  dustAllowed: boolean
}

export declare interface PreparedTransactionData {
  essence: TransactionPayloadEssence
  address_index_recorders: AddressIndexRecorder[]
}
export declare interface AddressIndexRecorder {
  account_index: number,
  input: Input,
  output: OutputResponse,
  address_index: number,
  chain: Segment[],
  internal: boolean,
  bech32_address: string,
}
export declare interface OutputResponse {
  messageId: string,
  transactionId: string,
  outputIndex: number,
  isSpent: boolean,
  output: Output,
  ledgerIndex: number,
}

export declare interface Segment {
  hardened: boolean,
  bs: number[],
}
