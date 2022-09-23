// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** Secret manager that uses a Ledger Nano hardware wallet or Speculos simulator. */
export interface LedgerNanoSecretManager {
    /** boolean indicates whether it's a simulator or not. */
    ledgerNano: boolean;
}

/** Secret manager that uses a mnemonic in plain memory. It's not recommended for production use. Use LedgerNano or Stronghold instead.. */
export interface MnemonicSecretManager {
    mnemonic: string;
}

/** Secret manager that uses Stronghold. */
export interface StrongholdSecretManager {
    stronghold: {
        password?: string;
        snapshotPath: string;
    };
}

/** Supported secret managers */
export type SecretManager =
    | LedgerNanoSecretManager
    | MnemonicSecretManager
    | StrongholdSecretManager;
