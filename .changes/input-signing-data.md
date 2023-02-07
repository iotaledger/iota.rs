
---
"nodejs-binding": patch
---

Fixed returned JSON value for `IInputSigningData`;
Renamed `IInputSigningData::outputMetaData` to `IInputSigningData::outputMetadata`;
Changed `ISegment::bs` from `Uint8Array` to `number[]` so that the serialization corresponds to what is expected;
