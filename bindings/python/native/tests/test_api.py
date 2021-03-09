# Copyright 2021 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_client
import os

TEST_NODE_URL = "https://api.lb-0.testnet.chrysalis2.com"

# NOTE! Load the seed from your env path instead
# NEVER assign the seed directly in your codes!
# DO NOT USE THIS!!:
# SEED = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2"

# USE THIS INSTEAD
SEED = os.getenv('NONSECURE_USE_OF_DEVELOPMENT_SEED_1')
EMPTY_ADDRESS = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
client = iota_client.Client(node=TEST_NODE_URL)

def test_get_health():
    health = client.get_health()

def test_get_info():
    node_info = client.get_info()

def test_get_tips():
    tips = client.get_tips()

def test_get_addresses():
    address_changed_list = client.get_addresses(
        seed=SEED, account_index=0, input_range_begin=0, input_range_end=10, get_all=True)

    # Get the (address, changed ) for the first found address
    address, changed = address_changed_list[0]
    balance = client.get_address_balance(address)
    
def test_get_address_balance():
    balance = client.get_address_balance(EMPTY_ADDRESS)

def test_get_address_outputs():
    outputs = client.get_address_outputs(EMPTY_ADDRESS)

def test_indexation_with_int_list_data():
    message_id_indexation = client.message(
        index="Hello", data=[84, 97, 110, 103, 108, 101])
    
def test_indexation_with_data_str():
    message_id_indexation = client.message(
        index="Hi", data_str="Tangle")
    
def test_get_message_index():
    message_id_indexation_queried = client.get_message_index("Hello")

def test_find_messages():
    messages = client.find_messages(indexation_keys=["Hello"])

def test_get_unspent_address():
    unspent_addresses = client.get_unspent_address(seed=SEED)

def test_get_balance_in_seed():
    balance = client.get_balance(seed=SEED)