# `Client::get_health`

## Description

Calls `GET /health`.

Returns the health of the node.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::get_info`

## Description

Calls `GET /api/v2/info`.

Returns general information about the node.

## Request

No parameters.

## Response

## Example

```rust
let info = client.get_info().await?;
```

Run `cargo run --example node_api_core_get_info --release -- [NODE URL]`.

# `Client::get_tips`

## Description

Calls `GET /api/v2/tips`.

Returns tips that are ideal for attaching a message.

## Request

No parameters.

## Response

## Example

```rust
let tips = client.get_tips().await?;
```

Run `cargo run --example node_api_core_get_tips --release -- [NODE URL]`.

# `Client::post_message`

## Description

Calls `POST /api/v2/messages`.

Submits a message as a JSON payload.

## Request

## Response

## Example

```rust
let message_id = client.post_message(&message).await?;
```

Run `cargo run --example node_api_core_post_message --release -- [NODE URL]`.

# `Client::post_message_raw`

## Description

Calls `POST /api/v2/messages`.

Submits a message as raw bytes.

## Request

## Response

## Example

```rust
let message_id = client.post_message_raw(&message).await?;
```

Run `cargo run --example node_api_core_post_message_raw --release -- [NODE URL]`.

# `Client::get_message`

## Description

Calls `GET /api/v2/messages/{messageId}`.

Returns message data as JSON by its identifier.

## Request

## Response

## Example

```rust
let message = client.get_message(&message_id).await?;
```

Run `cargo run --example node_api_core_get_message --release -- [NODE URL]`.

# `Client::get_message_raw`

## Description

Calls `GET /api/v2/messages/{messageId}`.

Returns message data as raw bytes by its identifier.

## Request

## Response

## Example

```rust
let message = client.get_message_raw(&message_id).await?;
```

Run `cargo run --example node_api_core_get_message_raw --release -- [NODE URL]`.

# `Client::get_message_metadata`

## Description

Calls `GET /api/v2/messages/{messageId}/metadata`.

Finds the metadata of a given message.

## Request

## Response

## Example

```rust
let message_metadata = client.get_message_metadata(&message_id).await?;
```

Run `cargo run --example node_api_core_get_message_metadata --release -- [NODE URL]`.

# `Client::get_message_children`

## Description

Calls `GET /api/v2/messages/{messageId}/children`.

Returns the children of a message.

## Request

## Response

## Example

```rust
let message_children = client.get_message_children(&message_id).await?;
```

Run `cargo run --example node_api_core_get_message_children --release -- [NODE URL]`.

# `Client::`

Calls `GET /api/v2/outputs/{outputId}`.

Finds an output by its identifier.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/outputs/{outputId}/metadata`.

Returns metadata about an output by its identifier.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

Calls `GET /api/v2/receipts`.

Returns all stored receipts.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/receipts/{migratedAt}`.

Returns all stored receipts for a given migration index.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::get_treasury`

Calls `GET /api/v2/treasury`.

Returns information about the treasury.

## Request

No parameters.

## Response

## Example

```rust
let treasury = client.get_treasury().await?;
```

Run `cargo run --example node_api_core_get_treasury --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/transactions/{transactionId}/included-message`.

Returns the included message of a transaction.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/milestones/{milestoneId}`.

Looks up a milestone by a given milestone ID.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/milestones/{milestoneId}/utxo-changes`.

Gets all UTXO changes of a given milestone by Milestone ID.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/milestones/by-index/{index}`.

Looks up a milestone by a given milestone index.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/milestones/by-index/{index}/utxo-changes`.

Gets all UTXO changes of a given milestone by milestone index.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/v2/whiteflag`.

Computes applied and confirmed merkle route hashes for a proposed milestone.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/peers`.

Gets information about the peers of the node.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/v2/peers`.

Adds a given peer to the node.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/v2/peers/{peerId}`.

Gets information about a given peer.

## Request

## Response

## Example

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `DELETE /api/v2/peers/{peerId}`.

Removes/disconnects a given peer.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/v2/control/database/prune`.

Prunes the node database.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/v2/control/snapshot/create`.

Creates a new snapshot
