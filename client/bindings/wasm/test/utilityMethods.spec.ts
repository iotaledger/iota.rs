import { Client } from '../lib';

const offlineClient = new Client({});

describe('Client utility methods', () => {
    it.skip('generates and validates mnemonic', async () => {
        const mnemonic = await offlineClient.generateMnemonic();

        // A mnemonic has 24 words
        await expect(
            mnemonic.split(' '),
        ).resolves.toBe(24);
    });

    it('converts address to hex and bech32', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';
        const hexAddress = await offlineClient.bech32ToHex(address);

        expect(hexAddress.slice(0, 2)).toBe('0x');

        const bech32Address = await offlineClient.hexToBech32(
            hexAddress,
            'rms',
        );

        expect(bech32Address).toBe(address);
    });

    it('converts hex public key to bech32 address', async () => {
        const hexPublicKey =
            '0x2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a';

        const address = await offlineClient.hexPublicKeyToBech32Address(
            hexPublicKey,
            'rms',
        );

        expect(address).toBe('rms1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx4aaacx');
    });

    it('validates address', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';

        const isAddressValid = await offlineClient.isAddressValid(address);

        expect(isAddressValid).toBeTruthy();
    });
});
