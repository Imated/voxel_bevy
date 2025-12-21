use bevy::prelude::{Component, IVec3};
use std::num::NonZeroU8;

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_HEIGHT: i32 = 256;

#[derive(Component, Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Chunk {
    blocks: Vec<Block>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block(0); (CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT) as usize],
        }
    }

    pub fn coords_by_index(mut index: i32) -> IVec3 {
        let z = index / (CHUNK_SIZE * CHUNK_HEIGHT);
        index -= z * (CHUNK_SIZE * CHUNK_HEIGHT);

        let y = index / CHUNK_SIZE;
        index -= y * CHUNK_SIZE;

        let x = index;

        IVec3 { x, y, z }
    }

    pub fn get_by_index(&self, index: i32) -> Block {
        if index >= self.blocks.len() as i32 {
            return Block(0);
        }
        self.blocks[index as usize]
    }

    pub fn set_by_index(&mut self, index: i32, id: Block) {
        if index >= self.blocks.len() as i32 {
            return;
        }
        self.blocks[index as usize] = id;
    }

    pub fn get_by_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return Block(0);
        }
        self.blocks[(x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_HEIGHT)) as usize]
    }

    pub fn set_by_xyz(&mut self, x: i32, y: i32, z: i32, id: Block) {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return;
        }
        self.blocks[(x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_HEIGHT)) as usize] = id;
    }

    pub fn get(&self, coords: IVec3) -> Block {
        if coords.x < 0
            || coords.x >= CHUNK_SIZE
            || coords.y < 0
            || coords.y >= CHUNK_HEIGHT
            || coords.z < 0
            || coords.z >= CHUNK_SIZE
        {
            return Block(0);
        }
        self.blocks
            [(coords.x + (coords.y * CHUNK_SIZE) + (coords.z * CHUNK_SIZE * CHUNK_HEIGHT)) as usize]
    }

    pub fn set(&mut self, coords: IVec3, id: Block) {
        if coords.x < 0
            || coords.x >= CHUNK_SIZE
            || coords.y < 0
            || coords.y >= CHUNK_HEIGHT
            || coords.z < 0
            || coords.z >= CHUNK_SIZE
        {
            return;
        }
        self.blocks[(coords.x + (coords.y * CHUNK_SIZE) + (coords.z * CHUNK_SIZE * CHUNK_HEIGHT))
            as usize] = id;
    }
}

/// Block representation
///
/// msb  ``u3: orientation``
///      ``u3: variant``
/// lsb  ``u10: id``
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Block(pub u16);

impl Block {
    pub const ID_MASK: u16 = 0x3FF; // Bottom 10 bits
    pub const VARIANT_MASK: u16 = 0x1C00; // Bits 10-12
    pub const ORIENTATION_MASK: u16 = 0xE000; // Last 3 bits

    pub fn new(data: u16) -> Block {
        Self(data)
    }

    pub fn from_id(id: u16) -> Block {
        let data = id & Self::ID_MASK;

        Self(data)
    }

    pub fn from_id_variant(id: u16, variant: u16) -> Block {
        let data = (id & Self::ID_MASK) | ((variant << 10) & Self::VARIANT_MASK);

        Self(data)
    }

    pub fn from_id_variant_orientation(id: u16, variant: u16, orientation: u16) -> Block {
        let data = (id & Self::ID_MASK)
            | ((variant << 10) & Self::VARIANT_MASK)
            | ((orientation << 13) & Self::ORIENTATION_MASK);

        Self(data)
    }

    pub fn id(&self) -> u16 {
        self.0 & Self::ID_MASK
    }

    pub fn variant(&self) -> u8 {
        ((self.0 & Self::VARIANT_MASK) >> 10) as u8
    }

    pub fn orientation(&self) -> u8 {
        ((self.0 & Self::ORIENTATION_MASK) >> 13) as u8
    }

    pub fn set_id(&mut self, id: u16) {
        self.0 = (self.0 & !Self::ID_MASK) | (id & Self::ID_MASK);
    }

    pub fn set_variant(&mut self, variant: u16) {
        self.0 = (self.0 & !Self::VARIANT_MASK) | ((variant << 10) & Self::VARIANT_MASK);
    }

    pub fn set_orientation(&mut self, orientation: u16) {
        self.0 =
            (self.0 & !Self::ORIENTATION_MASK) | ((orientation << 10) & Self::ORIENTATION_MASK);
    }
}
