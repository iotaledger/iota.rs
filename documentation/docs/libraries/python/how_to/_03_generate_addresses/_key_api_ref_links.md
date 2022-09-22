### Generate the Address

You can generate an address from a [mnemonic](../../../../how_tos/02_generate_mnemonic.mdx) by calling
the [`Client.generate_addresses(secret_manager, options)`]( ./../libraries/python/api_reference#generate_addresses)
function. If successful, the function will return a list of addresses from the secret manager.

#### Use a `SecretManager`

You should not pass the mnemonic as plaintext. Instead, you should create a `SecretManager`instance and pass that to
the `Client.generate_addresses(secret_manager, options)` function. This example uses
a [`MnemonicSecretManager`](./../libraries/python/api_reference#mnemonicsecretmanager-objects)

#### Add Options

You can customize the address by adding `options` to the `Client.generate_addresses(secret_manager, options)` function
call in the form of a JSON object.

##### accountIndex

You can set the account index by adding the `accountIndex` option.

##### Example

```json
{
  "accountIndex": 0
}
```

##### range

You can set the account builder range by specifying the `range` option. You will need to specify both a `start` and
an `end`.

##### Example

```json
{
  "range": {
    "start": 0,
    "end": 1
  }
})
```