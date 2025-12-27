use crate::world::World;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, ReflectResource, Time};
use bevy::prelude::{Reflect, Res, ResMut, Resource};
use bevy::time::common_conditions::on_timer;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use std::time::Duration;

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct WorldStats {
    pub loaded_chunks: usize,
    pub data_to_load: usize,
    pub data_to_unload: usize,
    pub mesh_to_load: usize,
    pub mesh_to_unload: usize,
    pub active_data_tasks: usize,
    pub active_mesh_tasks: usize,

    #[inspector(min = 0, max = 100)]
    pub sample_size: usize,

    pub loaded_chunk_positions: Vec<(i32, i32)>,
    pub data_load_queue: Vec<(i32, i32)>,
    pub mesh_load_queue: Vec<(i32, i32)>,
}

pub struct DebugWorldPlugin;

impl Plugin for DebugWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
            .init_resource::<WorldStats>()
            .register_type::<WorldStats>()
            .add_plugins(ResourceInspectorPlugin::<WorldStats>::default())
            .add_plugins(ResourceInspectorPlugin::<Time>::default())
            .add_systems(
                Update,
                Self::update_world_stats.run_if(on_timer(Duration::from_secs_f32(0.5))),
            );
    }
}

impl DebugWorldPlugin {
    pub fn update_world_stats(world: Res<World>, mut stats: ResMut<WorldStats>) {
        stats.loaded_chunks = world.loaded_chunks.len();
        stats.data_to_load = world.chunks_data_to_load.len();
        stats.data_to_unload = world.chunks_data_to_unload.len();
        stats.mesh_to_load = world.chunks_mesh_to_load.len();
        stats.mesh_to_unload = world.chunks_mesh_to_unload.len();
        stats.active_data_tasks = world.data_tasks.len();
        stats.active_mesh_tasks = world.mesh_tasks.len();

        if stats.sample_size == 0 {
            stats.sample_size = 10;
        }

        let limit = stats.sample_size;

        stats.loaded_chunk_positions = world
            .loaded_chunks
            .keys()
            .take(limit)
            .map(|&pos| (pos.0.x, pos.0.y))
            .collect();

        stats.data_load_queue = world
            .chunks_data_to_unload
            .iter()
            .take(limit)
            .map(|&pos| (pos.0.x, pos.0.y))
            .collect();

        stats.mesh_load_queue = world
            .chunks_mesh_to_load
            .iter()
            .take(limit)
            .map(|&pos| (pos.0.x, pos.0.y))
            .collect();
    }
}
