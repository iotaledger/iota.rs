interface CustomMatchers<R = unknown> {
    toBeValidAddress(): R;
    toBeValidMessageId(): R;
}

declare global {
    // eslint-disable-next-line @typescript-eslint/no-namespace
    namespace jest {
        interface Expect extends CustomMatchers {}
        interface Matchers<R> extends CustomMatchers<R> {}
        interface InverseAsymmetricMatchers extends CustomMatchers {}
    }
}

expect.extend({
    toBeValidAddress(received: string) {
        const pass = received.length === 63 && received.slice(0, 3) === 'rms';
        if (pass) {
            return {
                message: () => `expected invalid address`,
                pass: true,
            };
        } else {
            return {
                message: () =>
                    `
                     expected length: 63
                     received: ${received.length}
                     expected bech32hrp: "rms"
                     received: "${received.slice(0, 3)}"
                    `,
                pass: false,
            };
        }
    },
    toBeValidMessageId(received: string) {
        const pass = received.length === 66 && received.slice(0, 2) === '0x';
        if (pass) {
            return {
                message: () => `expected invalid message ID`,
                pass: true,
            };
        } else {
            return {
                message: () =>
                    `
                     expected length: 66
                     received: ${received.length}
                     expected prefix: "0x"
                     received: "${received.slice(0, 2)}"
                    `,
                pass: false,
            };
        }
    },
});

export {};
