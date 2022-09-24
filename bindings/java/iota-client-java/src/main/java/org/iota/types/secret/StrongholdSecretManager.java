// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;

public class StrongholdSecretManager extends SecretManager {

    private String password;
    private String timeout;
    private String snapshotPath;

    public StrongholdSecretManager withPassword(String password) {
        this.password = password;
        return this;
    }

    public StrongholdSecretManager withTimeout(String timeout) {
        this.timeout = timeout;
        return this;
    }

    public StrongholdSecretManager withSnapshotPath(String snapshotPath) {
        this.snapshotPath = snapshotPath;
        return this;
    }

    @Override
    public JsonObject getJson() {
        JsonObject dto = new JsonObject();
        dto.addProperty("password", password);
        dto.addProperty("timeout", timeout);
        dto.addProperty("snapshotPath", snapshotPath);

        JsonObject o = new JsonObject();
        o.add("stronghold", dto);

        return o;
    }
}


