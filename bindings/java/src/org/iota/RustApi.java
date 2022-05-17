package org.iota;

public class RustApi {

    static {
        System.loadLibrary("iota_java");
    }

    private static native String nativeCall(String clientConfig, String clientCommand);

    public static String call(ClientConfig config, ClientCommand command) {
        return RustApi.nativeCall(config.toString(), command.toString());
    }

}

