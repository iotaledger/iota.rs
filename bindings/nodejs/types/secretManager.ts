// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** Secret manager that uses a Ledger Nano hardware wallet or Speculos simulator. */
export interface LedgerNanoSecretManager {
    /** boolean indicates whether it's a simulator or not. */
    LedgerNano: boolean;
}

/** Secret manager that uses only a mnemonic. */
export interface MnemonicSecretManager {
    Mnemonic: string;
}

/** Secret manager that uses Stronghold. */
export interface StrongholdSecretManager {
    Stronghold: {
        password?: string;
        snapshotPath: string;
    };
}

/** Supported secret managers */
export type SecretManager =
    | LedgerNanoSecretManager
    | MnemonicSecretManager
    | StrongholdSecretManager;
