from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Get an outputs by its id
output = client.get_output('0x1e857d380f813d8035e487b6dfd2ff4740b6775273ba1b576f01381ba2a1a44c0000')
print(f'{output}')
