use bevy::mesh::{MeshVertexAttribute, VertexFormat};

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_SIZE2: i32 = 256;
pub const CHUNK_SIZE3: i32 = 4096;
pub const PADDED_CHUNK_SIZE: i32 = 18;
pub const PADDED_CHUNK_SIZE_USIZE: usize = 18;
pub const PADDED_CHUNK_SIZE2: i32 = 324;
pub const PADDED_CHUNK_SIZE2_USIZE: usize = 324;
pub const PADDED_CHUNK_SIZE3: i32 = 5832;
pub const PADDED_CHUNK_SIZE3_USIZE: usize = 5832;

pub const MAX_MESH_TASKS: usize = 32;
pub const MAX_DATA_TASKS: usize = 64;

pub const ATTRIBUTE_VOXEL: MeshVertexAttribute =
    MeshVertexAttribute::new("Voxel", 988540919, VertexFormat::Uint32);
