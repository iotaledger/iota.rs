# `Client::get_health`

## Description

Calls `GET /health`.

Returns the health of the node.

## Request

## Response

## Example

```rust
let health = client.get_health(&node).await?;
```

Run `cargo run --example node_api_core_get_health --release -- [NODE URL]`.

# `Client::get_info`

## Description

Calls `GET /api/core/v2/info`.

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

Calls `GET /api/core/v2/tips`.

Returns tips that are ideal for attaching a block.

## Request

No parameters.

## Response

## Example

```rust
let tips = client.get_tips().await?;
```

Run `cargo run --example node_api_core_get_tips --release -- [NODE URL]`.

# `Client::post_block`

## Description

Calls `POST /api/core/v2/blocks`.

Submits a block as a JSON payload.

## Request

## Response

## Example

```rust
let block_id = client.post_block(&block).await?;
```

Run `cargo run --example node_api_core_post_block --release -- [NODE URL]`.

# `Client::post_block_raw`

## Description

Calls `POST /api/core/v2/blocks`.

Submits a block as raw bytes.

## Request

## Response

## Example

```rust
let block_id = client.post_block_raw(&block).await?;
```

Run `cargo run --example node_api_core_post_block_raw --release -- [NODE URL]`.

# `Client::get_block`

## Description

Calls `GET /api/core/v2/blocks/{blockId}`.

Returns block data as JSON by its identifier.

## Request

## Response

## Example

```rust
let block = client.get_block(&block_id).await?;
```

Run `cargo run --example node_api_core_get_block --release -- [NODE URL]`.

# `Client::get_block_raw`

## Description

Calls `GET /api/core/v2/blocks/{blockId}`.

Returns block data as raw bytes by its identifier.

## Request

## Response

## Example

```rust
let block = client.get_block_raw(&block_id).await?;
```

Run `cargo run --example node_api_core_get_block_raw --release -- [NODE URL]`.

# `Client::get_block_metadata`

## Description

Calls `GET /api/core/v2/blocks/{blockId}/metadata`.

Finds the metadata of a given block.

## Request

## Response

## Example

```rust
let block_metadata = client.get_block_metadata(&block_id).await?;
```

Run `cargo run --example node_api_core_get_block_metadata --release -- [NODE URL]`.

# `Client::`

Calls `GET /api/core/v2/outputs/{outputId}`.

Finds an output by its identifier.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/core/v2/outputs/{outputId}/metadata`.

Returns metadata about an output by its identifier.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::get_receipts`

Calls `GET /api/core/v2/receipts`.

Returns all stored receipts.

## Request

## Response

## Example

```rust
let receipts = client.get_receipts().await?;
```

Run `cargo run --example node_api_core_get_receipts --release -- [NODE URL]`.

# `Client::get_receipts_migrated_at`

## Description

Calls `GET /api/core/v2/receipts/{migratedAt}`.

Returns all stored receipts for a given migration index.

## Request

## Response

## Example

```rust
let receipts = client.get_receipts_migrated_at(1_000_000).await?;
```

Run `cargo run --example node_api_core_get_receipts_migrated_at --release -- [NODE URL]`.

# `Client::get_treasury`

Calls `GET /api/core/v2/treasury`.

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

Calls `GET /api/core/v2/transactions/{transactionId}/included-block`.

Returns the included block of a transaction.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::get_milestone_by_id`

## Description

Calls `GET /api/core/v2/milestones/{milestoneId}`.

Returns milestone data as JSON by its identifier.

## Request

## Response

## Example

```rust
let milestone = client.get_milestone_by_id(&milestone_id).await?;
```

Run `cargo run --example node_api_core_get_milestone_by_id --release -- [NODE URL]`.

# `Client::get_milestone_by_id_raw`

## Description

Calls `GET /api/core/v2/milestones/{milestoneId}`.

Returns milestone data as raw bytes by its identifier.

## Request

## Response

## Example

```rust
let milestone = client.get_milestone_by_id_raw(&milestone_id).await?;
```

Run `cargo run --example node_api_core_get_milestone_by_id_raw --release -- [NODE URL]`.

# `Client::get_utxo_changes_by_id`

## Description

Calls `GET /api/core/v2/milestones/{milestoneId}/utxo-changes`.

Gets all UTXO changes of a given milestone by milestone identifier.

## Request

## Response

## Example

```rust
let utxo_changes = client.get_utxo_changes_by_id(&milestone_id).await?;
```

Run `cargo run --example node_api_core_get_utxo_changes_by_id --release -- [NODE URL]`.

# `Client::get_milestone_by_index`

## Description

Calls `GET /api/core/v2/milestones/by-index/{index}`.

Returns milestone data as JSON by its index.

## Request

## Response

## Example

```rust
let milestone = client.get_milestone_by_index(milestone_index).await?;
```

Run `cargo run --example node_api_core_get_milestone_by_index --release -- [NODE URL]`.

# `Client::get_milestone_by_index_raw`

## Description

Calls `GET /api/core/v2/milestones/by-index/{index}`.

Returns milestone data as raw bytes by its index.

## Request

## Response

## Example

```rust
let milestone = client.get_milestone_by_index_raw(milestone_index).await?;
```

Run `cargo run --example node_api_core_get_milestone_by_index_raw --release -- [NODE URL]`.

# `Client::get_utxo_changes_by_index`

## Description

Calls `GET /api/core/v2/milestones/by-index/{index}/utxo-changes`.

Gets all UTXO changes of a given milestone by milestone index.

## Request

## Response

## Example

```rust
let utxo_changes = client.get_utxo_changes_by_index(milestone_index).await?;
```

Run `cargo run --example node_api_core_get_utxo_changes_by_index --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/core/v2/whiteflag`.

Computes applied and confirmed merkle route hashes for a proposed milestone.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/core/v2/peers`.

Gets information about the peers of the node.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/core/v2/peers`.

Adds a given peer to the node.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `GET /api/core/v2/peers/{peerId}`.

Gets information about a given peer.

## Request

## Response

## Example

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `DELETE /api/core/v2/peers/{peerId}`.

Removes/disconnects a given peer.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/core/v2/control/database/prune`.

Prunes the node database.

## Request

## Response

## Example

```rust
```

Run `cargo run --example node_api_core_ --release -- [NODE URL]`.

# `Client::`

## Description

Calls `POST /api/core/v2/control/snapshot/create`.

Creates a new snapshot
