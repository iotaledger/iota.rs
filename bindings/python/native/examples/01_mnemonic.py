from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'offline': True})

# Generate a random BIP39 mnemonic
mnemonic = client.generate_mnemonic()
print(f'Mnemonic: {mnemonic}')
