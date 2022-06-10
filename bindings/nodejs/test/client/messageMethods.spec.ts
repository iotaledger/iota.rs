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
describe.skip('Block methods', () => {
    it('sends a block raw', async () => {
        const block = await client.generateBlock();

        const blockId = await client.postBlockRaw(block);

        expect(blockId).toBeValidBlockId();
    });

    it('finds blocks by block IDs', async () => {
        const blockIds = await client.getTips();
        const blocks = await client.findBlocks(blockIds);

        expect(blocks.length).toBe(blockIds.length);
    });

    it('gets block as raw bytes', async () => {
        const block = await client.generateBlock();
        const blockId = await client.postBlock(block);

        const blockRaw = await client.getBlockRaw(blockId);

        expect(blockRaw).toBeDefined();
    });

    it('promotes a block', async () => {
        const block = await client.generateBlock();
        const blockId = await client.postBlock(block);

        // Promote a block without checking if it should be promoted
        const promoteUnchecked = await client.promoteUnchecked(blockId);

        expect(promoteUnchecked[1].parents).toContain(blockId);

        // Returns expected error: no need to promote or reattach.
        await expect(client.promote(blockId)).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });

    it('reattaches a block', async () => {
        const block = await client.generateBlock();
        const blockId = await client.postBlock(block);

        // Reattach a block without checking if it should be reattached
        const reattachUnchecked = await client.reattachUnchecked(blockId);

        expect(reattachUnchecked[0]).toBeValidBlockId();
        expect(reattachUnchecked[1]).toBeDefined();

        // Returns expected error: no need to promote or reattach.
        await expect(client.reattach(blockId)).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });

    // Skip by default, retryUntilIncluded can be slow
    it.skip('retries a block', async () => {
        const block = await client.generateBlock();
        const blockId = await client.postBlock(block);

        // Retries (promotes or reattaches) a block for provided block id until it's included
        // (referenced by a milestone). Default interval is 5 seconds and max attempts is 40.
        const retryUntilIncluded = await client.retryUntilIncluded(
            blockId,
            2,
            5,
        );
        //Returns the included block at first position and additional reattached blocks
        expect(retryUntilIncluded[0][0]).toBe(blockId);

        // Returns expected error: no need to promote or reattach.
        await expect(client.retry(blockId)).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });
});
