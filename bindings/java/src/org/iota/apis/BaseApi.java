package org.iota.apis;

import org.iota.ClientConfig;

public class BaseApi {
    protected ClientConfig config;

    public BaseApi(ClientConfig config) {
        this.config = config;
    }
}
