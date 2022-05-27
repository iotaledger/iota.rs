package org.iota.main.types.responses.node_core_api;

import com.google.gson.JsonArray;
import org.iota.main.types.Peer;
import org.iota.main.types.responses.BaseApiResponse;
import org.iota.main.types.responses.ClientResponse;

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

}
