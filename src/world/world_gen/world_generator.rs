use crate::world::world_gen::biome::Biome;
use crate::world::world_gen::biome_warp::WarpSettings;
use noise::{Fbm, NoiseFn, Perlin};
use tinyvec::TinyVec;
use crate::utils::hash_u64;

const NEIGHBOR_OFFSETS: [(i32, i32); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// https://github.com/TanTanDev/worley_biomes/blob/main/src/worley.rs
pub struct WorldGenerator {
    pub zoom: f64,
    ///! high value: sharper borders, recommended: 0.0 -> 20.0
    pub sharpness: f64,
    pub k: usize,
    pub seed: u64,
    pub warp_settings: WarpSettings,
    ///! if set, biomes below this threshold, will not return from Worley::get()
    ///! recommended to be set, defaults to 0.01 = 1%
    pub kill_percent_threshold: Option<f64>,

    temperature_noise: Fbm<Perlin>,
    humidity_noise: Fbm<Perlin>,
}

impl WorldGenerator {
    fn new(seed: u64) -> Self {
        let mut temperature_noise = Fbm::new(seed as u32);
        temperature_noise.octaves = 4;
        temperature_noise.persistence = 0.5;
        temperature_noise.lacunarity = 2.0;

        let mut humidity_noise = Fbm::new(seed as u32 + 1337u32);
        humidity_noise.octaves = 5;
        humidity_noise.persistence = 0.5;
        humidity_noise.lacunarity = 2.0;

        Self {
            zoom: 100.0,
            sharpness: 20.0,
            k: 3,
            seed,
            warp_settings: WarpSettings {
                strength: 0.0,
                noise: Perlin::new(seed as u32),
            },
            kill_percent_threshold: Some(1.0),
            temperature_noise,
            humidity_noise,
        }
    }

    pub fn get_biomes_at(&self, x: f64, z: f64) -> TinyVec<[(f64, Biome); 3]> {
        let (x, z) = (x / self.zoom, z / self.zoom);
        let (x, z) = self.warp_settings.warp_coords(x, z);
        let cell_x = x.floor() as i32;
        let cell_z = z.floor() as i32;

        let mut candidates: [(f64, Biome); 9] = [(0.0, Biome::Unknown); 9];
        for (i, &(dx, dz)) in NEIGHBOR_OFFSETS.iter().enumerate() {
            let neighbor_x = cell_x + dx;
            let neighbor_z = cell_z + dz;
            let (point_x, point_z) = get_cell_feature_point(self.seed, neighbor_x, neighbor_z);
            let dist = ((x - point_x).powi(2) + (z - point_z).powi(2)).sqrt();
            let biome = self.get_biome_at(point_x, point_z);
            candidates[i] = (dist, biome);
        }

        let k = self.k.min(candidates.len());
        candidates.select_nth_unstable_by(k, |a, b| a.0.total_cmp(&b.0));

        let mut out = TinyVec::with_capacity(self.k);
        out
    }

    pub fn get_biome_at(&self, x: f64, z: f64) -> Biome {
        let temperature = self.temperature_noise.get([x / 800.0, z / 800.0]) * 200.0 - 100.0; // 0-100
        let humidity = self.temperature_noise.get([x / 600.0, z / 600.0]) * 200.0 - 100.0; // 0-100
        Biome::from_climate(
            temperature as f32,
            humidity as f32,
        )
    }
}

fn get_cell_feature_point(seed: u64, cell_x: i32, cell_z: i32) -> (f64, f64) {
    let h1 = hash_u64(seed.wrapping_add(1337), cell_x, cell_z);
    let h2 = hash_u64(seed.wrapping_add(7331), cell_x, cell_z);

    let fx = cell_x as f64 + ((h1 & 0xFFFF) as f64 / 65535.0);
    let fz = cell_z as f64 + ((h2 & 0xFFFF) as f64 / 65535.0);
    (fx, fz)
}
