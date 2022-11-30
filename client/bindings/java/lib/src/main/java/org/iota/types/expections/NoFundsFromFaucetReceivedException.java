// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.expections;

public class NoFundsFromFaucetReceivedException extends Exception {

    public NoFundsFromFaucetReceivedException() {
        super("faucet is not ready");
    }

}

