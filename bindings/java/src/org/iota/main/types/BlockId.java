package org.iota.main.types;

import java.util.Objects;

public class BlockId {

    private String blockId;

    public BlockId(String blockId) {
        this.blockId = blockId;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        BlockId outputId1 = (BlockId) o;
        return Objects.equals(blockId, outputId1.blockId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(blockId);
    }

    @Override
    public String toString() {
        return blockId;
    }
}
