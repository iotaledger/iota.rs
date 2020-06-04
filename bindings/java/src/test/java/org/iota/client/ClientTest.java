package org.iota.client;

import org.junit.Test;
import org.junit.Before;

import static org.hamcrest.Matchers.is;
import static org.junit.Assert.assertThat;

import org.iota.client.models.NodeInfo;

public class ClientTest {

    Client client;

    @Before
    public void setUp() {
        client = new Client("https://nodes.comnet.thetangle.org");
    }

    @Test
    public void shouldGetNodeInfo() {
        NodeInfo nodeInfo = client.getNodeInfo();
        assertThat(nodeInfo.appName, is("IRI Comnet"));
    }
}
