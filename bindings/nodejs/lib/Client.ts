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
} from '../types';
import type {
    INodeInfo,
    IUTXOInput,
    AddressTypes,
    IOutputResponse,
    IMessage,
    IMessageMetadata,
    PayloadTypes,
} from '@iota/types';

export class Client {
    private messageHandler: MessageHandler;

    constructor(options: IClientOptions) {
        this.messageHandler = new MessageHandler(options);
    }

    /**
     * Get info about the node
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

    async generateMnemonic(): Promise<string> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateMnemonic',
        });

        return JSON.parse(response).payload;
    }

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
        // TODO: should be outputIds: string[], fixed in https://github.com/iotaledger/iota.rs/pull/952
        outputs: IUTXOInput[],
        addresses: string[],
    ): Promise<IOutputResponse[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'FindOutputs',
            data: {
                outputs,
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

    // MQTT
    listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): void {
        return this.messageHandler.listen(topics, callback);
    }
}
