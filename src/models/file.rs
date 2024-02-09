use std::path::Path;

use cgmath::{Vector2, Vector3, Quaternion};
use forte_engine::{math::{transforms::Transform, vector::VectorExt}, render::render_engine::RenderEngine, utils::files::Files};
use serde::*;

use crate::models::{cubes::CubeModel, data::{CubeModelPart, CubeModelBone}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBFile {
    root: SBBone,
    absolute_path: Option<String>,
    texture: String,
    name: String
}

impl SBFile {
    pub fn load(path: &str) -> Self {
        // load sb file via serde
        let bytes = Files::load_bytes(path).unwrap();
        let str = String::from_utf8(bytes).unwrap();
        let mut file: SBFile = serde_json::from_str(str.as_str()).unwrap();

        // set absolute path if necessary
        if file.absolute_path.is_none() {
            let path = Path::new(path);
            let absolute_path = path.canonicalize().expect("Could not get absolute path while loading sb file!");
            let absolute_path = absolute_path.to_str().expect("Could not convert path to string while loading sb file!");
            file.absolute_path = Some(absolute_path.to_string());
        }

        return file;
    }

    pub fn as_model(&self, engine: &mut RenderEngine) -> CubeModel {
        // find texture via its path local to this file
        let path = Path::new(self.absolute_path.as_ref().expect("A loaded SB file did not have its absolute path set!"));
        let path = path.parent().unwrap().join(Path::new(&self.texture));

        // load texture
        let texture = engine.load_texture(path.to_str().unwrap()); // todo make sure this is not loaded multiple times
        let raw_tex = engine.texture(&texture);

        // get some texture info for generating the cube model
        let texture_size = raw_tex.texture.size();
        let texture_size = Vector2 { x: texture_size.width, y: texture_size.height };

        // generate the final cube model
        CubeModel::new(engine, Transform::default(), texture, self.root.as_cube_bone(texture_size), 0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBBone {
    pub name: Option<String>,
    pub position: Option<Vec<f32>>,
    pub rotation: Option<Vec<f32>>,
    pub scale: Option<Vec<f32>>,
    pub bones: Vec<SBBone>,
    pub parts: Vec<SBPart>
}

impl SBBone {
    pub fn as_cube_bone(&self, texture_size: Vector2<u32>) -> CubeModelBone {
        CubeModelBone { 
            label: self.name.clone(), 
            transform: Transform {
                position: decode_opt_vec(self.position.as_ref(), 0.0625, false),
                rotation: Quaternion::from(decode_opt_vec(self.rotation.as_ref(), 1.0, false).euler()),
                scale: decode_opt_vec(self.scale.as_ref(), 0.0625, true)
            }, 
            children: self.bones.iter().map(|bone| bone.as_cube_bone(texture_size)).collect::<Vec<CubeModelBone>>(), 
            parts: self.parts.iter().map(|part| part.as_cube_part(texture_size)).collect::<Vec<CubeModelPart>>() 
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBPart {
    pub name: Option<String>,
    pub position: Option<Vec<f32>>,
    pub rotation: Option<Vec<f32>>,
    pub scale: Option<Vec<f32>>,
    pub tex_offset: Vec<u32>
}

impl SBPart {
    pub fn as_cube_part(&self, texture_size: Vector2<u32>) -> CubeModelPart {
        CubeModelPart { 
            transform: Transform {
                position: decode_opt_vec(self.position.as_ref(), 0.0625, false),
                rotation: Quaternion::from(decode_opt_vec(self.rotation.as_ref(), 1.0, false).euler()),
                scale: decode_opt_vec(self.scale.as_ref(), 0.0625, true)
            }, 
            tex_offset: Vector2 { 
                x: self.tex_offset[0] as f32 / texture_size.x as f32, 
                y: self.tex_offset[1] as f32 / texture_size.y as f32 
            }
        }
    }
}

fn decode_opt_vec(opt: Option<&Vec<f32>>, mult: f32, default_ones: bool) -> Vector3<f32> {
    if opt.is_some() {
        let opt = opt.unwrap();
        Vector3 { x: opt[0], y: opt[1], z: opt[2] } * mult
    } 
    else if default_ones { Vector3 { x: 1.0, y: 1.0, z: 1.0 } }
    else { Vector3 { x: 0.0, y: 0.0, z: 0.0 } }
}
