# Migration Summary: GLFW → winit

## ✅ Migration Complete!

Your wgpu Rust project has been successfully migrated from GLFW to winit for Android compatibility.

## What Changed

### 1. **Window Management**
- **Before**: GLFW (desktop-only)
- **After**: winit (cross-platform: Windows, Linux, macOS, Android, iOS, Web)

### 2. **Dependencies** (Cargo.toml)
- ❌ Removed: `glfw = "0.59"`
- ✅ Added: `winit = "0.30"`
- ✅ Added: `log = "0.4"` (for logging)
- ✅ Added: `env_logger = "0.11"` (desktop logging)
- ✅ Added: `android_logger = "0.14"` (Android logging)
- ✅ Added: `ndk-glue = "0.7"` (Android NDK integration)

### 3. **Code Structure**

#### State struct
```rust
// Before (GLFW)
struct State<'a> {
    window: &'a mut Window,  // Mutable reference
    size: (i32, i32),        // Signed integers
    // ...
}

// After (winit)
struct State {
    window: Arc<Window>,      // Shared ownership
    size: (u32, u32),         // Unsigned integers
    // ...
}
```

#### Main function
```rust
// Before (GLFW)
fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mut glfw = glfw::init(...).unwrap();
    let (mut window, events) = glfw.create_window(...).unwrap();
    
    while !window.should_close() {
        glfw.poll_events();
        // event handling
    }
}

// After (winit)
fn main() {
    env_logger::init();  // Desktop logging
    let event_loop = EventLoop::new().unwrap();
    run_event_loop(event_loop);
}

// Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(...);
    let event_loop = EventLoop::builder()
        .with_android_app(app)
        .build()
        .unwrap();
    run_event_loop(event_loop);
}
```

#### Event Handling
```rust
// Before (GLFW)
for (_, event) in glfw::flush_messages(&events) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        WindowEvent::FramebufferSize(width, height) => {
            state.resize((width, height));
        }
        // ...
    }
}

// After (winit) - ApplicationHandler trait
impl ApplicationHandler for App {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { 
                event: KeyEvent { 
                    physical_key: PhysicalKey::Code(KeyCode::Escape), 
                    .. 
                }, 
                .. 
            } => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                state.resize((physical_size.width, physical_size.height));
            }
            WindowEvent::RedrawRequested => {
                state.render().unwrap();
                state.window.request_redraw();
            }
            // ...
        }
    }
}
```

## Key Differences

### 1. **Lifetime Management**
- GLFW: Required explicit lifetimes (`&'a mut Window`)
- winit: Uses `Arc<Window>` for shared ownership (needed for async operations)

### 2. **Event Loop**
- GLFW: Imperative `while` loop with manual polling
- winit: Event-driven `ApplicationHandler` trait pattern

### 3. **Size Types**
- GLFW: `(i32, i32)` for window sizes
- winit: `(u32, u32)` for window sizes (more correct - sizes can't be negative)

### 4. **Surface Creation**
- GLFW: `instance.create_surface(window.render_context())`
- winit: `instance.create_surface(window.clone())`

### 5. **Frame Timing**
- GLFW: Manual sleep to cap framerate
- winit: Built-in with `request_redraw()` and `about_to_wait()`

### 6. **Android Support**
- GLFW: ❌ Not supported
- winit: ✅ Full support with `android_main` entry point

## Testing

### Desktop (Windows)
```powershell
cargo run
# Press ESC to close
```

### Android
```powershell
# Build APK
cargo apk build --release

# Install and run
cargo apk run --release
```

## Files Modified

1. ✅ `src/main.rs` - Complete rewrite using winit
2. ✅ `Cargo.toml` - Updated dependencies and Android metadata
3. ✅ `AndroidManifest.xml` - Created for Android packaging
4. ✅ `.cargo/config.toml` - Created for Android NDK linker paths

## Next Steps

1. **Update NDK paths** in `.cargo/config.toml` if needed
2. **Test desktop build**: `cargo run`
3. **Build Android APK**: `cargo apk build --release`
4. **Deploy to device**: `cargo apk run --release`

## Benefits of winit

- ✅ **Cross-platform**: One codebase for desktop and mobile
- ✅ **Modern**: Active development, used by many Rust graphics projects
- ✅ **wgpu integration**: Native support for wgpu surface creation
- ✅ **Event-driven**: More efficient than polling
- ✅ **Android/iOS**: Full mobile platform support
- ✅ **Web**: Can compile to WASM for browser deployment

## Troubleshooting

### Desktop build errors
- Make sure you removed the old GLFW dependency
- Run `cargo clean` and `cargo build`

### Android build errors
- Verify NDK paths in `.cargo/config.toml`
- Check that Android targets are installed: `rustup target list --installed`
- Ensure cargo-apk is installed: `cargo install cargo-apk`

### Window doesn't appear
- Check the logs with `env_logger` (desktop) or `logcat` (Android)
- Verify wgpu backend is available (Vulkan on Android)

For more details, see `ANDROID_BUILD_GUIDE.md`.
