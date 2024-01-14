use cgmath::*;
use forte_cubes::models::{*, animations::{AnimatedModel, AnimController}, file::SBFile, data::CubeModelBone};
use forte_engine::{render::{primitives::cameras::*, render_engine::RenderEngine, render_utils, input::EngineInput}, math::{quaternion::QuaternionExt, vec::VecExt}, lights::{lights::LightUniform, LightEngine, SetupLights}, EngineApp, run_app};

#[derive(Debug)]
pub struct MainApp { 
    render_engine: RenderEngine,
    light_engine: LightEngine,
    model_engine: CubeEngine,
    camera: Camera, 
    controller: CameraController,

    model: AnimatedModel<TestAnimController>
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

        // create texture and model
        let model = SBFile::load("assets/warrior.json").as_model(&mut engine);

        // setup light engine
        let mut light_engine = LightEngine::new(&engine, [1.0, 1.0, 1.0]);
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
            model: AnimatedModel { model, controller: TestAnimController }
        }
    }

    fn input(&mut self, input: EngineInput) { self.controller.input(&input) }

    fn update(&mut self) {
        // update
        self.controller.update_camera(&mut self.camera);
        self.camera.update(&mut self.render_engine);
        self.model.update(&self.render_engine);
        self.light_engine.update(&self.render_engine);

        // render
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

            // draw cube model
            pass.prepare_cube_engine(&self.model_engine, &self.camera);
            pass.load_lights(&self.light_engine);
            pass.draw_animated_cube_model(&self.render_engine, &self.model_engine, &self.model);
        }

        render_utils::finalize_render(&mut self.render_engine, resources);
    }

    fn events_cleared(&mut self) { self.render_engine.next_frame(); }
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }
    fn exit(&mut self) {}
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
