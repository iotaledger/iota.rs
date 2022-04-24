// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MessageHandler } from './MessageHandler';
import type {
    ClientOptions,
    GenerateAddressesOptions,
    NodeInfo,
    OutputResponse,
} from '../types';
import type { Message } from '../types/message';
import type { MessageMetadata } from '../types/messageMetadata';
import type { GenerateMessageOptions } from '../types/generateMessageOptions';
import type { QueryParameter } from '../types/queryParameters';
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

    // MQTT
    listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): void {
        return this.messageHandler.listen(topics, callback);
    }
}
