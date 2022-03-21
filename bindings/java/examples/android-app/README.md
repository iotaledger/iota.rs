# Android development

Android development requires a more specific environment setup. In this tutorial we will guide you through the various ways to build an APK with `iota.rs` capabilities.

The first part of this tutorial shows you how to install the required tools for building an apk. Scroll [down](#using-pre-generated-binaries) for a guide using the precompiled binaries found on our release page [here](https://github.com/iotaledger/iota.rs/releases). The Java releases are tagged with `android-binding-vX.Y.Z`.

:::info
Since Android development mainly uses Gradle, we will use that for this tutorial
:::

## Setup by compiling

Compiling an Android app comes with the added task of building for multiple different device architectures.
In this tutorial we mention the followint 2 variables:
`$ARCH` and `$TARGET`.

These variables are used to generate correct binaries during cross compilation and linking.

#### List of target Devices
`$ARCH` and ->  `$TARGET` related to each other in the following manner: 
- `armeabi-v7a` -> `armv7-linux-androideabi`
- `arm64-v8a` -> `aarch64-linux-android`
- `x86` -> `i686-linux-android`
- `x86_64` -> `x86_64-linux-android`

When building an app, you as a developer must decide for how many architectures you make an APK for. Binaries need to be generated for each architecture.

The android-app example has a [build.gradle](../../../../bindings/java/examples/android-app/build.gradle) file that shows the enabled list in a variable called `archTriplets`. You can disable and enable them, as long as you have at least one. (This file will also automatically compile our binaries when we run gradle. Feel free to use it in your project!)

We will use `archTriplets` for the enabled list of device targets during this tutorial.

## Prerequisite

- Dependencies indicated in the [Getting started](getting_started.md) Prerequisite section
- Android NDK or Android Studio with NDK installed (If you extract make sure to make it executable `chmod -R +x android-ndk-VERSION` )

In order to cross compile the binaries for Android; we need the following target toolchains: (All of the enabled `archTriplets`) 
```
rustup target add \
    armv7-linux-androideabi \
    aarch64-linux-android \
    i686-linux-android \
    x86_64-linux-android
```

For this setup we use `$ANDROID_NDK_HOME` for the location of your NDK, wether you use Android studio or manual compilation
1. set `ANDROID_NDK_HOME` environment variable

Example: `export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/{A_VERSION_NUMBER}`

If you dont have `ANDROID_HOME`; Usually found at `/home/user/Android`. Make sure you have installed Android correctly.
Attempt to run `android` on your terminal. If this command does not open the Android SDK Manager, then your path is not configured correctly.

## 1. Generating the java files

### 1.1 Compiling a binary
In order to generate the Java source files; we need to run cargo manually once. 

This step will require you to run `cargo build --release` in `iota.rs/bindings/java`.

:::info
This step is simplifying the process by running an unnecesary build (We compile here for our current system). If you have a working environment already, you can run cargo with a `--target=$TARGET` to save time later on.
:::

### 1.2 Creating the jar
Afterwards, you need to run `./gradlew jar` in `iota.rs/bindings/java` in order to generate the jar file.

The jar will be found at `iota.rs/bindings/java/native/build/libs/native.jar`

## 2. Build the app

Building the actual app can be done through two different ways. Using Android Studio and by manual linking. 

The following 2 Sections describe both methods. 

#### Cross compile note

In order to build on windows, we need to add android triplets to our VCPKG and use that during compilation. 
[TODO]

Currently cross compiling has only worked on WSL/Linux.
If you wish to use Android Studio in Windows, first make the android target binaries in WSL/Linux, then copy them over to `src/main/jniLibs/$TARGET/`. (See step [Compiling a binary](#Adding%20shared%20library), but do that for each enabled target in WSL/Linux)

Afterwards you need to comment out all `archTriplets` in `build.gradle` in order for you not to regenerate them (and fail on Windows).

### 2.1 Android studio

Load the project under the `iota.rs/bindings/java` folder in Android studio.

Make sure you have an NDK and SDK: `file->Project Structure->SDK Location`. If the NDK location is marked grey, edit the `local.properties` like so: (This must be the location of `$ANDROID_NDK_HOME`, which still needs to be on your path)
```
ndk.dir=I\:\\Path\\To\\AndroidSDK\\ndk\\VERSION
```

If youre on linux/wsl, just run the app. On other platforms see the `Setup/Cross compile note` before running.

### 2.2 Manual linking

#### Preparing your terminal/environment

Add the standalone toolchain to the search path.

Example: `export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/{OS}/bin`

Replace `{OS}` with your OS name; on Linux this would be called `linux-x86_64`, windows `windows-x86_64` (`x86_64`depening on architecture type)

These toolchains can alternatively be prepended to the cargo command as well, but we wont discuss that in this tutorial.

#### Setting Cargo config
In order to compile the binaries for the various Android targets, we need to specify the targets to rust.

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

#### Generating the binaries
Now we need to generate binaries for all the enabled targets inside your `build.gradle` `archTriplets`.
The easiest way is to use the gradle build system. Using the `build.gradle` inside `examples/android-app`, we automatically build all enabled targets after running the following:
```
cd iota.rs/bindings/java
./gradlew build
```

Alternatively, you can also manually run the commands; 

1. Compile the binaries for the target 

Example: `cargo build --target aarch64-linux-android --release`


2. Adding shared library

> Copy `$ANDROID_NDK_HOME/sources/cxx-stl/llvm-libc++/libs/$ARCH/libc++_shared.so` to `src/main/libs/$ARCH/`

Example: `cp $ANDROID_NDK_HOME/sources/cxx-stl/llvm-libc++/libs/arm64-v8a/libc++_shared.so examples/android-app/src/main/libs/arm64-v8a`

#### Building your app

Assemble your android app with gradle using:
```
cd iota.rs/bindings/java
./gradlew aR
```

#### Signing your app

1. prepare a signing keystore; we will call it `signed_ks.jks`
> How to make: https://developer.android.com/studio/publish/app-signing#generate-key

2. Sign the apk

> `$ANDROID_HOME/build-tools/{VERSION}/apksigner sign --ks examples/android-app/signed_ks.jks --out examples/android-app/android-app-release-signed.apk -v examples/android-app/build/outputs/apk/release/android-app-release-unsigned.apk`

3. Connect device 
> https://developer.android.com/studio/command-line/adb#connect-to-a-device-over-wi-fi-android-11+

For example:
- `adb pair 192.168.0.x:x` 
- `adb connect 192.168.0.x:x`
- `adb install -r --fastdeploy examples/android-app/android-app-release-signed.apk`
- `adb shell am monitor`

## Using pre-generated binaries

It is very likely you dont want or need to compile by yourself. That is why we provide precompiled binaries found on our release page [here](https://github.com/iotaledger/iota.rs/releases). The Java releases are tagged with `android-binding-vX.Y.Z`. 

Install the files attached to the release so that you achieve the following directory structure: (extract the `jniLibs.zip` into `root_app/src/main/`)

```
root_app/src/main/
  libs/
    native.jar
  jniLibs/
    arm64-v8a/
      libc++_shared.so
      libiota_client_java.so
    armeabi-v7a/
      libc++_shared.so
      libiota_client_java.so
    x86/
      libc++_shared.so
      libiota_client_java.so
    x86_64/
      libc++_shared.so
      libiota_client_java.so
```

### Android studio 
Then using Android Studio, add the native.jar to your project by right clicking -> Add As Library... -> Select your Android app Module and press OK.

### Manual
Add the jar to your `build.gradle` dependencies section using; for example: `implementation files('src\\main\\libs\\native.jar')`

:::info
When trying to build the `android-app` example in the repository with precompiled binaries, comment out all the `archTriplets` inside `iota.rs/bindings/java/examples/android-app/build.gradle`. (Otherwise you will try to regenerate the `.so` files)
:::