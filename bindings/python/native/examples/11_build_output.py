from iota_client import IotaClient, MnemonicSecretManager

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

basic_output = client.build_basic_output(
    unlock_conditions=[
        {
            "type": 0,
            "address": {
                "type": 0,
                "pubKeyHash": client.bech32_to_hex("rms1qzpf0tzpf8yqej5zyhjl9k3km7y6j0xjnxxh7m2g3jtj2z5grej67sl6l46"),
            },
        },
    ],
    amount='1000000',
)
print(f'{basic_output}')
