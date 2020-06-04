package org.iota.client.models;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class NodeInfo extends Structure {

  public static class ByReference extends NodeInfo implements Structure.ByReference {
  }

  public static class ByValue extends NodeInfo implements Structure.ByValue {
  }

  public String appName;
  public String appVersion;
  public int latestMilestoneIndex;

  @Override
  protected List<String> getFieldOrder() {
    return Arrays.asList("appName", "appVersion", "latestMilestoneIndex");
  }

}