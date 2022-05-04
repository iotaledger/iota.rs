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
describe.skip('Message methods', () => {
    it('sends a message json', async () => {
        const message = await client.generateMessage();

        const jsonMessageId = await client.postMessageJson(message);

        expect(jsonMessageId).toBeValidMessageId();
    });

    it('gets message children', async () => {
        const message = await client.generateMessage();
        const messageId = await client.messageId(message);

        const messageChildren = await client.getMessageChildren(messageId);

        expect(messageChildren).toBeDefined();
    });

    it('finds messages by message IDs', async () => {
        const messageIds = await client.getTips();

        const messages = await client.findMessages(messageIds);

        expect(messages.length).toBe(messageIds.length);
    });

    // TODO: Error: 404 message not found. However, if calling getMessageData/metadata
    // on the same ID, the message is found
    it.skip('gets raw message', async () => {
        const message = await client.generateMessage();
        const messageId = await client.messageId(message);

        const messageRaw = await client.getMessageRaw(messageId);

        expect(messageRaw).toBeDefined();
    });
});
