// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { INodeInfo } from '@iota/types';

/** NodeInfo wrapper which contains the node info and the url from the node (useful when multiple nodes are used) */
export interface INodeInfoWrapper {
    /** The node info */
    nodeInfo: INodeInfo;
    /** The url of the node */
    url: string;
}
