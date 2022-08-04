from iota_client import IotaClient, client

# The client option
options = {'nodes': ['https://api.testnet.shimmer.network']}

# Create an IotaClient instance
client = IotaClient(options)

# Get the node info
response = client.get_info()
print(f'get_info() response: {response}')
