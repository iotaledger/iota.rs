from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

options = {
    # `hello` hex encoded
    "tag": '0x68656c6c6f',
    "data": '0x68656c6c6f',
}

# Create and post a block with a tagged data payload
block = client.build_and_post_block(None, options)
print(f'{block}')
