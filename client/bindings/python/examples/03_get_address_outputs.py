from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# Get output ids of basic outputs that can be controlled by this address without further unlock constraints
output_ids_response = client.basic_output_ids([{"address": 'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy'},
                                      {"hasExpiration": False},
                                      {"hasTimelock": False},
                                      {"hasStorageDepositReturn": False}, ])
print(f'{output_ids_response}')

# Get the outputs by their id
outputs = client.get_outputs(output_ids_response['items'])
print(f'{outputs}')
