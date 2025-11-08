use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex
{
    position: [f32; 3],
    color: [f32; 3],
}

pub struct Mesh
{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_of_vertices: u32,
    pub num_of_indices: u32,

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

pub fn make_quad(device: &wgpu::Device) -> Mesh
{
    let vertices: [Vertex; 4] = [
        Vertex {
            position: [-0.75, -0.75, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.75, -0.75, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.75, 0.75, 0.0],
            color: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.75, 0.75, 0.0],
            color: [0.0, 1.0, 1.0],
        },
    ];
    
    let vertex_buffer_init_descriptor = wgpu::util::BufferInitDescriptor 
    {
        label: Some("quad vertex buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    };

    let vertex_buffer_init = device.create_buffer_init(&vertex_buffer_init_descriptor);

    let indices: [u32; 6] = [
        0, 1, 2, 2, 3, 0
    ];
    
    let index_buffer_init_descriptor = wgpu::util::BufferInitDescriptor 
    {
        label: Some("quad index buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    };

    let index_buffer_init = device.create_buffer_init(&index_buffer_init_descriptor);
    
    return Mesh{
        vertex_buffer: vertex_buffer_init,
        index_buffer: index_buffer_init,
        num_of_vertices: 4,
        num_of_indices: 6,
    };
}