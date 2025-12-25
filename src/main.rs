mod block;
mod chunk;
mod chunk_render_plugin;
mod greedy_chunk_render_plugin;
mod quad;
mod world;
mod chunk_mesh;
mod chunk_loader;
mod debug_world;

use crate::greedy_chunk_render_plugin::GreedyChunkRenderPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, PostStartup, Startup};
use bevy::camera::Camera3d;
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::light::DirectionalLight;
use bevy::math::Vec3;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::{Window, default, With, Single, Commands, Transform};
use bevy::render::RenderPlugin;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin, PlayerPlugin};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::chunk_loader::{ChunkLoader, ChunkLoaderPlugin};
use crate::debug_world::DebugWorldPlugin;
use crate::world::WorldPlugin;

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
                        title: "🅱️oxel".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            WireframePlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),

            EguiPlugin::default(),
            WorldPlugin,
            ChunkLoaderPlugin,
            DebugWorldPlugin
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })
        //.add_plugins(ChunkRenderPlugin::default())
        .add_plugins(GreedyChunkRenderPlugin::default())
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(PostStartup, setup)
        .run();
}

pub fn setup(mut commands: Commands, mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    primary_cursor_options.grab_mode = CursorGrabMode::None;
    primary_cursor_options.visible = true;

    commands.spawn((Transform::default(), Camera3d::default(), ChunkLoader::new(6), FlyCam));

    commands.spawn((Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y), DirectionalLight {
        illuminance: 2_500.0,
        shadows_enabled: false,
        ..default()
    }));
}
