package node_api_core;

import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.ClientException;
import org.iota.types.Receipt;

public class GetReceipts {
    public static void main(String[] args) throws ClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Get the receipts.
        Receipt[] receipts = client.getReceipts();

        // Print the receipts.
        for (Receipt receipt : receipts)
            System.out.println(receipt);
    }
}