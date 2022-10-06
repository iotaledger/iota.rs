import { Client } from '../../lib';
import '../customMatchers';
import 'dotenv/config';

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
describe.skip('Output builder methods', () => {
    it('builds a basic output', async () => {
        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 0,
                end: 1,
            },
        });

        const hexAddress = await client.bech32ToHex(addresses[0]);

        // most simple basic output
        const basicOutput = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [
                {
                    type: 0,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
            ],
        });

        expect(basicOutput).toBeDefined();
    });

    it('builds an alias output', async () => {
        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 0,
                end: 1,
            },
        });

        const hexAddress = await client.bech32ToHex(addresses[0]);

        const aliasId =
            '0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa';
        // most simple alias output
        const aliasOutput = await client.buildAliasOutput({
            aliasId,
            unlockConditions: [
                {
                    type: 4,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
                {
                    type: 5,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
            ],
        });

        expect(aliasOutput).toBeDefined();
    });

    it('builds a foundry output', async () => {
        const aliasId =
            '0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa';

        // most simple foundry output
        const foundryOutput = await client.buildFoundryOutput({
            serialNumber: 1,
            nativeTokens: [
                {
                    id: '0x081e6439529b020328c08224b43172f282cb16649d50c891fa156365323667e47a0100000000',
                    amount: '0x32',
                },
            ],
            tokenScheme: {
                type: 0,
                meltedTokens: '0x0',
                mintedTokens: '0x32',
                maximumSupply: '0x64',
            },
            unlockConditions: [
                {
                    type: 6,
                    address: {
                        type: 8,
                        aliasId,
                    },
                },
            ],
        });

        expect(foundryOutput).toBeDefined();
    });

    it('builds an nft output', async () => {
        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 0,
                end: 1,
            },
        });

        const hexAddress = await client.bech32ToHex(addresses[0]);

        // most simple nft output
        const nftOutput = await client.buildNftOutput({
            nftId: '0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3',
            unlockConditions: [
                {
                    type: 0,
                    address: {
                        type: 0,
                        pubKeyHash: hexAddress,
                    },
                },
            ],
        });

        expect(nftOutput).toBeDefined();
    });
});
