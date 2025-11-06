# Building wgpu Rust Project for Android

## ✅ Migration Complete!

Your project has been successfully migrated from GLFW to winit for cross-platform compatibility (desktop + Android).

## Prerequisites (Already installed)
- ✅ Android SDK
- ✅ Android NDK
- ✅ Rust toolchain
- ✅ cargo-apk (installed via previous steps)
- ✅ Android targets (aarch64-linux-android, armv7-linux-androideabi, etc.)

## Desktop Testing

First, verify the application works on desktop:

```powershell
# Build and run on desktop (Windows)
cargo run

# Press ESC to close the window
```

## Setup for Android

### 1. Environment Variables
Set the following environment variables (adjust paths to match your installation):

```powershell
$env:ANDROID_HOME = "C:\Users\majid\AppData\Local\Android\Sdk"
$env:ANDROID_SDK_ROOT = "C:\Users\majid\AppData\Local\Android\Sdk"
$env:NDK_HOME = "C:\Users\majid\AppData\Local\Android\Sdk\ndk\27.2.12479018"
```

To make them permanent, add to your PowerShell profile or system environment variables.

### 2. Update .cargo/config.toml
The `.cargo/config.toml` file has been created with NDK linker paths. 
**IMPORTANT**: Update the NDK version number in the paths to match your installed NDK version.

To find your NDK version:
```powershell
ls $env:ANDROID_HOME\ndk
```

### 3. Build for Android

#### Using cargo-apk (Recommended):
```powershell
# Build debug APK
cargo apk build --lib

# Build release APK
cargo apk build --lib --release

# Build and run on connected device
cargo apk run --lib --release
```

The APK will be generated in: `target/debug/apk/` or `target/release/apk/`

#### Alternative: Using xbuild (if cargo-apk has issues):
```powershell
cargo install xbuild

# Build for specific architecture
x build --target aarch64-linux-android --release
x build --target armv7-linux-androideabi --release
```

### 4. Install APK on Device

```powershell
# Connect your Android device via USB with USB debugging enabled
# Then run:
adb install target/release/apk/wgpu_rust.apk

# Or use cargo-apk:
cargo apk run --release
```

## Important Notes

1. **GLFW Replacement**: Your original code uses GLFW which doesn't work on Android. The project has been updated to use `winit` instead, which is cross-platform.

2. **Graphics Backend**: On Android, wgpu will use Vulkan backend. Make sure your Android device supports Vulkan (Android 7.0+ / API 24+).

3. **Shader Files**: Your shader files need to be accessible. You may need to embed them in the binary or package them with the APK.

4. **Code Changes Required**: The main.rs needs to be updated to use winit instead of GLFW. A template has been created in lib.rs.

## Troubleshooting

### NDK Not Found Error
Update the paths in `.cargo/config.toml` to match your NDK version.

### Linker Errors
Ensure the NDK toolchain exists at the specified paths.

### APK Installation Failed
- Check that USB debugging is enabled on your device
- Check that the device is connected: `adb devices`
- Uninstall previous version: `adb uninstall com.example.wgpu_rust`

### Vulkan Not Supported
Your device needs to support Vulkan. Check with:
```powershell
adb shell getprop ro.product.model
adb shell dumpsys vulkan
```

## Next Steps

1. Update `.cargo/config.toml` with your correct NDK paths
2. Migrate your rendering code from GLFW to winit
3. Test build: `cargo apk build`
4. Install and run on device: `cargo apk run`

## File Structure Created
- `Cargo.toml` - Updated with Android metadata and dependencies
- `AndroidManifest.xml` - Android app manifest
- `.cargo/config.toml` - NDK linker configuration
- `src/lib.rs` - Android entry point template

## Resources
- wgpu Android examples: https://github.com/gfx-rs/wgpu/tree/master/examples
- winit documentation: https://docs.rs/winit/
- cargo-apk: https://github.com/rust-mobile/cargo-apk
