use std::marker::PhantomData;

use cgmath::*;
use forte_engine::{math::transforms::Transform, render::{resources::Handle, textures::textures::Texture, render_engine::RenderEngine}};

use crate::models::data::*;

#[derive(Debug)]
pub struct CubeModel {
    pub transform: Transform,
    pub texture: Handle<Texture>,
    pub bone: CubeModelBone,
    pub(crate) data: CubeModelData
}

impl CubeModel {
    /// Creates a new cube model
    /// 
    /// # Arguments
    /// * `engine` - A instance of the render engine so that buffers can be created
    /// * `transform` - The transform of this object (position, rotation, scale)
    /// * `texture` - The texture to be used for this object
    /// * `bone` - The root bone of the model
    /// * `possible_size` - The possible count of all parts and its children.  This can be left as 0 or be inaccurate, as the actual size is taken when the model is autmatically, this value only serves to make the creation for effecient if it is accurate.
    pub fn new(engine: &RenderEngine, transform: Transform, texture: Handle<Texture>, bone: CubeModelBone, possible_size: usize) -> Self {
        // get texture dimensions
        let texture_ref = engine.texture(&texture);
        let texture_size = Vector2 {
            x: texture_ref.texture.width() as f32,
            y: texture_ref.texture.height() as f32
        };

        // render initial cube instances
        let instances = CubeModel::render_bones(&transform, &bone, texture_size, 16.0, possible_size);
        
        // return new instance of self
        return Self { transform, texture, bone, data: CubeModelData::new(engine, instances) }
    }

    /// Updates this cube model so that any changes to this models transform or any of its parts transforms is reflected by the model.
    /// 
    /// # Arguments
    /// * `engine` - A instance of the render engine as this is required to update the internal buffer
    pub fn update(&mut self, engine: &RenderEngine) { 
        // get texture dimensions
        let texture_ref = engine.texture(&self.texture);
        let texture_size = Vector2 {
            x: texture_ref.texture.width() as f32,
            y: texture_ref.texture.height() as f32
        };

        // render cube instances and update data buffer
        let instances = CubeModel::render_bones(&self.transform, &self.bone, texture_size, 16.0, self.data.size as usize);
        self.data.update(engine, instances);
    }

    /// Renders a given list of parts into a list of transforms with the given offset tranform.
    /// 
    /// # Arguments
    /// * `transform` - The offset transform.
    /// * `bone` - A bone to be rendered.
    /// * `texture_size` - The size of the texture being used for this model
    /// * `px_per_unit` - Number of pixels per 3d unit
    /// * `possible_size` - A possible count of the number of parts and their children.  This can be left at 0 but it will make this method much more effecient.
    /// 
    /// # Returns
    /// A list of raw transforms that can be passed into any shader as instances
    pub(crate) fn render_bones(transform: &Transform, bone: &CubeModelBone, texture_size: Vector2<f32>, px_per_unit: f32, possible_size: usize) -> Vec<CubeInstance> {
        let mut result: Vec<CubeInstance> = Vec::with_capacity(possible_size);
        CubeModel::recr_render_bone(&mut result, &transform.to_mat(), transform.rotation, bone, texture_size, px_per_unit);
        return result;
    }

    fn recr_render_bone(result: &mut Vec<CubeInstance>, previous: &Matrix4<f32>, rotation: Quaternion<f32>, bone: &CubeModelBone, texture_size: Vector2<f32>, px_per_unit: f32) {
        let bone_matrix = previous * bone.transform.to_mat();
        let rotation = bone.transform.rotation * rotation;

        // render parts
        bone.parts.iter().for_each(|part| {
            let matrix = bone_matrix * part.transform.to_mat();
            result.push(CubeInstance::generate(
                &matrix, 
                &(part.transform.rotation * rotation),
                &part.transform.scale, 
                &part.tex_offset, 
                &texture_size, 
                px_per_unit
            ));
        });

        // render children
        bone.children.iter().for_each(|bone| {
            CubeModel::recr_render_bone(result, &bone_matrix, rotation, bone, texture_size, px_per_unit);
        });
    }

    /// Clones this model
    /// 
    /// # Arguments
    /// * `engine` - A render engine instance to clone the model data
    /// 
    /// # Returns
    /// A clone of this model.
    pub fn clone(&self, engine: &RenderEngine) -> Self {
        // get texture dimensions
        let texture_ref = engine.texture(&self.texture);
        let texture_size = Vector2 {
            x: texture_ref.texture.width() as f32,
            y: texture_ref.texture.height() as f32
        };

        // create new self
        Self {
            transform: self.transform.clone(),
            texture: Handle::<Texture> { hash: self.texture.hash, data: PhantomData::default() },
            bone: self.bone.clone(),
            data: CubeModelData::new(engine, CubeModel::render_bones(&self.transform, &self.bone, texture_size, 16.0, self.data.size as usize)),
        }
    }
}
