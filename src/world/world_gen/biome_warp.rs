use noise::{NoiseFn, Perlin};

// https://github.com/TanTanDev/worley_biomes/blob/main/src/warp.rs
#[derive(Default)]
pub struct WarpSettings {
    pub strength: f64,
    pub noise: Perlin,
}

impl WarpSettings {
    pub fn warp_coords(&self, x: f64, z: f64) -> (f64, f64) {
        let nx = self.noise.get([x, z]);
        let nz = self.noise.get([x + 103.0, z]);
        (x + nx * self.strength, z + nz * self.strength)
    }
}
