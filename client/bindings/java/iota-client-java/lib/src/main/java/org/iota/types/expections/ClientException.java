// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.expections;

public class ClientException extends Exception {

    private String methodName;

    public ClientException(String methodName, String message) {
        super(message);
    }

    public String getMethodName() {
        return methodName;
    }

}