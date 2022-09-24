:::danger

This example uses a mnemonic stored within the `.env` file. This is not safe, so you should not use this in a production
environment.

:::

### Generate the Address

You can generate an address from a [mnemonic](../../../../how_tos/02_generate_mnemonic.mdx) by calling
the [`Client.generateAddresses(secretManager, generateAddressesOptions)`](./../libraries/nodejs/references/classes/Client#generateAddresses)
function. If successful, the function will return a string representing the generated address.

#### Use a `SecretManager`

You should not pass the mnemonic as plaintext. Instead, you should create
a [`SecretManager`](./..libraries/nodejs/references/api_ref#secretmanager) instance and pass that to the
`Client.generateAddresses(secretManager, generateAddressesOptions)` function.

#### Add `generateAddressesOptions`

You can also pass a list
of [`generateAddressOptions`](./../libraries/nodejs/references/interfaces/IGenerateAddressesOptions) to
the `Client.generateAddresses(secretManager, generateAddressesOptions)` function.

##### `internal`

This `generateAddressesOption` indicates if the address you are about to create will be `internal` or `public`.

##### `bech32Hrp`

By default, the `Client.generateAddresses(secretManager, generateAddressesOptions)` will get the `bech32Hrp` from the
node's information. If you want to generate an address `offline`, you will need to set this option manually. 
