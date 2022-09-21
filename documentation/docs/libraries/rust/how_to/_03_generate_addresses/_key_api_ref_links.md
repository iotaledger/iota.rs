:::danger

This example uses a mnemonic stored within the `.env` file. This is not safe, so you should not use this in a production
environment.

:::

### Generate the Address

You can generate an address from a [mnemonic](../../../../how_tos/02_generate_mnemonic.mdx) by calling
the [`Client.get_addresses(self, SecretManager)`](iota_client/client/struct.Client.html#method.get_addresses)
function. If successful, the function will return a list of addresses from the secret manager.

#### Use a `SecretManager`

You should not pass the mnemonic as plaintext. Instead, you should create
a [`SecretManager`](iota_client/secret/enum.SecretManager.html) instance and pass that to the
`Client.get_addresses(self, SecretManager)` function.

#### Chaining Calls

You can customize the address by chaining calls to the `Client.get_addresses(self, SecretManager)` function.

##### with_bech32_hrp

You can set the human-readable part of the Bech32 by chaining a call
to [with_bech32_hrp(self, bech32_hrp)](iota_client/api/address/struct.GetAddressesBuilder.html#method.with_bech32_hrp)

##### with_account_index

You can set the account index by chaining a call
to [with_bech32_hrp(self, account_index)](iota_client/api/address/struct.GetAddressesBuilder.html#method.with_account_index)
.

##### with_range

You can set the account builder range by chaining a call
to [with_range(self, range)](iota_client/api/address/struct.GetAddressesBuilder.html#method.with_range).