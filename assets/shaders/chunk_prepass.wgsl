#import bevy_pbr::{
    prepass_bindings,
    mesh_functions,
    prepass_io::{FragmentOutput},
    skinning,
    morph,
    mesh_view_bindings::{view, previous_view_proj},
}

#import bevy_pbr::mesh_functions::{mesh_normal_local_to_world, get_world_from_local, mesh_position_local_to_clip}
#import bevy_render::instance_index::{get_instance_index}

/// -----------------VERTEX------------------

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) vertex_data: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
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
fn prepass_vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(vertex.vertex_data & x_positive_bits(5u));
    let y = f32(vertex.vertex_data >> 5u & x_positive_bits(5u));
    let z = f32(vertex.vertex_data >> 10u & x_positive_bits(5u));
    let normal_index = vertex.vertex_data >> 15u & x_positive_bits(3u);
    let block_index = vertex.vertex_data >> 18u & x_positive_bits(10u);

    let local_position = vec4<f32>(x, y, z, 1.0);
    let model = get_world_from_local(vertex.instance_index);
    let normal = normals[normal_index];

    out.clip_position = mesh_position_local_to_clip(model, local_position);
    out.world_normal = mesh_normal_local_to_world(normal, vertex.instance_index);

    return out;
}

/// -----------------FRAGMENT------------------
#ifdef PREPASS_FRAGMENT
@fragment
fn fragment(in: VertexOutput) -> prepass_io::FragmentOutput {
    var out: FragmentOutput;

    out.frag_depth = in.clip_position.z;
#ifdef NORMAL_PREPASS
    out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
#endif

    return out;
}
#endif