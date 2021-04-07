import os
import iota_client

# Get the seed from environment variable
IOTA_SEED_SECRET = os.getenv('IOTA_SEED_SECRET')
if not IOTA_SEED_SECRET:
    raise Exception("Please define environment variable called `IOTA_SEED_SECRET`")

client = iota_client.Client()

message = client.message(
    seed=IOTA_SEED_SECRET,
    outputs=[
        {
            'address': 'atoi1qqydc70mpjdvl8l2wyseaseqwzhmedzzxrn4l9g2c8wdcsmhldz0ulwjxpz',
            'amount': 1_000_000
        }
    ]
)
print(message)