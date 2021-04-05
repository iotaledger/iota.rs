import iota_client
client = iota_client.Client()

print("Return a balance for a single address:")
print(
    client.get_address_balance("atoi1qp9427varyc05py79ajku89xarfgkj74tpel5egr9y7xu3wpfc4lkpx0l86")
)

print("Return a balance for the given seed and account_index:")
print(
    client.get_balance(
        seed="b3d7092195c36d47133ff786d4b0a1ef2ee6a0052f6e87b6dc337935c70c531e",
        account_index=0,
        initial_address_index=0
    )
)
