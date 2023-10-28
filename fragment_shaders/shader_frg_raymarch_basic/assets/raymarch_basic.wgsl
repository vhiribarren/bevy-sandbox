#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals
#import alea_sandbox_lib::random random


fn sdf_sphere(pos: vec3<f32>, radius: f32) -> f32 {
    return length(pos) - radius;
}

fn sdf_plane(pos: vec3<f32>) -> f32 {
    return pos.y;
}

fn translate_coords(pos: vec3<f32>, translation: vec3<f32>) -> vec3<f32> {
    return pos - translation;
}

fn uv_to_righthanded_coords() {

}

const SCAN_MAX_RADIUS: f32 = 20.0;
const SCAN_MIN_RADIUS: f32 = 0.001;
const SCAN_MAX_STEPS: u32 = 200u;

fn scene(pos: vec3<f32>) -> f32 {
    var dist = SCAN_MAX_RADIUS;
    dist = min(dist, sdf_sphere(translate_coords(pos, vec3(0., 0., -5.0)), 0.5));
    dist = min(dist, sdf_plane(translate_coords(pos, vec3(0., -0.5, 0.0))));
    return dist;
}

fn raymarch(origin: vec3<f32>, direction: vec3<f32>) -> f32 {
    var total_dist = 0.0;
    var current_pos = origin;
    var norm_dir = normalize(direction);
    for (var i = 0u; i < SCAN_MAX_STEPS; i++) {
        let current_sdf = scene(current_pos);
        if current_sdf > SCAN_MAX_RADIUS {
            return SCAN_MAX_RADIUS;
        } else if current_sdf > SCAN_MIN_RADIUS {
            current_pos += current_sdf * norm_dir;
            total_dist += current_sdf;
        } else {
            return total_dist;
        }
    }
    return total_dist;
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let size = in.position.xy / in.uv;
    let ratio = size.x / size.y;
    // Shift coords to be in centered right-hand coordinates
    let centered_uv = in.uv - 0.5;
    let pos = centered_uv * vec2(ratio, -1.0);
    let camera = vec3(0.0, 0.0, 5.0);
    // Compute ray
    let origin = vec3(pos, 0.0);
    let direction = origin - camera;
    // Ray marching
    var val = 1.0-raymarch(origin, direction)/SCAN_MAX_RADIUS;
    return vec4<f32>(vec3(val), 1.0);
}