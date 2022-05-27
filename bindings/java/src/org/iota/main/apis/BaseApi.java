package org.iota.main.apis;

import com.google.gson.Gson;
import org.iota.main.types.*;
import org.iota.main.types.responses.*;

public class BaseApi {

    protected ClientConfig clientConfig;

    protected BaseApi(ClientConfig clientConfig) {
        this.clientConfig = clientConfig;
    }

    static {
        System.loadLibrary("iota_client");
    }

    private static native String callNativeLibrary(String clientConfig, String clientCommand);

    protected ClientResponse callBaseApi(ClientCommand command) throws ClientException {
        System.out.println(command);
        BaseApiResponse response = new Gson().fromJson(callNativeLibrary(clientConfig.toString(), command.toString()), BaseApiResponse.class);
        System.out.println(response);

        switch (response.getType()) {
            case "Panic":
                throw new RuntimeException(response.toString());
            case "Error":
                throw new ClientException(command.methodName, response.getPayload().getAsJsonObject().toString());
                // Node Core API responses
            case "Health": {
                return new HealthResponse(response);
            }
            case "Info": {
                return new NodeInfoResponse(response);
            }
            case "Tips": {
                return new TipsResponse(response);
            }
            case "PostBlockSuccessful": {
                return new PostBlockResponse(response);
            }
            case "Block":
            case "IncludedBlock":
            case "GeneratedBlock": {
                return new BlockResponse(response);
            }
            case "BlockRaw": {
                return new BlockRawResponse(response);
            }
            case "BlockMetadata": {
                return new BlockMetadataResponse(response);
            }
            case "BlockChildren": {
                return new BlockChildrenResponse(response);
            }
            case "Output": {
                return new OutputResponse(response);
            }
            case "OutputMetadata": {
                return new OutputMetadataResponse(response);
            }
            case "ReceiptsMigratedAtMilestone": {
                return new ReceiptsMigratedAtResponse(response);
            }
            case "Receipts": {
                return new ReceiptsResponse(response);
            }
            case "Treasury": {
                return new TreasuryResponse(response);
            }
            case "Milestone": {
                return new MilestoneResponse(response);
            }
            case "MilestoneRaw": {
                return new MilestoneRawResponse(response);
            }
            case "MilestoneUtxoChanges": {
                return new UtxoChangesResponse(response);
            }
            case "Peers": {
                return new PeersResponse(response);
            }
            case "GeneratedAddresses": {
                return new GenerateAddressesResponse(response);
            }
            case "Faucet": {
                return new FaucetResponse(response);
            }
            case "Bech32ToHex": {
                return new Bech32ToHexResponse(response);
            }
            case "TransactionId": {
                return new TransactionIdResponse(response);
            }

            default: {
                throw new RuntimeException("no match: " + response.getType());
            }
        }
    }

    protected static class ClientCommand {

        private CommandType commandType;
        private String methodName;
        private String methodParams;


        public ClientCommand(CommandType commandType, String methodName) {
            this.commandType = commandType;
            this.methodName = methodName;
        }

        public ClientCommand(CommandType commandType, String methodName, String methodParams) {
            this.commandType = commandType;
            this.methodName = methodName;
            this.methodParams = methodParams;
        }

        @Override
        public String toString() {
            return "{\"cmd\":\"" + commandType.toString() + "\",\"payload\":{\"name\":\"" + methodName + "\"" + (methodParams != null ? ",\"data\":" + methodParams : "") + "}}";
        }

        protected enum CommandType {
            CallClientMethod
        }
    }
}