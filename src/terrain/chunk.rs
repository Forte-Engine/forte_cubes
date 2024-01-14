use std::marker::PhantomData;

use cgmath::*;
use forte_engine::{math::transforms::Transform, render::{primitives::{mesh::Mesh, transforms::TransformRaw, vertices::Vertex}, resources::Handle, render_engine::RenderEngine}};
use wgpu::util::DeviceExt;

use crate::terrain::{blocks::*, ChunkEngine};

const CHUNK_SIZE: usize = 16;

// todo general block renderer that can be easily repurosed for non-standard rendering, like a fench post
// todo allow for tile entities (may have to wait)

#[derive(Debug)]
pub struct Chunk<T: BlockDefinitions<M>, M: MaterialDef + 'static> {
    pub id: u32,
    pub transform: Transform,
    data: [[[(u16, u16); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    buffer: Option<wgpu::Buffer>,
    handle: Option<Handle<Mesh>>,
    phantom_definitions: PhantomData<T>,
    phantom_material: PhantomData<M>
}

impl <T: BlockDefinitions<M>, M: MaterialDef + 'static> Chunk<T, M> {
    pub fn empty(id: u32) -> Self { Self::new(id, [[[(0, 0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) }
    pub fn set(&mut self, location: Vector3<usize>, value: M, data: u16) { self.data[location.x][location.y][location.z] = (value.into(), data); }
    pub fn get(&self, location: Vector3<usize>) -> M { self.data[location.x][location.y][location.z].0.into() }
    pub fn handle(&self) -> Option<&Handle<Mesh>> { self.handle.as_ref() }
    pub fn buffer(&self) -> Option<&wgpu::Buffer> { self.buffer.as_ref() }

    pub fn new(id: u32, data: [[[(u16, u16); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> Self { 
        Self { 
            id, 
            transform: Transform::default(), 
            data, 
            buffer: None,
            handle: None,
            phantom_definitions: PhantomData::default(),
            phantom_material: PhantomData::default()
        } 
    }

    pub(crate) fn render_buffer(&mut self, engine: &RenderEngine) {
        // make sure buffer exists
        if self.buffer.is_none() {
            let buffer = engine.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: &bytemuck::cast_slice(&[TransformRaw::from_generic(&self.transform)]),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                }
            );
            self.buffer = Some(buffer);
        }
        // otherwise, update the buffer
        else {
            let buffer = self.buffer.as_ref().expect("Could not ensure buffer for chunk!");
            engine.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[TransformRaw::from_generic(&self.transform)]));
        }
    }

    pub fn ensure_handle_buffer(&mut self, engine: &mut RenderEngine, chunk_engine: &ChunkEngine<T, M>) -> &Handle<Mesh> {
        if self.handle.is_none() { self.gen_mesh(engine, chunk_engine); }
        self.handle.as_ref().expect("Could not ensure handle in chunk!")
    }

    pub fn gen_mesh(&mut self, engine: &mut RenderEngine, chunk_engine: &ChunkEngine<T, M>) {
        // create vertices and indices lists
        let mut vertices: Vec<Vertex> = vec![];

        // get atlas texture size
        let texture = engine.texture(&chunk_engine.atlas);
        let tex_size = Vector2 { x: texture.texture.width(), y: texture.texture.height() };

        // generate chunk
        for x in 0 .. CHUNK_SIZE {
            for y in 0 .. CHUNK_SIZE {
                for z in 0 .. CHUNK_SIZE {
                    self.gen_cube(Vector3 { x, y, z }, &mut vertices, tex_size);
                }
            }
        }

        // create final mesh
        let mesh = engine.create_mesh(format!("chunk_{}", self.id).as_str(), &vertices, &[]);
        self.handle = Some(mesh);
    }

    fn gen_cube(&mut self, position: Vector3<usize>, vertices: &mut Vec<Vertex>, tex_size: Vector2<u32>) {
        // get current block
        let current = &T::DEFINITIONS[self.get(position).into() as usize];
        let combined = Vector3 { x: position.x as f32, y: position.y as f32, z: position.z as f32 };

        // get relative block types and combined transform position and chunk position
        let above = if position.y < CHUNK_SIZE - 1 { self.get(Vector3 { x: position.x, y: position.y + 1, z: position.z }) } else { 1.into() };
        let below = if position.y > 0 { self.get(Vector3 { x: position.x, y: position.y - 1, z: position.z }) } else { 1.into() };
        let north = if position.z < CHUNK_SIZE - 1 { self.get(Vector3 { x: position.x, y: position.y, z: position.z + 1 }) } else { 1.into() };
        let south = if position.z > 0 { self.get(Vector3 { x: position.x, y: position.y, z: position.z - 1 }) } else { 1.into() };
        let east = if position.x < CHUNK_SIZE - 1 { self.get(Vector3 { x: position.x + 1, y: position.y, z: position.z }) } else { 1.into() };
        let west = if position.x > 0 { self.get(Vector3 { x: position.x - 1, y: position.y, z: position.z }) } else { 1.into() };

        // get definitions for relatives
        let above = &T::DEFINITIONS[above.into() as usize];
        let below = &T::DEFINITIONS[below.into() as usize];
        let north = &T::DEFINITIONS[north.into() as usize];
        let south = &T::DEFINITIONS[south.into() as usize];
        let east = &T::DEFINITIONS[east.into() as usize];
        let west = &T::DEFINITIONS[west.into() as usize];

        // render
        let vec = current.renderer.render(combined, tex_size, above, below, north, south, east, west);
        vertices.extend(vec);
    }
}
