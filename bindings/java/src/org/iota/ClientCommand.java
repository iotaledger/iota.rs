package org.iota;

public class ClientCommand {
    private ClientCommandType cmd;
    private String payload;

    public ClientCommand(ClientCommandType cmd, String payload) {
        this.cmd = cmd;
        this.payload = payload;
    }

    @Override
    public String toString() {
        return "{ \"cmd\": \"" + cmd.toString() + "\", \"payload\" :" + payload + "}";
    }
}