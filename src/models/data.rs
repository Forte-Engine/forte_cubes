use cgmath::{Vector2, Vector3, Quaternion, Matrix3};
use forte_engine::{render::render_engine::RenderEngine, math::transforms::Transform};
use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct CubeModelBone {
    pub label: Option<String>,
    pub transform: Transform,
    pub children: Vec<CubeModelBone>,
    pub parts: Vec<CubeModelPart>
}

#[derive(Debug, Clone, Copy)]
pub struct CubeModelPart {
    pub transform: Transform,
    pub tex_offset: Vector2<f32>
}

#[derive(Debug)]
pub struct CubeModelData {
    pub cube_buffer: wgpu::Buffer,
    pub size: u32
}

impl CubeModelData {
    pub fn new(engine: &RenderEngine, default: Vec<CubeInstance>) -> Self {
        let cube_buffer = engine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: &bytemuck::cast_slice(&default),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        Self { cube_buffer, size: default.len() as u32 }
    }

    pub fn update(&self, engine: &RenderEngine, new: Vec<CubeInstance>) {
        if new.len() as u32 != self.size { panic!("Transform vectors must be of a consistent size!"); }
        engine.queue.write_buffer(&self.cube_buffer, 0, bytemuck::cast_slice(&new));
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CubeInstance {
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
    pub tex_splits: [[f32; 4]; 2]
}

impl CubeInstance {
    pub fn generate(
        matrix: &cgmath::Matrix4<f32>, 
        rotation: &Quaternion<f32>,
        local_scale: &Vector3<f32>,
        local_tex_offset: &Vector2<f32>, 
        texture_size: &Vector2<f32>, 
        px_per_unit: f32
    ) -> Self {
        let x_scale = (local_scale.x * px_per_unit) / texture_size.x;
        let y_y_scale = (local_scale.y * px_per_unit) / texture_size.y;
        let z_x_scale = (local_scale.z * px_per_unit) / texture_size.x;
        let z_y_scale = (local_scale.z * px_per_unit) / texture_size.y;

        Self {
            model: [
                matrix.x.into(),
                matrix.y.into(),
                matrix.z.into(),
                matrix.w.into(),
            ],
            normal: Matrix3::from(*rotation).into(),
            tex_splits: [
                [
                    local_tex_offset.x,
                    local_tex_offset.x + z_x_scale,
                    local_tex_offset.x + z_x_scale + x_scale,
                    local_tex_offset.x + z_x_scale + x_scale + z_x_scale,
                ],
                [
                    local_tex_offset.x + z_x_scale + x_scale + z_x_scale + x_scale,
                    local_tex_offset.y,
                    local_tex_offset.y + z_y_scale,
                    local_tex_offset.y + z_y_scale + y_y_scale,
                ]
            ]
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5, not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 25]>() as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 29]>() as wgpu::BufferAddress,
                    shader_location: 13,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}