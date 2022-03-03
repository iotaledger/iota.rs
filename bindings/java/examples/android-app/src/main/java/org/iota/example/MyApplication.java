package org.iota.example;

import android.app.Application;
import android.content.Context;
import android.os.Environment;
import android.util.Log;

import java.io.File;
import java.nio.file.Paths;
import java.util.Arrays;
import java.nio.file.Path;

import org.iota.wallet.*;
import org.iota.wallet.local.*;

public final class MyApplication extends Application {
    private static MyApplication sSelf;
    private static final String TAG = "Wallet.rs";

    private Account account;

    public MyApplication(Context context) {
        super();
        sSelf = this;

        NativeAPI.verifyLink();

        File folder = context.getExternalFilesDir(null);

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
        AccountManagerBuilder builder = AccountManager.Builder().withStorage(folder.getPath(), null);

        AccountManager manager = builder.finish();
        manager.setStrongholdPassword("YepThisISSecure");

        // Generate your own for peristance:
        // String mnemonic = manager.generateMnemonic();

        // null means "generate one for me"
        manager.storeMnemonic(AccountSignerType.STRONGHOLD, null);

        BrokerOptions mqtt = new BrokerOptions();

        ClientOptions clientOptions = new ClientOptionsBuilder()
                .withNode("https://chrysalis-nodes.iota.cafe:443")
                .withMqttBrokerOptions(mqtt)
                .build();

        this.account = manager
                .createAccount(clientOptions)
                .signerType(AccountSignerType.STRONGHOLD)
                .alias("alias1")
                .initialise();
    }
}
