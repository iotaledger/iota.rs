from iota_client import IotaClient
import json

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

hexAddress = client.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

alias_id = '0x0000000000000000000000000000000000000000000000000000000000000000'
# `hello` hex encoded
state_metadata = '0x68656c6c6f'
unlock_conditions = [
    # StateControllerAddressUnlockCondition
    {'type': 4, 'address': {'type': 0, 'pubKeyHash': hexAddress}},
    # GovernorAddressUnlockCondition
    {'type': 5, 'address': {'type': 0, 'pubKeyHash': hexAddress}},
]
features = [
    {
        # sender feature
        'type': 0,
        'address': {
            'type': 0,
            'pubKeyHash': hexAddress,
        },
    },
    {
        # MetadataFeature
        'type': 2,
        # `hello` hex encoded
        'data': '0x68656c6c6f',
    }
]
immutable_features = [
    {
        # issuer feature
        'type': 1,
        'address': {
            'type': 0,
            'pubKeyHash': hexAddress,
        },
    },
    {
        # MetadataFeature
        'type': 2,
        # `hello` hex encoded
        'data': '0x68656c6c6f',
    },
]

# Build alias output
alias_output = client.build_alias_output(
    alias_id=alias_id,
    state_metadata=state_metadata,
    unlock_conditions=unlock_conditions,
    features=features,
    immutable_features=immutable_features
)

# Print the output
print(json.dumps(alias_output, indent=4))
