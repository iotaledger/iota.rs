from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient()

# Generate a random BIP39 mnemonic
mnemonic = client.generate_mnemonic()
print(f'Mnemonic: {mnemonic}')
