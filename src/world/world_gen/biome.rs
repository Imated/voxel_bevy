use crate::world::world_gen::climate::Climate;
use std::cmp::PartialEq;
use std::ops::Range;
use crate::utils::RangeExtensions;

const BIOME_DATA: &'static [(Biome, BiomeProperties)] = &[
    (
        Biome::Plains,
        BiomeProperties {
            climate: Climate {
                temperature: 55.0..80.0,
                humidity: 60.0..90.0,
            },
        },
    ),
    (
        Biome::Desert,
        BiomeProperties {
            climate: Climate {
                temperature: 75.0..105.0,
                humidity: 10.0..30.0,
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
}

impl Biome {
    pub fn properties(self) -> &'static BiomeProperties {
        BIOME_DATA
            .iter()
            .find(|(biome, _)| *biome == self)
            .map(|(_, props)| props)
            .expect("Biome properties not found")
    }

    pub fn from_climate(temperature: f32, humidity: f32) -> Biome {
        BIOME_DATA
            .iter()
            .filter(|(biome, _)| *biome != Biome::Unknown)
            .min_by_key(|(_, properties)| {
                let temp_diff = (temperature - properties.climate.temperature.mid()).abs();
                let humidity_diff = (humidity - properties.climate.humidity.mid()).abs();
                ((temp_diff + humidity_diff) * 1000.0) as i32
            })
            .map(|(biome, _)| *biome)
            .unwrap_or(Biome::Unknown)
    }
}

pub struct BiomeProperties {
    pub climate: Climate,
}
