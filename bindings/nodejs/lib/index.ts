// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { LoggerConfig } from '../types/loggerConfig';
import { initLogger as initLoggerBinding } from './bindings';
export * from './MessageHandler';
export * from './Client';
export * from './constants';
export * from './utils';
const initLogger = (config: LoggerConfig) =>
    initLoggerBinding(JSON.stringify(config));

export { initLogger };
