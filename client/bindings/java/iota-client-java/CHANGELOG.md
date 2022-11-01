# Changelog

## 1.0.0-rc.2 - 2022-11-01

### Added

- Allow integers values in QueryParams;
- Added following methods to the `MiscellaneousApi`:
  - `getTokenSupply()`;
  - `getProtocolParameters()`;

### Changed

- Improve performance of tests by avoiding unnecessary indexer requests;

## 1.0.0-rc.1 - 2022-09-29

### Added

- Client APIs:
    - `BaseApi`;
    - `HighLevelApi`;
    - `MiscellaneousApi`;
    - `NodeCoreApi`;
    - `NodeIndexerApi`;
    - `UtilsApi`;

- Examples:
    - `CreateBlock`;
    - `GenerateAddresses`
    - `GenerateMnemonic`;
    - `GetBlock`;
    - `GetBlockMetadata`;
    - `GetBlockRaw`;
    - `GetHealth`;
    - `GetInfo`;
    - `GetMilestoneById`;
    - `GetMilestoneByIdRaw`;
    - `GetMilestoneByIndex`;
    - `GetMilestoneByIndexRaw`;
    - `GetOutputs`;
    - `GetReceipts`;
    - `GetReceiptsMigratedAt`;
    - `GetTips`;
    - `GetTreasury`;
    - `GetUtxoChangesById`;
    - `GetUtxoChangesByIndex`;
    - `PostBlock`;
    - `PostBlockRaw`;
    - `PrepareAndSignTransaction`;

### Changed

- Rust interaction through a JSON passing approach;

### Removed

- All glue code;
