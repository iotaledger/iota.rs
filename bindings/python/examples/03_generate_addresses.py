import os
import iota_client

# Get the seed from environment variable
IOTA_SEED_SECRET = os.getenv('IOTA_SEED_SECRET')
if not IOTA_SEED_SECRET:
    raise Exception("Please define environment variable called `IOTA_SEED_SECRET`")

client = iota_client.Client()

address_changed_list = client.get_addresses(
    seed=IOTA_SEED_SECRET,
    account_index=0,
    input_range_begin=0,
    input_range_end=10,
    get_all=True
)
print(address_changed_list)