package org.iota.client;

import org.iota.client.models.NodeInfo;

public class Main {
    
    public static void main(String[] args) {
        Client client = new Client("https://nodes.comnet.thetangle.org");
        NodeInfo nodeInfo = client.getNodeInfo();
        System.out.println(nodeInfo.appName);
        System.out.println(nodeInfo.appVersion);
        System.out.println(nodeInfo.latestMilestoneIndex);
    }
    
}
