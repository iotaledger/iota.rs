import org.iota.Client;
import org.iota.apis.NodeIndexerApi;
import org.iota.types.Block;
import org.iota.types.expections.ClientException;
import org.iota.types.ids.BlockId;
import org.iota.types.ids.MilestoneId;
import org.iota.types.ids.OutputId;

public class ExampleUtils {
    public static BlockId setUpBlockId(Client client) throws ClientException {
        return client.getTips()[0];
    }

    public static Block setUpBlock(Client client) throws ClientException {
        return client.getBlock(setUpBlockId(client));
    }

    public static byte[] setUpBlockRaw(Client client) throws ClientException {
        return client.getBlockRaw(setUpBlockId(client));
    }

    public static MilestoneId setUpMilestoneId(Client client) throws ClientException {
        return new MilestoneId(client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("confirmedMilestone").getAsJsonObject().get("milestoneId").getAsString());
    }

    public static int setUpMilestoneIndex(Client client) throws ClientException {
        return client.getNodeInfo().getNodeInfo().get("status").getAsJsonObject().get("confirmedMilestone").getAsJsonObject().get("index").getAsInt();
    }

    public static OutputId setUpOutputId(Client client) throws ClientException {
        return client.getBasicOutputIds(new NodeIndexerApi.QueryParams()).getItems()[0];
    }
}