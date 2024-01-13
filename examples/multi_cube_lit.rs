use cgmath::*;
use forte_cubes::models::{CubeEngine, animations::{AnimatedModel, AnimController}, file::SBFile, data::CubeModelBone, DrawCubes};
use forte_engine::{lights::{LightEngine, lights::LightUniform}, render::{primitives::cameras::{Camera, CameraController}, RenderEngineApp, render_engine::{RenderEngine, RenderEngineInput}, run_app}, math::{quaternion::QuaternionExt, vec::VecExt}};

#[derive(Debug)]
pub struct MainApp { 
    light_engine: LightEngine,
    model_engine: CubeEngine,
    camera: Camera, 
    controller: CameraController,

    model: AnimatedModel<TestAnimController>
}

impl RenderEngineApp for MainApp {
    fn create(engine: &mut RenderEngine) -> Self {
        // create light engine
        let mut light_engine = LightEngine::new(engine, [0.1, 0.1, 0.1]);
        light_engine.add_light(0, LightUniform::new([2.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], f32::MAX, 1.0, 1000.0));
        light_engine.add_light(1, LightUniform::new([0.0, 2.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], f32::MAX, 1.0, 1000.0));
        light_engine.add_light(2, LightUniform::new([0.0, 0.0, 2.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], f32::MAX, 1.0, 1000.0));

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

        // create texture and model
        let model = SBFile::load("assets/warrior.json").as_model(engine);

        // create instance of self
        Self {
            light_engine, model_engine, camera,
            controller: camera_controller,
            model: AnimatedModel { model, controller: TestAnimController }
        }
    }

    fn input(&mut self, _engine: &mut RenderEngine, input: RenderEngineInput) { self.controller.input(&input) }

    fn update(&mut self, engine: &mut RenderEngine) {
        self.controller.update_camera(&mut self.camera);
        self.camera.update(engine);
        self.model.update(engine);
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

        // draw cube model
        pass.prepare_cube_engine(&self.model_engine, &self.camera);
        pass.set_bind_group(2, self.light_engine.bind_group(), &[]);
        pass.draw_animated_cube_model(&engine, &self.model_engine, &self.model);
    }

    fn exit(&mut self, _engine: &mut RenderEngine) {}
}

#[derive(Debug, Clone)]
pub struct TestAnimController;
impl AnimController for TestAnimController {
    fn update(&mut self, engine: &RenderEngine, root: &mut CubeModelBone) {
        root.transform.rotation = Euler::<Deg<f32>>::new(Deg(0.0), Deg(engine.time_since_start * 45.0), Deg(0.0)).into();

        root.children.for_each_mut(|bone| {
            let label = if bone.label.is_some() { bone.label.as_ref().unwrap() } else { return };
            let sintime = f32::sin(engine.time_since_start * 5.0);
            match label.as_str() {
                "left leg" => {
                    bone.transform.rotation = Quaternion::euler_deg_x(sintime * 30.0);
                    bone.children.for_each_mut(|bone| { bone.transform.rotation = Quaternion::euler_deg_x(sintime * 10.0 - 10.0); });
                }
                "right leg" => {
                    bone.transform.rotation = Quaternion::euler_deg_x(-sintime * 30.0);
                    bone.children.for_each_mut(|bone| { bone.transform.rotation = Quaternion::euler_deg_x(-sintime * 10.0 - 10.0); });
                }
                "left arm" => {
                    bone.transform.rotation = Quaternion::euler_deg_x(sintime * 30.0);
                    bone.children.for_each_mut(|bone| { bone.transform.rotation = Quaternion::euler_deg_x(sintime * 10.0 + 10.0); });
                }
                "right arm" => {
                    bone.transform.rotation = Quaternion::euler_deg_x(-sintime * 30.0);
                    bone.children.for_each_mut(|bone| { bone.transform.rotation = Quaternion::euler_deg_x(-sintime * 10.0 + 10.0); });
                }
                _ => {}
            }
        });
    }
}

fn main() {
    pollster::block_on(run_app::<MainApp>());
}
