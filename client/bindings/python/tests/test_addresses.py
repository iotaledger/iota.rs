
# Copyright 2022 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_client import IotaClient, MnemonicSecretManager
import json

# Read the test vector
tv = dict()
with open('../../tests/fixtures/test_vectors.json') as json_file:
    tv = json.load(json_file)

client = IotaClient()

def test_mnemonic_address_generation():
    mnemonic_address_test_cases = tv['general']['address_generations']

    for test in mnemonic_address_test_cases:
        secret_manager = MnemonicSecretManager(test['mnemonic'])
        
        generated_address = client.generate_addresses(secret_manager, 
            account_index=test['account_index'],
            start=test['address_index'],
            end=test['address_index']+1,
            internal=test['internal'],
            coin_type = test['coin_type'],
            bech32_hrp = test['bech32_hrp'],
        )

        assert test['bech32_address'] == generated_address[0]
