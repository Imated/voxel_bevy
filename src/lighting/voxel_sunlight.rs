use bevy::color::Color;
use bevy::math::Vec4;
use bevy::prelude::ReflectComponent;
use bevy::prelude::ReflectDefault;
use bevy::prelude::{Component, Reflect};
use bevy::render::render_resource::ShaderType;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, Debug, Clone, Copy, Reflect, InspectorOptions)]
#[reflect(Component, Default, Debug, Clone)]
pub struct VoxelSunlight {
    pub color: Color,
    pub illuminance: f32,
}

impl Default for VoxelSunlight {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            illuminance: 0.25,
        }
    }
}

#[repr(C)]
#[derive(ShaderType, Clone, Default)]
pub struct GlobalLightingUniform {
    pub color_intensity: Vec4,
    pub ambient_color_intensity: Vec4,
    pub sun_direction: Vec4, // w = padding
}
