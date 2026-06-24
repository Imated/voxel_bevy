#![allow(dead_code)]
#![deny(unreachable_patterns, unused_must_use, unsafe_code)]

pub mod rendering;
pub mod world;

use crate::world::chunk::chunk_loader::{ChunkLoader, ChunkLoaderPlugin};
use crate::world::world::{World, WorldPlugin};
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, PostStartup};
use bevy::camera::Camera3d;
use bevy::color::Color;
use bevy::color::palettes::css::{GREEN, ORANGE_RED};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::diagnostic::{
    FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::ecs::bundle::Bundles;
use bevy::light::light_consts::lux::OVERCAST_DAY;
use bevy::light::{CascadeShadowConfigBuilder, DirectionalLight, GlobalAmbientLight};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Commands, Single, TextFont, Transform, Window, With, default};
use bevy::window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowPlugin};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use std::f32::consts::PI;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "🅱️oxel".to_string(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin::default(),
            WorldPlugin,
            ChunkLoaderPlugin,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont::default(),
                    text_color: Color::Srgba(GREEN),
                    enabled: true,
                    refresh_interval: Duration::from_millis(100),
                    frame_time_graph_config: FrameTimeGraphConfig::target_fps(244.0),
                },
            },
        ))
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
        ChunkLoader::new(32),
        FlyCam,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder::default().build(),
    ));

    commands.insert_resource(GlobalAmbientLight {
        color: ORANGE_RED.into(),
        brightness: 200.0,
        ..default()
    });
}
