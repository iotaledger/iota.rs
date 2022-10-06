import { printExpected, printReceived, matcherHint } from 'jest-matcher-utils';

interface CustomMatchers<R = unknown> {
    toBeValidAddress(): R;
    toBeValidBlockId(): R;
    toBeValidOutputId(): R;
}

declare global {
    // eslint-disable-next-line @typescript-eslint/no-namespace
    namespace jest {
        interface Expect extends CustomMatchers {}
        interface Matchers<R> extends CustomMatchers<R> {}
        interface InverseAsymmetricMatchers extends CustomMatchers {}
    }
}

const failMessage =
    (received: string, length: number, prefix: string, not: boolean) => () =>
        `${matcherHint(
            `${not ? '.not' : ''}.toHaveLengthAndPrefix`,
            'received',
            'length, prefix',
        )}
Expected${not ? ' not' : ''}:
  length: ${printExpected(length)}
  prefix: ${printExpected(prefix)}
Received: 
  length: ${printReceived(received.length)}
  prefix: ${printReceived(received.slice(0, prefix.length))}`;

const idMatcher = (received: string, length: number, prefix: string) => {
    const pass = received.length === length && received.startsWith(prefix);
    return {
        message: failMessage(received, length, prefix, pass),
        pass,
    };
};

expect.extend({
    toBeValidAddress(received: string) {
        return idMatcher(received, 63, 'rms');
    },
    toBeValidBlockId(received: string) {
        return idMatcher(received, 66, '0x');
    },
    toBeValidOutputId(received: string) {
        return idMatcher(received, 70, '0x');
    },
});
