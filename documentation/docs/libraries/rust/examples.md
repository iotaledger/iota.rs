# Examples

It's possible to send transactions with iota.rs, but we strongly recommend to use official `wallet.rs` library together with `stronghold.rs` enclave for value-based transfers. This combination incorporates the best security practices while dealing with seeds, related addresses and `UTXO`. See more information on [wallet docs](https://chrysalis.docs.iota.org/libraries/wallet).

```bash
git clone https://github.com/iotaledger/iota.rs
```

```bash
cd iota.rs
```

Rename the `.env.example` file to `.env`.

Run the examples like:

```bash
cargo run --example 01_get_info --release
```

```rust
{@include: ../../../../examples/01_get_info.rs}
```

```rust
{@include: ../../../../examples/02_generate_seed.rs}
```

```rust
{@include: ../../../../examples/03_generate_addresses.rs}
```

```rust
{@include: ../../../../examples/04_get_balance.rs}
```

```rust
{@include: ../../../../examples/05_get_address_outputs.rs}
```

```rust
{@include: ../../../../examples/06_simple_message.rs}
```

```rust
{@include: ../../../../examples/07_get_message_metadata.rs}
```

```rust
{@include: ../../../../examples/08_data_message.rs}
```

```rust
{@include: ../../../../examples/09_transaction.rs}
```

```rust
{@include: ../../../../examples/10_mqtt.rs}
```

You can find more advanced examples in the [examples](https://github.com/iotaledger/iota.rs/tree/dev/examples) folder.
