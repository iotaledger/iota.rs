from iota_client import *
import json

# Create an IotaClient instance
client = IotaClient(nodes = ['https://api.testnet.shimmer.network'])

hex_address = client.bech32_to_hex("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")

address_unlock_condition = UnlockCondition(
    UnlockConditionType.Address,
    Address(AddressType.ED25519, hex_address)
)

# Build most basic output with amound and a single address unlock condition
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=2))

# Output with metadata feature block
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
    ],
    features=[
        Feature(FeatureType.Metadata, data='0x48656c6c6f2c20576f726c6421')
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=2))

# Output with storage deposit return
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        UnlockCondition(type=UnlockConditionType.StorageDepositReturn, amount=1000000,
                        return_address=Address(AddressType.ED25519, hex_address))
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=2))

# Output with expiration
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        UnlockCondition(type=UnlockConditionType.Expiration, unix_time=1,
                        return_address=Address(AddressType.ED25519, hex_address))
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=2))

# Output with timelock
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        UnlockCondition(type=UnlockConditionType.Timelock, unix_time=1)
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=2))
