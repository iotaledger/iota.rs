import type { IBlock, IOutputResponse, ITaggedDataPayload } from '@iota/types';
import { Client, utf8ToHex } from '../../lib';
import '../customMatchers';
import 'dotenv/config';
import * as addressOutputs from '../fixtures/addressOutputs.json';

const client = new Client({
    nodes: [
        {
            url: process.env.NODE_URL || 'http://localhost:14265',
        },
    ],
    localPow: true,
});

const secretManager = {
    mnemonic:
        'endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river',
};

// Skip for CI
describe.skip('Main examples', () => {
    it('gets info about the node', async () => {
        const info = await client.getInfo();

        expect(info.nodeInfo.protocol.bech32Hrp).toBe('rms');
        expect(info.nodeInfo.protocol.minPowScore).toBe(1000);
    });

    it('generates a mnemonic', async () => {
        const mnemonic = await client.generateMnemonic();

        expect(mnemonic).toBeDefined();
    });

    it('generates addresses', async () => {
        const addresses = await client.generateAddresses(secretManager, {
            accountIndex: 0,
            range: {
                start: 0,
                end: 5,
            },
            bech32Hrp: 'rms',
        });

        expect(addresses.length).toBe(5);

        addresses.forEach((address) => {
            expect(address).toBeValidAddress();
        });
    });

    it('gets address outputs', async () => {
        const outputIdsResponse = await client.basicOutputIds([
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
            { hasExpiration: false },
            { hasTimelock: false },
            { hasStorageDepositReturn: false },
        ]);

        outputIdsResponse.items.forEach((id) => expect(id).toBeValidOutputId());

        const addressOutputs = await client.getOutputs(outputIdsResponse.items);

        expect(addressOutputs).toBeDefined();

        addressOutputs.forEach((output) => {
            expect(output.metadata.blockId).toBeValidBlockId();
        });
    });

    it('gets the output of a known output ID', async () => {
        const output = await client.getOutput(
            '0xc1d95ac9c8c0237c6929faf427556c3562055a7155c6d336ee7891691d5525c90100',
        );

        expect(output.metadata.blockId).toBeValidBlockId();
    });

    it('gets the balance of an address', async () => {
        // Generate the first address
        const addresses = await client.generateAddresses(secretManager, {
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        });
        expect(addresses[0]).toBeValidAddress();

        // Get output ids of outputs that can be controlled by this address without further unlock constraints
        const outputIdsResponse = await client.basicOutputIds([
            { address: addresses[0] },
            { hasExpiration: false },
            { hasTimelock: false },
            { hasStorageDepositReturn: false },
        ]);
        outputIdsResponse.items.forEach((id) => expect(id).toBeValidOutputId());

        // Get outputs by their IDs
        const addressOutputs = await client.getOutputs(outputIdsResponse.items);
        expect(addressOutputs).toBeDefined();
    });

    it('calculates the balance of an address', () => {
        const testOutputs = addressOutputs as IOutputResponse[];

        // Calculate the total amount and native tokens
        let totalAmount = 0;
        const totalNativeTokens: { [id: string]: number } = {};
        for (const outputResponse of testOutputs) {
            const output = outputResponse['output'];

            if ('nativeTokens' in output) {
                output.nativeTokens?.forEach(
                    (token) =>
                        (totalNativeTokens[token.id] =
                            (totalNativeTokens[token.id] || 0) +
                            parseInt(token.amount)),
                );
            }

            totalAmount += parseInt(output.amount);
        }

        expect(totalAmount).toBe(1960954000);
        expect(Object.keys(totalNativeTokens).length).toBe(2);
        expect(
            Object.values(totalNativeTokens).reduce(
                (acc: number, val: number) => acc + val,
            ),
        ).toBe(200);
    });

    it('sends a block', async () => {
        const blockIdAndBlock = await client.buildAndPostBlock();

        expect(blockIdAndBlock[0]).toBeValidBlockId();
    });

    it('gets block data', async () => {
        const blockIdAndBlock = await client.buildAndPostBlock();

        const blockData = await client.getBlock(blockIdAndBlock[0]);
        const blockMetadata = await client.getBlockMetadata(blockIdAndBlock[0]);

        expect(blockData).toStrictEqual<IBlock>(blockIdAndBlock[1]);
        expect(blockMetadata.blockId).toBeValidBlockId();
    });

    it('sends a block with a tagged data payload', async () => {
        const blockIdAndBlock = await client.buildAndPostBlock(secretManager, {
            tag: utf8ToHex('Hello'),
            data: utf8ToHex('Tangle'),
        });

        const fetchedBlock = await client.getBlock(blockIdAndBlock[0]);

        expect(fetchedBlock.payload).toStrictEqual<ITaggedDataPayload>({
            type: 5,
            tag: utf8ToHex('Hello'),
            data: utf8ToHex('Tangle'),
        });
    });

    it('sends a transaction', async () => {
        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 1,
                end: 2,
            },
        });

        const blockIdAndBlock = await client.buildAndPostBlock(secretManager, {
            output: {
                address: addresses[0],
                amount: '1000000',
            },
        });

        expect(blockIdAndBlock[0]).toBeValidBlockId();
    });
});
