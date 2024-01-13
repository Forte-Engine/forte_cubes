use std::{fmt::Debug, marker::PhantomData};

use blocks::*;
use chunk::Chunk;
use forte_engine::{render::{pipelines::Pipeline, textures::textures::Texture, resources::Handle, render_engine::{RenderEngine, DrawMesh}, primitives::{transforms::TransformRaw, cameras::Camera, vertices::Vertex}}, lights::lights::LightUniform};

pub mod blocks;
pub mod chunk;
pub mod lookup;

#[derive(Debug)]
pub struct ChunkEngine<T: BlockDefinitions<M>, M: MaterialDef + 'static> {
    pipeline: Pipeline,
    atlas: Handle<Texture>,
    phantom_def: PhantomData<T>,
    phantom_mat: PhantomData<M>
}


#[include_wgsl_oil::include_wgsl_oil("../../shaders/terrain.wgsl")]
mod terrain_shader {}

impl <T: BlockDefinitions<M>, M: MaterialDef + 'static> ChunkEngine<T, M> {
    pub fn new(engine: &mut RenderEngine) -> Self {
        Self { 
            pipeline: Pipeline::new(
                "chunk", engine, terrain_shader::SOURCE,
                &[Vertex::desc(), TransformRaw::desc()],
                &[
                    &engine.device.create_bind_group_layout(&Camera::BIND_LAYOUT),
                    &engine.device.create_bind_group_layout(&Texture::BIND_LAYOUT),
                    &engine.device.create_bind_group_layout(&LightUniform::BIND_LAYOUT)
                ]
            ),
            atlas: engine.load_texture(T::ATLAS),
            phantom_def: PhantomData::default(),
            phantom_mat: PhantomData::default()
        }
    }
}

pub trait DrawChunks<'a, 'b> where 'b: 'a {
    fn prepare_chunk_draw<T: BlockDefinitions<M>, M: MaterialDef + 'static>(
        &mut self, 
        engine: &'b ChunkEngine<T, M>, 
        camera: &'b Camera
    );

    fn draw_chunk<T: BlockDefinitions<M>, M: MaterialDef + 'static>(
        &mut self,
        engine: &'b RenderEngine,
        chunk_engine: &'b ChunkEngine<T, M>, 
        chunk: &'b mut Chunk<T, M>
    );
}

impl<'a, 'b> DrawChunks<'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
    fn prepare_chunk_draw<T: BlockDefinitions<M>, M: MaterialDef + 'static>(
        &mut self, 
        engine: &'b ChunkEngine<T, M>, 
        camera: &'b Camera
    ) {
        self.prepare_draw(&engine.pipeline, camera);
    }

    fn draw_chunk<T: BlockDefinitions<M>, M: MaterialDef + 'static>(
        &mut self,
        engine: &'b RenderEngine,
        chunk_engine: &'b ChunkEngine<T, M>, 
        chunk: &'b mut Chunk<T, M>
    ) {
        chunk.render_buffer(engine);
        let handle = chunk.handle().expect("Chunk must have rendered mesh to be drawn!");
        let buffer = chunk.buffer().expect("Buffer did not render!");
        self.draw_list_mesh(engine, handle, &chunk_engine.atlas, buffer, 1);
    }
}
