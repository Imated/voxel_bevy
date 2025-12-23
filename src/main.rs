mod block;
mod chunk;
mod chunk_render_plugin;
mod greedy_chunk_render_plugin;
mod quad;

use crate::chunk_render_plugin::ChunkRenderPlugin;
use crate::greedy_chunk_render_plugin::GreedyChunkRenderPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup};
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::{Window, default};
use bevy::render::RenderPlugin;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::window::{PresentMode, WindowPlugin};
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        // WARN this is a native only feature. It will not work with webgl or webgpu
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Voxel".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            WireframePlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })
        //.add_plugins(ChunkRenderPlugin::default())
        .add_plugins(GreedyChunkRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .run();
}
