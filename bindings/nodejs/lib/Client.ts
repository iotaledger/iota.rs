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

    async getOutputIds(queryParameters: Address[]) {
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

    async generateMnemonic() {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateMnemonic',
        });

        return JSON.parse(response).payload;
    }

    async mnemonicToHexSeed(mnemonic: string) {
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
    ) {
        const response = await this.messageHandler.callClientMethod({
            name: 'GenerateAddresses',
            data: {
                signer: mnemonic,
                options: generateAddressesOptions,
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
