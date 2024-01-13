use std::fmt::Debug;

use cgmath::*;
use forte_engine::{render::primitives::vertices::Vertex, math::vec::VecExt};

use crate::terrain::lookup;

#[macro_export]
macro_rules! define_blocks_materials {
    (
        $blocks_name:ident, 
        $mat_name:ident, 
        $atlas:expr, 
        [$($variant:ident => { transparent: $transparent:expr, renderer: $renderer:expr }),*]
    ) => {
        // create material of all variants
        #[derive(Clone, Copy, Debug)]
        #[repr(u16)]
        enum $mat_name {
            $($variant,)*
        }

        // convert to and from u16
        impl Into<u16> for $mat_name {
            fn into(self) -> u16 { self as u16 }
        }

        impl From<u16> for $mat_name {
            fn from(value: u16) -> Self {
                unsafe { std::mem::transmute(value) }
            }
        }

        // make the mateial a material definition
        use forte_cubes::terrain::blocks::{MaterialDef, BlockDef, BlockDefinitions};
        impl MaterialDef for $mat_name {}

        // generate block defintions
        #[derive(Debug)]
        pub struct $blocks_name;
        impl BlockDefinitions<$mat_name> for $blocks_name {
            const ATLAS: &'static str = $atlas;
            const DEFINITIONS: &'static [BlockDef<$mat_name>] = &[
                $(BlockDef {
                    material: $mat_name::$variant,
                    transparent: $transparent,
                    renderer: $renderer
                }),*
            ];
        }
    };
}

pub trait MaterialDef: Clone + Copy + Debug + Into<u16> + From<u16> {}

pub trait BlockDefinitions<M: MaterialDef + 'static> {
    const ATLAS: &'static str;
    const DEFINITIONS: &'static [BlockDef<M>];
}

pub struct BlockDef<M: MaterialDef + 'static> {
    pub material: M,
    pub transparent: bool,
    pub renderer: BlockRenderer<M>
}

pub enum BlockRenderer<M: MaterialDef + 'static> {
    None,
    Standard(u16, u16, u16, u16, u16, u16),
    Custom(
        fn(
            &BlockDef<M>, 
            &BlockDef<M>, 
            &BlockDef<M>, 
            &BlockDef<M>, 
            &BlockDef<M>, 
            &BlockDef<M>
        ) -> Vec<Vertex>
    ),
    BlockEntity
}

impl <'a, M: MaterialDef + 'static> BlockRenderer<M> {
    pub fn render(
        &self,
        position: Vector3<f32>, 
        tex_size: Vector2<u32>,
        above: &BlockDef<M>, 
        below: &BlockDef<M>, 
        north: &BlockDef<M>, 
        south: &BlockDef<M>, 
        east: &BlockDef<M>, 
        west: &BlockDef<M>
    ) -> Vec<Vertex> {
        // render a vector of vertices
        let mut vec = match self {
            // if none or block entity, rendering now is not necessary
            Self::None | Self::BlockEntity => Vec::new(),

            // do custom rendering
            Self::Custom(callback) => 
                callback(above, below, north, south, east, west),

            // do standard above, below, north, south, east, west rendedring
            Self::Standard(tex_above, tex_below, tex_north, tex_south, tex_east, tex_west) => {
                let mut vec = Vec::new();
                if above.transparent { Self::append_face(&mut vec, &lookup::CUBE_TOP, tex_size, tex_above) }
                if below.transparent { Self::append_face(&mut vec, &lookup::CUBE_BOTTOM, tex_size, tex_below) }
                if north.transparent { Self::append_face(&mut vec, &lookup::CUBE_NORTH, tex_size, tex_north) }
                if south.transparent { Self::append_face(&mut vec, &lookup::CUBE_SOUTH, tex_size, tex_south) }
                if east.transparent { Self::append_face(&mut vec, &lookup::CUBE_EAST, tex_size, tex_east) }
                if west.transparent { Self::append_face(&mut vec, &lookup::CUBE_WEST, tex_size, tex_west) }
                vec
            }
        };

        // position vertices
        vec.for_each_mut(|vertex| { 
            vertex.position[0] += position.x; 
            vertex.position[1] += position.y; 
            vertex.position[2] += position.z; 
        });

        // return vertices
        return vec;
    }

    fn append_face(target: &mut Vec<Vertex>, input: &[Vertex], tex_size: Vector2<u32>, idx: &u16) {
        // create input vertices
        let mut input = input.clone().to_vec();

        // reposition texture coords
        let width = 16.0 / tex_size.x as f32;
        let height = 16.0 / tex_size.y as f32;
        let horizontal_position = *idx as f32 * width;
        let layer = horizontal_position.floor();
        let x = horizontal_position - layer;
        let y = height as f32 * layer;
        input.for_each_mut(|vertex| {
            vertex.tex_coords[0] = vertex.tex_coords[0] * width + x;
            vertex.tex_coords[1] = vertex.tex_coords[1] * height + y;
        });

        // add vertices
        target.extend(input);
    }
}