import type {
    __GetOutputsMethod__,
    __GetOutputsPayload__,
    __GetInfoPayloadMethod__,
    __GetInfoPayload__,
    __GetOutputIdsPayloadMethod__,
    __GetOutputIdsPayload__,
    __GetOutputPayloadMethod__,
    __GetOutputPayload__,
} from './client';

export type __ClientPayloadMethods__ =
    | __GetInfoPayloadMethod__
    | __GetOutputPayloadMethod__
    | __GetOutputIdsPayloadMethod__
    | __GetOutputsMethod__;

export type __SendMessagePayload__ =
    | __GetInfoPayload__
    | __GetOutputPayload__
    | __GetOutputIdsPayload__
    | __GetOutputsPayload__;
