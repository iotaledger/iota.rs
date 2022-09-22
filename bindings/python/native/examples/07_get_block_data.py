from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Create and post a block without payload
blockIdAndBlock = client.build_and_post_block()
print(f'{blockIdAndBlock}')

# Get the metadata for the block
metadata = client.get_block_metadata(blockIdAndBlock[0])
print(f'{metadata}')

# Request the block by its id
block = client.get_block_data(blockIdAndBlock[0])
print(f'{block}')
