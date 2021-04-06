import os
import iota_client

# Get the seed from environment variable
IOTA_SEED_SECRET = os.getenv('IOTA_SEED_SECRET')
if not IOTA_SEED_SECRET:
    raise Exception("Please define environment variable called `IOTA_SEED_SECRET`")

client = iota_client.Client()

print("Return a balance for a single address:")
print(
    client.get_address_balance("atoi1qp9427varyc05py79ajku89xarfgkj74tpel5egr9y7xu3wpfc4lkpx0l86")
)

print("Return a balance for the given seed and account_index:")
print(
    client.get_balance(
        seed=IOTA_SEED_SECRET,
        account_index=0,
        initial_address_index=0
    )
)
