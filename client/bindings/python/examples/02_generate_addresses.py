from iota_client import IotaClient, MnemonicSecretManager

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})

# In this example we will create addresses from a mnemonic

secret_manager = MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river")

# Generate public address with default account index and range.
addresses = client.generate_addresses(secret_manager, options={})

print('List of generated public addresses:', *addresses, sep='\n')
print()

# Generate public address with custom account index and range.
addresses = client.generate_addresses(secret_manager, {
    "accountIndex": 0,
    "range": {
        "start": 0,
        "end": 4,
    },
})

print('List of generated public addresses:', *addresses, sep='\n')
print()

# Generate internal addresses with custom account index and range.
addresses = client.generate_addresses(secret_manager, {
    "accountIndex": 0,
    "range": {
        "start": 0,
        "end": 4,
    },
    "internal": True,
})

print('List of generated internal addresses:', *addresses, sep='\n')
print()

# Generate addresses with providing all inputs, that way it can also be done offline without a node.
addresses = client.generate_addresses(secret_manager, {
    "coinType": 4219, # Shimmer coin type
    "accountIndex": 0,
    "range": {
        "start": 0,
        "end": 4,
    },
    "internal": False,
    # Generating addresses with client.generateAddresses(secretManager, options={}), will by default get the bech32_hrp (Bech32
    # human readable part) from the node info, generating it "offline" requires setting it in the generateAddressesOptions
    "bech32Hrp": 'rms'
})

print('List of offline generated public addresses:', *addresses, sep='\n')
print()
