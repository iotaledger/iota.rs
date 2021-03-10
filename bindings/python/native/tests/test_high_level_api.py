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

def test_indexation_with_int_list_data():
    message_id_indexation = client.message(
        index=tv['INDEXATION']['INDEX'][0], data=tv['INDEXATION']['DATA'][0])
    assert isinstance(message_id_indexation, dict)

def test_get_addresses():
    address_changed_list = client.get_addresses(
        seed=tv['NONSECURE_SEED'][0], account_index=0, input_range_begin=0, input_range_end=10, get_all=True)
    assert isinstance(address_changed_list, list)

    # Get the (address, changed ) for the first found address
    address, changed = address_changed_list[0]
    balance = client.get_address_balance(address)
    assert isinstance(balance, dict) and 'balance' in balance
    
def test_indexation_with_data_str():
    message_id_indexation = client.message(
        index=tv['INDEXATION']['INDEX'][1], data_str=tv['INDEXATION']['DATA_STRING'][0])
    assert isinstance(message_id_indexation, dict)
    
def test_get_message_index():
    message_id_indexation_queried = client.get_message_index(tv['INDEXATION']['INDEX'][0])
    assert isinstance(message_id_indexation_queried, list)

def test_find_messages():
    messages = client.find_messages(indexation_keys=tv['INDEXATION']['INDEX'])
    assert isinstance(messages, list)

def test_get_unspent_address():
    unspent_addresses = client.get_unspent_address(seed=tv['NONSECURE_SEED'][0])
    assert isinstance(unspent_addresses, tuple)

def test_get_balance_in_seed():
    balance = client.get_balance(seed=tv['NONSECURE_SEED'][1])
    assert isinstance(balance, int)