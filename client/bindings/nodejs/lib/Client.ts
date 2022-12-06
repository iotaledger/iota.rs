// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { MessageHandler } from './MessageHandler';
import type {
    IClientOptions,
    IGenerateAddressesOptions,
    IBuildBlockOptions,
    QueryParameter,
    IPreparedTransactionData,
    BlockId,
    INetworkInfo,
    SecretManager,
    INode,
    IAuth,
    IBasicOutputBuilderOptions,
    IAliasOutputBuilderOptions,
    IFoundryOutputBuilderOptions,
    INftOutputBuilderOptions,
    FoundryQueryParameter,
    NftQueryParameter,
    AliasQueryParameter,
    LedgerNanoStatus,
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
    INodeInfo,
    IReceiptsResponse,
    ITreasury,
    IBasicOutput,
    IAliasOutput,
    IFoundryOutput,
    INftOutput,
    INodeInfoProtocol,
} from '@iota/types';
import type { INodeInfoWrapper } from '../types/nodeInfo';

/** The Client to interact with nodes. */
export class Client {
    private messageHandler: MessageHandler;

    constructor(options: IClientOptions) {
        this.messageHandler = new MessageHandler(options);
    }

    /**
     * Returns the node information together with the url of the used node
     * @returns { Promise<INodeInfoWrapper> }.
     */
    async getInfo(): Promise<INodeInfoWrapper> {
        const response = await this.messageHandler.sendMessage({
            name: 'getInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Gets the network related information such as network_id and min_pow_score
     */
    async getNetworkInfo(): Promise<INetworkInfo> {
        const response = await this.messageHandler.sendMessage({
            name: 'getNetworkInfo',
        });

        return JSON.parse(response).payload;
    }

    /** Fetch basic output IDs based on query parameters */
    async basicOutputIds(queryParameters: QueryParameter[]): Promise<string[]> {
        const response = await this.messageHandler.sendMessage({
            name: 'basicOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Get output from a known outputID */
    async getOutput(outputId: string): Promise<IOutputResponse> {
        const response = await this.messageHandler.sendMessage({
            name: 'getOutput',
            data: {
                outputId,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Fetch OutputResponse from provided OutputIds (requests are sent in parallel) */
    async getOutputs(outputIds: string[]): Promise<IOutputResponse[]> {
        const response = await this.messageHandler.sendMessage({
            name: 'getOutputs',
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
        const response = await this.messageHandler.sendMessage({
            name: 'generateMnemonic',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns a hex encoded seed for a mnemonic.
     */
    async mnemonicToHexSeed(mnemonic: string): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'mnemonicToHexSeed',
            data: {
                mnemonic,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Computes the alias id for the given alias output id.
     */
    async computeAliasId(outputId: string): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'computeAliasId',
            data: {
                outputId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Computes the NFT id for the given NFT output id.
     */
    async computeNftId(outputId: string): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'computeNftId',
            data: {
                outputId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Computes the foundry id.
     */
    async computeFoundryId(
        aliasAddress: string,
        serialNumber: number,
        tokenSchemeKind: number,
    ): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'computeFoundryId',
            data: {
                aliasAddress,
                serialNumber,
                tokenSchemeKind,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Generate addresses */
    async generateAddresses(
        secretManager: SecretManager,
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<string[]> {
        const response = await this.messageHandler.sendMessage({
            name: 'generateAddresses',
            data: {
                secretManager,
                options: generateAddressesOptions,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Build and post a block */
    async buildAndPostBlock(
        secretManager?: SecretManager,
        options?: IBuildBlockOptions,
    ): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.sendMessage({
            name: 'buildAndPostBlock',
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
        const response = await this.messageHandler.sendMessage({
            name: 'getTips',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Post block in JSON format, returns the block ID.
     */
    async postBlock(block: IBlock): Promise<BlockId> {
        const response = await this.messageHandler.sendMessage({
            name: 'postBlock',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block as JSON.
     */
    async getBlock(blockId: BlockId): Promise<IBlock> {
        const response = await this.messageHandler.sendMessage({
            name: 'getBlock',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block metadata.
     */
    async getBlockMetadata(blockId: BlockId): Promise<IBlockMetadata> {
        const response = await this.messageHandler.sendMessage({
            name: 'getBlockMetadata',
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
        const response = await this.messageHandler.sendMessage({
            name: 'findInputs',
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
        const response = await this.messageHandler.sendMessage({
            name: 'findOutputs',
            data: {
                outputIds,
                addresses,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the status of a Ledger Nano
     */
    async getLedgerNanoStatus(isSimulator: boolean): Promise<LedgerNanoStatus> {
        const response = await this.messageHandler.sendMessage({
            name: 'getLedgerNanoStatus',
            data: {
                isSimulator,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Prepare a transaction for signing
     */
    async prepareTransaction(
        secretManager?: SecretManager,
        options?: IBuildBlockOptions,
    ): Promise<IPreparedTransactionData> {
        const response = await this.messageHandler.sendMessage({
            name: 'prepareTransaction',
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
        const response = await this.messageHandler.sendMessage({
            name: 'storeMnemonic',
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
        const response = await this.messageHandler.sendMessage({
            name: 'signTransaction',
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
    async postBlockPayload(payload: PayloadTypes): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.sendMessage({
            name: 'postBlockPayload',
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
        const response = await this.messageHandler.sendMessage({
            name: 'parseBech32Address',
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
        const response = await this.messageHandler.sendMessage({
            name: 'blockId',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get a node candidate from the healthy node pool.
     */
    async getNode(): Promise<INode> {
        const response = await this.messageHandler.sendMessage({
            name: 'getNode',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the network id of the node we're connecting to.
     */
    async getNetworkId(): Promise<number> {
        const response = await this.messageHandler.sendMessage({
            name: 'getNetworkId',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the bech32_hrp.
     */
    async getBech32Hrp(): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'getBech32Hrp',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the min PoW score.
     */
    async getMinPowScore(): Promise<number> {
        const response = await this.messageHandler.sendMessage({
            name: 'getMinPowScore',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the tips interval.
     */
    async getTipsInterval(): Promise<number> {
        const response = await this.messageHandler.sendMessage({
            name: 'getTipsInterval',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the token supply.
     */
    async getTokenSupply(): Promise<string> {
        return (await this.getProtocolParameters()).tokenSupply;
    }

    /**
     * Returns the protocol parameters.
     */
    async getProtocolParameters(): Promise<INodeInfoProtocol> {
        const response = await this.messageHandler.sendMessage({
            name: 'getProtocolParameters',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns if local pow should be used or not.
     */
    async getLocalPow(): Promise<boolean> {
        const response = await this.messageHandler.sendMessage({
            name: 'getLocalPow',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get fallback to local proof of work timeout.
     */
    async getFallbackToLocalPow(): Promise<boolean> {
        const response = await this.messageHandler.sendMessage({
            name: 'getFallbackToLocalPow',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get health of node by input url.
     */
    async getHealth(url: string): Promise<boolean> {
        const response = await this.messageHandler.sendMessage({
            name: 'getHealth',
            data: {
                url,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get info of node with input url.
     */
    async getNodeInfo(url: string, auth?: IAuth): Promise<INodeInfo> {
        const response = await this.messageHandler.sendMessage({
            name: 'getNodeInfo',
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
        const response = await this.messageHandler.sendMessage({
            name: 'getPeers',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Post block as raw bytes, returns the block ID.
     */
    async postBlockRaw(block: IBlock): Promise<BlockId> {
        const response = await this.messageHandler.sendMessage({
            name: 'postBlockRaw',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block as raw bytes.
     */
    async getBlockRaw(blockId: BlockId): Promise<Uint8Array> {
        const response = await this.messageHandler.sendMessage({
            name: 'getBlockRaw',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Look up a milestone by a given milestone index.
     */
    async getMilestoneById(milestoneId: string): Promise<IMilestonePayload> {
        const response = await this.messageHandler.sendMessage({
            name: 'getMilestoneById',
            data: {
                milestoneId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns all UTXO changes that happened at a specific milestone.
     */
    async getUtxoChangesById(
        milestoneId: string,
    ): Promise<IMilestoneUtxoChangesResponse> {
        const response = await this.messageHandler.sendMessage({
            name: 'getUtxoChangesById',
            data: {
                milestoneId,
            },
        });

        return JSON.parse(response).payload;
    }
    /**
     * Look up a milestone by a given milestone index.
     */
    async getMilestoneByIndex(index: number): Promise<IMilestonePayload> {
        const response = await this.messageHandler.sendMessage({
            name: 'getMilestoneByIndex',
            data: {
                index,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns all UTXO changes that happened at a specific milestone.
     */
    async getUtxoChangesByIndex(
        index: number,
    ): Promise<IMilestoneUtxoChangesResponse> {
        const response = await this.messageHandler.sendMessage({
            name: 'getUtxoChangesByIndex',
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
        const response = await this.messageHandler.sendMessage({
            name: 'getReceipts',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the receipts by the given milestone index.
     */
    async getReceiptsMigratedAt(
        milestoneIndex: number,
    ): Promise<IReceiptsResponse[]> {
        const response = await this.messageHandler.sendMessage({
            name: 'getReceiptsMigratedAt',
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
        const response = await this.messageHandler.sendMessage({
            name: 'getTreasury',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the included block of the transaction.
     */
    async getIncludedBlock(transactionId: string): Promise<IBlock> {
        const response = await this.messageHandler.sendMessage({
            name: 'getIncludedBlock',
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
        const response = await this.messageHandler.sendMessage({
            name: 'bech32ToHex',
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
        const response = await this.messageHandler.sendMessage({
            name: 'hexToBech32',
            data: {
                hex,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Transforms an alias id to a bech32 encoded address.
     */
    async aliasIdToBech32(
        aliasId: string,
        bech32Hrp?: string,
    ): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'aliasIdToBech32',
            data: {
                aliasId,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Transforms an nft id to a bech32 encoded address.
     */
    async nftIdToBech32(nftId: string, bech32Hrp?: string): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'nftIdToBech32',
            data: {
                nftId,
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
        const response = await this.messageHandler.sendMessage({
            name: 'hexPublicKeyToBech32Address',
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
        const response = await this.messageHandler.sendMessage({
            name: 'isAddressValid',
            data: {
                address,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch alias output IDs
     */
    async aliasOutputIds(
        queryParameters: AliasQueryParameter[],
    ): Promise<string[]> {
        const response = await this.messageHandler.sendMessage({
            name: 'aliasOutputIds',
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
        const response = await this.messageHandler.sendMessage({
            name: 'aliasOutputId',
            data: {
                aliasId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch NFT output IDs
     */
    async nftOutputIds(
        queryParameters: NftQueryParameter[],
    ): Promise<string[]> {
        const response = await this.messageHandler.sendMessage({
            name: 'nftOutputIds',
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
        const response = await this.messageHandler.sendMessage({
            name: 'nftOutputId',
            data: {
                nftId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch Foundry Output IDs
     */
    async foundryOutputIds(
        queryParameters: FoundryQueryParameter[],
    ): Promise<string[]> {
        const response = await this.messageHandler.sendMessage({
            name: 'foundryOutputIds',
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
        const response = await this.messageHandler.sendMessage({
            name: 'foundryOutputId',
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
        const response = await this.messageHandler.sendMessage({
            name: 'tryGetOutputs',
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
        const response = await this.messageHandler.sendMessage({
            name: 'findBlocks',
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
        const response = await this.messageHandler.sendMessage({
            name: 'retry',
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
        const response = await this.messageHandler.sendMessage({
            name: 'retryUntilIncluded',
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
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<string> {
        const response = await this.messageHandler.sendMessage({
            name: 'consolidateFunds',
            data: {
                secretManager,
                generateAddressesOptions,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
     * confirmed for a while.
     */
    async reattach(blockId: BlockId): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.sendMessage({
            name: 'reattach',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Reattach a block without checking if it should be reattached
     */
    async reattachUnchecked(blockId: BlockId): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.sendMessage({
            name: 'reattachUnchecked',
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
        const response = await this.messageHandler.sendMessage({
            name: 'promote',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }
    /**
     * Promote a block without checking if it should be promoted
     */
    async promoteUnchecked(blockId: BlockId): Promise<[BlockId, IBlock]> {
        const response = await this.messageHandler.sendMessage({
            name: 'promoteUnchecked',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the unhealthy nodes.
     */
    async unhealthyNodes(): Promise<Set<INode>> {
        const response = await this.messageHandler.sendMessage({
            name: 'unhealthyNodes',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Build a Basic Output.
     */
    async buildBasicOutput(
        options: IBasicOutputBuilderOptions,
    ): Promise<IBasicOutput> {
        const response = await this.messageHandler.sendMessage({
            name: 'buildBasicOutput',
            data: options,
        });

        return JSON.parse(response).payload;
    }

    /**
     * Build an Alias Output.
     */
    async buildAliasOutput(
        options: IAliasOutputBuilderOptions,
    ): Promise<IAliasOutput> {
        const response = await this.messageHandler.sendMessage({
            name: 'buildAliasOutput',
            data: options,
        });

        return JSON.parse(response).payload;
    }

    /**
     * Build a Foundry Output.
     */
    async buildFoundryOutput(
        options: IFoundryOutputBuilderOptions,
    ): Promise<IFoundryOutput> {
        const response = await this.messageHandler.sendMessage({
            name: 'buildFoundryOutput',
            data: options,
        });

        return JSON.parse(response).payload;
    }

    /**
     * Build an Nft Output.
     */
    async buildNftOutput(
        options: INftOutputBuilderOptions,
    ): Promise<INftOutput> {
        const response = await this.messageHandler.sendMessage({
            name: 'buildNftOutput',
            data: options,
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
