
---
"nodejs-binding": patch
---

### Added

- `OutputIdsResponse`;

### Changed

- `Client::{aliasOutputIds, basicOutputIds, foundryOutputIds, nftOutputIds}` will not do automatic pagination if `QueryParameter::Cursor(_)` is provided and return type from `string[]` to `OutputIdsResponse`;