from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
