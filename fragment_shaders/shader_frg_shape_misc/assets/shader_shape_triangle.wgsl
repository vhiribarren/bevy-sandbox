#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

fn divide_space(point_1: vec2<f32>, point_2: vec2<f32>, current_pos: vec2<f32>) -> f32 {
    let diff = point_2 - point_1;
    let deriv = diff.y / diff.x;
    let origin = point_1.y - deriv*point_1.x;
    let y = deriv * current_pos.x + origin;
    if point_2.x > point_1.x {
        return 1.0-step(y, current_pos.y);
    }
    else {
        return step(y, current_pos.y);
    }
}

fn draw_triangle(point_1: vec2<f32>, point_2: vec2<f32>, point_3: vec2<f32>, current_pos: vec2<f32>) -> f32 {
    var pixel = divide_space(point_1, point_2, current_pos);
    pixel *= divide_space(point_2, point_3, current_pos);
    pixel *= divide_space(point_3, point_1, current_pos);
    return pixel;
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var pixel = draw_triangle(vec2(0.3, 0.8), vec2(0.7, 0.8), vec2(0.5, 0.2), in.uv);
    return vec4(vec3(pixel), 1.0);
} 