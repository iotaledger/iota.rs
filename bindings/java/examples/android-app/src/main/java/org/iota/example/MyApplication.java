package org.iota.example;

import android.app.Application;
import android.content.Context;
import android.os.Environment;
import android.util.Log;

import java.io.File;
import java.nio.file.Paths;
import java.util.Arrays;
import java.nio.file.Path;

import org.iota.client.*;
import org.iota.client.local.*;

public final class MyApplication extends Application {
    private static MyApplication sSelf;
    private static final String TAG = "Client.rs";

    private Account account;

    public MyApplication(Context context) {
        super();
        sSelf = this;

        NativeAPI.verifyLink();

        // Beware: All builder patterns return NEW instances on each method call.
        // Mutating the old builder after a builder call will not result in a change on
        // the second call
        // This is due to the JNI bindings not beeing able to call non-reference methods
        // in rust
        // Example that doesnt work:
        // AccountManagerBuilder builder = AccountManager.Builder();
        // builder.withStorage(storageFolder.toString(), null);
        // AccountManager manager = builder.finish();
        //
        // Explanation: builder.withStorage returns a new builder instance, and .finish
        // is called on the old one
        Client iota = Client.Builder().withNode(nodeUrl) // Insert your node URL here
            // .withNodeAuth("https://somechrysalisiotanode.com", "jwt_or_null",
            // "name_or_null", "password_or_null") //
            // Optional authentication
            .withLocalPow(true)
            .finish();

        NodeInfoWrapper info = iota.getInfo();
        System.out.println("Node url: " + info.url());
        System.out.println("Node Info: " + info.nodeInfo());
    }
}
