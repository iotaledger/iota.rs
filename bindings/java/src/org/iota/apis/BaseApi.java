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
