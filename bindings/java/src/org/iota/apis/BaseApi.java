package org.iota.apis;

import org.iota.ClientConfig;

public class BaseApi {

    protected ClientConfig clientConfig;

    protected BaseApi(ClientConfig clientConfig) {
        this.clientConfig = clientConfig;
    }

    static {
        System.loadLibrary("iota_client");
    }

    private static native String callNativeLibrary(String clientConfig, String clientCommand);

    protected String callBaseApi(ClientCommand command) {
        return callNativeLibrary(clientConfig.toString(), command.toString());
    }

    protected static class ClientCommand {

        private CommandType commandType;
        private String payload;

        public ClientCommand(CommandType commandType, String payload) {
            this.commandType = commandType;
            this.payload = payload;
        }

        @Override
        public String toString() {
            return "{ \"cmd\": \"" + commandType.toString() + "\", \"payload\" :" + payload + "}";
        }

        protected enum CommandType {
            CallClientMethod
        }
    }
}
