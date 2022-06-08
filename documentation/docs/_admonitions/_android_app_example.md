:::info android-app-example

When trying to build the `android-app` example in the repository with precompiled binaries, comment out all the `archTriplets` inside `iota.rs/bindings/java/examples/android-app/build.gradle` (otherwise you will try to regenerate the `.so` files).

:::
