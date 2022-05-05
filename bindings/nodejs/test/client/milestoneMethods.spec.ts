import { Client } from '../../lib';
import '../customMatchers';
import 'dotenv/config';

const client = new Client({
    nodes: [
        {
            // Insert your node URL here.
            url: process.env.NODE_URL || 'http://localhost:14265',
            disabled: false,
        },
    ],
    localPow: true,
});

// Skip for CI
describe.skip('Milestone methods', () => {
    it('gets milestone by given milestone ID', async () => {
        const info = await client.getInfo();
        const milestoneId = info.nodeinfo.status.confirmedMilestone.milestoneId;

        const milestone = await client.getMilestoneByMilestoneId(milestoneId);

        expect(milestone.index).toEqual(
            info.nodeinfo.status.confirmedMilestone.index,
        );
    });

    it('gets all UTXO changes of a given milestone by milestone ID', async () => {
        const info = await client.getInfo();
        const milestoneId = info.nodeinfo.status.confirmedMilestone.milestoneId;

        const milestoneUtxoChanges = await client.getUtxoChangesByMilestoneId(
            milestoneId,
        );

        expect(milestoneUtxoChanges.index).toEqual(
            info.nodeinfo.status.confirmedMilestone.index,
        );
    });

    it('gets milestone by given milestone index', async () => {
        const info = await client.getInfo();
        const milestoneIndex = info.nodeinfo.status.confirmedMilestone.index;

        const milestone = await client.getMilestoneByMilestoneIndex(
            milestoneIndex,
        );

        expect(milestone.index).toEqual(milestoneIndex);
    });

    it('gets all UTXO changes of a given milestone by milestone index', async () => {
        const info = await client.getInfo();
        const milestoneIndex = info.nodeinfo.status.confirmedMilestone.index;

        const milestoneUtxoChanges =
            await client.getUtxoChangesByMilestoneIndex(milestoneIndex);

        expect(milestoneUtxoChanges.index).toEqual(milestoneIndex);
    });
});
