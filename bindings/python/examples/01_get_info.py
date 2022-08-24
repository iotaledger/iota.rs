import iota_client

# create a client with a node
client = iota_client.Client(
    nodes_name_password=[['https://api.lb-0.h.chrysalis-devnet.iota.cafe']])

print(client.get_info())
