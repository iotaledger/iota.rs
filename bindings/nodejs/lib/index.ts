// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { initLogger as initLoggerBinding } from './bindings';
export * from './MessageHandler';
export * from './Client';

const initLogger = (config: any) => initLoggerBinding(JSON.stringify(config));

export { initLogger };
