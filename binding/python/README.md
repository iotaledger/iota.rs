# IOTA Client Python Library

## Requirements
- Rust 1.45.0+
- Python 3.6+

## Try Run w/ Local Hornet

1. Run your local Hornet
- `$ git clone git@github.com:gohornet/hornet.git`
- checkout `chrysalis-pt2` branch
- Modify your `create_snapshot_alphanet.sh`, change the seed from `6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92` to `625d17d4a4b21cd5edeb57544b9d2d66ce22985fb61f17d1d7cae958d0068618`
```bash
#!/bin/bash
rm snapshots/alphanet/export.bin
mkdir -p snapshots/alphanet/
go run main.go tool snapgen alphanet1 625d17d4a4b21cd5edeb57544b9d2d66ce22985fb61f17d1d7cae958d0068618 snapshots/alphanet/export.bin
```
- Modify `config_alphanet.json`, enable proof of work.
```json
"enableProofOfWork": true,
```
- `$ ./run_coo_bootstrap.sh `

2. Build the iota-client-python library
- **IMPORTANT**: Enable the `binding/python/iota-core-python` in your `Cargo.toml`
```toml
[workspace]
members = [
    "iota-core",
    "binding/python/iota-core-python",
    "iota-client",
    "examples",
]
``` 
- `$ cargo build --release`
- The built library is located in `target/release/`
- On MacOS, rename `libiota_client.dylib` to `iota_client.so`, on Windows `libiota_client.dll` to `iota_client.pyd`, and on Linux `libiota_client.so` to `iota_client.so`.
- Copy your renamed library to `binding/python/`
- Go to `binding/python/`
- `$ python example.py`

## Python Example
```python
import iota_client as iota

# Create your client instance
client = iota.Client("http://0.0.0.0:14265")

# Send your token
message_id = client.send(seed="256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2",
                         path="m/",
                         address="6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92",
                         value=200)
print(f'Message ID: {message_id}')

# Check the balance
balance = client.get_address_balances(
    "6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92")

print(f'Balance: {balance}')
```