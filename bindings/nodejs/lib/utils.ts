/** Convert UTF8 string to an array of bytes */
export const utf8ToBytes = (utf8: string) => {
    const utf8Encode = new TextEncoder();
    return Array.from(utf8Encode.encode(utf8));
};

/** Convert hex encoded string to UTF8 string */
export const hexToUtf8 = (hex: string) =>
    decodeURIComponent(hex.replace('0x', '').replace(/[0-9a-f]{2}/g, '%$&'));

/** Convert UTF8 string to hex encoded string */
export const utf8ToHex = (utf8: string) =>
    '0x' + Buffer.from(utf8, 'utf8').toString('hex');
