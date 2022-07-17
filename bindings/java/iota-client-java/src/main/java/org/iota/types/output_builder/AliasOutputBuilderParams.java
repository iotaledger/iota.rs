package org.iota.types.output_builder;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.Feature;
import org.iota.types.NativeToken;
import org.iota.types.UnlockCondition;
import org.iota.types.ids.AliasId;

import java.util.List;

public class AliasOutputBuilderParams {

    private String amount;
    private List<NativeToken> nativeTokens;
    private AliasId aliasId;
    private Integer stateIndex;
    private byte[] stateMetadata;
    private Integer foundry_counter;
    private List<UnlockCondition> unlockConditions;
    private List<Feature> features;
    private List<Feature> immutableFeatures;

    public AliasOutputBuilderParams(String amount, List<NativeToken> nativeTokens, AliasId aliasId, Integer stateIndex, byte[] stateMetadata, Integer foundry_counter, List<UnlockCondition> unlockConditions, List<Feature> features, List<Feature> immutableFeatures) {
        this.amount = amount;
        this.nativeTokens = nativeTokens;
        this.aliasId = aliasId;
        this.stateIndex = stateIndex;
        this.stateMetadata = stateMetadata;
        this.foundry_counter = foundry_counter;
        this.unlockConditions = unlockConditions;
        this.features = features;
        this.immutableFeatures = immutableFeatures;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();

        o.addProperty("amount", amount);

        if(nativeTokens != null) {
            JsonArray array = new JsonArray();
            for(NativeToken nativeToken: nativeTokens)
                array.add(nativeToken.getJson());
            o.add("nativeTokens", array);
        } else {
            o.add("nativeTokens", null);
        }

        o.addProperty("aliasId", aliasId != null ? aliasId.toString() : null);

        o.addProperty("stateIndex", stateIndex);

        if(stateMetadata != null) {
            JsonArray array = new JsonArray();
            for(byte tagByte: stateMetadata)
                array.add(tagByte & 0xFF);
            o.add("stateMetadata", array);
        } else {
            o.add("stateMetadata", null);
        }

        o.addProperty("foundry_counter", foundry_counter);

        if(unlockConditions != null) {
            JsonArray array = new JsonArray();
            for(UnlockCondition unlockCondition: unlockConditions)
                array.add(unlockCondition.getJson());
            o.add("unlockConditions", array);
        } else {
            o.add("unlockConditions", null);
        }

        if(features != null) {
            JsonArray array = new JsonArray();
            for(Feature feature: features)
                array.add(feature.getJson());
            o.add("features", array);
        } else {
            o.add("features", null);
        }

        if(immutableFeatures != null) {
            JsonArray array = new JsonArray();
            for(Feature feature: immutableFeatures)
                array.add(feature.getJson());
            o.add("immutableFeatures", array);
        } else {
            o.add("immutableFeatures", null);
        }

        return o;
    }

}