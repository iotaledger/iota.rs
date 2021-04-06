import iota_client
client = iota_client.Client()

outputs = client.get_address_outputs("atoi1qp9427varyc05py79ajku89xarfgkj74tpel5egr9y7xu3wpfc4lkpx0l86")
for output in outputs:
    print(f"Output index: {output['index']}; raw transaction id: {output['transaction_id']}")
    encoded_hex = "".join(f"{i:0>2x}" for i in output["transaction_id"]) + f"{output['index']:0>4x}"
    print(f"`output_id` encoded in hex: {encoded_hex}")
