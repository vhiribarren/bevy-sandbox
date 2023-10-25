#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var col = 0.5 + 0.5*cos(globals.time + in.uv.xyx + vec3(0.0,2.0,4.0));
    return vec4<f32>(col, 1.0);
}