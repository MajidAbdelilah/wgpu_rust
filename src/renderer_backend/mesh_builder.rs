use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex
{
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex
{
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static>
    {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = [
            wgpu::VertexAttribute
            {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute
            {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 3]>() as u64,
                shader_location: 1,
            },
        ];

        wgpu::VertexBufferLayout 
        {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer
{
    let vertices: [Vertex; 3] = [
        Vertex {
            position: [-0.75, -0.75, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.75, -0.75, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.75, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];
    
    let buffer_init_descriptor = wgpu::util::BufferInitDescriptor 
    {
        label: Some("triangle vertex buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    };

    let buffer_init = device.create_buffer_init(&buffer_init_descriptor);
    return buffer_init;
}