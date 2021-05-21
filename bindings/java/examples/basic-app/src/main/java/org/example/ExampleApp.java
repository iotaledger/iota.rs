package org.example;

import java.util.Arrays;

import org.iota.client.*;
import org.iota.client.local.*;

public class ExampleApp {

    static {
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

    private static Client node() {
        String nodeUrl = "https://chrysalis-nodes.iota.cafe:443";
        Client iota = Client.Builder().withNode(nodeUrl) // Insert your node URL here
                // .withNodeSyncDisabled()
                // .with_node_auth("https://somechrysalisiotanode.com", "name", "password") //
                // Optional authentication
                .finish();
        return iota;
    }

    public static void nodeInfo() {
        try {
            String nodeUrl = "https://chrysalis-nodes.iota.cafe:443";
            Client iota = node();

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
            Client iota = node();

            String seed = "NONSECURE_USE_OF_DEVELOPMENT_SEED_1";
            String[] addresses = new GetAddressesBuilderApi(seed).with_client(iota).with_range(0, 10).finish();
            System.out.println(Arrays.toString(addresses));
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    public static void getBalance() {
        try {
            Client iota = node();

            String seed = "NONSECURE_USE_OF_DEVELOPMENT_SEED_1";

            long seed_balance = iota.getBalance(seed).finish();
            System.out.println("Account balance: " + seed_balance);

            String address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

            BalanceAddressResponse response = iota.getAddress().balance(address);
            System.out.println("The balance of " + address + " is " + response.balance());

            UtxoInput[] outputs = iota.getAddress().outputs(address, new OutputsOptions());
            System.out.println("The outputs of address " + address + " are: " + Arrays.toString(outputs));
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    public static void getOutputs() {
        try {
            Client iota = node();

            String address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

            UtxoInput[] outputs = iota.getAddress().outputs(address, new OutputsOptions());
            System.out.println("The outputs of address " + address + " are: " + Arrays.toString(outputs));
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    public static void simpleMessage() {
        try {
            Client iota = node();
            Message message = iota.message().finish();

            System.out.println(
                    "Empty message sent: https://explorer.iota.org/mainnet/message/" + message.id().toString());
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
