// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { MessageHandler } from './MessageHandler';
import type {
    IClientOptions,
    IGenerateAddressesOptions,
    IGenerateBlockOptions,
    QueryParameter,
    IPreparedTransactionData,
    BlockId,
    INetworkInfo,
    SecretManager,
    INode,
    IAuth,
    IRange,
    INodeInfo,
} from '../types';
import type {
    IUTXOInput,
    AddressTypes,
    IOutputResponse,
    IBlock,
    IBlockMetadata,
    PayloadTypes,
    IPeer,
    IMilestonePayload,
    IMilestoneUtxoChangesResponse,
    IReceiptsResponse,
    ITreasury,
} from '@iota/types';
import type { INodeInfoWrapper } from '../types/nodeInfo';

export class Client {
    private messageHandler: MessageHandler;

    constructor(options: IClientOptions) {
        this.messageHandler = new MessageHandler(options);
    }

    /**
     * Returns the node information together with the url of the used node
     */
    async getInfo(): Promise<INodeInfoWrapper> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Gets the network related information such as network_id and min_pow_score
     */
    async getNetworkInfo(): Promise<INetworkInfo> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetNetworkInfo',
        });

        return JSON.parse(response).payload;
    }

    /** Get output IDs based on query parameters */
    async outputIds(queryParameters: QueryParameter[]): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'OutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Get output from a known outputID */
    async getOutput(outputId: string): Promise<IOutputResponse> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetOutput',
            data: {
                outputId,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Fetch OutputResponse from provided OutputIds (requests are sent in parallel) */
    async getOutputs(outputIds: string[]): Promise<IOutputResponse[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetOutputs',
            data: {
                outputIds,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Generates a new mnemonic.
     */
    async generateMnemonic(): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateMnemonic',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns a hex encoded seed for a mnemonic.
     */
    async mnemonicToHexSeed(mnemonic: string): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'MnemonicToHexSeed',
            data: {
                mnemonic,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Generate addresses */
    async generateAddresses(
        secretManager: SecretManager,
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateAddresses',
            data: {
                secretManager,
                options: generateAddressesOptions,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Generate client block */
    async generateBlock(
        secretManager?: SecretManager,
        options?: IGenerateBlockOptions,
    ): Promise<IBlock> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateBlock',
            data: {
                secretManager,
                options,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns tips that are ideal for attaching a block.
     * The tips can be considered as non-lazy and are therefore ideal for attaching a block.
     */
    async getTips(): Promise<BlockId[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetTips',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Send block, returns the block ID.
     */
    async postBlock(block: IBlock): Promise<BlockId> {
        const response = await this.messageHandler.callClientMethod({
            name: 'PostBlock',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block data with block ID
     */
    async getBlock(blockId: BlockId): Promise<IBlock> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetBlock',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block metadata with block ID
     */
    async getBlockMetadata(blockId: BlockId): Promise<IBlockMetadata> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetBlockMetadata',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Find inputs from addresses for a provided amount (useful for offline signing)
     */
    async findInputs(
        addresses: string[],
        amount: number,
    ): Promise<IUTXOInput[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'FindInputs',
            data: {
                addresses,
                amount,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Find all outputs based on the requests criteria. This method will try to query multiple nodes if
     * the request amount exceeds individual node limit.
     */
    async findOutputs(
        outputIds: string[],
        addresses: string[],
    ): Promise<IOutputResponse[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'FindOutputs',
            data: {
                outputIds,
                addresses,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Prepare a transaction for signing
     */
    async prepareTransaction(
        secretManager?: SecretManager,
        options?: IGenerateBlockOptions,
    ): Promise<IPreparedTransactionData> {
        const response = await this.messageHandler.callClientMethod({
            name: 'PrepareTransaction',
            data: {
                secretManager,
                options,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Store a mnemonic in the Stronghold vault
     */
    async storeMnemonic(
        secretManager: SecretManager,
        mnemonic: string,
    ): Promise<void> {
        const response = await this.messageHandler.callClientMethod({
            name: 'StoreMnemonic',
            data: {
                secretManager,
                mnemonic,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Sign a transaction
     */
    async signTransaction(
        secretManager: SecretManager,
        preparedTransactionData: IPreparedTransactionData,
    ): Promise<PayloadTypes> {
        const response = await this.messageHandler.callClientMethod({
            name: 'SignTransaction',
            data: {
                secretManager,
                preparedTransactionData,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Submit a payload in a block
     */
    async submitPayload(payload: PayloadTypes): Promise<IBlock> {
        const response = await this.messageHandler.callClientMethod({
            name: 'SubmitPayload',
            data: {
                payload,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns a valid Address parsed from a String.
     */
    async parseBech32Address(address: string): Promise<AddressTypes> {
        const response = await this.messageHandler.callClientMethod({
            name: 'ParseBech32Address',
            data: {
                address,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns a block ID (Blake2b256 hash of the block bytes)
     */
    async blockId(block: IBlock): Promise<BlockId> {
        const response = await this.messageHandler.callClientMethod({
            name: 'BlockId',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get a node candidate from the synced node pool.
     */
    async getNode(): Promise<INode> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetNode',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the network id of the node we're connecting to.
     */
    async getNetworkId(): Promise<number> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetNetworkId',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the bech32_hrp.
     */
    async getBech32Hrp(): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetBech32Hrp',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the min PoW score.
     */
    async getMinPowScore(): Promise<number> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMinPoWScore',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the tips interval.
     */
    async getTipsInterval(): Promise<number> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetTipsInterval',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns if local pow should be used or not.
     */
    async getLocalPow(): Promise<boolean> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetLocalPoW',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get fallback to local proof of work timeout.
     */
    async getFallbackToLocalPow(): Promise<boolean> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetFallbackToLocalPoW',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get health of node with input url.
     */
    async getNodeHealth(url: string): Promise<boolean> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetNodeHealth',
            data: {
                url,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get current node health.
     */
    async getHealth(): Promise<boolean> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetHealth',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get info of node with input url.
     */
    async getNodeInfo(url: string, auth?: IAuth): Promise<INodeInfo> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetNodeInfo',
            data: {
                url,
                auth,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get peers.
     */
    async getPeers(): Promise<IPeer[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetPeers',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Post block json.
     */
    async postBlockJson(block: IBlock): Promise<BlockId> {
        const response = await this.messageHandler.callClientMethod({
            name: 'PostBlockJson',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block raw.
     */
    async getBlockRaw(blockId: BlockId): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetBlockRaw',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block children.
     */
    async getBlockChildren(blockId: BlockId): Promise<BlockId[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetBlockChildren',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Look up a milestone by a given milestone index.
     */
    async getMilestoneByMilestoneId(
        milestoneId: string,
    ): Promise<IMilestonePayload> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMilestoneByMilestoneId',
            data: {
                milestoneId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns all UTXO changes that happened at a specific milestone.
     */
    async getUtxoChangesByMilestoneId(
        milestoneId: string,
    ): Promise<IMilestoneUtxoChangesResponse> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetUtxoChangesByMilestoneId',
            data: {
                milestoneId,
            },
        });

        return JSON.parse(response).payload;
    }
    /**
     * Look up a milestone by a given milestone index.
     */
    async getMilestoneByMilestoneIndex(
        index: number,
        ): Promise<IMilestonePayload> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMilestoneByMilestoneIndex',
            data: {
                index,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns all UTXO changes that happened at a specific milestone.
     */
    async getUtxoChangesByMilestoneIndex(
        index: number,
    ): Promise<IMilestoneUtxoChangesResponse> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetUtxoChangesByMilestoneIndex',
            data: {
                index,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get receipts.
     */
    async getReceipts(): Promise<IReceiptsResponse> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetReceipts',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the receipts by the given milestone index.
     */
    async getReceiptsMigratedAt(
        milestoneIndex: number,
    ): Promise<IReceiptsResponse[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetReceiptsMigratedAt',
            data: {
                milestoneIndex,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the treasury output.
     */
    async getTreasury(): Promise<ITreasury> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetTreasury',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the included block of the transaction.
     */
    async getIncludedBlock(transactionId: string): Promise<IBlock> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetIncludedBlock',
            data: {
                transactionId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Transforms bech32 to hex.
     */
    async bech32ToHex(bech32: string): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'Bech32ToHex',
            data: {
                bech32,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Transforms a hex encoded address to a bech32 encoded address.
     */
    async hexToBech32(hex: string, bech32Hrp?: string): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'HexToBech32',
            data: {
                hex,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Transforms a hex encoded public key to a bech32 encoded address.
     */
    async hexPublicKeyToBech32Address(
        hex: string,
        bech32Hrp?: string,
    ): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'HexPublicKeyToBech32Address',
            data: {
                hex,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Checks if a String is a valid bech32 encoded address.
     */
    async isAddressValid(address: string): Promise<boolean> {
        const response = await this.messageHandler.callClientMethod({
            name: 'IsAddressValid',
            data: {
                address,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch aliases output IDs
     */
    async aliasesOutputIds(
        queryParameters: QueryParameter[],
    ): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'AliasesOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch alias output ID
     */
    async aliasOutputId(aliasId: string): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'AliasOutputId',
            data: {
                aliasId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch NFTs output IDs
     */
    async nftsOutputIds(queryParameters: QueryParameter[]): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'NftsOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch NFT output ID
     */
    async nftOutputId(nftId: string): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'NftOutputId',
            data: {
                nftId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch Foundries Output IDs
     */
    async foundriesOutputIds(
        queryParameters: QueryParameter[],
    ): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'FoundriesOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch Foundry Output ID
     */
    async foundryOutputId(foundryId: string): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'FoundryOutputId',
            data: {
                foundryId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Try to get OutputResponse from provided OutputIds (requests are sent
     * in parallel and errors are ignored, can be useful for spent outputs)
     */
    async tryGetOutputs(outputIds: string[]): Promise<IOutputResponse[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'TryGetOutputs',
            data: {
                outputIds,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Find all blocks by provided block IDs.
     */
    async findBlocks(blockIds: BlockId[]): Promise<IBlock[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'FindBlocks',
            data: {
                blockIds,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Retries (promotes or reattaches) a block for provided block id. Block should be
     * retried only if they are valid and haven't been confirmed for a while.
     */
    async retry(blockId: BlockId): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'Retry',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
     * milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
     * position and additional reattached blocks
     */
    async retryUntilIncluded(
        blockId: BlockId,
        interval?: number,
        maxAttempts?: number,
    ): Promise<[BlockId, IBlock][]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'RetryUntilIncluded',
            data: {
                blockId,
                interval,
                maxAttempts,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
     * Returns the address to which the funds got consolidated, if any were available
     */
    async consolidateFunds(
        secretManager: SecretManager,
        accountIndex: number,
        addressRange?: IRange,
    ): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'ConsolidateFunds',
            data: {
                secretManager,
                accountIndex,
                addressRange,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
     * confirmed for a while.
     */
    async reattach(blockId: BlockId): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'Reattach',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Reattach a block without checking if it should be reattached
     */
    async reattachUnchecked(
        blockId: BlockId,
    ): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'ReattachUnchecked',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
     * method should error out and should not allow unnecessary promotions.
     */
    async promote(blockId: BlockId): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'Promote',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }
    /**
     * Promote a block without checking if it should be promoted
     */
    async promoteUnchecked(
        blockId: BlockId,
    ): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'PromoteUnchecked',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the unsynced nodes.
     */
    async unsyncedNodes(): Promise<Set<INode>> {
        const response = await this.messageHandler.callClientMethod({
            name: 'UnsyncedNodes',
        });

        return JSON.parse(response).payload;
    }

    // MQTT
    listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): void {
        return this.messageHandler.listen(topics, callback);
    }
}
