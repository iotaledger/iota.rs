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
            String[] addresses = new GetAddressesBuilderApi(seed).withClient(iota).withRange(0, 10).finish();
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

    public static void getMessageMetadata() {
        try {
            Client iota = node();
            Message message = iota.message().finish();

            MessageMetadata metadata = iota.getMessage().metadata(message.id());

            System.out.println("Message metadata: " + metadata);
        } catch (ClientException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    public static void getDataMessage() {
        Client iota = node();

        Message message = iota.message().withIndexString("Hello").withDataString("Tangle").finish();

        System.out.println("Message sent https://explorer.iota.org/testnet/message/" + message.id());

        MessageId[] fetched_message_ids = iota.getMessage().indexString("Hello");
        System.out.println("Messages with Hello index: " + Arrays.toString(fetched_message_ids));
    }

    public static void transaction() {
        Client iota = node();

        String seed_1 = "NONSECURE_USE_OF_DEVELOPMENT_SEED_1";

        Message message = iota
            .message()
            .withSeed(seed_1)
            // Insert the output address and amount to spent. The amount cannot be zero.
            .withOutput(
                // We generate an address from our seed so that we send the funds to ourselves
                        iota.getAddresses(seed_1).withRange(0, 1).finish()[0], 1000000
            ).finish();

        System.out.println("Transaction sent: https://explorer.iota.org/testnet/message/" +  message.id());
    }

    public static void mqtt() {
        Client iota = node();

        MqttListener listener = new MqttListener() {
            @Override
            public void onEvent(TopicEvent event) {
                System.out.println(event);
            }
        };

        iota.subscriber().withTopic(new Topic("messages")).subscribe(listener);
    }

}
