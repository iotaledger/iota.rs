package org.iota.main.types.responses;

import com.google.gson.JsonArray;
import org.iota.main.types.Peer;

public class PeersResponse extends ClientResponse {

    private Peer[] peers;

    public PeersResponse(BaseApiResponse response) {
        super(response);

        JsonArray peers = response.getPayload().getAsJsonArray();
        this.peers = new Peer[peers.size()];
        for (int i = 0; i < peers.size(); i++) {
            this.peers[i] = new Peer(peers.get(i).getAsJsonObject());
        }
    }

    public Peer[] getPeers() {
        return peers;
    }

    @Override
    public String toString() {
        return "PeersResponse{" +
                "response=" + response +
                '}';
    }
}
