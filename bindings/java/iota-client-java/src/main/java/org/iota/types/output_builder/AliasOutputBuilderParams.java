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
    private byte[] stateMetadata;
    private Integer foundryCounter;
    private UnlockCondition[] unlockConditions;
    private Feature[] features;
    private Feature[] immutableFeatures;

    public AliasOutputBuilderParams(String amount, NativeToken[] nativeTokens, AliasId aliasId, Integer stateIndex, byte[] stateMetadata, Integer foundryCounter, UnlockCondition[] unlockConditions, Feature[] features, Feature[] immutableFeatures) {
        this.amount = amount;
        this.nativeTokens = nativeTokens;
        this.aliasId = aliasId;
        this.stateIndex = stateIndex;
        this.stateMetadata = stateMetadata;
        this.foundryCounter = foundryCounter;
        this.unlockConditions = unlockConditions;
        this.features = features;
        this.immutableFeatures = immutableFeatures;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();

        o.addProperty("amount", amount);
        o.add("nativeTokens", JsonUtils.toJson(nativeTokens));
        o.addProperty("aliasId", aliasId != null ? aliasId.toString() : null);
        o.addProperty("stateIndex", stateIndex);
        o.add("stateMetadata", JsonUtils.toJson(stateMetadata));
        o.addProperty("foundryCounter", foundryCounter);
        o.add("unlockConditions", JsonUtils.toJson(unlockConditions));
        o.add("features", JsonUtils.toJson(features));
        o.add("immutableFeatures", JsonUtils.toJson(immutableFeatures));

        return o;
    }

}