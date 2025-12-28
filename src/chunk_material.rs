use bevy::asset::Asset;
use bevy::math::Vec4;
use bevy::pbr::Material;
use bevy::reflect::Reflect;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

#[derive(ShaderType, Clone, Default, Reflect, Debug)]
pub struct MaterialProperties {
    pub base_color: Vec4,
    pub emissive_color_intensity: Vec4,
    pub metallic_roughness_tbd_tbd: Vec4,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct ChunkMaterial {
    #[uniform(0)]
    pub material_properties: MaterialProperties,
}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/testt.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/testt.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/chunk_prepass.wgsl".into()
    }

    fn prepass_fragment_shader() -> ShaderRef {
        "shaders/chunk_prepass.wgsl".into()
    }
}
