import iota_client
import os
LOCAL_NODE_URL = "http://0.0.0.0:14265"
# Warning!! Load the seed from your env path instead
# NEVER assign the seed directly in your codes!

# DO NOT USE THIS!!:
# SEED = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2"

# USE THIS INSTEAD
SEED = os.getenv('MY_IOTA_SEED')

ADDRESS_TEST = ["iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4",
                "iot1qxgamuxntdxq06q4zpmvmdnrerj2f94058ge3flfyx567unw25amvr978uw"]
client = iota_client.Client(
    node=LOCAL_NODE_URL, node_sync_disabled=True)
print(f'Health: {client.get_health()}')
print(f'Node Info: {client.get_info()}')
print(f'Tips: {client.get_tips()}')
print(
    f'find_addresses(): {client.find_addresses(seed=SEED, account_index=0, begin=0, end=10, get_all=True)}')
print(
    f'get_address_balance() for genesis address: {client.get_address_balance(ADDRESS_TEST[0])}')
print(
    f'get_address_balance() for empty address: {client.get_address_balance(ADDRESS_TEST[1])}')
print(
    f'get_address_outputs(): {client.get_address_outputs(ADDRESS_TEST[1])}')
message_id = client.send(
    seed=SEED, outputs=[{'address': ADDRESS_TEST[1], 'amount':100}])
print(f'send() to {ADDRESS_TEST[1]}\nmessage_id = {message_id}')
print(f'Check http://127.0.0.1:14265/api/v1/messages/{message_id}')
print(
    f'get_address_balance(): {client.get_address_balance(ADDRESS_TEST[1])}')
print(client.get_message_data.__doc__)
