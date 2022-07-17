package org.iota.types.output_builder;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.Feature;
import org.iota.types.NativeToken;
import org.iota.types.TokenScheme;
import org.iota.types.UnlockCondition;

import java.util.List;

public class FoundryOutputBuilderParams {
    private String amount;
    private List<NativeToken> nativeTokens;

    private int serialNumber;
    private TokenScheme tokenScheme;
    private List<UnlockCondition> unlockConditions;
    private List<Feature> features;
    private List<Feature> immutableFeatures;

    public FoundryOutputBuilderParams(String amount, List<NativeToken> nativeTokens, int serialNumber, TokenScheme tokenScheme, List<UnlockCondition> unlockConditions, List<Feature> features, List<Feature> immutableFeatures) {
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

        if (nativeTokens != null) {
            JsonArray array = new JsonArray();
            for (NativeToken nativeToken : nativeTokens)
                array.add(nativeToken.getJson());
            o.add("nativeTokens", array);
        } else {
            o.add("nativeTokens", null);
        }

        o.addProperty("serialNumber", serialNumber);

        o.add("tokenScheme", tokenScheme != null ? tokenScheme.getJson() : null);

        if (unlockConditions != null) {
            JsonArray array = new JsonArray();
            for (UnlockCondition unlockCondition : unlockConditions)
                array.add(unlockCondition.getJson());
            o.add("unlockConditions", array);
        } else {
            o.add("unlockConditions", null);
        }

        if (features != null) {
            JsonArray array = new JsonArray();
            for (Feature feature : features)
                array.add(feature.getJson());
            o.add("features", array);
        } else {
            o.add("features", null);
        }

        if (immutableFeatures != null) {
            JsonArray array = new JsonArray();
            for (Feature feature : immutableFeatures)
                array.add(feature.getJson());
            o.add("immutableFeatures", array);
        } else {
            o.add("immutableFeatures", null);
        }

        return o;
    }

}