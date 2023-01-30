from iota_client import IotaClient, MnemonicSecretManager

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Configure foundry output
# TODO: replace with your own values
serial_number = 1
token_scheme = { 'type': 0, 'meltedTokens': '0x0', 'mintedTokens': '0x32', 'maximumSupply': '0x64' }
unlock_conditions = [{ 'type': 6, 'address': { 'type': 8, 'aliasId': "0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa" }}]

# Configure and build and foundry output
output = client.build_foundry_output(
    1,
    token_scheme,
    unlock_conditions,
)

# Print the output
print(output)

