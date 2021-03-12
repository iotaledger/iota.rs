# Copyright 2021 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_client
import json
import os

# Read the test vector
tv = dict()
with open('../../../fixtures/test_vectors.json') as json_file:
    tv = json.load(json_file)

client = iota_client.Client(node=tv['NODE_URL'])
message_id_indexation = tv['MESSAGE_ID'][0]


def test_message():
    global message_id_indexation
    message_id_indexation_dict = client.message(
        index=tv['INDEXATION']['INDEX'][0], data=tv['INDEXATION']['DATA'][0])
    assert isinstance(message_id_indexation_dict['message_id'], str)

    # Update the global message_id_indexation
    message_id_indexation = message_id_indexation_dict['message_id']


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


def test_find_messages():
    messages = client.find_messages(indexation_keys=[tv['INDEXATION']['INDEX'][2]])
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
