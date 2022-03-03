# Dependencies:
For this setup we use `$ANDROID_NDK_HOME` for the location of your NDK, wether you use Android studio or manual compilation

- Dependencies of the Iota.rs README.md for compiling iota.rs normally
- Java & JDK (Make sure JAVA_HOME env variable) is set
- Android NDK or Android Studio with NDK installed (If you extract make sure to make it executable `chmod -R +x android-ndk-VERSION` )
- [Rustup]
- Cargo ndk (`cargo install cargo-ndk`)
- Cargo fmt (`rustup component add rustfmt`)

Android Toolchains: 
```
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```

# Setup

## Generating the java files
In order to generate the Java source files; we need to run cargo manually once. 

This step will require you to run `cargo build --release --target=$TARGET` in `iota.rs/bindings/java`.

Replace `$TARGET` with one of the enabled targets inside you `build.gradle` `archTriplets` (options are `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`)

Afterwards, you need to run `./gradlew jar` in `iota.rs/bindings/java` in order to generate the jar file.
The jar will be found at `iota.rs/bindings/java/native/build/libs/native.jar`

## Android studio

Load the project under the `iota.rs/bindings/java` folder in Android studio.

Make sure you have an NDK and SDK: `file->Project Structure->SDK Location`. If the NDK location is marked grey, edit the local.properties like so: (This must be the location of `$ANDROID_NDK_HOME`, which still needs to be on your path)
```
ndk.dir=I\:\\Path\\To\\AndroidSDK\\ndk\\VERSION
```

If youre on linux/wsl, just run the app. On other platforms see the `Setup/Cross compile note` before running.

### Cross compile note

In order to build on windows, we need to add android triplets to our VCPKG and use that during compilation. 
[TODO]

Currently cross compiling has only worked on WSL/Linux.
If you wish to use android studio in windows, first make the android target binaries in WSL/Linux, then copy them over to `src/main/jniLibs/$TARGET/`. (See step "Generating the java files", but do that for each enabled target in WSL/Linux)

Afterwards you need to comment out all `archTriplets` in `build.gradle` in order for you not to regenerate them (and fail on Windows).

## Manual

set `ANDROID_NDK_HOME` environment variable

Example: `export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/{A_VERSION_NUMBER}`

If you dont have `ANDROID_HOME`; Usually found at `/home/user/Android`

Add the standalone toolchain to the search path.
`export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/{ARCH}/bin`

Replace {ARCH} with your OS folder; linux would be called `linux-x86_64`, windows `windows-x86_64`

### Generating the binaries

In order to compile the binaries for the various Android targets, we need to specify the targets.

Create or update the following file: `~/.cargo/config`.
Replace each instance of `$ANDROID_NDK_HOME` with the actual location (variables do not work) and add the text below to the config file.
```
[target.armv7-linux-androideabi]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"

[target.aarch64-linux-android]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"

[target.i686-linux-android]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang"
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"

[target.x86_64-linux-android]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang"
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-ar"
```

Then in `iota.rs/bindings/java`, proceed to run 

`BINDGEN_EXTRA_CLANG_ARGS="--target=$TARGET" cargo build --release --target=$TARGET` 

for each of the enabled targets inside your `build.gradle` `archTriplets`.

Replace both `$TARGET` with each archTriplets values (`armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`)

### Adding shared library
For each target you enable in `build.gradle` `archTriplets` do the following:
> Copy `$ANDROID_NDK_HOME/sources/cxx-stl/llvm-libc++/libs/$ARCH/libc++_shared.so`
> to `src/main/libs/$ARCH/`

`$ARCH` Should be replaced with each enabled `archTriplets` key. (options are armeabi-v7a, arm64-v8a, x86, x86_64)


### Building and testing

Assemble your app with gradle using:
```
cd iota.rs/bindings/java
./gradlew aR
```

Have a signing keystore ready; I call it `signed_apk.jks`
How to make: https://developer.android.com/studio/publish/app-signing#generate-key

Sign the apk:
`$ANDROID_HOME/build-tools/{VERSION}/apksigner sign --ks examples/android-app/signed_apk.jks --out examples/android-app/android-app-release-signed.apk -v examples/android-app/build/outputs/apk/release/android-app-release-unsigned.apk`

Connect device (https://developer.android.com/studio/command-line/adb#connect-to-a-device-over-wi-fi-android-11+)

`adb pair 192.168.0.x:x` 

`adb connect 192.168.0.x:x`

Run on device:
`adb install -r --fastdeploy examples/android-app/android-app-release-signed.apk`

Monitor app start:
`adb shell am monitor`