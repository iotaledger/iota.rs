import iota_client

# client will connect to testnet by default
client = iota_client.Client()
print(client.get_info())
