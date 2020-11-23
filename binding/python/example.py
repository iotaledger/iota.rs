import iota_client as iota

# Create your client instance
client = iota.Client("http://0.0.0.0:14265")

# Send your token
message_id = client.send(seed="256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2",
                         path="m/",
                         address="6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92",
                         value=200)
print(f'Message ID: {message_id}')

# Check the balance
balance = client.get_address_balances(
    "6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92")

print(f'Balance: {balance}')
