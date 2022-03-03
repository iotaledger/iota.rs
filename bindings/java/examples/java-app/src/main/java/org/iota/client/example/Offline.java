package org.iota.client.example;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;

import org.iota.client.*;

public class Offline {

    private Client offline_client;
    private String seed;
    private String url;

    public Offline(String url, String seed) {
        this.offline_client = Client.Builder().withNode(url).withOfflineMode().finish();
        this.seed = seed;
        this.url = url;
    }

    // Generate addresses for our seed, if you aready know adress, this isnt needed
    public String[] generateAddresses() {
        return offline_client.getAddresses(seed).withRange(0, 10).withBech32Hrp("atoi").finish();
    }

    // This uses an online client to find the inputs of the addresses
    public String prepareTransaction(String[] inputAddresses, String toAddress, long amount) {
        UtxoInput[] inputs = this.findInputsOnline(inputAddresses, amount);

        ClientMessageBuilder transactionBuilder = offline_client.message();
        for (UtxoInput input : inputs) {
            transactionBuilder = transactionBuilder.withInput(input);
        }

        PreparedTransactionData preparedTransactionData = transactionBuilder.withOutput(toAddress, amount)
                .prepareTransaction();

        return preparedTransactionData.serialize();
    }

    private UtxoInput[] findInputsOnline(String[] inputAddresses, long forAmount) {
        Client online_client = Client.Builder().withNode(this.url).finish();

        return online_client.findInputs(inputAddresses, forAmount);
    }

    public String signTransaction(String preparedTransactionString) {
        PreparedTransactionData preparedTransaction = PreparedTransactionData.deserialize(preparedTransactionString);
        MessagePayload signedTransaction = offline_client.message().signTransaction(preparedTransaction, this.seed, 0,
                100);

        return signedTransaction.serialize();
    }

    public Message sendMessage(String messagePayloadString) {
        MessagePayload signedPayload = MessagePayload.deserialize(messagePayloadString);

        // Send online!
        Client online_client = Client.Builder().withNode(this.url).finish();
        return online_client.message().finish(signedPayload);
    }
}
