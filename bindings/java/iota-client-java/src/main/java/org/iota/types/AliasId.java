package org.iota.types;

import java.util.Objects;

public class AliasId {

    private String aliasId;

    public AliasId(String aliasId) {
        this.aliasId = aliasId;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        AliasId outputId1 = (AliasId) o;
        return Objects.equals(aliasId, outputId1.aliasId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(aliasId);
    }

    @Override
    public String toString() {
        return aliasId;
    }
}
