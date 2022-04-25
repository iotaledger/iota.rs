// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { MessageHandler } from './MessageHandler';
import type {
    Address,
    ClientOptions,
    GenerateAddressesOptions,
    NodeInfo,
    OutputResponse,
} from '../types';
import type { Message } from '../types/message';
import type { MessageMetadata } from '../types/messageMetadata';
import type { GenerateMessageOptions } from '../types/generateMessageOptions';
import type { QueryParameter } from '../types/queryParameters';
import type { UTXOInput } from '../types/inputs/UTXOInput';
import type { Payload } from '../types/payloads';
import type { PreparedTransactionData } from '../types/preparedTransactionData';

export class Client {
    private messageHandler: MessageHandler;

    constructor(options: ClientOptions) {
        this.messageHandler = new MessageHandler(options);
    }

    async getInfo(): Promise<NodeInfo> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Gets the network related information such as network_id and min_pow_score
     */
    async getNetworkInfo() {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetNetworkInfo',
        });

        return JSON.parse(response).payload;
    }

    // TODO: proper type for queryParameters
    async outputIds(queryParameters: QueryParameter[]): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'OutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    async getOutput(outputId: string): Promise<OutputResponse> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetOutput',
            data: {
                outputId,
            },
        });

        return JSON.parse(response).payload;
    }

    async getOutputs(outputIds: string[]): Promise<OutputResponse[]> {
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
        mnemonic: string,
        generateAddressesOptions: GenerateAddressesOptions,
    ): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateAddresses',
            data: {
                signer: mnemonic,
                options: generateAddressesOptions,
            },
        });

        return JSON.parse(response).payload;
    }

    async generateMessage(
        signer?: string,
        options?: GenerateMessageOptions,
    ): Promise<Message> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateMessage',
            data: {
                signer,
                options,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns tips that are ideal for attaching a message.
     * The tips can be considered as non-lazy and are therefore ideal for attaching a message.
     */
    async getTips(): Promise<string[]> {
        const response = await this.messageHandler.callClientMethod({
            name: 'GetTips',
        });

        return JSON.parse(response).payload;
    }

    async postMessage(message: Message) {
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
    async getMessageData(messageId: string): Promise<Message> {
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
    async getMessageMetadata(messageId: string): Promise<MessageMetadata> {
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
    ): Promise<UTXOInput[]> {
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
        outputs: UTXOInput[],
        addresses: string[],
    ): Promise<UTXOInput[]> {
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
        signer?: string,
        options?: GenerateMessageOptions,
    ) {
        const response = await this.messageHandler.callClientMethod({
            name: 'PrepareTransaction',
            data: {
                signer,
                options,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Sign a transaction
     */
    async signTransaction(
        signer: string,
        preparedTransactionData: PreparedTransactionData,
    ) {
        const response = await this.messageHandler.callClientMethod({
            name: 'SignTransaction',
            data: {
                signer,
                preparedTransactionData,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Submit a payload in a message
     */
    async submitPayload(payload: Payload) {
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
    async parseBech32Address(address: string): Promise<Address> {
        const response = await this.messageHandler.callClientMethod({
            name: 'ParseBech32Address',
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
