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
describe.skip('Milestone methods', () => {
    it('gets milestone by given milestone ID', async () => {
        const info = await client.getInfo();
        const milestoneId = info.nodeInfo.status.confirmedMilestone.milestoneId;

        if (milestoneId !== undefined) {
            const milestone = await client.getMilestoneById(milestoneId);

            expect(milestone.index).toEqual(
                info.nodeInfo.status.confirmedMilestone.index,
            );
        }
    });

    it('gets all UTXO changes of a given milestone by milestone ID', async () => {
        const info = await client.getInfo();
        const milestoneId = info.nodeInfo.status.confirmedMilestone.milestoneId;

        if (milestoneId !== undefined) {
            const milestoneUtxoChanges = await client.getUtxoChangesById(
                milestoneId,
            );

            expect(milestoneUtxoChanges.index).toEqual(
                info.nodeInfo.status.confirmedMilestone.index,
            );
        }
    });

    it('gets milestone by given milestone index', async () => {
        const info = await client.getInfo();
        const milestoneIndex = info.nodeInfo.status.confirmedMilestone.index;

        const milestone = await client.getMilestoneByIndex(milestoneIndex);

        expect(milestone.index).toEqual(milestoneIndex);
    });

    it('gets all UTXO changes of a given milestone by milestone index', async () => {
        const info = await client.getInfo();
        const milestoneIndex = info.nodeInfo.status.confirmedMilestone.index;

        const milestoneUtxoChanges = await client.getUtxoChangesByIndex(
            milestoneIndex,
        );

        expect(milestoneUtxoChanges.index).toEqual(milestoneIndex);
    });
});
