package org.iota.types;

import com.google.gson.JsonArray;
import org.iota.types.ids.AbstractId;

public class JsonUtils<T> {

    public static JsonArray toJson(AbstractId[] array) {
        if (array != null) {
            JsonArray a = new JsonArray();
            for (AbstractId o : array) {
                a.add(o.toString());
            }
            return a;
        } else {
            return null;
        }
    }

    public static JsonArray toJson(AbstractObject[] array) {
        if (array != null) {
            JsonArray a = new JsonArray();
            for (AbstractObject o : array) {
                a.add(o.toJson());
            }
            return a;
        } else {
            return null;
        }
    }

    public static JsonArray toJson(String[] array) {
        if (array != null) {
            JsonArray a = new JsonArray();
            for (String s : array) {
                a.add(s);
            }
            return a;
        } else {
            return null;
        }
    }

    public static JsonArray toJson(byte[] array) {
        if (array != null) {
            JsonArray a = new JsonArray();
            for (byte b : array) {
                a.add(b & 0xFF);
            }
            return a;
        } else {
            return null;
        }
    }

}
