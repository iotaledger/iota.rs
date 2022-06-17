package org.iota.types;

import java.util.Objects;

public class OutputId {

    private String outputId;

    public OutputId(String outputId) {
        this.outputId = outputId;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        OutputId outputId1 = (OutputId) o;
        return Objects.equals(outputId, outputId1.outputId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(outputId);
    }

    @Override
    public String toString() {
        return outputId;
    }
}
