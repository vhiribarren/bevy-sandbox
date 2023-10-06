#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

struct CustomMaterial {
    frequence: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

fn random (uv: vec2<f32>) -> f32 {
    return fract(sin(dot(uv.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3(random(floor(material.frequence*in.uv))), 1.0);
}