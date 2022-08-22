from iota_client import IotaClient, LedgerNanoSecretManager

# In this example we will get the ledger status and generate an address
# To use the ledger nano simulator clone https://github.com/iotaledger/ledger-shimmer-app, run `git submodule init && git submodule update --recursive`,
# then `./build.sh -m nanos|nanox|nanosplus -s` and use `True` in `LedgerNanoSecretManager(True)`.

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

is_simulator = True

secret_manager = LedgerNanoSecretManager(is_simulator)

# Get the Ledger Nano status.
ledger_nano_status = client.get_ledger_nano_status(is_simulator)

print(f'Ledger Nano status: {ledger_nano_status}')

# Generate public address with custom account index and range.
address = client.generate_addresses(secret_manager, {
    "accountIndex": 0,
    "range": {
        "start": 0,
        "end": 1,
    },
})

print(f'Address: {address[0]}')