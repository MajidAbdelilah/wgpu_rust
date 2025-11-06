# Quick Reference: Building for Android

## ✅ Prerequisites Complete
- Android SDK installed
- Android NDK installed
- Rust targets added
- cargo-apk installed
- Code migrated from GLFW to winit

## Before Building for Android

### 1. Find Your NDK Version
```powershell
ls $env:ANDROID_HOME\ndk
# Example output: 27.2.12479018
```

### 2. Update `.cargo/config.toml`
Replace the NDK version in all paths:
```toml
# Change this version number to match your NDK:
C:\\Users\\majid\\AppData\\Local\\Android\\Sdk\\ndk\\27.2.12479018\\...
```

### 3. Set Environment Variables
```powershell
$env:ANDROID_HOME = "C:\Users\majid\AppData\Local\Android\Sdk"
$env:NDK_HOME = "$env:ANDROID_HOME\ndk\<YOUR_NDK_VERSION>"
```

## Build Commands

### Desktop (Test First!)
```powershell
# Debug build
cargo build

# Run on desktop
cargo run

# Release build
cargo build --release
cargo run --release
```

### Android
```powershell
# Build APK (debug)
cargo apk build --lib

# Build APK (release)
cargo apk build --lib --release

# Install and run on device
cargo apk run --lib --release

# Just install APK
adb install target/release/apk/wgpu_rust.apk
```

## Automated Build Script
Use the provided script:
```powershell
.\build-android.ps1
```

## Device Preparation

### Enable USB Debugging
1. Go to Settings → About Phone
2. Tap "Build Number" 7 times
3. Go to Settings → Developer Options
4. Enable "USB Debugging"

### Connect Device
```powershell
# Check device is connected
adb devices

# View logs from your app
adb logcat -s wgpu_rust
```

## Troubleshooting

### "NDK not found"
→ Update `.cargo/config.toml` with correct NDK path

### "No devices found"
→ Enable USB debugging and run `adb devices`

### "APK installation failed"
→ Uninstall old version: `adb uninstall com.example.wgpu_rust`

### "Vulkan not supported"
→ Your device needs Android 7.0+ with Vulkan support
→ Check: `adb shell dumpsys vulkan`

### Build errors
→ Run `cargo clean` then rebuild
→ Verify all targets installed: `rustup target list --installed`

## File Locations

### APK Output
- Debug: `target/debug/apk/wgpu_rust.apk`
- Release: `target/release/apk/wgpu_rust.apk`

### Logs
- Desktop: Console output
- Android: `adb logcat`

## Key Differences: Desktop vs Android

| Feature | Desktop | Android |
|---------|---------|---------|
| Graphics Backend | DX12/Vulkan/Metal | Vulkan only |
| Entry Point | `main()` | `android_main()` |
| Logger | `env_logger` | `android_logger` |
| Debugging | cargo run | adb logcat |
| File Access | Direct filesystem | Android storage APIs |

## Important Notes

1. **First time**: Build may take 10-15 minutes (compiling all deps)
2. **Subsequent builds**: Much faster (incremental compilation)
3. **Release builds**: Always use `--release` for Android (much faster)
4. **Testing**: Test on desktop first before building for Android
5. **Shaders**: Embedded in binary, no need to package separately

## Resources

- Full guide: `ANDROID_BUILD_GUIDE.md`
- Migration details: `MIGRATION_SUMMARY.md`
- Build script: `build-android.ps1`

## Quick Test

```powershell
# 1. Test desktop
cargo run

# 2. Build for Android
cargo apk build --lib --release

# 3. Deploy to device
cargo apk run --lib --release

# 4. View logs
adb logcat -s wgpu_rust
```

That's it! 🚀
