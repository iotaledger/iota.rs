from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

address = 'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy'

# Get output ids of basic outputs that can be controlled by this address without further unlock constraints
output_ids_response = client.basic_output_ids([{"address": address},
                                      {"hasExpiration": False},
                                      {"hasTimelock": False},
                                      {"hasStorageDepositReturn": False}, ])
print(f'{output_ids_response}')

# Get the outputs by their id
outputs = client.get_outputs(output_ids_response['items'])
print(f'{outputs}')


# Calculate the total amount and native tokens
total_amount = 0
native_tokens = []
for output_response in outputs:
    output = output_response['output']
    total_amount += int(output['amount'])
    if 'nativeTokens' in output:
        native_tokens.append(output['nativeTokens'])

print(
    f'Outputs controlled by {address} have {total_amount} glow and native tokens: {native_tokens}')
