import type {
    __GetInfoPayloadMethod__,
    __GetInfoPayload__,
    __GetOutputPayloadMethod__,
    __GetOutputPayload__,
} from './client';

export type __ClientPayloadMethods__ =
    | __GetInfoPayloadMethod__
    | __GetOutputPayloadMethod__;

export type __SendMessagePayload__ = __GetInfoPayload__ | __GetOutputPayload__;
