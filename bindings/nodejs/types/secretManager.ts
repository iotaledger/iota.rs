// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

export interface MnemonicSecretManager {
    Mnemonic: string;
}

export interface StrongholdSecretManager {
    Stronghold: {
        password?: string;
        snapshotPath?: string;
    };
}

export type SecretManager = MnemonicSecretManager | StrongholdSecretManager;
