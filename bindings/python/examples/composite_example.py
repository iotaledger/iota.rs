import os
from urllib.request import urlopen as uReq
from urllib.error import HTTPError
import iota_client
import sqlite3


def consolidation():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    address = client.consolidate_funds(seed, 0, 0, 150)
    print(f'Funds consolidated to {address}')


def create_max_dust():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    seed_2 = os.getenv('MY_IOTA_SEED_2')
    new_addresses = client.get_addresses(
        seed_2, input_range_begin=0, input_range_end=1)
    print(f'New addresses: {new_addresses}')

    try:
        dust_allowance_message = client.message(seed=seed, dust_allowance_outputs=[
            {'address': new_addresses[0][0],
             'amount': 10_000_000}])

        message_id = dust_allowance_message['message_id']
        print(f'Message id: {message_id}')

        client.retry_until_included(message_id)

        # Split funds to own addresses
        addresses = client.get_addresses(
            seed, input_range_begin=1, input_range_end=101)
        outputs = []
        for address in addresses:
            outputs.append(
                {'address': address[0],
                 'amount': 1_000_001})

        message = client.message(seed=seed, outputs=outputs)
        message_id = message['message_id']
        print(
            f'First transaction sent: https://explorer.iota.org/devnet/message/{message_id}')
        client.retry_until_included(message_id)

        # At this point we have 100 Mi on 100 addresses and we will just send it to the final address
        # We use the outputs directly so we don't double spend them
        initial_outputs = []
        for index, output in enumerate(message['payload']['transaction']['essence']['outputs']):
            # Only include 1 Mi outputs, otherwise it fails for the remainder address
            if output['signature_locked_single']['amount'] == 1_000_001:
                transaction_id = message['payload']['transaction']['essence']['inputs'][0]['transaction_id']
                initial_outputs.append(
                    {'transaction_id': transaction_id.encode('ascii'), 'index': index})

        first_address_old_seed = client.get_addresses(
            seed, input_range_begin=0, input_range_end=1)
        for i, output in enumerate(initial_outputs):
            message = client.message(seed, inputs=[output[i]], input_range_begin=1, input_range_end=101,  outputs=[
                {'address': new_addresses[0][0], 'amount': 1},
                {'address': first_address_old_seed[0][0], 'amount': 1_000_000}])
            message_id = message['message_id']
            print(
                f'Transaction {i} sent: https://explorer.iota.org/devnet/message/{message_id}')
            client.retry_until_included(message_id)

        # Send all funds back to first address
        total_balance = client.get_balance(seed)
        print(f'Total balance: {total_balance}')

        message = client.message(seed=seed, outputs=[
            {'address': first_address_old_seed[0][0],
             'amount': total_balance}])
        message_id = message['message_id']

        print(
            f'Final tx sent: https://explorer.iota.org/devnet/message/{message_id}')
        iota.retry_until_included(message_id)

    except ValueError as e:
        print(e)
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


def custom_inputs():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    addresses = client.get_addresses(
        seed, input_range_begin=0, input_range_end=1)
    print(f'addresses: {addresses}')
    outputs = client.get_address_outputs(addresses[0][0])
    print(f'outputs: {outputs}')
    try:
        message = client.message(seed=seed, inputs=[outputs], outputs=[
            {'address': 'atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r',
                                 'amount': 1000}])
        message_id = message['message_id']
        print(
            f'Transaction sent: https://explorer.iota.org/devnet/message/{message_id}')
    except:
        print(f'Please send tokens to {addresses[0][0]}')
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


def custom_parent():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])
    parent = 'b5634e05a7c665d7f87330a53633f001a5d1d96b346dc98dc225c4d6c204f23b'

    try:
        message = client.message(parents=[parent])
        message_id = message['message_id']
        print(
            f'Empty message sent: https://explorer.iota.org/devnet/message/{message_id}')
    except:
        print('Please select a valid parent message id')


def custom_payload():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    indexation_key = 'My indexation payload key'
    indexation_data = 'My indexation payload Data'
    message = client.message(index=indexation_key, data_str=indexation_data)
    print(f'The sent message: {message}')


def dust():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    try:
        message = client.message(seed=seed, dust_allowance_outputs=[
            {'address': 'atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf',
             'amount': 1_000_000}],
            outputs=[{'address': 'atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf',
                      'amount': 1}])
        message_id = message['message_id']
        print(
            f'First transaction sent: https://explorer.iota.org/devnet/message/{message_id}')
    except ValueError as e:
        print(e)
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


def get_fund():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    addresses = client.get_addresses(
        seed, account_index=0, input_range_begin=0, input_range_end=1)
    print(f'Addresses: {addresses}')
    address = addresses[0][0]

    try:
        my_url = f'https://faucet.chrysalis-devnet.iota.cafe/api?address={address}'
        print(f'my_url: {my_url}')

        uClient = uReq(my_url)
        response = uClient.read()
        print(response)
        uClient.close()
    except HTTPError as e:
        print(e)
        print('Please try it after 60 secs')


def indexation():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    indexation_key = 'Hello'
    indexation_data = 'Tangle'
    message = client.message(index=indexation_key, data_str=indexation_data)

    fetched_message_ids = client.get_message_index('Hello')
    print(f'Fetched message ids: {fetched_message_ids}')

    fetched_message_data = client.get_message_data(fetched_message_ids[0])
    print(f'Fetched message data: {fetched_message_data}')


def message_time():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    indexation_key = 'Hello'
    indexation_data = 'Tangle'
    message = client.message(index=indexation_key, data_str=indexation_data)
    print(message)

    message_id = message['message_id']
    print(f'Message id: {message_id}')

    client.retry_until_included(message_id)
    metadata = client.get_message_metadata(message_id)
    print(f'Metadata: {metadata}')

    milestone_index = metadata['milestone_index']
    print(f'Milestone index: {milestone_index}')
    if not (milestone_index is None):
        milestone = client.get_milestone(milestone_index)
        print(f'Message got referenced by milestone {milestone}')
    else:
        print('Message is not referenced by a milestone')


def mnemonic():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    mnemonic = client.generate_mnemonic()
    print(f'Generated mnemonic: {mnemonic}')

    seed = client.mnemonic_to_hex_seed(mnemonic)
    addresses = client.get_addresses(
        seed, input_range_begin=0, input_range_end=2)
    print(f'List of generated public addresses: {addresses}')


def multiple_outputs():

    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    # Seed must contain non-zero balance
    seed = os.getenv('MY_IOTA_SEED')
    try:
        message = client.message(seed,
                                 outputs=[{'address': 'atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf',
                                           'amount': 3_000_000},
                                          {'address': 'atoi1qz4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337m3lwl2',
                                           'amount': 2_800_000},
                                          {'address': 'atoi1qzumqjtucwglfja746vvmr7n54ep88kcu2qvaquqrnx9qs2z8f4t6d7muyq',
                                           'amount': 3_000_000}])
        message_id = message['message_id']
        print(
            f'Transaction sent: https://explorer.iota.org/devnet/message/{message_id}')
    except ValueError as e:
        print(e)
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


def peer():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])
    try:
        peers = client.get_peers()
        print(f'Peers: {peers}')
    except ValueError as e:
        print(e)
        print("You don't have the permission to get the peers.")


def quorum():
    node_1 = 'https://api.lb-0.h.chrysalis-devnet.iota.cafe'
    node_2 = 'https://api.lb-1.h.chrysalis-devnet.iota.cafe'
    node_3 = 'https://api.lb-0.h.chrysalis-devnet.iota.cafe'
    try:
        client = iota_client.Client(nodes_name_password=[
                                    [node_1], [node_2], [node_3]], quorum=True, quorum_size=3, quorum_threshold=66)

        seed = os.getenv('MY_IOTA_SEED')

        seed_balance = client.get_balance(seed)
        print(f'Account balance: {seed_balance}')
    except ValueError as e:
        print(e)
        print('Please provide enough healthy nodes.')


def search_address():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])
    seed = os.getenv('MY_IOTA_SEED')
    address = client.get_addresses(
        seed, input_range_begin=9, input_range_end=10)[0][0]
    print(f'Address: {address}')

    info = client.get_info()
    bech32_hrp = info['nodeinfo']['bech32_hrp']
    searched_address = client.search_address(
        seed, bech32_hrp, 0, 0, 10, address)

    print(
        f'Address index: {searched_address[0]}\nIs internal address: {searched_address[1]}')


def send_all():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    seed_2 = os.getenv('MY_IOTA_SEED_2')

    total_balance = client.get_balance(seed, account_index=0)
    print(f'Total balance: {total_balance}')

    try:
        address = client.get_addresses(
            seed_2, input_range_begin=0, input_range_end=1)[0][0]
        print(f'Address: {address}')
        message = client.message(seed_2, outputs=[
            {'address': address,
             'amount': total_balance}])
        message_id = message['message_id']
        print(
            f'Transaction sent: https://explorer.iota.org/devnet/message/{message_id}')
        client.retry_until_included(message_id)
    except ValueError as e:
        print(e)
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


def split_all():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    seed_2 = os.getenv('MY_IOTA_SEED_2')

    total_balance = client.get_balance(seed)
    print(f'Total balance: {total_balance}')

    if total_balance == 0:
        print('Addresses belonging to the seed should contain tokens!')
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')
        return

    available = total_balance

    # Get the ceiling of the input range end
    input_range_end = int((total_balance+999_999)/1_000_000)
    addresses_from_seed_2 = client.get_addresses(
        seed_2, input_range_begin=0, input_range_end=input_range_end)

    outputs = []
    for i in range(input_range_end):
        amount = 1_000_000

        # Don't add more than we have or is allowed; One less here for remaining iotas
        if available == 0 or i > 125:
            break

        available -= amount
        # Add last amount so we don't create dust
        if available < amount:
            amount += available
            available = 0
        outputs.append({'address': addresses_from_seed_2[i], 'amount': amount})

    message = client.message(seed, outputs=outputs)
    message_id = message['message_id']
    print(
        f'Transaction sent: https://explorer.iota.org/devnet/message/{message_id}')


def split_outputs_single_address():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')

    # Split funds to own addresses

    addresses = client.get_addresses(
        seed, input_range_begin=1, input_range_end=101)

    outputs = []
    for output in addresses:
        address = output[0]
        outputs.append({'address': address, 'amount': 1_000_000})
    print(f'Outputs: {outputs}')

    try:
        message = client.message(seed, outputs=outputs)
        message_id = message['message_id']

        print(
            f'First transaction sent: https://explorer.iota.org/devnet/message/{messag_id}')
        client.retry_until_included(message_id)

        # At this point we have 100 Mi on 100 addresses and we will just send it to the final address
        # We use the outputs directly so we don't double spend them
        initial_outputs = []
        for index, output in enumerate(message['payload']['transaction']['essence']['outputs']):
            if output['signature_locked_single']['amount'] == 1_000_000:
                transaction_id = message['payload']['transaction']['essence']['inputs'][0]['transaction_id']
                initial_outputs.append(
                    {'transaction_id': transaction_id.encode('ascii'), 'index': index})

        for i, output in enumerate(initial_outputs):
            message = client.message(seed, inputs=[output[i]], input_range_begin=1, input_range_end=101,  outputs=[
                {'address': 'atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r', 'amount': 1_000_000}])
            message_id = message['message_id']
            print(
                f'Transaction {i} sent: https://explorer.iota.org/devnet/message/{message_id}')
            client.retry_until_included(message_id)

    except ValueError as e:
        print(e)
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


def storage():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    indexation_key = 'Hello'
    indexation_data = 'Tangle'
    message = client.message(index=indexation_key, data_str=indexation_data)

    message_id = message['message_id']
    db = 'my-storage.db'

    con = sqlite3.connect(db)

    cur = con.cursor()

    # Create table
    cur.execute('''CREATE TABLE message_ids (message_id text)''')

    # Insert a row of data
    cur.execute(f"INSERT INTO message_ids VALUES ('{message_id}')")

    # Save (commit) the changes
    con.commit()

    # We can also close the connection if we are done with it.
    # Just be sure any changes have been committed or they will be lost.
    con.close()

    # Connect to the database and get the message ids
    con = sqlite3.connect(db)
    cur = con.cursor()

    for row in cur.execute('SELECT * FROM message_ids'):
        print(f'Message ID from storage: {row[0]}')


def transaction():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    seed_2 = os.getenv('MY_IOTA_SEED_2')

    addresses_seed_2 = client.get_addresses(
        seed_2, input_range_begin=0, input_range_end=1)
    address = addresses_seed_2[0][0]

    try:
        # Send the first transaction
        message = client.message(
            seed, outputs=[{'address': address, 'amount': 3_000_000}])
        message_id = message['message_id']
        print(
            f'First transaction sent: https://explorer.iota.org/devnet/message/{message_id}')

        client.retry_until_included(message_id)

        # Send the second transaction
        addresses_seed_2 = client.get_addresses(
            seed_2, input_range_begin=1, input_range_end=2)
        address = addresses_seed_2[0][0]

        message = client.message(
            seed, outputs=[{'address': address, 'amount': 3_000_000}])

        message_id = message['message_id']
        print(
            f'Second transaction sent: https://explorer.iota.org/devnet/message/{message_id}')

        client.retry_until_included(message_id)

        # Send the third transaction
        addresses_seed_2 = client.get_addresses(
            seed_2, input_range_begin=2, input_range_end=3)
        address = addresses_seed_2[0][0]

        message = client.message(
            seed, outputs=[{'address': address, 'amount': 3_000_000}])

        message_id = message['message_id']
        print(
            f'Third transaction sent: https://explorer.iota.org/devnet/message/{message_id}')

        client.retry_until_included(message_id)

        # Send the last transaction
        outputs = []
        outputs.append({'address': client.get_addresses(
            seed, input_range_begin=1, input_range_end=2)[0][0], 'amount': 3_000_000})
        outputs.append({'address': client.get_addresses(
            seed, input_range_begin=2, input_range_end=3)[0][0], 'amount': 3_000_000})

        client.message(seed_2, outputs=outputs)

        message_id = message['message_id']
        print(
            f'Last transaction sent: https://explorer.iota.org/devnet/message/{message_id}')

        client.retry_until_included(message_id)

        message_metadata = client.get_message_metadata(message_id)
        ledger_inclusion_state = message_metadata['ledger_inclusion_state']
        print(f'Ledger Inclusion State: {ledger_inclusion_state}')

    except ValueError as e:
        print(e)
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


def txspam():
    node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"
    client = iota_client.Client(nodes_name_password=[[node_url]])

    seed = os.getenv('MY_IOTA_SEED')
    # Split funds to own addresses
    addresses = client.get_addresses(
        seed, account_index=0, input_range_begin=0, input_range_end=10)

    outputs = []
    for output in addresses:
        outputs.append({'address': output[0], 'amount': 1_000_000})
    print(f'Outputs: {outputs}')

    try:
        message = client.message(seed, outputs=outputs)
        print(message)
        message_id = message['message_id']
        print(
            f'First transaction sent: https://explorer.iota.org/devnet/message/{message_id}')

        client.retry_until_included(message_id)

        # At this point we have 10 Mi on 10 addresses and we will just send it to their addresses again
        # Use own outputs directly so we don't double spend them
        initial_outputs = []
        for index, initial_output in enumerate(message['payload']['transaction']['essence']['outputs']):
            transaction_id = message['payload']['transaction']['essence']['inputs'][0]['transaction_id']
            initial_outputs.append(
                {'transaction_id': transaction_id.encode('ascii'), 'index': index})

        for i, output in enumerate(addresses):
            message = client.message(seed, inputs=initial_outputs[i], outpus=[
                {'address': output[0], 'amount': 1_000_000}])
            message_id = message['message_id']
            print(
                f'Transaction sent: https://explorer.iota.org/devnet/message/{message_id}')

    except ValueError as e:
        print(e)
        print('Website to get test tokens: https://faucet.chrysalis-devnet.iota.cafe/')


if __name__ == '__main__':
    """Please uncomment the example function to use it.
    """
    consolidation()
    # create_max_dust()
    # custom_inputs()
    # custom_parent()
    # custom_payload()
    # dust()
    # get_fund()
    # indexation()
    # message_time()
    # mnemonic()
    # multiple_outputs()
    # peer()
    # quorum()
    # search_address()
    # send_all()
    # split_all()
    # split_outputs_single_address()
    # storage()
    # transaction()
    # txspam()
