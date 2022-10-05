// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { ILoggerConfig } from '../types/loggerConfig';
import { initLogger as initLoggerBinding } from './bindings';

const defaultLoggerConfig: ILoggerConfig = {
    colorEnabled: true,
    name: './client.log',
    levelFilter: 'debug',
};

/** Initialize logger, if no arguments are provided a default config will be used. */
export const initLogger = (config: ILoggerConfig = defaultLoggerConfig) =>
    initLoggerBinding(JSON.stringify(config));
