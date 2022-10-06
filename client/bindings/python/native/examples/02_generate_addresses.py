from iota_client import IotaClient, MnemonicSecretManager

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# In this example we will create addresses from a mnemonic

secret_manager = MnemonicSecretManager("flame fever pig forward exact dash body idea link scrub tennis minute " +
    "surge unaware prosper over waste kitten ceiling human knife arch situate civil")


# Generate public address with custom account index and range.
address = client.generate_addresses(secret_manager, {
    "accountIndex": 0,
    "range": {
        "start": 0,
        "end": 1,
    },
})

print(f'Address: {address[0]}')
