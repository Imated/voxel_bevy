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

use std::f32::consts::PI;
use crate::chunk_loader::{ChunkLoader, ChunkLoaderPlugin};
use crate::chunk_material::ChunkMaterial;
use crate::debug_world::DebugWorldPlugin;
use crate::lighting::rendering::CustomRenderPlugin;
use crate::lighting::voxel_sunlight::VoxelSunlight;
use crate::world::WorldPlugin;
use bevy::app::{App, PluginGroup, PostStartup};
use bevy::camera::Camera3d;
use bevy::color::LinearRgba;
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{vec3, EulerRot, Quat, Vec3};
use bevy::pbr::wireframe::WireframeConfig;
use bevy::pbr::MaterialPlugin;
use bevy::prelude::{default, Color, Commands, Single, Transform, Window, With};
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowPlugin};
use bevy::DefaultPlugins;
use bevy::light::DirectionalLight;
use bevy::light::light_consts::lux::{FULL_DAYLIGHT, OVERCAST_DAY};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin};
use bevy::prelude::Name;

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
            EguiPlugin::default(),
            WorldPlugin,
            ChunkLoaderPlugin,
            DebugWorldPlugin,
            MaterialPlugin::<ChunkMaterial>::default(),
            CustomRenderPlugin
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })
        .register_type::<VoxelSunlight>()
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
        ChunkLoader::new(6),
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
            shadows_enabled: true,
            ..default()
        },
        Name::new("Sun")
    ));
}
