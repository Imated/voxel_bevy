use crate::utils::RangeExtensions;
use crate::world::world_gen::climate::Climate;

const BIOME_DATA: &'static [(Biome, BiomeProperties)] = &[
    (
        Biome::Ice,
        BiomeProperties {
            climate: Climate {
                temperature: 0.0..35.0,
                humidity: 0.0..100.0,  // Ice can be dry or snowy
            },
        },
    ),
    (
        Biome::Tundra,
        BiomeProperties {
            climate: Climate {
                temperature: 20.0..45.0,
                humidity: 0.0..100.0,
            },
        },
    ),
    (
        Biome::Plains,
        BiomeProperties {
            climate: Climate {
                temperature: 40.0..65.0,
                humidity: 30.0..60.0,
            },
        },
    ),
    (
        Biome::Desert,
        BiomeProperties {
            climate: Climate {
                temperature: 55.0..100.0,
                humidity: 0.0..35.0,
            },
        },
    ),
    (
        Biome::Tropical,
        BiomeProperties {
            climate: Climate {
                temperature: 55.0..100.0,
                humidity: 35.0..100.0,
            },
        },
    ),
];

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Biome {
    #[default]
    Unknown,
    Plains,
    Desert,
    Ice,
    Tundra,
    Tropical,
}

impl Biome {
    pub fn properties(self) -> &'static BiomeProperties {
        BIOME_DATA
            .iter()
            .find(|(biome, _)| *biome == self)
            .map(|(_, props)| props)
            .expect("Biome properties not found")
    }

    pub fn from_climate(temperature: f64, humidity: f64, variation: f64) -> Biome {
        let candidates: Vec<Biome> = BIOME_DATA
            .iter()
            .filter(|(biome, props)| {
                *biome != Biome::Unknown
                    && props.climate.temperature.contains(&(temperature as f32))
                    && props.climate.humidity.contains(&(humidity as f32))
            })
            .map(|(biome, _)| *biome)
            .collect();

        if candidates.is_empty() {
            return Self::closest_climate(temperature, humidity);
        }

        let index = (variation * candidates.len() as f64) as usize;
        candidates[index.min(candidates.len() - 1)]
    }

    fn closest_climate(temperature: f64, humidity: f64) -> Biome {
        BIOME_DATA
            .iter()
            .filter(|(biome, _)| *biome != Biome::Unknown)
            .min_by_key(|(_, properties)| {
                let temp_diff = (temperature as f32 - properties.climate.temperature.mid()).abs();
                let humidity_diff = (humidity as f32 - properties.climate.humidity.mid()).abs();
                ((temp_diff + humidity_diff) * 1000.0) as i32
            })
            .map(|(biome, _)| *biome)
            .unwrap_or(Biome::Unknown)
    }
}

pub struct BiomeProperties {
    pub climate: Climate,
}
