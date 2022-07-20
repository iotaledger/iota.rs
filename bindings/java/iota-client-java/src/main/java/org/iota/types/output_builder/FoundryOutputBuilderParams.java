package org.iota.types.output_builder;

import com.google.gson.JsonObject;
import org.iota.types.*;

public class FoundryOutputBuilderParams {

    private String amount;
    private NativeToken[] nativeTokens;
    private int serialNumber;
    private TokenScheme tokenScheme;
    private UnlockCondition[] unlockConditions;
    private Feature[] features;
    private Feature[] immutableFeatures;

    public FoundryOutputBuilderParams withAmount(String amount) {
        this.amount = amount;
        return this;
    }

    public FoundryOutputBuilderParams withNativeTokens(NativeToken[] nativeTokens) {
        this.nativeTokens = nativeTokens;
        return this;
    }

    public FoundryOutputBuilderParams withSerialNumber(int serialNumber) {
        this.serialNumber = serialNumber;
        return this;
    }

    public FoundryOutputBuilderParams withTokenScheme(TokenScheme tokenScheme) {
        this.tokenScheme = tokenScheme;
        return this;
    }

    public FoundryOutputBuilderParams withUnlockConditions(UnlockCondition[] unlockConditions) {
        this.unlockConditions = unlockConditions;
        return this;
    }

    public FoundryOutputBuilderParams withFeatures(Feature[] features) {
        this.features = features;
        return this;
    }

    public FoundryOutputBuilderParams withImmutableFeatures(Feature[] immutableFeatures) {
        this.immutableFeatures = immutableFeatures;
        return this;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("amount", amount);
        o.add("nativeTokens", JsonUtils.toJson(nativeTokens));
        o.addProperty("serialNumber", serialNumber);
        o.add("tokenScheme", tokenScheme != null ? tokenScheme.toJson() : null);
        o.add("unlockConditions", JsonUtils.toJson(unlockConditions));
        o.add("features", JsonUtils.toJson(features));
        o.add("immutableFeatures", JsonUtils.toJson(immutableFeatures));

        return o;
    }

}