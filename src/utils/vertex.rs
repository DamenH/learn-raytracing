#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const CLIP_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.999, 0.999, 0.0],
        tex_coords: [1., 1.],
    }, // A
    Vertex {
        position: [-0.999, 0.999, 0.0],
        tex_coords: [0., 1.],
    }, // B
    Vertex {
        position: [-0.999, -0.999, 0.0],
        tex_coords: [0., 0.],
    }, // C
    Vertex {
        position: [0.999, -0.999, 0.0],
        tex_coords: [1., 0.],
    }, // D
];
pub const CLIP_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
