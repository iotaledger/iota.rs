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
        const messageId = await client.postMessage(message);

        const messageRaw = await client.getMessageRaw(messageId);

        expect(messageRaw).toBeDefined();
    });

    it('promotes a message', async () => {
        const message = await client.generateMessage();
        const messageId = await client.postMessage(message);

        // Promote a message without checking if it should be promoted
        const promoteUnchecked = await client.promoteUnchecked(messageId);

        expect(promoteUnchecked[1].parentMessageIds).toContain(messageId);

        // Returns expected error: no need to promote or reattach.
        await expect(client.promote(messageId)).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });

    it('reattaches a message', async () => {
        const message = await client.generateMessage();
        const messageId = await client.postMessage(message);

        // Reattach a message without checking if it should be reattached
        const reattachUnchecked = await client.reattachUnchecked(messageId);

        expect(reattachUnchecked[0]).toBeValidMessageId();
        expect(reattachUnchecked[1]).toBeDefined();

        // Returns expected error: no need to promote or reattach.
        await expect(client.reattach(messageId)).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });

    // Skip by default, retryUntilIncluded can be slow
    it.skip('retries a message', async () => {
        const message = await client.generateMessage();
        const messageId = await client.postMessage(message);

        // Retries (promotes or reattaches) a message for provided message id until it's included
        // (referenced by a milestone). Default interval is 5 seconds and max attempts is 40.
        const retryUntilIncluded = await client.retryUntilIncluded(
            messageId,
            2,
            5,
        );
        //Returns the included message at first position and additional reattached messages
        expect(retryUntilIncluded[0][0]).toBe(messageId);

        // Returns expected error: no need to promote or reattach.
        await expect(client.retry(messageId)).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });
});
