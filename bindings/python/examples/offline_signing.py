# Copyright 2021 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_client
import time
import os
import hashlib

# Create the Client object in offline mode
client_offline = iota_client.Client(offline=True)

# Create the Client object in online mode
client = iota_client.Client(
    nodes_name_password=[['https://api.lb-0.h.chrysalis-devnet.iota.cafe']])

# Random seed (NOTE: DO NOT USE THIS for production)
rnd_seed = hashlib.sha256(os.urandom(256)).hexdigest()


def generate_addresses():
    """Return the generate addresses in offline mode."""

    print('Start to generate addresses...')

    # Generate addresses offline
    addresses_index = client_offline.get_addresses(
        rnd_seed, input_range_begin=0, input_range_end=10, bech32_hrp="atoi")

    # Get the addresses only
    addresses = [a[0] for a in addresses_index]

    print(f'Generated offline addresses:\n{addresses}')

    return addresses


def transaction_preparation(addresses):
    """Return the prepared transaction data."""

    print("Start to prepare the transaction...")

    # Address to which we want to send the amount
    address = "atoi1qruzprxum2934lr3p77t96pzlecxv8pjzvtjrzdcgh2f5exa22n6gek0qdq"
    amount = 1_000_000

    inputs = client.find_inputs(addresses, amount)
    print(f'Inputs: {inputs}')

    # Prepare transaction
    prepared_transaction_data = client.prepare_transaction(
        inputs=inputs, outputs=[{'address': address, 'amount': amount}])

    print(f'Prepared transaction data {prepared_transaction_data}')

    return prepared_transaction_data


def transaction_signing(prepared_transaction_data):
    """Return the singed transaction."""

    print("Start to sign the transaction...")

    # Sign prepared transaction offline
    signed_transaction = client_offline.sign_transaction(
        prepared_transaction_data=prepared_transaction_data, seed=rnd_seed, start_index=0, end_index=100)

    print(f'Singed transaction: {signed_transaction}')

    return signed_transaction


def message_sending(signed_transaction):
    """Send the message"""

    print("Start to send the message...")

    # Send offline signed transaction online
    message = client.finish_message(signed_transaction)
    print(f'Sent message: {message}')
    message_id = message['message_id']

    print(
        f'Transaction sent: https://explorer.iota.org/devnet/message/{message_id}')


if __name__ == '__main__':
    addresses = generate_addresses()

    address = addresses[0]
    print(f'Please send tokens to {address}')
    print('Websites to send tokens: https://faucet.chrysalis-devnet.iota.cafe/ or https://faucet.tanglekit.de/')
    print(
        f'After sending tokens, please check https://explorer.iota.org/devnet/addr/{address}')
    input(
        f'Press Enter to continue after the address already gets tokens successfully...')

    prepared_transaction_data = transaction_preparation(addresses)
    signed_transaction = transaction_signing(prepared_transaction_data)
    message_sending(signed_transaction)
