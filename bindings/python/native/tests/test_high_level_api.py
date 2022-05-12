# Copyright 2021 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_client
import json
import os

# Read the test vector
tv = dict()
with open('../../../tests/fixtures/test_vectors.json') as json_file:
    tv = json.load(json_file)
general_tv = tv['general']
tv = tv['python']

client = iota_client.Client(nodes_name_password=[[tv['NODE_URL']]])
message_id_indexation = tv['MESSAGE_ID'][0]


def test_message():
    global message_id_indexation
    message_id_indexation_dict = client.message(
        index=tv['INDEXATION']['INDEX'][0], data=tv['INDEXATION']['DATA'][0])
    assert isinstance(message_id_indexation_dict['message_id'], str)

    # Update the global message_id_indexation
    message_id_indexation = message_id_indexation_dict['message_id']


def test_get_output_amount_and_address():
    output = tv['OUTPUT_DTO']
    try:
        client.get_output_amount_and_address(output)
    except ValueError as e:
        assert "Treasury output is no supported" in str(e)


def test_prepare_transaction():
    inputs = tv['OFFLINE_SIGNING']['inputs']
    outputs = tv['OFFLINE_SIGNING']['outputs']
    try:
        prepared_transaction_data = client.prepare_transaction(inputs, outputs)
    except ValueError as e:
        assert "The wallet account doesn't have enough balance" in str(e)


def test_sign_transaction():
    prepared_transaction_data = tv['OFFLINE_SIGNING']['prepared_transaction_data']
    seed = tv['NONSECURE_SEED'][0]
    try:
        signed_transaction = client.sign_transaction(
            prepared_transaction_data, seed, 0, 100)
    except ValueError as e:
        assert "not found in range" in str(e)


def test_finish_message():
    singed_transaction = tv['OFFLINE_SIGNING']['signed_transaction']
    try:
        client.finish_message(singed_transaction)
    except BaseException as e:
        assert "invalid SignatureLockedSingleOutput" in str(e)


def test_get_message_metadata():
    message_metadata = client.get_message_metadata(message_id_indexation)
    assert isinstance(message_metadata,
                      dict) and 'message_id' in message_metadata


def test_get_message_data():
    message_data = client.get_message_data(message_id_indexation)
    assert isinstance(message_data, dict) and 'message_id' in message_data


def test_get_message_raw():
    message_raw = client.get_message_raw(message_id_indexation)
    assert isinstance(message_raw, str)


def test_get_message_children():
    children = client.get_message_children(message_id_indexation)
    assert isinstance(children, list)


def test_get_message_index():
    message_index = client.get_message_index(message_id_indexation)
    assert isinstance(message_index, list)


def test_get_message_id():
    message_payload = tv['MESSAGE_PAYLOAD']
    json_payload = str(json.dumps(message_payload, indent=4))
    try:
        id = client.get_message_id(json_payload)
    except ValueError as e:
        assert "invalid message" in str(e)


def test_get_transaction_id():
    transaction_payload = tv['TRANSACTION_PAYLOAD']
    transaction_id = tv['TRANSACTION_PAYLOAD_ID']
    try:
        id = client.get_transaction_id(transaction_payload)
        assert id == transaction_id
    except ValueError as e:
        assert "invalid transaction" in str(e)


def test_find_messages():
    messages = client.find_messages(
        indexation_keys=[tv['INDEXATION']['INDEX'][2]])
    assert isinstance(messages, list)


def test_get_unspent_address():
    unspent_addresses = client.get_unspent_address(
        seed=tv['NONSECURE_SEED'][0])
    assert isinstance(unspent_addresses, tuple)


def test_get_addresses_and_get_address_balance():
    address_changed_list = client.get_addresses(
        seed=tv['NONSECURE_SEED'][0], account_index=0, input_range_begin=0, input_range_end=10, get_all=True)
    assert isinstance(address_changed_list, list)

    # Get the (address, changed ) for the first found address
    address, changed = address_changed_list[0]
    balance = client.get_address_balance(address)
    assert isinstance(balance, dict) and 'balance' in balance


def test_get_balance_in_seed():
    balance = client.get_balance(seed=tv['NONSECURE_SEED'][1])
    assert isinstance(balance, int)


def test_get_address_balances():
    addresses = [tv['ADDRESS'][0]]
    address_balance_pairs = client.get_address_balances(addresses)
    assert isinstance(addresses, list)


def test_generate_mnemonic():
    mnemonic = client.generate_mnemonic()
    assert isinstance(mnemonic, str)


def test_mnemonic_to_hex_seed():
    mnemonic = client.generate_mnemonic()
    seed = client.mnemonic_to_hex_seed(mnemonic)
    assert isinstance(seed, str)


def test_find_inputs():
    addresses = [tv['ADDRESS'][0]]
    amount = 100
    try:
        inputs = client.find_inputs(addresses, amount)
        assert isinstance(inputs, list)
    except ValueError as e:
        assert "The wallet account doesn't have enough balance." in str(e)


def test_bech32_to_hex():
    bech32 = tv['BECH32_HEX_PAIR']['bech32']
    hex_str = tv['BECH32_HEX_PAIR']['hex']
    result = client.bech32_to_hex(bech32)
    assert hex_str == result


def test_hex_to_bech32():
    bech32 = tv['BECH32_HEX_PAIR']['bech32']
    hex_str = tv['BECH32_HEX_PAIR']['hex']
    result = client.hex_to_bech32(hex_str)
    assert bech32 == result


def test_indexation_with_data_str():
    message_id_indexation = client.message(
        index=tv['INDEXATION']['INDEX'][1], data_str=tv['INDEXATION']['DATA_STRING'][0])
    assert isinstance(message_id_indexation, dict)


def test_is_address_valid():
    assert client.is_address_valid(tv['ADDRESS'][0]) == True
    assert client.is_address_valid(tv['ADDRESS'][1]) == False


def test_retry():
    try:
        retried_message = client.retry(message_id_indexation)
        # Should not be able to retry
        assert False
    except ValueError as e:
        assert "doesn't need to be promoted or reattached" in str(e)


def test_retry_until_included():
    try:
        result = client.retry_until_included(
            message_id_indexation, max_attempts=1)
        assert isinstance(result, list)
    except ValueError as e:
        assert "couldn't get included into the Tangle" in str(e)


def test_consolidate_funds():
    seed = tv['NONSECURE_SEED'][0]
    result = client.consolidate_funds(seed, 0, 0, 1)
    assert isinstance(result, str)


def test_search_address():
    seed = tv['NONSECURE_SEED'][0]
    address = bech32_address = tv['BECH32_ADDRESS']
    bech32_hrp = tv['BECH32_HRP']
    searched_address = client.search_address(
        seed, bech32_hrp, 0, 0, 10, address)
    assert isinstance(searched_address, tuple)


def test_reattach():
    try:
        retried_message = client.reattach(message_id_indexation)
        # Should not be able to reattach
        assert False
    except ValueError as e:
        assert "doesn't need to be promoted or reattached" in str(e)


def test_promote():
    try:
        retried_message = client.promote(message_id_indexation)
        # Should not be able to promote
        assert False
    except ValueError as e:
        assert "doesn't need to be promoted or reattached" in str(e)


def test_hex_public_key_to_bech32_address():

    hex_public_key = tv['HEX_PUBLIC_KEY']
    bech32_hrp = tv['BECH32_HRP']
    bech32_address = tv['BECH32_ADDRESS']
    converted_address = client.hex_public_key_to_bech32_address(
        hex_public_key, bech32_hrp)

    assert bech32_address == converted_address


def test_mnemonic_address_generation():
    mnemonic = general_tv['MNEMNONIC']
    mnemonic_address = general_tv['MNEMNONIC_ADDRESS']
    mnemonic_seed = client.mnemonic_to_hex_seed(mnemonic)

    generated_address, _ = client.get_addresses(
        seed=mnemonic_seed, account_index=0, input_range_begin=0, input_range_end=1, bech32_hrp="iota")[0]
    assert mnemonic_address == generated_address
