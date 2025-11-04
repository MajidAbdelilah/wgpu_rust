use core::time;
use std::{thread::sleep};

use glfw::{Action, Context, Key, Window, WindowEvent, WindowHint, fail_on_errors};
use wgpu::wgc::{device::queue, instance};

struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window,
}

impl<'a> State<'a> 
{

    async fn new(window: &'a mut Window) -> Self 
    {
        let size = window.get_framebuffer_size();

        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(&instance_descriptor);
        println!("bitch!!!");
        ;
        let surface = instance.create_surface(window.render_context()).unwrap();
        // let surface = instance.create_surface(window.render_context()).unwrap();

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
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

        Self{
            instance,
            surface,
            device,
            queue,
            config,
            size,
            window
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

        command_encoder.begin_render_pass(&render_pass_descriptor);
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();
        
        Ok(())
    }

    fn resize(&mut self, new_size: (i32, i32))
    {
        if(new_size.0 <= 0 || new_size.1 <= 0)
        {
            return;
        }
        
        self.size = new_size;
        self.config.width = new_size.0 as u32;
        self.config.height = new_size.1 as u32;

        self.surface.configure(&self.device, &self.config);
    }

    fn update_surface(&mut self)
    {
        self.surface = self.instance.create_surface(self.window.render_context()).unwrap();
    }
}

async fn run() 
{
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    // glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    let (mut window, events) = glfw
        .create_window(800, 600, "wgpu", glfw::WindowMode::Windowed).unwrap();
        

    // window.set_all_polling(true);
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_pos_polling(true);

    let mut state = State::new(&mut window).await;

    while !state.window.should_close() {
        let start_time = std::time::Instant::now();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => 
                {
                    state.window.set_should_close(true);
                }

                glfw::WindowEvent::FramebufferSize(width, height ) =>
                {
                    state.update_surface();
                    state.resize((width, height));
            
                }
                glfw::WindowEvent::Pos(..) =>{
                    // state.update_surface();
                    // state.resize(state.size);

                }
                e => {
                    println!("{:?}", e);
                }
            }
        }
        
        match state.render()
        {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                state.update_surface();
                state.resize(state.size);
            }
            Err(e) => {eprintln!("{:?}", e)},
        }
        
        // state.window.swap_buffers();
        let frame_time = start_time.elapsed();
        let sleep_time = time::Duration::from_secs_f32(1.0) / 60;
        if sleep_time > frame_time
        {
            sleep(sleep_time - frame_time);
        }
    }
}

fn main()
{
    pollster::block_on(run());
}
