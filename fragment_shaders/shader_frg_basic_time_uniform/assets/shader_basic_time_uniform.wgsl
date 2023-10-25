#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct CustomMaterial {
    time: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;


@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var col = 0.5 + 0.5*cos(material.time + in.uv.xyx + vec3(0.0,2.0,4.0));
    return vec4<f32>(col, 1.0);
}