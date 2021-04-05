import iota_client
client = iota_client.Client()

address_changed_list = client.get_addresses(
    seed="b3d7092195c36d47133ff786d4b0a1ef2ee6a0052f6e87b6dc337935c70c531e",
    account_index=0,
    input_range_begin=0,
    input_range_end=10,
    get_all=True
)
print(address_changed_list)