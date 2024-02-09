use animations::{AnimatedModel, AnimController};
use cubes::*;
use data::*;
use forte_engine::{lights::lights::LightUniform, primitives::{cameras::Camera, mesh::Mesh, textures::Texture, vertices::Vertex}, render::{pipelines::Pipeline, render_engine::RenderEngine}, utils::resources::Handle};

pub mod animations;
pub mod cubes;
pub mod data;
pub mod file;

const VERTICES: &[Vertex] = &[
    // south Z-
    Vertex { position: [ -0.5, -0.5, -0.5 ], tex_coords: [ 0.50, 1.00 ], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [  0.5, -0.5, -0.5 ], tex_coords: [ 0.25, 1.00 ], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [  0.5,  0.5, -0.5 ], tex_coords: [ 0.25, 0.50 ], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [ -0.5,  0.5, -0.5 ], tex_coords: [ 0.50, 0.50 ], normal: [0.0, 0.0, -1.0] },

    // north Z+
    Vertex { position: [ -0.5, -0.5, 0.5 ], tex_coords: [ 0.75, 1.00 ], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [  0.5, -0.5, 0.5 ], tex_coords: [ 1.00, 1.00 ], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [  0.5,  0.5, 0.5 ], tex_coords: [ 1.00, 0.50 ], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [ -0.5,  0.5, 0.5 ], tex_coords: [ 0.75, 0.50 ], normal: [0.0, 0.0, 1.0] },
    
    // west X-
    Vertex { position: [ -0.5,  0.5, -0.5 ], tex_coords: [ 0.50, 0.50 ], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [ -0.5, -0.5, -0.5 ], tex_coords: [ 0.50, 1.00 ], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [ -0.5, -0.5,  0.5 ], tex_coords: [ 0.75, 1.00 ], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [ -0.5,  0.5,  0.5 ], tex_coords: [ 0.75, 0.50 ], normal: [-1.0, 0.0, 0.0] },
    
    // east X+
    Vertex { position: [ 0.5, -0.5, -0.5 ], tex_coords: [ 0.25, 1.00 ], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5 ], tex_coords: [ 0.25, 0.50 ], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5 ], tex_coords: [ 0.00, 0.50 ], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5 ], tex_coords: [ 0.00, 1.00 ], normal: [1.0, 0.0, 0.0] },
    
    // bottom Y-
    Vertex { position: [ -0.5, -0.5, -0.5 ], tex_coords: [ 0.50, 0.50 ], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [  0.5, -0.5, -0.5 ], tex_coords: [ 0.75, 0.50 ], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [  0.5, -0.5,  0.5 ], tex_coords: [ 0.75, 0.00 ], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [ -0.5, -0.5,  0.5 ], tex_coords: [ 0.50, 0.00 ], normal: [0.0, -1.0, 0.0] },
    
    // top Y+
    Vertex { position: [  0.5,  0.5, -0.5 ], tex_coords: [ 0.25, 0.00 ], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [ -0.5,  0.5, -0.5 ], tex_coords: [ 0.50, 0.00 ], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [ -0.5,  0.5,  0.5 ], tex_coords: [ 0.50, 0.50 ], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [  0.5,  0.5,  0.5 ], tex_coords: [ 0.25, 0.50 ], normal: [0.0, 1.0, 0.0] }
];

const INDICES: &[u16] = &[
    0,   3,  2,
    2,   1,  0,
    4,   5,  6,
    6,   7,  4,
    11,  8,  9,
    9,  10, 11,
    12, 13, 14,
    14, 15, 12,
    16, 17, 18,
    18, 19, 16,
    20, 21, 22,
    22, 23, 20
];

#[derive(Debug)]
pub struct CubeEngine {
    pipeline: Pipeline,
    mesh: Handle<Mesh>
}

#[include_wgsl_oil::include_wgsl_oil("../../shaders/cubes.wgsl")]
mod cube_shader {}

impl CubeEngine {
    pub fn mesh(&self) -> &Handle<Mesh> { &self.mesh }

    pub fn new(engine: &mut RenderEngine) -> Self {
        // create render pipeline
        let pipeline = Pipeline::new(
            "std", engine, cube_shader::SOURCE,
            &[Vertex::desc(), CubeInstance::desc()],
            &[
                &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
                &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
                &engine.device.create_bind_group_layout(&LightUniform::BIND_LAYOUT)
            ],
            true
        );

        let mesh = engine.create_mesh("cube_engine_mesh", VERTICES, INDICES);

        Self { pipeline, mesh }
    }

    pub fn setup<'b, 'a>(&'b self, pass: &'b mut wgpu::RenderPass<'a>, camera: &'b Camera) where 'b: 'a {
        pass.set_pipeline(&self.pipeline.render_pipeline);
        pass.set_bind_group(0, &camera.bind_group, &[]);
    }
}

pub trait DrawCubes<'a, 'b> where 'b: 'a {
    fn prepare_cube_engine(
        &mut self,
        engine: &'b CubeEngine,
        camera: &'b Camera
    );

    fn draw_cubes_raw(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        data: &'b CubeModelData
    );

    fn draw_cube_model(
        &mut self,
        engine: &'b RenderEngine,
        cube_engine: &'b CubeEngine,
        model: &'b CubeModel
    );

    fn draw_animated_cube_model<T: AnimController>(
        &mut self,
        engine: &'b RenderEngine,
        cube_engine: &'b CubeEngine,
        model: &'b AnimatedModel<T>
    );
}

impl<'a, 'b> DrawCubes<'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
    fn prepare_cube_engine(
        &mut self,
        engine: &'b CubeEngine,
        camera: &'b Camera
    ) {
        self.set_pipeline(&engine.pipeline.render_pipeline);
        self.set_bind_group(0, &camera.bind_group, &[]);
    }

    fn draw_cubes_raw(
        &mut self,
        engine: &'b RenderEngine,
        mesh: &'b Handle<Mesh>,
        texture: &'b Handle<Texture>,
        data: &'b CubeModelData
    ) {
        engine.draw_textured_mesh(
            self, mesh, texture, 
            &data.cube_buffer, 
            data.size
        );
    }

    fn draw_cube_model(
        &mut self,
        engine: &'b RenderEngine,
        cube_engine: &'b CubeEngine,
        model: &'b CubeModel
    ) {
        self.draw_cubes_raw(engine, &cube_engine.mesh, &model.texture, &model.data);
    }

    fn draw_animated_cube_model<T: AnimController>(
        &mut self,
        engine: &'b RenderEngine,
        cube_engine: &'b CubeEngine,
        model: &'b AnimatedModel<T>
    ) {
        self.draw_cube_model(engine, cube_engine, &model.model);
    }
}
