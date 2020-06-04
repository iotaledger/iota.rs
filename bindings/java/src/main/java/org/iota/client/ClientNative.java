package org.iota.client;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;

import org.iota.client.models.NodeInfo;

public interface ClientNative extends Library {
    String JNA_LIBRARY_NAME = "iota";
    NativeLibrary JNA_NATIVE_LIB = NativeLibrary.getInstance(JNA_LIBRARY_NAME);

    ClientNative INSTANCE = Native.loadLibrary(JNA_LIBRARY_NAME, ClientNative.class);

    void iota_init(String url);
    NodeInfo iota_get_node_info();
}
