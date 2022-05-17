package org.iota;

public class Main {

    public static void main(String[] args) {
        Client c = new Client(new ClientConfig(
                "{\"primaryNode\":null,\"primaryPoWNode\":null,\"nodes\":[\"https://api.alphanet.iotaledger.net\"],\"permanodes\":null,\"nodeSyncEnabled\":true,\"nodeSyncInterval\":{\"secs\":60,\"nanos\":0},\"quorum\":false,\"minQuorumSize\":3,\"quorumThreshold\":66,\"network\":null,\"networkId\":null,\"bech32HRP\":\"rms\",\"minPoWScore\":4000.0,\"localPow\":true,\"fallbackToLocalPow\":true,\"tipsInterval\":15,\"rentStructure\":{\"vByteCost\":500,\"vByteFactorKey\":10,\"vByteFactorData\":1},\"apiTimeout\":{\"secs\":15,\"nanos\":0},\"remotePowTimeout\":{\"secs\":100,\"nanos\":0},\"offline\":false,\"powWorkerCount\":null}"
        ));

        System.out.println(c.getNodeInfo());
        System.out.println(c.getHealth("https://api.alphanet.iotaledger.net"));
        System.out.println(c.getTips());
        System.out.println(c.getMessage("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getMessageMetadata("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
    }
}
