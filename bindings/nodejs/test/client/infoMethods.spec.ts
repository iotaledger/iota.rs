import { Client } from '../../lib';
import '../customMatchers';
import 'dotenv/config';

const client = new Client({
    nodes: [
        {
            url: process.env.NODE_URL || 'http://localhost:14265',
            disabled: false,
        },
    ],
    localPow: true,
});

// Skip for CI
describe.skip('Client info methods', () => {
    it('gets a node candidate from the synced node pool', async () => {
        const nodeInfo = await client.getNode();

        expect(nodeInfo.disabled).not.toBeTruthy();
    });

    it('gets info about node by url', async () => {
        const nodeInfo = await client.getNode();

        const nodeInfoByUrl = await client.getNodeInfo(nodeInfo.url);

        expect(nodeInfoByUrl).toBeDefined();
    });

    it('gets health of node', async () => {
        const health = await client.getHealth();

        expect(health).toBeTruthy();
    });

    it('gets health of node with input url', async () => {
        const nodeInfo = await client.getNode();

        const nodeHealth = await client.getNodeHealth(nodeInfo.url);

        expect(nodeHealth).toBeTruthy();
    });

    it('gets the unsynced nodes', async () => {
        const unsyncedNodes = await client.unsyncedNodes();

        expect(unsyncedNodes).toBeDefined();
    });

    it('gets tips', async () => {
        const tips = await client.getTips();

        expect(tips.length).toBeGreaterThan(0);
    });

    // TODO: fix error: missing or malformed JWT
    it.skip('gets peers', async () => {
        const peers = await client.getPeers();

        expect(peers).toBeDefined();
    });

    it('gets networkInfo', async () => {
        const networkInfo = await client.getNetworkInfo();

        expect(networkInfo.localPow).toBeTruthy();
        expect(networkInfo.minPoWScore).toBe(1000);
        expect(networkInfo.bech32HRP).toBe('rms');
    });

    it('gets networkId', async () => {
        const networkId = await client.getNetworkId();

        expect(networkId).toBeDefined();
    });

    it('gets bech32Hrp', async () => {
        const bech32Hrp = await client.getBech32Hrp();

        expect(bech32Hrp).toBeDefined();
    });

    it('gets minimum PoW score', async () => {
        const minPowScore = await client.getMinPowScore();

        expect(minPowScore).toBeDefined();
    });

    it('gets tips interval', async () => {
        const tipsInterval = await client.getTipsInterval();

        expect(tipsInterval).toBeDefined();
    });

    it('gets local PoW status', async () => {
        const localPow = await client.getLocalPow();

        expect(localPow).toBeTruthy();
    });

    it('gets fallback to local PoW status', async () => {
        const fallbackToLocalPow = await client.getFallbackToLocalPow();

        expect(fallbackToLocalPow).toBeTruthy();
    });
});
