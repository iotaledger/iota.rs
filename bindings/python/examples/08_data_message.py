import iota_client
client = iota_client.Client()

# encoding utf string into list of bytes
some_utf_data = "some utf based data".encode("utf8")

message = client.message(
    index="some_data_index", data=some_utf_data
)
print(message)