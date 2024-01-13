use cgmath::{Vector3, Zero, Quaternion, Rotation3};
use forte_cubes::models::{CubeEngine, data::{CubeModelData, CubeInstance}, DrawCubes};
use forte_engine::{render::{primitives::cameras::*, textures::textures::Texture, resources::Handle, RenderEngineApp, render_engine::{RenderEngine, RenderEngineInput}, run_app}, math::transforms::Transform, lights::{LightEngine, lights::LightUniform, SetupLights}};
use winit::event::{ElementState, VirtualKeyCode};

#[derive(Debug)]
pub struct MainApp { 
    light_engine: LightEngine,
    model_engine: CubeEngine,
    camera: Camera, 
    controller: CameraController,

    texture: Handle<Texture>,
    data: CubeModelData
}

impl RenderEngineApp for MainApp {
    fn create(engine: &mut RenderEngine) -> Self {
        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(engine);
        let camera_controller = CameraController::new(0.02);

        // create model engine
        let model_engine = CubeEngine::new(engine);

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
            model_engine, camera,
            controller: camera_controller,
            texture: texture_handle, data
        }
    }

    fn input(&mut self, _engine: &mut RenderEngine, input: RenderEngineInput) {
        match input {
            RenderEngineInput::KeyInput(key, state) => {
                let pressed = state == ElementState::Pressed;
                match key {
                    VirtualKeyCode::W => self.controller.set_forward(pressed),
                    VirtualKeyCode::S => self.controller.set_backward(pressed),
                    VirtualKeyCode::A => self.controller.set_left(pressed),
                    VirtualKeyCode::D => self.controller.set_right(pressed),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, engine: &mut RenderEngine) {
        self.controller.update_camera(&mut self.camera);
        self.camera.update(engine);
        self.light_engine.update(engine);
    }

    fn render(&mut self, engine: &mut RenderEngine, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
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

        let texture = engine.texture(&self.texture);

        let transform = Transform {
            position: Vector3::zero(),
            rotation: 
                Quaternion::from_angle_y(cgmath::Deg(engine.time_since_start * 45.0)) *
                Quaternion::from_angle_z(cgmath::Deg(engine.time_since_start * 45.0)),
            scale: Vector3 { x: 1.0, y: 1.0, z: 1.0 }
        };
        self.data.update(engine, vec![
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
        pass.load_lights(&self.light_engine);
        pass.draw_cubes_raw(engine, self.model_engine.mesh(), &self.texture, &self.data);
    }

    fn exit(&mut self, _engine: &mut RenderEngine) {}
}

fn main() {
    pollster::block_on(run_app::<MainApp>());
}
