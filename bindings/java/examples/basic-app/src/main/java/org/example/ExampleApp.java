package org.example;

import java.nio.file.Paths;
import java.util.Arrays;
import java.nio.file.Path;

import org.iota.client.*;
import org.iota.client.local.*;

public class ExampleApp {
    public static void main(String[] args) {

        try {
            new ExampleApp();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public ExampleApp() {
        try {
            NativeAPI.verifyLink();

            String nodeUrl = "https://chrysalis-nodes.iota.cafe:443";
            Client iota = Client.Builder().withNode(nodeUrl) // Insert your node URL here
                    // .withNodeSyncDisabled()
                    // .with_node_auth("https://somechrysalisiotanode.com", "name", "password") //
                    // Optional authentication
                    .finish();

            System.out.println("Node healthy: " + iota.getHealth());

            NodeInfoWrapper info = iota.getInfo();
            System.out.println("Node url: " + info.getUrl());
            System.out.println("Node Info: " + info.nodeInfo());
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
