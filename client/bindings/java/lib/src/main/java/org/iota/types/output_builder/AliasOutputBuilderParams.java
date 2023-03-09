package org.iota.types.output_builder;

import com.google.gson.JsonObject;
import org.iota.types.JsonUtils;
import org.iota.types.Feature;
import org.iota.types.NativeToken;
import org.iota.types.UnlockCondition;
import org.iota.types.ids.AliasId;

public class AliasOutputBuilderParams {

    private String amount;
    private NativeToken[] nativeTokens;
    private AliasId aliasId;
    private Integer stateIndex;
    private String stateMetadata;
    private Integer foundryCounter;
    private UnlockCondition[] unlockConditions;
    private Feature[] features;
    private Feature[] immutableFeatures;

    public AliasOutputBuilderParams withAmount(String amount) {
        this.amount = amount;
        return this;
    }

    public AliasOutputBuilderParams withNativeTokens(NativeToken[] nativeTokens) {
        this.nativeTokens = nativeTokens;
        return this;
    }

    public AliasOutputBuilderParams withAliasId(AliasId aliasId) {
        this.aliasId = aliasId;
        return this;
    }

    public AliasOutputBuilderParams withStateIndex(Integer stateIndex) {
        this.stateIndex = stateIndex;
        return this;
    }

    public AliasOutputBuilderParams withStateMetadata(String stateMetadata) {
        this.stateMetadata = stateMetadata;
        return this;
    }

    public AliasOutputBuilderParams withFoundryCounter(Integer foundryCounter) {
        this.foundryCounter = foundryCounter;
        return this;
    }

    public AliasOutputBuilderParams withUnlockConditions(UnlockCondition[] unlockConditions) {
        this.unlockConditions = unlockConditions;
        return this;
    }

    public AliasOutputBuilderParams withFeatures(Feature[] features) {
        this.features = features;
        return this;
    }

    public AliasOutputBuilderParams withImmutableFeatures(Feature[] immutableFeatures) {
        this.immutableFeatures = immutableFeatures;
        return this;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("amount", amount);
        o.add("nativeTokens", JsonUtils.toJson(nativeTokens));
        o.addProperty("aliasId", aliasId != null ? aliasId.toString() : null);
        o.addProperty("stateIndex", stateIndex);
        o.addProperty("stateMetadata", stateMetadata);
        o.addProperty("foundryCounter", foundryCounter);
        o.add("unlockConditions", JsonUtils.toJson(unlockConditions));
        o.add("features", JsonUtils.toJson(features));
        o.add("immutableFeatures", JsonUtils.toJson(immutableFeatures));

        return o;
    }

}