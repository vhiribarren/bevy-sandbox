#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var x = in.uv.x;
    var y =  in.uv.y;
    return vec4<f32>(x, y, 1.0, 1.0);
}