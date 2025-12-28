#import bevy_pbr::{
    forward_io::FragmentOutput,
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, calculate_view, prepare_world_normal},
    pbr_types::pbr_input_new,
    mesh_functions::{mesh_position_local_to_clip, mesh_normal_local_to_world, get_world_from_local},
    mesh_bindings::mesh,
}

#import bevy_render::view::View

const PI: f32 = 3.14159265359;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) instance_index: u32,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let world_from_local = get_world_from_local(vertex.instance_index);

    out.clip_position = mesh_position_local_to_clip(
        world_from_local,
        vec4<f32>(vertex.position, 1.0),
    );
    out.world_position = world_from_local * vec4<f32>(vertex.position, 1.0);
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.instance_index = vertex.instance_index;

    return out;
}

struct GlobalLighting {
    color_intensity: vec4<f32>,
    ambient_color_intensity: vec4<f32>,
    sun_direction: vec4<f32>,
}

struct MaterialProperties {
    base_color: vec4<f32>,
    emissive_color_intensity: vec4<f32>,
    metallic_roughness_tbd_tbd: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<storage, read> global_lighting: GlobalLighting;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> material: MaterialProperties;
@group(0) @binding(0) var<uniform> view: View;

fn F_Schlick(F0: vec3<f32>, f90: f32, VdotH: f32) -> vec3<f32> {
    return F0 + (vec3<f32>(f90) - F0) * pow(1.0 - VdotH, 5.0);
}

fn G_Smith(alpha: f32, NdotL: f32, NdotV: f32) -> f32 {
    let k = alpha / 2.0;
    let GGXL = NdotL / (NdotL * (1.0 - k) + k);
    let GGXV = NdotV / (NdotV * (1.0 - k) + k);
    return GGXL * GGXV;
}

fn D_GGX(alpha: f32, NdotH: f32) -> f32 {
    let a2 = alpha * alpha;
    let NdotH2 = NdotH * NdotH;
    let denom = (NdotH2 * (a2 - 1.0) + 1.0);
    return a2 / (PI * denom * denom);
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var pbr_input = pbr_input_new();

    pbr_input.flags = mesh[in.instance_index].flags;
    pbr_input.V = calculate_view(in.world_position, false);
    pbr_input.frag_coord = in.clip_position;
    pbr_input.world_position = in.world_position;

    pbr_input.world_normal = prepare_world_normal(
        in.world_normal,
        false,
        false,
    );

    pbr_input.N = normalize(pbr_input.world_normal);

    let base_color = material.base_color;
    let emissive_color = material.emissive_color_intensity.rgb;
    let emissive_intensity = material.emissive_color_intensity.w;
    let metallic = material.metallic_roughness_tbd_tbd.x;
    let roughness = material.metallic_roughness_tbd_tbd.y;

    pbr_input.material.base_color = base_color;
    pbr_input.material.perceptual_roughness = roughness;
    pbr_input.material.metallic = metallic;

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}