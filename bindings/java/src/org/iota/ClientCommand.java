package org.iota;

public class ClientCommand {
    private Command cmd;
    private String payload;

    public ClientCommand(Command cmd, String payload) {
        this.cmd = cmd;
        this.payload = payload;
    }

    @Override
    public String toString() {
        return "{ \"cmd\": \"" + cmd.toString() + "\", \"payload\" :" + payload + "}";
    }
}

enum Command {
    CallClientMethod,
}