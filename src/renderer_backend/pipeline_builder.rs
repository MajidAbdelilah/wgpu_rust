use std::env::current_dir;
use std::fs;

// Embed shader at compile time for Android compatibility
const SHADER_SOURCE: &str = include_str!("../shaders/shader.wgsl");

pub struct PipelineBuilder
{
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
    use_embedded_shader: bool,
}

impl PipelineBuilder {
    pub fn new() -> Self
    {
        PipelineBuilder { 
            shader_filename: "dummy".to_string(), 
            vertex_entry: "dummy".to_string(), 
            fragment_entry: "dummy".to_string(), 
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            #[cfg(target_os = "android")]
            use_embedded_shader: true,
            #[cfg(not(target_os = "android"))]
            use_embedded_shader: false,
        }
    }

    pub fn set_shader_module(
        &mut self,
        shader_filename: &str,
        vertex_entry: &str,
        fragment_entry: &str
    )
    {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }
    
    pub fn set_pixel_format(
        &mut self,
        pixel_format: wgpu::TextureFormat,
    )
    {
        self.pixel_format = pixel_format;
    }

    pub fn build_pipline(&mut self, device: &wgpu::Device) -> wgpu::RenderPipeline
    {
        let source_code = if self.use_embedded_shader {
            // Use embedded shader for Android
            SHADER_SOURCE.to_string()
        } else {
            // Read shader from file for desktop
            let mut filepath = current_dir().unwrap();
            filepath.push("src/");
            filepath.push(self.shader_filename.as_str());
            let filepath = filepath.into_os_string().into_string().unwrap();
            fs::read_to_string(filepath).expect("can't read shader file")
        };

        let shader_module_descriptor = wgpu::ShaderModuleDescriptor
        {
            label: Some("shader module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = device.create_shader_module(shader_module_descriptor);

        let pipeline_layout_discriptor = wgpu::PipelineLayoutDescriptor
        {
            label: Some("render pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        };
        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_discriptor);

        let render_targets = [Some(wgpu::ColorTargetState
            {
                format: self.pixel_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            }
        )];

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor
        {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),

            vertex: wgpu::VertexState
            {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },

            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None, 
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: Some(wgpu::Face::Back), 
                unclipped_depth: false, 
                polygon_mode: wgpu::PolygonMode::Fill, 
                conservative: false 
            },

            fragment: Some(wgpu::FragmentState
            {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                targets: &render_targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),

            depth_stencil: None,
            multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
            multiview: None,
            cache: None,
        };
        
        device.create_render_pipeline(&render_pipeline_descriptor)
    }
}