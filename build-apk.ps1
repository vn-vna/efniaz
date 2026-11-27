Set-Location E:\Projects\efniaz

# Use the Android tools already installed with Unity.
$android = "$env:ProgramFiles\Unity\Hub\Editor\6000.0.72f1\Editor\Data\PlaybackEngines\AndroidPlayer"
$env:ANDROID_HOME = "$android\SDK"
$env:ANDROID_NDK_HOME = "$android\NDK"
$env:JAVA_HOME = "$android\OpenJDK"
$env:CARGO_APK_RELEASE_KEYSTORE = "$env:USERPROFILE\.android\debug.keystore"
$env:CARGO_APK_RELEASE_KEYSTORE_PASSWORD = "android"

# Use the cargo-apk installation from the scaffold verification.
$env:Path = "$env:LOCALAPPDATA\Temp\opencode\efniaz-cargo-apk\bin;$env:Path"

cargo apk build -p wear-os --lib --release
cargo apk build -p companion --lib --release
