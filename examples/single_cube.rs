use cgmath::{Vector3, Zero, Quaternion, Rotation3};
use forte_cubes::models::{CubeEngine, data::{CubeModelData, CubeInstance}, DrawCubes};
use forte_engine::{component_app::EngineComponent, inputs::winit_input::EngineInput, lights::{lights::LightUniform, LightEngine}, math::transforms::Transform, primitives::{cameras::Camera, textures::Texture}, render::{render_engine::RenderEngine, render_utils}, run_app, utils::{camera_controller::CameraController, resources::Handle}, EngineApp};
use winit::event::ElementState;

#[derive(Debug)]
pub struct MainApp { 
    render_engine: RenderEngine,
    light_engine: LightEngine,
    model_engine: CubeEngine,
    camera: Camera, 
    controller: CameraController,

    texture: Handle<Texture>,
    data: CubeModelData
}

impl EngineApp for MainApp {
    fn create(mut engine: RenderEngine) -> Self {
        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(&mut engine);
        let camera_controller = CameraController::new(0.02);

        // create model engine
        let model_engine = CubeEngine::new(&mut engine);

        let texture_handle = engine.load_texture("assets/cube_test_texture.png");
        let texture = engine.texture(&texture_handle);
        let data = CubeModelData::new(&engine, vec![
            CubeInstance::generate(
                &Transform {
                    position: (0.0, 0.0, 0.0).into(),
                    rotation: (0.0, 0.0, 0.0, 1.0).into(),
                    scale: (1.0, 1.0, 1.0).into()
                }.to_mat(),
                &(0.0, 0.0, 0.0, 1.0).into(),
                &(1.0, 1.0, 1.0).into(),
                &(0.0, 0.0).into(),
                &(texture.texture.width() as f32, texture.texture.height() as f32).into(),
                16.0
            )
        ]);

        // setup light engine
        let mut light_engine = LightEngine::create(&mut engine);
        light_engine.set_ambient_color([1.0, 1.0, 1.0]);
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
            render_engine: engine,
            light_engine, model_engine, camera,
            controller: camera_controller,
            texture: texture_handle, data
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

            let texture = self.render_engine.texture(&self.texture);

            let transform = Transform {
                position: Vector3::zero(),
                rotation: 
                    Quaternion::from_angle_y(cgmath::Deg(self.render_engine.time_since_start * 45.0)) *
                    Quaternion::from_angle_z(cgmath::Deg(self.render_engine.time_since_start * 45.0)),
                scale: Vector3 { x: 1.0, y: 1.0, z: 1.0 }
            };
            self.data.update(&self.render_engine, vec![
                CubeInstance::generate(
                    &transform.to_mat(),
                    &transform.rotation,
                    &(1.0, 1.0, 1.0).into(),
                    &(0.0, 0.0).into(),
                    &(texture.texture.width() as f32, texture.texture.height() as f32).into(),
                    16.0
                )
            ]);

            pass.prepare_cube_engine(&self.model_engine, &self.camera);
            self.light_engine.render(&self.render_engine, &mut pass);
            pass.draw_cubes_raw(&self.render_engine, self.model_engine.mesh(), &self.texture, &self.data);
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
