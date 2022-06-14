package org.iota.main.types;

import java.util.Objects;

public class NftId {

    private String nftId;

    public NftId(String nftId) {
        this.nftId = nftId;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        NftId outputId1 = (NftId) o;
        return Objects.equals(nftId, outputId1.nftId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(nftId);
    }

    @Override
    public String toString() {
        return nftId;
    }
}
