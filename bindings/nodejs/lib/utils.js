// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

function promisify(fn) {
  return function () {
    return new Promise((resolve, reject) =>
      fn.apply(this, [
        ...Array.from(arguments),
        (err, data) => {
          if (err) {
            reject(err);
          } else {
            resolve(data);
          }
        },
      ]),
    );
  };
}

class RemainderValueStrategy {
  changeAddress() {
    return {
      strategy: 'ChangeAddress',
      value: null,
    };
  }

  reuseAddress() {
    return {
      strategy: 'ReuseAddress',
      value: null,
    };
  }

  accountAddress(address) {
    return {
      strategy: 'AccountAddress',
      value: address,
    };
  }
}

class OutputKind {
  constructor() {}

  static signatureLockedSingle() {
    return 'SignatureLockedSingle';
  }

  static signatureLockedDustAllowance() {
    return 'SignatureLockedDustAllowance';
  }
}

module.exports.promisify = promisify;
module.exports.RemainderValueStrategy = new RemainderValueStrategy();
module.exports.OutputKind = OutputKind;
