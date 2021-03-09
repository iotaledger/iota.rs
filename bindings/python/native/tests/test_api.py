# Copyright 2021 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_client
import os

TEST_NODE_URL = "https://api.lb-0.testnet.chrysalis2.com"

# NOTE! Load the seed from your env path instead
# NEVER assign the seed directly in your codes!
# DO NOT USE THIS!!
# SEED = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2"

# USE THIS INSTEAD
SEED = os.getenv('NONSECURE_USE_OF_DEVELOPMENT_SEED_1')
EMPTY_ADDRESS = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
client = iota_client.Client(node=TEST_NODE_URL)

def test_get_health():
    health = client.get_health()
    assert isinstance(health, bool)

def test_get_info():
    node_info = client.get_info()
    assert isinstance(node_info, dict)

def test_get_tips():
    tips = client.get_tips()
    assert isinstance(tips, list)

def test_get_addresses():
    address_changed_list = client.get_addresses(
        seed=SEED, account_index=0, input_range_begin=0, input_range_end=10, get_all=True)

    assert isinstance(address_changed_list, list)
    # Get the (address, changed ) for the first found address
    address, changed = address_changed_list[0]
    balance = client.get_address_balance(address)
    assert isinstance(balance, dict)
    
def test_get_address_balance():
    balance = client.get_address_balance(EMPTY_ADDRESS)
    assert isinstance(balance, dict)

def test_get_address_outputs():
    outputs = client.get_address_outputs(EMPTY_ADDRESS)
    assert isinstance(outputs, list)

def test_indexation_with_int_list_data():
    message_id_indexation = client.message(
        index="Hello", data=[84, 97, 110, 103, 108, 101])
    assert isinstance(message_id_indexation, dict)
    
def test_indexation_with_data_str():
    message_id_indexation = client.message(
        index="Hi", data_str="Tangle")
    assert isinstance(message_id_indexation, dict)
    
def test_get_message_index():
    message_id_indexation_queried = client.get_message_index("Hello")
    assert isinstance(message_id_indexation_queried, list)

def test_find_messages():
    messages = client.find_messages(indexation_keys=["Hello"])
    assert isinstance(messages, list)

def test_get_unspent_address():
    unspent_addresses = client.get_unspent_address(seed=SEED)
    assert isinstance(unspent_addresses, tuple)

def test_get_balance_in_seed():
    balance = client.get_balance(seed=SEED)
    assert isinstance(balance, int)