cd /home/brord/
sudo apt-get install unzip
wget https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
unzip sdk-tools-linux-4333796.zip -d Android
rm sdk-tools-linux-4333796.zip
sudo apt-get install -y lib32z1 openjdk-8-jdk
export JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64
export PATH=$PATH:$JAVA_HOME/bin
printf "\n\nexport JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64\nexport PATH=\$PATH:\$JAVA_HOME/bin" >> ~/.bashrc
cd Android/tools/bin
./sdkmanager "platform-tools" "platforms;android-26" "build-tools;26.0.3"
export ANDROID_HOME=/home/brord/Android
export PATH=$PATH:$ANDROID_HOME/tools
export PATH=$PATH:$ANDROID_HOME/platform-tools
printf "\n\nexport ANDROID_HOME=/home/brord/Android\nexport PATH=\$PATH:\$ANDROID_HOME/tools\nexport PATH=\$PATH:\$ANDROID_HOME/platform-tools" >> ~/.bashrc
android update sdk --no-ui
sudo apt-get install gradle
gradle -v
adb start-server
