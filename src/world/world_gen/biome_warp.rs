use noise::{NoiseFn, Perlin};

// https://github.com/TanTanDev/worley_biomes/blob/main/src/warp.rs
#[derive(Default, Copy, Clone, Debug)]
pub struct WarpSettings {
    pub strength: f64,
    pub noise: Perlin,
}

impl WarpSettings {
    pub fn warp_coords(&self, x: f64, z: f64) -> (f64, f64) {
        let offset_x = self.noise.get([x * 0.3, z * 0.3]) * self.strength
            + self.noise.get([x * 1.5, z * 1.5]) * self.strength * 0.25;
        let offset_z = self.noise.get([x * 0.3 + 100.0, z * 0.3 + 100.0]) * self.strength
            + self.noise.get([x * 1.5 + 100.0, z * 1.5 + 100.0]) * self.strength * 0.25;
        (x + offset_x, z + offset_z)
    }
}
