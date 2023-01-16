from iota_client import IotaClient, MnemonicSecretManager

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Configure alias output
# TODO: replace with your own values
alias_id = "0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa"
unlock_conditions = [
    { 'type': 4, 'address': { 'type': 0, 'pubKeyHash': '0xa7ebd1b1dbe267ab52fadb04fb777fdd1ed9fca72db01de879bf7bb846e0fc7a' } },
    { 'type': 5, 'address': { 'type': 0, 'pubKeyHash': '0xa7ebd1b1dbe267ab52fadb04fb777fdd1ed9fca72db01de879bf7bb846e0fc7a' } },
]

# Build alias output
output = client.build_alias_output(
    alias_id,
    unlock_conditions,
)

# Print the output
print(output)

