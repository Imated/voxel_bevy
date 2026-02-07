#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct ChunkSectionMesh {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

impl ChunkSectionMesh {
    pub fn new(vertices: Vec<u32>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }
}
