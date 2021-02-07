# IOTA Client Python Library

## Requirements
- Rust 1.45.0+
- Python 3.6+

## Try Run w/ Local Hornet

1. Run your local Hornet
- `$ git clone git@github.com:gohornet/hornet.git`
- checkout `chrysalis-pt2` branch
- Modify your `create_snapshot_alphanet.sh`, modify Line 14 to `go run ../main.go tool snapgen alphanet1 96f9de0989e77d0e150e850a5a600e83045fa57419eaf3b20225b763d4e23813 snapshots/alphanet1/full_export.bin`
- `$ ./run_coo_bootstrap.sh `

2. Build the iota-client-python library by yourself (Optional)
- **IMPORTANT**: Enable the `bindings/python/native` in your `Cargo.toml`
```toml
[workspace]
members = [
    "iota-core",
    "bindings/python/native",
    "iota-client",
    "examples",
]
``` 
- `$ cargo build --release`
- The built library is located in `target/release/`
- On MacOS, rename `libiota_client.dylib` to `iota_client.so`, on Windows, use `iota_client.dll` directly, and on Linux `libiota_client.so` to `iota_client.so`.
- Copy your renamed library to `bindings/python/examples/`
- Go to `binding/python/`
- `$ python example.py`

## Python Example
```python
import iota_client
import os
LOCAL_NODE_URL = "http://0.0.0.0:14265"

# NOTE! Load the seed from your env path instead
# NEVER assign the seed directly in your codes!
# DO NOT USE THIS!!:
# SEED = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2"

# USE THIS INSTEAD
SEED = os.getenv('MY_IOTA_SEED')

EMPTY_ADDRESS = "iot1qxgamuxntdxq06q4zpmvmdnrerj2f94058ge3flfyx567unw25amvr978uw"
client = iota_client.Client(
    node=LOCAL_NODE_URL, node_sync_disabled=True)


def main():
    print('get_health()')
    print(f'health: client.get_health()')

    print('get_info()')
    print(f'node_info: client.get_info()')

    print('get_tips()')
    print(f'tips: client.get_tips()')

    print('find_addresses')
    address_changed_list = client.find_addresses(
        seed=SEED, account_index=0, begin=0, end=10, get_all=True)
    print(f'address_changed list: {address_changed_list}')

    # Get the (address, changed ) for the first found address
    address, changed = address_changed_list[0]
    print(f'get_address_balance() for address {address}')
    print(f'balance: {client.get_address_balance(address)}')

    print(f'get_address_balance() for address {EMPTY_ADDRESS}')
    print(f'balance: {client.get_address_balance(EMPTY_ADDRESS)}')

    print(f'get_address_outputs() for address {EMPTY_ADDRESS}')
    print(f'outputs(): {client.get_address_outputs(EMPTY_ADDRESS)}')

    print(f'message() 100 tokens to address {EMPTY_ADDRESS}')
    message_id = client.message(
        seed=SEED, outputs=[{'address': EMPTY_ADDRESS, 'amount': 100}])
    print(f'Token sent with message_id: {message_id}')
    print(f'Please check http://127.0.0.1:14265/api/v1/messages/{message_id}')

    print(f'get_message_metadata() for message_id {message_id}')
    message_metadata = client.get_message_metadata(message_id)
    print(f'message_metadata: {message_metadata}')

    print(f'get_message_data() for message_id {message_id}')
    message_data = client.get_message_data(message_id)
    print(f'message_data: {message_data}')

    print(f'get_message_raw() for message_id {message_id}')
    message_raw = client.get_message_raw(message_id)
    print(f"message_raw: {bytearray(message_raw, 'utf-8')}")

    print(f'get_message_children() for message_id {message_id}')
    children = client.get_message_children(message_id)
    print(f"children: {children}")

    print(f'message() Indexation')
    message_id_indexation = client.message(
        index="Hello", data=bytes("Tangle", 'utf-8'))
    print(f'Indexation sent with message_id: {message_id_indexation}')
    print(
        f'Please check http://127.0.0.1:14265/api/v1/messages/{message_id_indexation}')

    print(f"get_message_index() for index 'Hello'")
    message_id_indexation_queried = client.get_message_index("Hello")
    print(f'Indexation: {message_id_indexation_queried}')

    print(f"find_messages() for indexation_keys = ['Hello']")
    messages = client.find_messages(indexation_keys=["Hello"])
    print(f'Messages: {messages}')

    print(f"get_unspent_address()")
    unspent_addresses = client.get_unspent_address(seed=SEED)
    print(f'(unspent_address, index): {unspent_addresses}')

    print(f"get_balance()")
    balance = client.get_balance(seed=SEED)
    print(f'balance: {balance}')

    addresses = []
    for address, _changed in address_changed_list:
        addresses.append(address)
    print(f"get_address_balances() for {addresses}")
    balances = client.get_address_balances(addresses)
    print(f'balances: {balance}')


if __name__ == "__main__":
    main()
```