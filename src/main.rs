#![allow(dead_code)]
#![deny(unreachable_patterns, unused_must_use, unsafe_code)]

mod block;
mod constants;
mod rendering;
pub mod utils;
mod world;

use crate::rendering::rendering::CustomRenderPlugin;
use crate::world::chunks::chunk_loader::{ChunkLoader, ChunkLoaderPlugin};
use crate::world::chunks::chunk_material::ChunkMaterial;
use crate::world::world::WorldPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, PostStartup};
use bevy::camera::Camera3d;
use bevy::color::LinearRgba;
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::{
    FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::light::DirectionalLight;
use bevy::light::light_consts::lux::OVERCAST_DAY;
use bevy::math::{Quat, vec3};
use bevy::pbr::MaterialPlugin;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::Name;
use bevy::prelude::{Color, Commands, Single, Transform, Window, With, default};
use bevy::render::RenderPlugin;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowPlugin};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use std::f32::consts::PI;
use crate::rendering::skybox_material::SkyboxMaterial;
use crate::rendering::skybox_plugin::{Skybox, SkyboxPlugin};

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
            SystemInformationDiagnosticsPlugin::default(),
            //EguiPlugin::default(),
            WorldPlugin,
            ChunkLoaderPlugin,
            //DebugWorldPlugin,
            MaterialPlugin::<ChunkMaterial>::default(),
            MaterialPlugin::<SkyboxMaterial>::default(),
            CustomRenderPlugin,
            SkyboxPlugin,
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        })
        .insert_resource(MovementSettings {
            sensitivity: 0.00006,
            speed: 128.0,
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
        Transform::from_xyz(0.0, 48.0, 0.0),
        Camera3d::default(),
        ChunkLoader::new(96),
        FlyCam,
        Skybox,
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
