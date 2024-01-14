use std::fmt::Debug;

use cgmath::*;
use forte_engine::{render::primitives::vertices::Vertex, math::vec::VecExt};

use crate::terrain::lookup;

/// A macro to make creating block and material definitions easier, as a material definition is just a quick enum u16 represented enum to reference a block definition.
/// See the documentation for `MaterialDef`, `BlockDef` and `BlockDefinitions` for more info.
/// 
/// Example
/// ```rust
/// define_blocks_materials!(
///     Blocks,                                 // The name that will be given to the created `BlockDefinitions instance.`
///     Material,                               // The name that will be given to the created `MaterialDef` instance.
///     "assets/test_blocks.png",               // The path to the texture atlas to be used for these blocks.
///     [                                       // An array of block definitions mapped to material definitions.
///         AIR => {                            // Start with the material for the block
///             transparent: true,              // Tell the engine if it will be transparent
///             renderer: BlockRenderer::None   // Tell the engine what `BlockRenderer` to use.  See the `BlockRenderer` documentation for more info.
///         },
///         GRASS => {
///             transparent: false,
///             renderer: BlockRenderer::Standard(0, 1, 0, 0, 0, 0)
///         }
///     ]
/// );
/// ```
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

/// This trait is to be implemented by all materials that use this terrain module.
/// They should be represented as a u16 so as those u16's are saved in the terrain chunks.
pub trait MaterialDef: Clone + Copy + Debug + Into<u16> + From<u16> {}

/// This trait is implemented by block definitions to be a continer for information needed for block definitions.
pub trait BlockDefinitions<M: MaterialDef + 'static> {
    /// The path to the texture atlas to be used to render all of the following `BlockDef`s
    const ATLAS: &'static str;
    /// An array of `BlockDef`s that are part of the `BlockDefinition` trait.  See `BlockDef` documentation for more info.
    const DEFINITIONS: &'static [BlockDef<M>];
}

/// This trait is implemented by each individual block type and contains information on how that block should be rendered.
pub struct BlockDef<M: MaterialDef + 'static> {
    /// The material that this `BlockDef` belongs too.  In an array of `BlockDef`s, each material should only appear once.
    pub material: M,
    /// Is this block transparent?
    pub transparent: bool,
    /// The `BlockRenderer` to be used to draw this block.  See `BlockRenderer` documentation for more info.
    pub renderer: BlockRenderer<M>
}

/// The `BlockRenderer` enum defines how a block should be rendered.
/// 
/// Generic M: MaterialDef + 'static
pub enum BlockRenderer<M: MaterialDef + 'static> {
    /// Render nothing.
    None,
    /// Just render a standard 1x1x1 block.  The u16's represent the atlas indices to texture each face of the cube.  They are in the order above, below, north, south, east, west.
    Standard(u16, u16, u16, u16, u16, u16),
    /// A csutom renderer that takes in a function that renders a `Vec<Vertex>` from 6 block definitions for each above, below, north, south, east, west.
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
    /// Effectively renders nothing.  It just marks a `BlockDef` that this is a block entity that needs to be rendered later.
    BlockEntity
}

impl <M: MaterialDef + 'static> BlockRenderer<M> {
    /// This function renders this block into a `Vec<Vertex>`
    /// 
    /// Arguments:
    /// * &self - This block renderer
    /// * position: Vector3<f32> - The position of this block in the chunk.
    /// * tex_size: Vector2<u32> - The size of the texture.
    /// * above: &BlockDef<M> - The block above.
    /// * below: &BlockDef<M> - The block below.
    /// * north: &BlockDef<M> - The block north.
    /// * south: &BlockDef<M> - The block south.
    /// * east: &BlockDef<M> - The block east.
    /// * west: &BlockDef<M> - The block west.
    /// 
    /// Retuns a rendered `Vec<Vertex>` of all the vertices of this block.
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