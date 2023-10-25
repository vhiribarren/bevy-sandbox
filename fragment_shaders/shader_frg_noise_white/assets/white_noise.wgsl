#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals
#import alea_sandbox_lib::random random

struct CustomMaterial {
    frequence: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3(random(floor(material.frequence*in.uv))), 1.0);
}