package org.iota.types.output_builder;

import com.google.gson.JsonObject;
import org.iota.types.JsonUtils;
import org.iota.types.Feature;
import org.iota.types.NativeToken;
import org.iota.types.TokenScheme;
import org.iota.types.UnlockCondition;

public class FoundryOutputBuilderParams {
    private String amount;
    private NativeToken[] nativeTokens;
    private int serialNumber;
    private TokenScheme tokenScheme;
    private UnlockCondition[] unlockConditions;
    private Feature[] features;
    private Feature[] immutableFeatures;

    public FoundryOutputBuilderParams(String amount, NativeToken[] nativeTokens, int serialNumber, TokenScheme tokenScheme, UnlockCondition[] unlockConditions, Feature[] features, Feature[] immutableFeatures) {
        this.amount = amount;
        this.nativeTokens = nativeTokens;
        this.serialNumber = serialNumber;
        this.tokenScheme = tokenScheme;
        this.unlockConditions = unlockConditions;
        this.features = features;
        this.immutableFeatures = immutableFeatures;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();

        o.addProperty("amount", amount);
        o.add("nativeTokens", JsonUtils.toJson(nativeTokens));
        o.addProperty("serialNumber", serialNumber);
        o.add("tokenScheme", tokenScheme != null ? tokenScheme.getJson() : null);
        o.add("unlockConditions", JsonUtils.toJson(unlockConditions));
        o.add("features", JsonUtils.toJson(features));
        o.add("immutableFeatures", JsonUtils.toJson(immutableFeatures));

        return o;
    }

}