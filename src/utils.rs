use bevy::math::UVec3;

pub fn pack_vertex(position: UVec3, normal: u32, block_type: u32, section_y: u32) -> u32 {
    position.x
        | position.y << 5u32
        | position.z << 10u32
        | normal << 15u32
        | block_type << 18u32
        | section_y << 28u32
}