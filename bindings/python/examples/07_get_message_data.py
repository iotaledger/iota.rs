import iota_client
client = iota_client.Client()

message = client.get_message_data("e2daa4c6b012b615becd6c12189b2c9e701ba0d53b31a15425b21af5105fc086")
message_meta = client.get_message_metadata("e2daa4c6b012b615becd6c12189b2c9e701ba0d53b31a15425b21af5105fc086")

print("Message meta data:")
print(message_meta)
print("Message data:")
print(message)