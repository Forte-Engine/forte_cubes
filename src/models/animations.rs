use std::fmt::Debug;

use forte_engine::render::render_engine::RenderEngine;

use crate::models::{data::CubeModelBone, cubes::CubeModel};

#[derive(Debug)]
pub struct AnimatedModel<T: AnimController> {
    pub model: CubeModel,
    pub controller: T
}

impl <T: AnimController> AnimatedModel<T> {
    pub fn new(model: CubeModel, controller: T) -> Self { Self { model, controller } }
    pub fn update(&mut self, engine: &RenderEngine) { self.controller.update(engine, &mut self.model.bone); self.model.update(engine); }
    pub fn clone(&self, engine: &RenderEngine) -> Self { Self { model: self.model.clone(engine), controller: self.controller.clone() } }
}

pub trait AnimController: Debug + Clone {
    fn update(&mut self, engine: &RenderEngine, root: &mut CubeModelBone);
}


