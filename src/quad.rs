use bevy::math::IVec3;
use crate::utils::pack_vertex;

// based on https://github.com/TanTanDev/binary_greedy_mesher_demo/blob/main/src/quad.rs
#[derive(Copy, Clone)]
pub enum Direction {
    Left = 0,
    Right,
    Down,
    Up,
    Back,
    Forward,
}

impl Direction {
    pub fn world_to_sample(&self, offset: i32, x: i32, y: i32) -> IVec3 {
        match self {
            Direction::Up => IVec3::new(x, offset, y),
            Direction::Down => IVec3::new(x, offset, y),
            Direction::Left => IVec3::new(offset, y, x),
            Direction::Right => IVec3::new(offset, y, x),
            Direction::Forward => IVec3::new(x, y, offset),
            Direction::Back => IVec3::new(x, y, offset),
        }
    }

    pub fn normals(&self) -> [f32; 3] {
        match self {
            Direction::Up => [0.0, 1.0, 0.0],
            Direction::Down => [0.0, -1.0, 0.0],
            Direction::Left => [-1.0, 0.0, 0.0],
            Direction::Right => [1.0, 0.0, 0.0],
            Direction::Forward => [0.0, 0.0, -1.0],
            Direction::Back => [0.0, 0.0, 1.0],
        }
    }

    pub fn normal_index(&self) -> u32 {
        match self {
            Direction::Left => 0u32,
            Direction::Right => 1u32,
            Direction::Down => 2u32,
            Direction::Up => 3u32,
            Direction::Forward => 4u32,
            Direction::Back => 5u32,
        }
    }

    pub fn should_reverse(&self) -> bool {
        match self {
            Direction::Up => true,      //+1
            Direction::Down => false,   //-1
            Direction::Left => false,   //-1
            Direction::Right => true,   //+1
            Direction::Forward => true, //-1
            Direction::Back => false,   //+1
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GreedyQuad {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl GreedyQuad {
    pub fn append_vertices(
        &self,
        vertices: &mut Vec<u32>,
        face_dir: Direction,
        block_type: u32,
        offset: i32,
    ) {
        let face_offset = match face_dir {
            Direction::Up | Direction::Right | Direction::Back => 1i32,
            _ => 0i32,
        };
        let offset = offset + face_offset;

        let v0 = pack_vertex(face_dir.world_to_sample(offset, self.x as i32, self.y as i32).as_uvec3(), face_dir.normal_index(), block_type);
        let v1 = pack_vertex(face_dir.world_to_sample(offset, (self.x + self.w) as i32, self.y as i32).as_uvec3(), face_dir.normal_index(), block_type);
        let v2 = pack_vertex(face_dir.world_to_sample(offset, (self.x + self.w) as i32, (self.y + self.h) as i32).as_uvec3(), face_dir.normal_index(), block_type);
        let v3 = pack_vertex(face_dir.world_to_sample(offset, self.x as i32, (self.y + self.h) as i32).as_uvec3(), face_dir.normal_index(), block_type);
        let mut new_vertices = vec![v0, v1, v2, v3];

        if face_dir.should_reverse() {
            let o = new_vertices.split_off(1);
            o.into_iter().rev().for_each(|i| new_vertices.push(i));
        }

        vertices.extend(new_vertices);
    }
}
