import type { Address } from '../address';

export type __GetInfoPayloadMethod__ = {
    name: 'GetInfo';
};

export type __GetInfoPayload__ = {
    cmd: 'CallClientMethod';
    payload: __GetInfoPayloadMethod__;
};

export type __GetOutputPayloadMethod__ = {
    name: 'GetOutput';
    data: {
        outputId: string;
    };
};

export type __GetOutputPayload__ = {
    cmd: 'CallClientMethod';
    payload: __GetOutputPayloadMethod__;
};

export type __GetOutputIdsPayloadMethod__ = {
    name: 'OutputIds';
    data: {
        queryParameters: Address[];
    };
};

export type __GetOutputIdsPayload__ = {
    cmd: 'CallClientMethod';
    payload: __GetOutputPayloadMethod__;
};

export type __GetOutputsMethod__ = {
    name: 'GetOutputs';
    data: {
        outputIds: string[];
    };
};

export type __GetOutputsPayload__ = {
    cmd: 'CallClientMethod';
    payload: __GetOutputsMethod__;
};
