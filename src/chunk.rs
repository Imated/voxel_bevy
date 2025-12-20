use bevy::prelude::Component;
use std::ops::Not;

#[derive(Component)]
pub struct Chunk {
    blocks: Vec<Block>,
}

/// Block representation
///
/// msb  ``u3: orientation``
///      ``u3: variant``
/// lsb  ``u10: id``
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Block(u16);

impl Block {
    pub const ID_MASK: u16 = 0x3FF; // Bottom 10 bits
    pub const VARIANT_MASK: u16 = 0x1C00; // Bits 10-12
    pub const ORIENTATION_MASK: u16 = 0xE000; // Last 3 bits

    pub fn new(data: u16) -> Block {
        Self(data)
    }

    pub fn from_id(id: u16) -> Block {
        let data = id & 0x3FF;

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
        self.0 = (self.0 & !Self::ID_MASK) |
                (id & Self::ID_MASK);
    }

    pub fn set_variant(&mut self, variant: u16) {
        self.0 = (self.0 & !Self::VARIANT_MASK) |
            ((variant << 10) & Self::VARIANT_MASK);
    }

    pub fn set_orientation(&mut self, orientation: u16) {
        self.0 = (self.0 & !Self::ORIENTATION_MASK) |
            ((orientation << 10) & Self::ORIENTATION_MASK);
    }
}
