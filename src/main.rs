use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread::sleep;
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent, KeyEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use winit::keyboard::{PhysicalKey, KeyCode};

mod renderer_backend;

use renderer_backend::pipeline_builder::PipelineBuilder;
use renderer_backend::mesh_builder;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    
    let event_loop = EventLoop::builder()
        .with_android_app(app)
        .build()
        .unwrap();
    
    run_event_loop(event_loop);
}


struct State {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    triangle_mesh: wgpu::Buffer
}

impl State 
{
    async fn new(window: Arc<Window>) -> Self 
    {
        let size = window.inner_size();
        let size = (size.width.max(1), size.height.max(1));

        let instance_descriptor = wgpu::InstanceDescriptor {
            #[cfg(target_os = "android")]
            backends: wgpu::Backends::VULKAN,
            #[cfg(target_os = "windows")]
            backends: wgpu::Backends::DX12,
            #[cfg(target_os = "linux")]
            backends: wgpu::Backends::VULKAN,
            #[cfg(not(any(target_os = "android", target_os = "windows", target_os = "linux")))]
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(&instance_descriptor);
        
        log::info!("Creating surface...");
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptionsBase{
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor{
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("device"),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter()
        .copied().filter(|f | f.is_srgb())
        .next().unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

        let triangle_mesh = mesh_builder::make_triangle(&device);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.add_vertex_buffer_layouts(mesh_builder::Vertex::get_layout());
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipline(&device);

        Self{
            instance,
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            triangle_mesh,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    {
        let drawable = self.surface.get_current_texture()?;
        let texture_view_descriptor = wgpu::TextureViewDescriptor::default();
        let texture_view = drawable.texture.create_view(&texture_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor{
            label: Some("render encoder"),
        };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment{
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations { 
                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.45, g: 0.45, b: 0.0, a: 0.0 }),
                store: wgpu::StoreOp::Store, 
            },
            depth_slice: None,
        };
        let render_pass_descriptor = wgpu::RenderPassDescriptor{
            label: Some("render pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();
        
        Ok(())
    }

    fn resize(&mut self, new_size: (u32, u32))
    {
        if new_size.0 == 0 || new_size.1 == 0
        {
            return;
        }
        
        self.size = new_size;
        self.config.width = new_size.0;
        self.config.height = new_size.1;

        self.surface.configure(&self.device, &self.config);
        log::info!("Resized to {}x{}", new_size.0, new_size.1);
    }

    fn update_surface(&mut self)
    {
        self.surface = self.instance.create_surface(self.window.clone()).unwrap();
        self.surface.configure(&self.device, &self.config);
    }
    
    fn input(&mut self, event: &WindowEvent) -> bool
    {
        match event {
            WindowEvent::KeyboardInput { 
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => {
                log::info!("Escape pressed, closing...");
                return true;
            }
            _ => false,
        }
    }
}

struct App {
    state: Option<State>,
    last_frame_time: Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("App resumed");
        
        if self.state.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("wgpu Rust")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600));
            
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            log::info!("Window created");
            
            // Create state asynchronously
            let state = pollster::block_on(State::new(window));
            self.state = Some(state);
            log::info!("State initialized");
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        log::info!("App suspended");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(state) = &mut self.state {
            // Handle input
            if state.input(&event) {
                event_loop.exit();
                return;
            }

            match event {
                WindowEvent::CloseRequested => {
                    log::info!("Close requested");
                    event_loop.exit();
                }
                
                WindowEvent::Resized(physical_size) => {
                    log::info!("Resized to {:?}", physical_size);
                    state.resize((physical_size.width, physical_size.height));
                }
                
                WindowEvent::RedrawRequested => {
                    let start_time = Instant::now();
                    
                    // Render frame
                    match state.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            log::warn!("Surface error, recreating...");
                            state.update_surface();
                            state.resize(state.size);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::error!("Out of memory!");
                            event_loop.exit();
                        }
                        Err(e) => {
                            log::error!("Render error: {:?}", e);
                        }
                    }
                    
                    // Frame timing
                    let frame_time = start_time.elapsed();
                    let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);
                    
                    if target_frame_time > frame_time {
                        let sleep_time = target_frame_time - frame_time;
                        sleep(sleep_time);
                        
                        let total_frame_time = start_time.elapsed();
                        let fps = 1.0 / total_frame_time.as_secs_f32();
                        log::debug!(
                            "Frame time: {:.2}ms, Sleep time: {:.2}ms, FPS: {:.1}",
                            frame_time.as_secs_f32() * 1000.0,
                            sleep_time.as_secs_f32() * 1000.0,
                            fps
                        );
                    } else {
                        let fps = 1.0 / frame_time.as_secs_f32();
                        log::debug!(
                            "Frame time: {:.2}ms (no sleep), FPS: {:.1}",
                            frame_time.as_secs_f32() * 1000.0,
                            fps
                        );
                    }
                    
                    // Request next frame
                    state.window.request_redraw();
                }
                
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }
}

fn run_event_loop(event_loop: EventLoop<()>) {
    let mut app = App { 
        state: None,
        last_frame_time: Instant::now(),
    };
    
    event_loop.set_control_flow(ControlFlow::Poll);
    
    match event_loop.run_app(&mut app) {
        Ok(_) => log::info!("Event loop exited normally"),
        Err(e) => log::error!("Event loop error: {:?}", e),
    }
}

#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::init();
    log::info!("Starting wgpu application...");
    
    let event_loop = EventLoop::new().unwrap();
    run_event_loop(event_loop);
}

#[cfg(target_os = "android")]
fn main() {
    // Android entry point is android_main, this is never called
}
