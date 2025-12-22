mod chunk;
mod chunk_render_plugin;
mod greedy_chunk_render_plugin;
mod quad;
mod block;

use crate::chunk_render_plugin::ChunkRenderPlugin;
use bevy::app::{App, PluginGroup};
use bevy::color::palettes::basic::WHITE;
use bevy::DefaultPlugins;
use bevy::log::LogPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::default;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::RenderPlugin;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy_flycam::PlayerPlugin;
use crate::greedy_chunk_render_plugin::GreedyChunkRenderPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),

            WireframePlugin::default(),
        )).insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })
        //.add_plugins(ChunkRenderPlugin::default())
        .add_plugins(GreedyChunkRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .run();
}
