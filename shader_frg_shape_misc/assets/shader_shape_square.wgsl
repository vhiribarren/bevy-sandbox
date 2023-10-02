#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

const DRAW_PRECISION: f32 = 0.005;
const CURVE_COLOR =  vec4<f32>(1.0, 0.0, 0.0, 1.0);
const BACKGROUND_COLOR = vec4<f32>(0.5, 0.5, 0.8, 1.0);

fn draw_curve(y_loc: f32, value: f32, prec: f32) -> f32 {
    return smoothstep(y_loc - prec, y_loc, value) - smoothstep(y_loc, y_loc + prec, value);
}

fn curve_fn(x: f32) -> f32 {
    return 0.5 * (1.0 + cos(10.0*x));
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let x = in.uv.x + globals.time/10.;
    let y = curve_fn(x);
    let value = draw_curve(in.uv.y, y, DRAW_PRECISION);
    let point_color = (1. - value) * BACKGROUND_COLOR + value * CURVE_COLOR;
    return CURVE_COLOR;
} 