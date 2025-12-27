use bevy::math::IVec3;

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
        vertices: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        face_dir: Direction,
        offset: i32,
    ) {
        let face_offset = match face_dir {
            Direction::Up | Direction::Right | Direction::Back => 1i32,
            _ => 0i32,
        };
        let offset = offset + face_offset;

        let v0 = face_dir
            .world_to_sample(offset, self.x as i32, self.y as i32)
            .as_vec3()
            .to_array();
        let v1 = face_dir
            .world_to_sample(offset, (self.x + self.w) as i32, self.y as i32)
            .as_vec3()
            .to_array();
        let v2 = face_dir
            .world_to_sample(offset, (self.x + self.w) as i32, (self.y + self.h) as i32)
            .as_vec3()
            .to_array();
        let v3 = face_dir
            .world_to_sample(offset, self.x as i32, (self.y + self.h) as i32)
            .as_vec3()
            .to_array();
        let mut new_vertices = vec![v0, v1, v2, v3];

        if face_dir.should_reverse() {
            let o = new_vertices.split_off(1);
            o.into_iter().rev().for_each(|i| new_vertices.push(i));
        }

        vertices.extend(new_vertices);

        for _ in 0..4 {
            normals.push(face_dir.normals());
        }
    }
}
