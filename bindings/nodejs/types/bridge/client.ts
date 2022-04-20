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
        output_id: string;
    };
};

export type __GetOutputPayload__ = {
    cmd: 'CallClientMethod';
    payload: __GetOutputPayloadMethod__;
};
