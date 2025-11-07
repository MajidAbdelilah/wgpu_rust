struct Vertex
{
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};
struct vertex_payload
{
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> vertex_payload
{
    
    var out: vertex_payload;
    out.position = vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    
    return out;
}

@fragment
fn fs_main(in: vertex_payload) -> @location(0) vec4<f32>
{
    return vec4<f32>(in.color, 1.0);
}