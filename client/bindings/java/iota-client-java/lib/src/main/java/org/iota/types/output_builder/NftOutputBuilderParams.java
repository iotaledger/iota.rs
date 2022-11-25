package org.iota.types.output_builder;

import com.google.gson.JsonObject;
import org.iota.types.Feature;
import org.iota.types.JsonUtils;
import org.iota.types.NativeToken;
import org.iota.types.UnlockCondition;
import org.iota.types.ids.NftId;

public class NftOutputBuilderParams {

    private String amount;
    private NativeToken[] nativeTokens;
    private NftId nftId;
    private UnlockCondition[] unlockConditions;
    private Feature[] features;
    private Feature[] immutableFeatures;

    public NftOutputBuilderParams withAmount(String amount) {
        this.amount = amount;
        return this;
    }

    public NftOutputBuilderParams withNativeTokens(NativeToken[] nativeTokens) {
        this.nativeTokens = nativeTokens;
        return this;
    }

    public NftOutputBuilderParams withNftId(NftId nftId) {
        this.nftId = nftId;
        return this;
    }

    public NftOutputBuilderParams withUnlockConditions(UnlockCondition[] unlockConditions) {
        this.unlockConditions = unlockConditions;
        return this;
    }

    public NftOutputBuilderParams withFeatures(Feature[] features) {
        this.features = features;
        return this;
    }

    public NftOutputBuilderParams withImmutableFeatures(Feature[] immutableFeatures) {
        this.immutableFeatures = immutableFeatures;
        return this;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        o.addProperty("amount", amount);
        o.add("nativeTokens", JsonUtils.toJson(nativeTokens));
        o.addProperty("nftId", nftId != null ? nftId.toString() : null);
        o.add("unlockConditions", JsonUtils.toJson(unlockConditions));
        o.add("features", JsonUtils.toJson(features));
        o.add("immutableFeatures", JsonUtils.toJson(immutableFeatures));

        return o;
    }

}