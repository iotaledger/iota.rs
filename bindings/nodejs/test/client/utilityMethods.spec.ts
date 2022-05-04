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

describe('Client utility methods', () => {
    it('converts address to hex and bech32', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';
        const hexAddress = await client.bech32ToHex(address);

        expect(hexAddress.slice(0, 2)).toBe('0x');

        const bech32Address = await client.hexToBech32(hexAddress, 'rms');

        expect(bech32Address).toBe(address);
    });

    it('converts hex public key to bech32 address', async () => {
        const hexPublicKey =
            '2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a';

        const address = await client.hexPublicKeyToBech32Address(
            hexPublicKey,
            'rms',
        );

        expect(address).toBeValidAddress();
    });

    it('validates address', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';

        const isAddressValid = await client.isAddressValid(address);

        expect(isAddressValid).toBeTruthy();
    });
});
