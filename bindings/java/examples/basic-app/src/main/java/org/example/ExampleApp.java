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

        // TODO: Make listeners with the Sync trait
        // iota.subscriber().withTopic(Topic.from("messages")).subscribe(listener);
    }

    public static void consolidate() {
        Client iota = node();

        String seed = "NONSECURE_USE_OF_DEVELOPMENT_SEED_1";

        // Here all funds will be send to the address with the lowest index in the range
        String address = Util.consolidateFunds(iota, seed, 0, 0, 150);

        System.out.println("Funds consolidated to" + address);
    }

    public static void createMaxDust(){
        Client iota = node();
        String seed = "NONSECURE_USE_OF_DEVELOPMENT_SEED_1";
        String seed_2 = "NONSECURE_USE_OF_DEVELOPMENT_SEED_2";

        String[] new_addresses = iota.getAddresses(seed_2).withRange(0, 1).finish();

        Message dustAllowanceMessage = iota
            .message()
            .withSeed(seed)
            .withDustAllowanceOutput(new_addresses[0], 10_000_000)
            .finish();

        MessageWrap[] msgs = iota.retryUntilIncluded(dustAllowanceMessage.id(), -1, -1);

        // Split funds to own addresses
        String[] addresses = iota
            .getAddresses(seed)
            // We start from index 1 so we can send remaining balance to the address with index 0
            .withRange(1, 101)
            .finish();

        ClientMessageBuilder message_builder = iota.message().withSeed(seed);
        for (String address : addresses) {
            // Make sure to re-set the builder as the instance is a clone of the old one due to JNI limits
            message_builder = message_builder.withOutput(address, 1_000_001);
        }
        Message message = message_builder.finish();

        System.out.println(
            "First transaction sent: https://explorer.iota.org/testnet/message/" + message.id()
        );

        msgs = iota.retryUntilIncluded(message.id(), -1, -1);

        // At this point we have 100 Mi on 100 addresses and we will just send it to the final address
        // We use the outputs directly so we don't double spend them
        
        let mut initial_outputs = Vec::new();
        if let Some(Payload::Transaction(tx)) = message.payload() {
            match tx.essence() {
                Essence::Regular(essence) => {
                    for (index, output) in essence.outputs().iter().enumerate() {
                        // Only include 1 Mi outputs, otherwise it fails for the remainder address
                        if let Output::SignatureLockedSingle(output) = output {
                            if output.amount() == 1_000_001 {
                                initial_outputs.push(UtxoInput::new(tx.id(), index as u16)?);
                            }
                        }
                    }
                }
                _ => {
                    panic!("Non-existing essence type");
                }
            }
        }
        String[] first_address_old_seed = iota.getAddresses(seed).withRange(0, 1).finish();
        let mut sent_messages = Vec::new();
        for (index, output) in initial_outputs.into_iter().enumerate() {
            let message_id = iota
                .message()
                .with_seed(&seed)
                .with_input(output)
                .with_input_range(1..101)
                .with_output(&new_address[0], 1)?
                // send remaining iotas back
                .with_output(&first_address_old_seed[0], 1_000_000)?
                .finish()
                .await?
                .id()
                .0;
            println!(
                "Transaction {} sent: https://explorer.iota.org/testnet/message/{}",
                index, message_id
            );
            sent_messages.push(message_id);
        }
        // only check last message, if this gets confirmed all other messages should also be confirmed
        msgs = iota.retryUntilIncluded(sent_messages[sent_messages.length], -1, -1);
        // Send all funds back to first address
        long total_balance = iota.getBalance(seed).finish();

        System.out.println("Total balance: " + total_balance);

        Message message = iota
            .message()
            .withSeed(seed)
            .withOutput(first_address_old_seed[0], total_balance)
            .finish();

        System.out.println("Final tx sent: https://explorer.iota.org/testnet/message/" + message.id());

        msgs = iota.retryUntilIncluded(message.id(), -1, -1);
    }
}
