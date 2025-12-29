mod block;
mod chunk;
mod chunk_loader;
mod chunk_material;
mod chunk_mesh;
mod debug_world;
mod greedy_chunk_render_plugin;
mod lighting;
mod quad;
mod section_neighbors;
mod world;
mod utils;

use crate::chunk_loader::{ChunkLoader, ChunkLoaderPlugin};
use crate::chunk_material::ChunkMaterial;
use crate::debug_world::DebugWorldPlugin;
use crate::lighting::rendering::CustomRenderPlugin;
use crate::world::WorldPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, PostStartup};
use bevy::camera::Camera3d;
use bevy::color::LinearRgba;
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::light::DirectionalLight;
use bevy::light::light_consts::lux::OVERCAST_DAY;
use bevy::math::{Quat, vec3};
use bevy::pbr::MaterialPlugin;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{Msaa, Name};
use bevy::prelude::{Color, Commands, Single, Transform, Window, With, default};
use bevy::render::RenderPlugin;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use std::f32::consts::PI;

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
            //WireframePlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            //EguiPlugin::default(),
            WorldPlugin,
            ChunkLoaderPlugin,
            //DebugWorldPlugin,
            MaterialPlugin::<ChunkMaterial>::default(),
            CustomRenderPlugin,
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(PostStartup, setup)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    primary_cursor_options.grab_mode = CursorGrabMode::None;
    primary_cursor_options.visible = true;

    commands.spawn((
        Transform::default(),
        Camera3d::default(),
        ChunkLoader::new(2),
        FlyCam,
    ));

    commands.spawn((
        Transform {
            translation: vec3(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        DirectionalLight {
            illuminance: OVERCAST_DAY,
            color: Color::LinearRgba(LinearRgba::rgb(243.0 / 255.0, 195.0 / 255.0, 110.0 / 255.0)),
            shadows_enabled: false,
            ..default()
        },
        Name::new("Sun"),
    ));
}
