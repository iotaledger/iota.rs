import type { IMessage, ITaggedDataPayload } from '@iota/types';
import { Client, utf8ToBytes, utf8ToHex } from '../../lib';
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

const secretManager = JSON.stringify({
    Mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
});

describe('Client', () => {
    it('gets info about the node', async () => {
        const info = await client.getInfo();

        // @ts-ignore: INodeInfo type is incorrect
        expect(info.nodeinfo.protocol.bech32HRP).toBe('rms');
        // @ts-ignore: INodeInfo type is incorrect
        expect(info.nodeinfo.protocol.minPoWScore).toBe(1000);
    });

    it('generates a mnemonic', async () => {
        const mnemonic = await client.generateMnemonic();

        // TODO: verify mnemonic is valid?
        expect(mnemonic).toBeDefined();
    });

    it('generates addresses', async () => {
        // @ts-ignore: clientMethods expect secretManager to be JSON string
        // fixed in https://github.com/iotaledger/iota.rs/pull/960
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
        const outputIds = await client.outputIds([
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
            { hasExpirationCondition: false },
            { hasTimelockCondition: false },
            { hasStorageDepositReturnCondition: false },
        ]);

        const addressOutputs = await client.getOutputs(outputIds);

        expect(addressOutputs).toBeDefined();

        addressOutputs.forEach((output) => {
            expect(output.messageId).toBeValidMessageId();
        });
    });

    it('gets the output of a known output ID', async () => {
        const output = await client.getOutput(
            '0xee8255ece109f4d460fa85d34f2a5f152014633db571220c84d6ebb944f129c00000',
        );

        expect(output.messageId).toBeValidMessageId();
    });

    // TODO: implement this test
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    it.skip('gets the balance of an address', () => {});

    it('sends a message', async () => {
        const message = await client.generateMessage();

        const messageId = await client.postMessage(message);

        expect(messageId).toBeValidMessageId();
    });

    it('gets message data', async () => {
        const message = await client.generateMessage();

        // Send message
        const messageId = await client.postMessage(message);

        const messageData = await client.getMessageData(messageId);
        const messageMetadata = await client.getMessageMetadata(messageId);

        expect(messageData).toStrictEqual<IMessage>(message);
        expect(messageMetadata.messageId).toBeValidMessageId();
    });

    it('sends a message with a tagged data payload', async () => {
        // @ts-ignore: clientMethods expect secretManager to be JSON string
        // fixed in https://github.com/iotaledger/iota.rs/pull/960
        const message = await client.generateMessage(secretManager, {
            tag: utf8ToBytes('Hello'),
            data: utf8ToBytes('Tangle'),
        });

        // Send message
        const messageId = await client.postMessage(message);

        const fetchedMessage = await client.getMessageData(messageId);

        expect(fetchedMessage.payload).toStrictEqual<ITaggedDataPayload>({
            type: 5,
            tag: utf8ToHex('Hello'),
            data: utf8ToHex('Tangle'),
        });
    });

    // Test currently broken, error: tokenId
    it.skip('sends a transaction', async () => {
        // @ts-ignore: clientMethods expect secretManager to be JSON string
        // fixed in https://github.com/iotaledger/iota.rs/pull/960
        const addresses = await client.generateAddresses(secretManager, {
            range: {
                start: 1,
                end: 2,
            },
        });

        // @ts-ignore: clientMethods expect secretManager to be JSON string
        // fixed in https://github.com/iotaledger/iota.rs/pull/960
        // TODO: fix error: {"type":"MessageDtoError","error":"tokenId"}
        const message = await client.generateMessage(secretManager, {
            output: {
                address: addresses[0],
                amount: '1000000',
            },
        });

        // Send transaction
        const messageId = await client.postMessage(message);

        expect(messageId).toBeValidMessageId();
    });

    it('converts address', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';
        const hexAddress = await client.bech32ToHex(address);

        expect(hexAddress.slice(0, 2)).toBe('0x');

        const bech32Address = await client.hexToBech32(hexAddress, 'rms');

        expect(bech32Address).toBe(address);
    });

    it('gets tips', async () => {
        const tips = await client.getTips();

        expect(tips[0]).toBeValidMessageId();
    });
});
