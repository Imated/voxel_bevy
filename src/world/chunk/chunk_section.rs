use crate::world::block::Block;
use crate::world::chunk::chunk::{CHUNK_SIZE, CHUNK_SIZEI};

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ChunkSection {
    blocks: Vec<Block>,
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block(0); (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
        }
    }

    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        for i in 0..CHUNK_SIZEI * CHUNK_SIZEI * CHUNK_SIZEI {
            if self.blocks[i as usize].id() != 0 {
                empty = false;
            }
        }

        empty
    }

    pub fn get_by_xyz(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        if x < 0 || x >= CHUNK_SIZEI || y < 0 || y >= CHUNK_SIZEI || z < 0 || z >= CHUNK_SIZEI {
            return None;
        }

        Some(self.blocks[(x + (y * CHUNK_SIZEI) + (z * CHUNK_SIZEI * CHUNK_SIZEI)) as usize])
    }

    pub fn set_by_xyz(&mut self, x: i32, y: i32, z: i32, id: Block) {
        if x < 0 || x >= CHUNK_SIZEI || y < 0 || y >= CHUNK_SIZEI || z < 0 || z >= CHUNK_SIZEI {
            return;
        }

        self.blocks[(x + (y * CHUNK_SIZEI) + (z * CHUNK_SIZEI * CHUNK_SIZEI)) as usize] = id;
    }
}
