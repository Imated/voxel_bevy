#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct ChunkMesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
}

impl ChunkMesh {
    pub fn new(vertices: Vec<[f32; 3]>, normals: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            normals,
        }
    }
}