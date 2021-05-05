// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.client.local;

public class ClientException extends RuntimeException {

    public ClientException() {
        super();
    }

    public ClientException(String errorMessage) {
        super(errorMessage);
    }
}
