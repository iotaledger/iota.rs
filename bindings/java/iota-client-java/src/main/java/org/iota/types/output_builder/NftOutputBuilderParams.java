package org.iota.types.output_builder;

import com.google.gson.JsonObject;
import org.iota.types.JsonUtils;
import org.iota.types.Feature;
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

    public NftOutputBuilderParams(String amount, NativeToken[] nativeTokens, NftId nftId, UnlockCondition[] unlockConditions, Feature[] features, Feature[] immutableFeatures) {
        this.amount = amount;
        this.nativeTokens = nativeTokens;
        this.nftId = nftId;
        this.unlockConditions = unlockConditions;
        this.features = features;
        this.immutableFeatures = immutableFeatures;
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