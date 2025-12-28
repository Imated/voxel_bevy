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
    @location(0) vertex_data: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) instance_index: u32,
}

var<private> normals: array<vec3<f32>,6> = array<vec3<f32>,6> (
	vec3<f32>(-1.0, 0.0, 0.0), // Left
	vec3<f32>(1.0, 0.0, 0.0), // Right
	vec3<f32>(0.0, -1.0, 0.0), // Down
	vec3<f32>(0.0, 1.0, 0.0), // Up
	vec3<f32>(0.0, 0.0, -1.0), // Forward
	vec3<f32>(0.0, 0.0, 1.0) // Back
);

fn x_positive_bits(bits: u32) -> u32{
    return (1u << bits) - 1u;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(vertex.vertex_data & x_positive_bits(5u));
    let y = f32(vertex.vertex_data >> 5u & x_positive_bits(5u));
    let z = f32(vertex.vertex_data >> 10u & x_positive_bits(5u));
    let normal_index = vertex.vertex_data >> 15u & x_positive_bits(3u);
    let block_index = vertex.vertex_data >> 18u & x_positive_bits(10u);

    let local_position = vec4<f32>(x, y, z, 1.0);
    let model = get_world_from_local(vertex.instance_index);
    let world_position = model * local_position;
    let normal = normals[normal_index];

    out.clip_position = mesh_position_local_to_clip(model, local_position);
    out.world_position = world_position;
    out.world_normal = mesh_normal_local_to_world(normal, vertex.instance_index);
    out.instance_index = vertex.instance_index;

    return out;
}

struct MaterialProperties {
    base_color: vec4<f32>,
    emissive_color_intensity: vec4<f32>,
    metallic_roughness_tbd_tbd: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: MaterialProperties;
@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    //var pbr_input = pbr_input_new();

    //pbr_input.flags = mesh[in.instance_index].flags;
    //pbr_input.V = calculate_view(in.world_position, false);
    //pbr_input.frag_coord = in.clip_position;
    //pbr_input.world_position = in.world_position;

    //pbr_input.world_normal = prepare_world_normal(
    //    in.world_normal,
    //    false,
    //    false,
    //);

    //pbr_input.N = normalize(pbr_input.world_normal);

    let base_color = material.base_color.rgb;
    let emissive_color = material.emissive_color_intensity.rgb;
    let emissive_intensity = material.emissive_color_intensity.w;
    let metallic = material.metallic_roughness_tbd_tbd.x;
    let roughness = material.metallic_roughness_tbd_tbd.y;

    //pbr_input.material.base_color = base_color;
    //pbr_input.material.perceptual_roughness = roughness;
    //pbr_input.material.metallic = metallic;

    var out: FragmentOutput;
    //out.color = apply_pbr_lighting(pbr_input);
    //out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    let N = normalize(in.world_normal);
    let V = normalize(view.world_position - in.world_position.xyz);
    let L = normalize(vec3<f32>(0.5, 0.8, 0.3));
    let diffuse = max(dot(N, L), 0.0);
    let H = normalize(L + V);
    let spec = pow(max(dot(N, H), 0.0), 32.0) * 0.2;
    let ambient = 0.1;
    let lighting = ambient + diffuse + spec;


    out.color = vec4<f32>(base_color * lighting, 1.0);

    return out;
}