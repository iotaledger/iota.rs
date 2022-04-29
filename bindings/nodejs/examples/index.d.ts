// Implementation notes:
// Types like SecretManager need to be passed to the message handler as a string
// Passing Stringified<SecretManager> to the method will not typerror, but any other
// string will.

// Caveat: Passing a SecretManager that hasn't been stringified will not typeerror,
// but it will return an error from the message handler: `invalid type: map, expected a string`

type Stringified<T> = string & {
    [P in keyof T]: T[P];
};

interface JSON {
    stringify<T>(value: T): string & Stringified<T>;
}
