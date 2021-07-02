# Examples

It's possible to send transactions with iota.rs, but we strongly recommend to use official `wallet.rs` library together with `stronghold.rs` enclave for value-based transfers. This combination incorporates the best security practices while dealing with seeds, related addresses and `UTXO`. See more information on [wallet docs](https://chrysalis.docs.iota.org/libraries/wallet.html).

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
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 01_get_info --release

use iota_client::Client;

/// In this example we will get information about the node

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert your node URL here
        // Node with optional authentication
        // .with_node_auth(
        //     "https://somechrysalisiotanode.com",
        //     Some("Some JWT"),
        //     Some(("name", "password")),
        // )
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Node Info: {:?}", info);
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 02_generate_seed --release

use iota_client::crypto::signatures::ed25519::SecretKey;

/// In this example we will generate a seed

#[tokio::main]
async fn main() {
    let secret_key = SecretKey::generate().unwrap();
    println!("{}", hex::encode(&secret_key.to_le_bytes()));
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 03_generate_addresses --release

use iota_client::{api::GetAddressesBuilder, Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will create addresses from a seed defined in .env

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    // Generate addresses with default account index and range
    let addresses = iota.get_addresses(&seed).finish().await.unwrap();
    println!("List of generated public addresses:\n{:?}\n", addresses);

    // Generate addresses with custom account index and range
    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await
        .unwrap();

    println!("List of generated public addresses:\n{:?}\n", addresses);

    // Generate public (false) & internal (true) addresses
    let addresses = iota.get_addresses(&seed).with_range(0..4).get_all().await.unwrap();
    println!("List of generated public and internal addresses:\n{:?}\n", addresses);

    // Generate public addresses offline with the bech32_hrp defined
    let addresses = GetAddressesBuilder::new(&seed)
        .with_bech32_hrp("atoi".into())
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await
        .unwrap();

    println!("List of offline generated public addresses:\n{:?}\n", addresses);
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 04_get_balance --release

use iota_client::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the account balance of a known seed and the balance and outputs of a known address

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert your node URL here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    let seed_balance = iota.get_balance(&seed).finish().await.unwrap();
    println!("Account balance: {:?}i\n", seed_balance);

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let response = iota.get_address().balance(&address).await.unwrap();
    println!("The balance of {:?} is {:?}i\n", address, response.balance);

    let outputs = iota.get_address().outputs(&address, Default::default()).await.unwrap();

    println!("The outputs of address {:?} are: {:?}", address, outputs);
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 05_get_address_outputs --release

use iota_client::{Client, Result};

/// In this example we will get the outputs of a known address

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let outputs = iota.get_address().outputs(&address, Default::default()).await.unwrap();

    println!("The outputs of address {:?} are: {:?}", address, outputs);
    Ok(())
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 06_simple_message --release

use iota_client::{Client, Result};

/// In this example we will send a message without a payload

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    let message = iota.message().finish().await?;

    println!(
        "Empty message sent: https://explorer.iota.org/testnet/message/{}",
        message.id().0
    );
    Ok(())
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 07_get_message_metadata --release

use iota_client::{Client, Result};

/// In this example we will send a message and get the metadata for it

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    let message = iota.message().finish().await?;

    let metadata = iota.get_message().metadata(&message.id().0).await?;
    println!("Message metadata: {:?}", metadata);
    Ok(())
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 08_data_message --release

use iota_client::{Client, Result};

/// In this example we will send a message without a payload

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")?
        // .with_permanode("http://18.196.167.57:8000/api/permanode/", None, None)?
        .finish()
        .await?;

    let message = iota
        .message()
        .with_index("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Message sent https://explorer.iota.org/testnet/message/{}\n",
        message.id().0
    );

    let fetched_message_ids = iota.get_message().index("Hello").await.unwrap();
    println!("Messages with Hello index: {:?}", fetched_message_ids);
    Ok(())
}
```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 09_transaction --release

use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send a transaction

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let seed_1 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    let message = iota
        .message()
        .with_seed(&seed_1)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            &iota.get_addresses(&seed_1).with_range(1..2).finish().await?[0],
            1_000_000,
        )?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/testnet/message/{}",
        message.id().0
    );
    Ok(())
}

```

```rust
// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 10_mqtt --release

use iota_client::{bee_message::Message, Client, MqttEvent, Result, Topic};
use std::sync::{mpsc::channel, Arc, Mutex};

// Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let mut iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    let (tx, rx) = channel();
    let tx = Arc::new(Mutex::new(tx));

    let mut event_rx = iota.mqtt_event_receiver();
    tokio::spawn(async move {
        while event_rx.changed().await.is_ok() {
            let event = event_rx.borrow();
            if *event == MqttEvent::Disconnected {
                println!("mqtt disconnected");
                std::process::exit(1);
            }
        }
    });

    iota.subscriber()
        .with_topics(vec![
            Topic::new("milestones/latest").unwrap(),
            Topic::new("messages").unwrap(),
        ])
        .subscribe(move |event| {
            match event.topic.as_str() {
                "messages" => {
                    let message: Message = serde_json::from_str(&event.payload).unwrap();
                    println!("{:?}", event);
                    println!("{:?}", message);
                }
                _ => println!("{:?}", event),
            }
            tx.lock().unwrap().send(()).unwrap();
        })
        .await
        .unwrap();

    for _ in 0..10 {
        rx.recv().unwrap();
    }

    iota.subscriber().disconnect().await.unwrap();
    // alternatively
    // iota.subscriber().unsubscribe().unwrap();
    Ok(())
}
```

You can find more advanced examples in the [examples](https://github.com/iotaledger/iota.rs/tree/dev/examples) folder.
