package org.iota.types.output_builder;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.iota.types.Feature;
import org.iota.types.NativeToken;
import org.iota.types.UnlockCondition;

import java.util.List;

public class BasicOutputBuilderParams {
    private String amount;
    private List<NativeToken> nativeTokens;
    private List<UnlockCondition> unlockConditions;
    private List<Feature> features;

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

        if (features != null) {
            JsonArray array = new JsonArray();
            for (Feature feature : features)
                array.add(feature.getJson());
            o.add("features", array);
        } else {
            o.add("features", null);
        }

        return o;
    }

}