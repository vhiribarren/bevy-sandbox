#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

fn draw_cube(top_left: vec2<f32>, width: f32, height: f32, current_pos: vec2<f32>) -> f32 {
    let bottom_right = top_left + vec2(width, height);
    let tl_pixels = step(top_left, current_pos);
    let br_pixels = 1.0 - step(bottom_right, current_pos);
    return tl_pixels.x * tl_pixels.y * br_pixels.x * br_pixels.y;
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var pixel = draw_cube(vec2(0.1, 0.5), 0.7, 0.2, in.uv);
    return vec4(vec3(pixel), 1.0);
} 