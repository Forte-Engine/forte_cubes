use cgmath::{Quaternion, Rotation3, Vector2, Zero};
use forte_cubes::models::{CubeEngine, cubes::CubeModel, data::{CubeModelBone, CubeModelPart}, DrawCubes};
use forte_engine::{render::{primitives::cameras::{CameraController, Camera}, render_engine::{RenderEngine, RenderEngineInput}, RenderEngineApp, run_app}, math::transforms::Transform, lights::{LightEngine, lights::LightUniform, SetupLights}};
use winit::event::{ElementState, VirtualKeyCode};

#[derive(Debug)]
pub struct MainApp { 
    light_engine: LightEngine,
    model_engine: CubeEngine,
    camera: Camera, 
    controller: CameraController,

    model: CubeModel
}

impl RenderEngineApp for MainApp {
    fn create(engine: &mut RenderEngine) -> Self {
        // generate camera
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 10.0).into();
        camera.update(engine);
        let camera_controller = CameraController::new(0.02);

        // create model engine
        let model_engine = CubeEngine::new(engine);

        // create texture and model
        let texture = engine.load_texture("assets/cube_test_texture.png");
        let model = CubeModel::new(
            &engine, 
            Transform::default(), texture, 
            CubeModelBone { label: Some("root".into()), transform: Transform::default(), children: vec![
                CubeModelBone {
                    label: Some("Right Arm".into()),
                    transform: Transform { position: (3.0, 0.0, 0.0).into(), ..Default::default() },
                    parts: vec![CubeModelPart { transform: Transform::default(), tex_offset: Vector2::zero() }],
                    children: vec![
                        CubeModelBone {
                            label: None,
                            transform: Transform { position: (0.0, 3.0, 0.0).into(), ..Default::default() },
                            parts: vec![CubeModelPart { transform: Transform { scale: (0.5, 0.5, 0.5).into(), ..Default::default() }, tex_offset: Vector2 { x: 0.0, y: 0.5 } }],
                            children: vec![]
                        }
                    ]
                },
                CubeModelBone {
                    label: Some("Left Arm".into()),
                    transform: Transform { position: (-3.0, 0.0, 0.0).into(), ..Default::default() },
                    parts: vec![CubeModelPart { transform: Transform::default(), tex_offset: Vector2::zero() }],
                    children: vec![
                        CubeModelBone {
                            label: None,
                            transform: Transform { position: (0.0, 3.0, 0.0).into(), ..Default::default() },
                            parts: vec![CubeModelPart { transform: Transform { scale: (0.5, 0.5, 0.5).into(), ..Default::default() }, tex_offset: Vector2 { x: 0.0, y: 0.5 } }],
                            children: vec![]
                        }
                    ]
                },
            ], parts: Vec::new() }, 
            4
        );

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
            model
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

        // rotate model
        self.model.transform.rotation = Quaternion::from_angle_y(cgmath::Deg(engine.time_since_start * 45.0));
        self.model.update(engine);

        // oscillate part children
        self.model.bone.children.iter_mut().enumerate().for_each(|(index, init_part)| {
            init_part.children.iter_mut().for_each(|part| {
                part.transform.position.y = f32::cos(engine.time_since_start + (index as f32 * 2.0)) * 1.5 + 3.0;
            });
        });

        // draw cube model
        pass.prepare_cube_engine(&self.model_engine, &self.camera);
        pass.load_lights(&self.light_engine);
        pass.draw_cube_model(&engine, &self.model_engine, &self.model);
    }

    fn exit(&mut self, _engine: &mut RenderEngine) {}
}

fn main() {
    pollster::block_on(run_app::<MainApp>());
}
