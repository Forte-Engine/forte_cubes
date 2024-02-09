use cgmath::Vector3;
use forte_cubes::{terrain::{chunk::Chunk, blocks::*}, define_blocks_materials};
use forte_engine::{component_app::EngineComponent, inputs::winit_input::EngineInput, lights::{lights::LightUniform, LightEngine}, primitives::{cameras::Camera, textures::Texture, transforms::TransformRaw, vertices::Vertex}, render::{pipelines::Pipeline, render_engine::RenderEngine, render_utils}, run_app, utils::{camera_controller::CameraController, resources::Handle}, EngineApp};
use winit::event::ElementState;

define_blocks_materials!(
    Blocks, 
    Material, 
    "assets/test_blocks.png",
    [
        AIR => {
            transparent: true,
            renderer: BlockRenderer::None
        },
        GRASS => {
            transparent: false,
            renderer: BlockRenderer::Standard(0, 1, 0, 0, 0, 0)
        }
    ]
);

#[derive(Debug)]
pub struct MainApp { 
    pipeline: Pipeline,
    render_engine: RenderEngine,
    light_engine: LightEngine,

    camera: Camera, 
    controller: CameraController,
    chunk: Chunk<Blocks, Material>,
    chunk_atlas: Handle<Texture>
}

#[include_wgsl_oil::include_wgsl_oil("../shaders/terrain.wgsl")]
mod terrain_shader {}

impl EngineApp for MainApp {
    fn create(mut engine: RenderEngine) -> Self {
        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 3.0, 5.0).into();
        camera.update(&mut engine);
        let camera_controller = CameraController::new(0.02);

        // create chunk
        let mut chunk = Chunk::empty(0);
        let chunk_atlas = engine.load_texture(Blocks::ATLAS);

        for x in 0 .. 15 {
            for z in 0 .. 15 {
                chunk.set(Vector3 { x, y: 0, z }, Material::GRASS, 0);

                if x == 5 && z == 5 {
                    chunk.set(Vector3 { x, y: 1, z }, Material::GRASS, 0);
                    chunk.set(Vector3 { x, y: 2, z }, Material::GRASS, 0);
                }

                if (x == 5 && (z == 3 || z == 7)) || ((x == 3 || x == 7) && z == 5) {
                    chunk.set(Vector3 { x, y: 1, z }, Material::GRASS, 0);
                }
            }
        }

        chunk.gen_mesh(&mut engine, &chunk_atlas);

        // setup light engine
        let mut light_engine = LightEngine::create(&mut engine);
        light_engine.set_ambient_color([0.1, 0.1, 0.1]);
        light_engine.add_light(0, LightUniform::new(
            [
                f32::cos(engine.time_since_start * 20.0) * 5.0 + 5.0, 
                7.0, 
                f32::sin(engine.time_since_start * 20.0) * 5.0 + 5.0
            ], 
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            100.0, 0.0, 1000.0
        ));

        // create instance of self
        Self {
            pipeline: Pipeline::new(
                "chunk", &engine, terrain_shader::SOURCE,
                &[Vertex::desc(), TransformRaw::desc()],
                &[
                    &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
                    &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
                    &engine.device.create_bind_group_layout(&LightUniform::BIND_LAYOUT)
                ],
                true
            ),
            render_engine: engine,
            light_engine,
            camera,
            controller: camera_controller,
            chunk,
            chunk_atlas
        }
    }

    fn start(&mut self) {}

    fn input(&mut self, input: EngineInput) {
        match input {
            EngineInput::KeyInput(key, state) => self.controller.key_input(key, matches!(state, ElementState::Pressed)),
            _ => {}
        }
    }

    fn update(&mut self) {
        // update
        self.controller.update_camera(&mut self.camera);
        self.camera.update(&mut self.render_engine);
        self.light_engine.update(&mut self.render_engine);

        // start render
        let resources = render_utils::prepare_render(&self.render_engine);
        let mut resources = if resources.is_ok() { resources.unwrap() } else { return };

        {
            // create render pass
            let mut pass = resources.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &resources.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.render_engine.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // draw chunk
            self.pipeline.bind(&mut pass);
            self.camera.bind(&mut pass, 0);
            self.light_engine.render(&self.render_engine, &mut pass);
            self.chunk.draw(&self.render_engine, &mut pass, &self.chunk_atlas);
        }

        // end render
        render_utils::finalize_render(&mut self.render_engine, resources);

        self.render_engine.next_frame();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }
    fn exit(&mut self) {}
}

fn main() {
    run_app::<MainApp>();
}
