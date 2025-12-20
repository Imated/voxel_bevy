use bevy::prelude::Component;

#[derive(Component)]
struct Chunk {
    blocks: Vec<Block>,
}

/// Block representation
///
/// msb  ``u3: orientation``
///      ``u3: variant``
/// lsb  ``u10: id``
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Block(u16);

impl Block {
    pub fn new(data: u16) -> Block {
        Self(data)
    }

    pub fn from_id(id: u16) -> Block {
        let data = id & 0b0000001111111111; // Bottom 10 bits of input id, orientation + variant all 0

        Self(data)
    }

    pub fn from_id_variant(id: u16, variant: u8) -> Block {
        let id = id & 0b0000001111111111; // Bottom 10 bits of input id
        let variant = variant & 0b00000111; // Bottom 3b of variant
        let data = id | ((variant as u16) << 10); // Place all values after each other

        Self(data)
    }

    pub fn from_id_variant_orientation(id: u16, variant: u8, orientation: u8) -> Block  {
        let id = id & 0b0000001111111111; // Bottom 10 bits of input id
        let variant = variant & 0b00000111; // Bottom 3b of variant
        let orientation = orientation & 0b00000111; // Bottom 3b of orientation
        let data = id | ((variant as u16) << 10) | ((orientation as u16) << 13); // Place all values after each other

        Self(data)
    }

    pub fn id(&self) -> u16 {
        self.0 & 0b0000001111111111
    }

    pub fn variant(&self) -> u8 {
        ((self.0 >> 10) & 0b00000111) as u8
    }

    pub fn orientation(&self) -> u8 {
        ((self.0 >> 13) & 0b00000111) as u8
    }

    pub fn set_id(&mut self, id: u16) {
        self.0 =
            (self.0 & 0b1111110000000000) | // keep variant + orientation, clear id
                (id & 0b0000001111111111);
    }

    pub fn set_variant(&mut self, variant: u8) {
        self.0 =
            (self.0 & 0b1110001111111111) | // clear bits 10–12
                (((variant & 0b00000111) as u16) << 10);
    }

    pub fn set_orientation(&mut self, orientation: u8) {
        self.0 =
            (self.0 & 0b0001111111111111) | // clear bits 13–15
                (((orientation & 0b00000111) as u16) << 13);
    }
}
