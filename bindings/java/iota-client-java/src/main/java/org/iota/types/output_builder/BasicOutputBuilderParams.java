package org.iota.types.output_builder;

import com.google.gson.JsonObject;
import org.iota.types.JsonUtils;
import org.iota.types.Feature;
import org.iota.types.NativeToken;
import org.iota.types.UnlockCondition;

public class BasicOutputBuilderParams {
    private String amount;
    private NativeToken[] nativeTokens;
    private UnlockCondition[] unlockConditions;
    private Feature[] features;

    public BasicOutputBuilderParams(String amount, NativeToken[] nativeTokens, UnlockCondition[] unlockConditions, Feature[] features) {
        this.amount = amount;
        this.nativeTokens = nativeTokens;
        this.unlockConditions = unlockConditions;
        this.features = features;
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