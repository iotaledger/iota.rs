package org.example;

import org.iota.client.*;
import org.iota.client.local.*;

public class ExampleApp {

    static {
        System.out.println("hello?");
        NativeAPI.verifyLink();
    }

    public static void main(String[] args) {

        try {
            new ExampleApp();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public ExampleApp() {

    }

    public static void nodeInfo() {
        try {
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

    public static void generateSeed() {
        try {
            SecretKey secret_key = SecretKey.generate();
            System.out.println(RustHex.encode(secret_key.toLeBytes()));
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    public static void generateAddresses() {
        try {
            String nodeUrl = "https://chrysalis-nodes.iota.cafe:443";
            Client iota = Client.Builder().withNode(nodeUrl) // Insert your node URL here
                    // .withNodeSyncDisabled()
                    // .with_node_auth("https://somechrysalisiotanode.com", "name", "password") //
                    // Optional authentication
                    .finish();

        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
