from iota_client import *
import json

# Create an IotaClient instance
client = IotaClient(nodes = ['https://api.testnet.shimmer.network'])

hexAddress = client.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

alias_id = '0x0000000000000000000000000000000000000000000000000000000000000000'
# `hello` hex encoded
state_metadata = '0x68656c6c6f'
unlock_conditions = [
    UnlockCondition(
        UnlockConditionType.StateControllerAddress,
        Address(AddressType.ED25519, hexAddress)
    ),
    UnlockCondition(
        UnlockConditionType.GovernorAddress,
        Address(AddressType.ED25519, hexAddress)
    )
]
features = [
    Feature(FeatureType.Sender, Address(AddressType.ED25519, hexAddress)),
    Feature(FeatureType.Metadata, data='0x68656c6c6f')
]
immutable_features = [
    Feature(FeatureType.Issuer, Address(AddressType.ED25519, hexAddress)),
    Feature(FeatureType.Metadata, data='0x68656c6c6f')
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
