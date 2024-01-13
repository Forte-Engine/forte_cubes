use cgmath::Vector3;
use forte_engine::{lights::{lights::LightUniform, LightEngine, SetupLights}, render::{primitives::cameras::{Camera, CameraController}, RenderEngineApp, render_engine::{RenderEngine, RenderEngineInput}, run_app}};
use forte_cubes::{terrain::{chunk::Chunk, ChunkEngine, DrawChunks, blocks::*}, define_blocks_materials};

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
    light_engine: LightEngine,
    camera: Camera, 
    controller: CameraController,

    chunk_engine: ChunkEngine<Blocks, Material>,
    chunk: Chunk<Blocks, Material>
}

impl RenderEngineApp for MainApp {
    fn create(engine: &mut RenderEngine) -> Self {
        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 3.0, 5.0).into();
        camera.update(engine);
        let camera_controller = CameraController::new(0.02);

        // create chunk
        let chunk_engine = ChunkEngine::new(engine);
        let mut chunk = Chunk::empty(0);
        // chunk.set(Vector3 { x: 1, y: 1, z: 1 }, Material::GRASS);

        for x in 0 .. 15 {
            for z in 0 .. 15 {
                chunk.set(Vector3 { x, y: 0, z }, Material::GRASS);

                if x == 5 && z == 5 {
                    chunk.set(Vector3 { x, y: 1, z }, Material::GRASS);
                    chunk.set(Vector3 { x, y: 2, z }, Material::GRASS);
                }

                if (x == 5 && (z == 3 || z == 7)) || ((x == 3 || x == 7) && z == 5) {
                    chunk.set(Vector3 { x, y: 1, z }, Material::GRASS);
                }
            }
        }

        chunk.gen_mesh(engine, &chunk_engine);

        // setup light engine
        let mut light_engine = LightEngine::new(engine, [1.0, 1.0, 1.0]);
        light_engine.add_light(0, LightUniform::new(
            [
                f32::cos(engine.time_since_start * 20.0) * 5.0 + 5.0, 
                2.0, 
                f32::sin(engine.time_since_start * 20.0) * 5.0 + 5.0
            ], 
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            0.0, 0.0, 0.0
        ));

        // create instance of self
        Self {
            light_engine,
            camera,
            controller: camera_controller,
            chunk_engine,
            chunk
        }
    }

    fn input(&mut self, _engine: &mut RenderEngine, input: RenderEngineInput) { self.controller.input(&input) }

    fn update(&mut self, engine: &mut RenderEngine) {
        self.controller.update_camera(&mut self.camera);
        self.camera.update(engine);
        self.light_engine.update(engine);
    }

    fn render(&mut self, engine: &mut RenderEngine, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        // create render pass
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
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
                view: &engine.depth_texture.view,
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
        pass.prepare_chunk_draw(&self.chunk_engine, &self.camera);
        pass.load_lights(&self.light_engine);
        pass.draw_chunk(engine, &self.chunk_engine, &mut self.chunk);
    }

    fn exit(&mut self, _engine: &mut RenderEngine) {}
}

fn main() {
    pollster::block_on(run_app::<MainApp>());
}
