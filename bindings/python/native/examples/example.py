from iota_client import IotaClient, client

# The node url
url = 'https://api.testnet.shimmer.network'

# The client option
options = {'nodes': [],
           'localPow': True,
           'fallbackToLocalPow': True,
           'offline': True}

# Create an IotaClient instance
client = IotaClient(options)

# Get the node health
response = client.get_node_health(url)
print(f'get_node_health() response: {response}')

# Get the node info
response = client.get_node_info(url)
print(f'get_node_info() response: {response}')
