use crate::utils::{hash_u64, smootherstep01};
use noise::{Fbm, NoiseFn, Perlin};
use tinyvec::TinyVec;
use crate::world::world_gen::biome::Biome;
use crate::world::world_gen::biome_warp::WarpSettings;

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
#[derive(Debug, Clone)]
pub struct BiomeGenerator {
    pub zoom: f64,
    // high value: sharper borders,
    pub sharpness: f64,
    pub k: usize,
    pub seed: u64,
    pub warp_settings: WarpSettings,
    // if set, biomes below this threshold will be ignored
    pub kill_percent_threshold: Option<f64>,

    pub temperature_noise: Fbm<Perlin>,
    pub humidity_noise: Fbm<Perlin>,
    pub biome_variation_noise: Fbm<Perlin>,

    pub temperature_scale: f64,
    pub humidity_scale: f64,
    pub variation_scale: f64,
}

impl BiomeGenerator {
    pub fn new(seed: u64) -> Self {
        let mut temperature_noise = Fbm::new(seed as u32);
        temperature_noise.octaves = 2;
        temperature_noise.persistence = 0.5;
        temperature_noise.lacunarity = 2.0;

        let mut humidity_noise = Fbm::new(seed as u32 + 1337u32);
        humidity_noise.octaves = 2;
        humidity_noise.persistence = 0.5;
        humidity_noise.lacunarity = 2.0;

        let mut biome_variation_noise = Fbm::new(seed as u32 + 9999);
        biome_variation_noise.octaves = 1;


        Self {
            zoom: 120.0,
            sharpness: 5.0,
            k: 3,
            seed,
            warp_settings: WarpSettings {
                strength: 0.8,
                noise: Perlin::new(seed as u32),
            },
            kill_percent_threshold: Some(0.05),
            temperature_noise,
            humidity_noise,
            biome_variation_noise,
            temperature_scale: 0.03, // .15
            humidity_scale: 0.05, // .25
            variation_scale: 0.05,
        }
    }

    pub fn get_biomes_at(&self, x: f64, z: f64) -> TinyVec<[(f64, Biome); 3]> {
        let (x, z) = (x / self.zoom, z / self.zoom);
        let (warped_x, warped_z) = self.warp_settings.warp_coords(x, z);
        let cell_x = warped_x.floor() as i32;
        let cell_z = warped_z.floor() as i32;

        let mut candidates: [(f64, Biome); 9] = [(0.0, Biome::Unknown); 9];
        for (i, &(dx, dz)) in NEIGHBOR_OFFSETS.iter().enumerate() {
            let neighbor_x = cell_x + dx;
            let neighbor_z = cell_z + dz;
            let (point_x, point_z) = get_cell_feature_point(self.seed, neighbor_x, neighbor_z);
            let dist = ((warped_x - point_x).powi(2) + (warped_z - point_z).powi(2)).sqrt();
            let biome = self.get_biome_at(point_x, point_z, neighbor_x, neighbor_z);
            candidates[i] = (dist, biome);
        }

        let k = self.k.min(candidates.len());
        candidates.select_nth_unstable_by(k, |a, b| a.0.total_cmp(&b.0));

        let mut sum = 0.0;
        let mut out = TinyVec::with_capacity(k);
        for (dist, biome) in candidates.into_iter().take(k) {
            let weight = (1.0 / dist.powf(self.sharpness)).clamp(0.0, 100.0);
            sum += weight;
            out.push((weight, biome));
        }

        for (weight, _) in out.iter_mut() {
            *weight /= sum;
        }

        if let Some(kill_percent_threshold) = self.kill_percent_threshold {
            out.retain(|(percent, _)| *percent > kill_percent_threshold);

            let new_sum: f64 = out.iter().map(|(w, _)| w).sum();
            if new_sum > 0.0 {
                for (weight, _) in out.iter_mut() {
                    *weight /= new_sum;
                }
            }
        }

        out
    }

    pub fn get_biome_at(&self, point_x: f64, point_z: f64, _cell_x: i32, _cell_z: i32) -> Biome {
        let temperature = self.get_temperature_at(point_x, point_z);
        let humidity = self.get_humidity_at(point_x, point_z);
        let variation = (self.biome_variation_noise.get([point_x * self.variation_scale, point_z * self.variation_scale]) + 1.0) * 0.5;

        Biome::from_climate(temperature, humidity, variation)
    }

    pub fn get_temperature_at(&self, x: f64, z: f64) -> f64 {
        let temp = (self.temperature_noise.get([x * self.temperature_scale, z * self.temperature_scale]) + 1.0) * 0.5;
        smootherstep01(temp).clamp(0.0, 1.0) * 100.0
    }

    pub fn get_humidity_at(&self, x: f64, z: f64) -> f64 {
        let humidity = (self.humidity_noise.get([x * self.humidity_scale, z * self.humidity_scale]) + 1.0) * 0.5;
        smootherstep01(humidity).clamp(0.0, 1.0) * 100.0
    }
}

pub(crate) fn get_cell_feature_point(seed: u64, cell_x: i32, cell_z: i32) -> (f64, f64) {
    let h1 = hash_u64(seed.wrapping_add(1337), cell_x, cell_z);
    let h2 = hash_u64(seed.wrapping_add(7331), cell_x, cell_z);

    let fx = cell_x as f64 + ((h1 & 0xFFFF) as f64 / 65535.0);
    let fz = cell_z as f64 + ((h2 & 0xFFFF) as f64 / 65535.0);
    (fx, fz)
}
