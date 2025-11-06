@echo off
REM Android Build Script for wgpu_rust
REM Run this script from the project root

echo Setting environment variables for current session...
setx ANDROID_HOME "C:\Users\majid\AppData\Local\Android\Sdk" >nul 2>&1
setx NDK_HOME "C:\Users\majid\AppData\Local\Android\Sdk\ndk\27.1.12297006" >nul 2>&1

REM Set for current session as well
set ANDROID_HOME=C:\Users\majid\AppData\Local\Android\Sdk
set ANDROID_SDK_ROOT=C:\Users\majid\AppData\Local\Android\Sdk
set NDK_HOME=C:\Users\majid\AppData\Local\Android\Sdk\ndk\27.1.12297006

echo ANDROID_HOME: %ANDROID_HOME%
echo NDK_HOME: %NDK_HOME%
echo Environment variables set permanently and for current session.
echo.

echo Building Android APK...
cargo apk build --lib

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Build successful!
    echo.
    echo APK location: target\release\apk\
    echo.
    @REM echo To install on device, run:
    cargo apk run --lib
    echo or
    echo   adb install target\release\apk\wgpu_rust.apk
) else (
    echo.
    echo Build failed! Check the errors above.
)

pause
