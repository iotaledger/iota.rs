# Copyright 2021 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_client
import json
import os

# Read the test vector
tv = dict()
with open('../../../tests/fixtures/test_vectors.json') as json_file:
    tv = json.load(json_file)
tv = tv['python']

client = iota_client.Client(nodes_name_password=[[tv['NODE_URL']]])


def test_get_health():
    health = client.get_health()
    assert isinstance(health, bool)


def test_get_info():
    node_info = client.get_info()
    assert isinstance(
        node_info, dict) and 'is_healthy' in node_info['nodeinfo']


def test_get_peers():
    try:
        # If the peers can be accessed, then the list of peers should return
        peers = client.get_peers()
        assert isinstance(peers, list)
    except ValueError as e:
        # Else the error must be access forbidden
        assert "status code 400" in str(e)


def test_get_tips():
    tips = client.get_tips()
    assert isinstance(tips, list)


def test_post_message():
    message = client.message(
        index=tv['INDEXATION']['INDEX'][0], data=tv['INDEXATION']['DATA'][0])
    message_id = client.post_message(message)
    assert isinstance(message_id, str) and len(
        message_id) == tv['MESSAGE_ID_LENGTH']


def test_get_output():
    try:
        output = client.get_output(tv['UTXOINPUT'][0])
        assert isinstance(output, dict) and 'output' in output
    except ValueError as e:
        assert 'output not found' in str(e)


def test_get_address_balance():
    balance = client.get_address_balance(tv['ADDRESS'][0])
    assert isinstance(balance, dict) and 'balance' in balance


def test_get_address_outputs():
    outputs = client.get_address_outputs(tv['ADDRESS'][0])
    assert isinstance(outputs, list)


def test_find_outputs():
    outputs = client.find_outputs(addresses=[tv['ADDRESS'][0]])
    assert isinstance(outputs, list)
    if len(outputs) > 0:
        assert 'message_id' in outputs[0]


def test_get_milestone():
    try:
        # If the milestone can be found, then a milestone dict should be returned
        milestone = client.get_milestone(1000)
        assert isinstance(milestone, dict) and 'message_id' in milestone
    except ValueError as e:
        # Else the error must be milestone not found or forbidden
        assert "milestone not found" in str(e) or "Forbidden" in str(e)


def test_get_milestone_utxo_changes():
    try:
        # If the milestone can be found, then a milestone utxo dict should be returned
        milestone_utxo = client.get_milestone_utxo_changes(1000)
        assert isinstance(
            milestone_utxo, dict) and 'consumed_outputs' in milestone_utxo
    except ValueError as e:
        # Else the error must be milestone not found
        assert ("load milestone diff for index" in str(
            e) and "key not found" in str(e)) or "Forbidden" in str(e)


def test_get_receipts():
    try:
        # If the receipts can be accessed, then the list of receipts should return
        receipt = client.get_receipts()
        assert isinstance(receipt, list)
    except ValueError as e:
        # Else the error must be access forbidden
        assert "Forbidden" in str(e)


def test_get_receipts_migrated_at():
    try:
        # If the receipts can be accessed, then the list of receipts should return
        receipts = client.get_receipts_migrated_at(100)
        assert isinstance(receipts, list)
    except ValueError as e:
        # Else the error must be access forbidden
        assert "Forbidden" in str(e)


def test_get_treasury():
    treasury_response = client.get_treasury()
    assert isinstance(treasury_response,
                      dict) and 'milestone_id' in treasury_response


def test_get_included_message():
    transaction_id = tv['TRANSACTION_ID'][0]
    try:
        response = client.get_included_message(transaction_id)
    except ValueError as e:
        assert "transaction not found" in str(e)
