package org.iota.main.types;

import java.util.Objects;

public class FoundryId {

    private String foundryId;

    public FoundryId(String foundryId) {
        this.foundryId = foundryId;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        FoundryId outputId1 = (FoundryId) o;
        return Objects.equals(foundryId, outputId1.foundryId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(foundryId);
    }

    @Override
    public String toString() {
        return foundryId;
    }
}
