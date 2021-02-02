export declare interface NodeInfo {
  name: string
  version: string
  isHealthy: boolean
  networkId: string
  bech32HRP: string
  minPoWScore: number
  latestMilestoneIndex: number
  solidMilestoneIndex: number
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
  type: number
  index: number
  messageId: string
  timestamp: number
  parentMessageIds: string[]
  inclusionMerkleProof: string
  publicKeys: string[]
  signature: string[]
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
