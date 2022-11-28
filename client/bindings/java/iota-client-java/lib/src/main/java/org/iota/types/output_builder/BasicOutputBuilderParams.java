package org.iota.types.output_builder;

import com.google.gson.JsonObject;
import org.iota.types.Feature;
import org.iota.types.JsonUtils;
import org.iota.types.NativeToken;
import org.iota.types.UnlockCondition;

public class BasicOutputBuilderParams {

    private String amount;
    private NativeToken[] nativeTokens;
    private UnlockCondition[] unlockConditions;
    private Feature[] features;

    public BasicOutputBuilderParams withAmount(String amount) {
        this.amount = amount;
        return this;
    }

    public BasicOutputBuilderParams withNativeTokens(NativeToken[] nativeTokens) {
        this.nativeTokens = nativeTokens;
        return this;
    }

    public BasicOutputBuilderParams withUnlockConditions(UnlockCondition[] unlockConditions) {
        this.unlockConditions = unlockConditions;
        return this;
    }

    public BasicOutputBuilderParams withFeatures(Feature[] features) {
        this.features = features;
        return this;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("amount", amount);
        o.add("nativeTokens", JsonUtils.toJson(nativeTokens));
        o.add("unlockConditions", JsonUtils.toJson(unlockConditions));
        o.add("features", JsonUtils.toJson(features));

        return o;
    }

}