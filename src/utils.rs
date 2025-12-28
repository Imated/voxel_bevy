use bevy::math::{Vec3, Vec4};

pub trait WithPadding {
    fn to_vec4(&self) -> Vec4;
}

impl WithPadding for Vec3 {
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 0.0)
    }
}