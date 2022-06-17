package org.iota.types;

import java.util.Objects;

public class MilestoneId {

    private String milestoneId;

    public MilestoneId(String milestoneId) {
        this.milestoneId = milestoneId;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        MilestoneId milestoneId1 = (MilestoneId) o;
        return Objects.equals(milestoneId, milestoneId1.milestoneId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(milestoneId);
    }

    @Override
    public String toString() {
        return milestoneId;
    }
}
