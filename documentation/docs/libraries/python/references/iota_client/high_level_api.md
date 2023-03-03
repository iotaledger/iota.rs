---
sidebar_label: high_level_api
title: iota_client.high_level_api
---

## HighLevelAPI Objects

```python
class HighLevelAPI(BaseAPI)
```

### get\_outputs

```python
def get_outputs(output_ids)
```

Fetch OutputResponse from provided OutputIds (requests are sent in parallel).

### try\_get\_outputs

```python
def try_get_outputs(output_ids)
```

Try to get OutputResponse from provided OutputIds.
Requests are sent in parallel and errors are ignored, can be useful for spent outputs.

### find\_blocks

```python
def find_blocks(block_ids)
```

Find all blocks by provided block IDs.

### retry

```python
def retry(block_id)
```

Retries (promotes or reattaches) a block for provided block id. Block should only be
retried only if they are valid and haven&#x27;t been confirmed for a while.

### retry\_until\_included

```python
def retry_until_included(block_id, interval=None, max_attempts=None)
```

Retries (promotes or reattaches) a block for provided block id until it&#x27;s included (referenced by a
milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
position and additional reattached blocks.

### consolidate\_funds

```python
def consolidate_funds(secret_manager, generate_addresses_options)
```

Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
Returns the address to which the funds got consolidated, if any were available.

### find\_inputs

```python
def find_inputs(addresses, amount)
```

Function to find inputs from addresses for a provided amount (useful for offline signing)

### find\_outputs

```python
def find_outputs(output_ids, addresses)
```

Find all outputs based on the requests criteria. This method will try to query multiple nodes if
the request amount exceeds individual node limit.

### reattach

```python
def reattach(block_id)
```

Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven&#x27;t been
confirmed for a while.

### reattach\_unchecked

```python
def reattach_unchecked(block_id)
```

Reattach a block without checking if it should be reattached.

### promote

```python
def promote(block_id)
```

Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
method should error out and should not allow unnecessary promotions.

### promote\_unchecked

```python
def promote_unchecked(block_id)
```

Promote a block without checking if it should be promoted.

