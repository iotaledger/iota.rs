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

// Skip for CI
describe.skip('UTXO methods', () => {
    it('gets receipts', async () => {
        const info = await client.getInfo();
        const milestoneIndex = info.nodeInfo.status.confirmedMilestone.index;

        const receipts = await client.getReceipts();
        expect(receipts).toBeDefined();

        const receiptsMigratedAt = await client.getReceiptsMigratedAt(
            milestoneIndex,
        );
        expect(receiptsMigratedAt).toBeDefined();
    });

    it('gets treasury', async () => {
        const treasury = await client.getTreasury();

        expect(treasury).toBeDefined();
    });

    it('gets aliases output IDs', async () => {
        const aliasesOutputIds = await client.aliasOutputIds([
            {
                stateController:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
        ]);

        expect(aliasesOutputIds).toBeDefined();
    });

    it('gets nfts output IDs', async () => {
        const nftsOutputIds = await client.nftOutputIds([
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
        ]);

        expect(nftsOutputIds).toBeDefined();
    });

    it('gets foundries output IDs', async () => {
        const foundriesOutputIds = await client.foundryOutputIds([
            {
                hasNativeTokens: true,
            },
        ]);

        expect(foundriesOutputIds).toBeDefined();
    });

    // TODO: get valid IDs to test with
    it('get alias/nft/foundry outputId rejects with 404 for invalid IDs', async () => {
        await expect(
            client.aliasOutputId(
                '0x03119f37e7ad40608fc7ab15db49390abc233648c95e78141ff2e298f60d7a95',
            ),
        ).rejects.toMatch('404');

        await expect(
            client.nftOutputId(
                '0x03119f37e7ad40608fc7ab15db49390abc233648c95e78141ff2e298f60d7a95',
            ),
        ).rejects.toMatch('404');

        await expect(
            client.foundryOutputId(
                '0x03119f37e7ad40608fc7ab15db49390abc233648c95e78141ff2e298f60d7a9541ff2e60d7a9',
            ),
        ).rejects.toMatch('404');
    });
});
