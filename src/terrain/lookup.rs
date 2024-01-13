#![allow(dead_code)]

use forte_engine::render::primitives::vertices::Vertex;

pub const CUBE_TOP: [Vertex; 6] = [
    Vertex { position: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [0.0, 1.0, 1.0], tex_coords: [0.0, 1.0], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [0.0, 1.0, 1.0], tex_coords: [0.0, 1.0], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 1.0, 0.0] },
];

pub const CUBE_BOTTOM: [Vertex; 6] = [
    Vertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [1.0, 0.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, -1.0, 0.0] },
    Vertex { position: [0.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, -1.0, 0.0] },
];

pub const CUBE_NORTH: [Vertex; 6] = [
    Vertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [0.0, 1.0, 1.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [0.0, 1.0, 1.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0] },
];

pub const CUBE_SOUTH: [Vertex; 6] = [
    Vertex { position: [0.0, 0.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [1.0, 0.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
];

pub const CUBE_EAST: [Vertex; 6] = [
    Vertex { position: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 0.0, 1.0], tex_coords: [0.0, 1.0], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], normal: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 0.0, 1.0], tex_coords: [0.0, 1.0], normal: [1.0, 0.0, 0.0] },
];

pub const CUBE_WEST: [Vertex; 6] = [
    Vertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [0.0, 1.0, 1.0], tex_coords: [1.0, 1.0], normal: [-1.0, 0.0, 0.0] },
    Vertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
];
