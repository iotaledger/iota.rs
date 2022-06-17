package org.iota.types;

public class ClientException extends Exception {

    private String methodName;

    public ClientException(String methodName, String message) {
        super(message);
    }

    public String getMethodName() {
        return methodName;
    }

}
