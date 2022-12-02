from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Create and post a block without payload
block = client.build_and_post_block()
print(f'{block}')
