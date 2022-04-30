// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { MessageHandler } from './MessageHandler';
import type {
    IClientOptions,
    IGenerateAddressesOptions,
    IGenerateMessageOptions,
    QueryParameter,
    IPreparedTransactionData,
    MessageId,
    INetworkInfo,
    SecretManager,
    INode,
    IAuth,
} from '../types';
import type {
    INodeInfo,
    IUTXOInput,
    AddressTypes,
    IOutputResponse,
    IMessage,
    IMessageMetadata,
    PayloadTypes,
    IPeer,
    IMilestoneResponse,
    IMilestoneUtxoChangesResponse,
    IReceiptsResponse,
    ITreasury,
} from '@iota/types';

export class Client {
    private messageHandler: MessageHandler;

    constructor(options: IClientOptions) {
        this.messageHandler = new MessageHandler(options);
    }

    /**
     * Returns the node information together with the url of the used node
     * TODO: INodeInfo is missing url and BaseTokenResponse
     */
    async getInfo(): Promise<INodeInfo> {
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

    async generateMessage(
        secretManager?: SecretManager,
        options?: IGenerateMessageOptions,
    ): Promise<IMessage> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateMessage',
            data: {
                secretManager,
                options,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns tips that are ideal for attaching a message.
     * The tips can be considered as non-lazy and are therefore ideal for attaching a message.
     */
    async getTips(): Promise<MessageId[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetTips',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Send message, returns the message ID.
     */
    async postMessage(message: IMessage): Promise<MessageId> {
        const response = await this.messageHandler.callClientMethod({
            name: 'PostMessage',
            data: {
                message,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get message data with message ID
     */
    async getMessageData(messageId: MessageId): Promise<IMessage> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMessageData',
            data: {
                messageId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get message metadata with message ID
     */
    async getMessageMetadata(messageId: MessageId): Promise<IMessageMetadata> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMessageMetadata',
            data: {
                messageId,
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
        options?: IGenerateMessageOptions,
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
     * Submit a payload in a message
     */
    async submitPayload(payload: PayloadTypes): Promise<IMessage> {
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
     * Returns a message ID (Blake2b256 hash of the message bytes)
     */
    async messageId(message: IMessage): Promise<MessageId> {
        const response = await this.messageHandler.callClientMethod({
            name: 'MessageId',
            data: {
                message,
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
     * Post message json.
     */
    async postMessageJson(message: IMessage): Promise<MessageId> {
        const response = await this.messageHandler.callClientMethod({
            name: 'PostMessageJson',
            data: {
                message,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get message raw.
     */
    async getMessageRaw(messageId: MessageId): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMessageRaw',
            data: {
                messageId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get message children.
     */
    async getMessageChildren(messageId: MessageId): Promise<MessageId[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMessageChildren',
            data: {
                messageId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Look up a milestone by a given milestone index.
     */
    async getMilestone(index: number): Promise<IMilestoneResponse> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMilestone',
            data: {
                index,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns all UTXO changes that happened at a specific milestone.
     */
    async getMilestoneUtxoChanges(
        index: number,
    ): Promise<IMilestoneUtxoChangesResponse> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetMilestoneUtxoChanges',
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
     * Returns the included message of the transaction.
     */
    async getIncludedMessage(transactionId: string): Promise<IMessage> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetIncludedMessage',
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
    async isAddressValid(address: string): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'IsAddressValid',
            data: {
                address,
            },
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
