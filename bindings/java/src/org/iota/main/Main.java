package org.iota.main;

import org.iota.main.types.ClientConfig;

public class Main {

    public static void main(String[] args) {
        Client c = new Client(new ClientConfig("{ \"nodes\": [\"https://api.alphanet.iotaledger.net\" ]}"));
        System.out.println(c.getNodeInfo());
        System.out.println(c.getHealth("https://api.alphanet.iotaledger.net"));
        System.out.println(c.getTips());
        System.out.println(c.getMessage("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getMessageRaw("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getMessageMetadata("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getMessageChildren("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getOutput("0x00000000000000000000000000000000000000000000000000000000000000000000"));
        System.out.println(c.getReceiptsMigratedAt(5));
        System.out.println(c.getReceipts());
        System.out.println(c.getTreasury());
        System.out.println(c.getIncludedMessage("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getMilestoneById("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getMilestoneByIndex(3));
        System.out.println(c.getUtxoChangesByIndex(3));
        System.out.println(c.getUtxoChangesById("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        //System.out.println(c.getMilestoneByIdRaw("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.getPeers());

        System.out.println(c.getOutputs(new String[]{"0x0d6f9fa4260ca292c737e45335e3d6d4098424854b1c01f02b20ce50618bd7910200", "0x0d6f9fa4260ca292c737e45335e3d6d4098424854b1c01f02b20ce50618bd7910300"}));
        System.out.println(c.tryGetOutputs(new String[]{"0x0d6f9fa4260ca292c737e45335e3d6d4098424854b1c01f02b20ce50618bd7910200", "0x0d6f9fa4260ca292c737e45335e3d6d4098424854b1c01f02b20ce50618bd7910300"}));
        //System.out.println(c.findMessages(new String[] {"0x0d6f9fa4260ca292c737e45335e3d6d4098424854b1c01f02b20ce50618bd7910200","0x0d6f9fa4260ca292c737e45335e3d6d4098424854b1c01f02b20ce50618bd7910300"}));
        //System.out.println(c.retry("0x0d6f9fa4260ca292c737e45335e3d6d4098424854b1c01f02b20ce50618bd7910200"));
        System.out.println(c.retryUntilIncluded("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3", 1, 2));
        System.out.println(c.findInputs(new String[]{"0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"}, 2));
        System.out.println(c.reattach("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.reattachUnchecked("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.promote("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
        System.out.println(c.promoteUnchecked("0x6832742bb65e05b5b4a469a8e89f9dd551199f4316538f37b4b39d83043029a3"));
    }
}
