// ----------------------------------------------------------------------------
// IMPORTANT - LICENSING
//
// This work was done by watching the work of Inigo Quilez, in the following
// place: https://www.youtube.com/watch?v=Cfe5UQ-1L9Q
//
// The shader was written and adapted by me by watching the video tutorial for
// educational purpose. It largely copies line of codes displayed in the video
// and it reproduces a final piece of code implementing a work of art. As such,
// Inigo Quilez is the sole owner of this art work, you are not allowed to use
// it unless the original author granted you his agreement.
//
// Copyright owner: Inigo Quilez Website: https://iquilezles.org/
// ----------------------------------------------------------------------------

//#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput
struct MeshVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

//#import bevy_sprite::mesh2d_view_bindings globals
struct Globals {
    time: f32,
    delta_time: f32,
    frame_count: u32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: f32
#endif
};
@group(0) @binding(1)
var<uniform> globals: Globals;

//#import alea_sandbox_lib::random random


const RAYMARCHING_MAX_STEPS = 200u;
const RAYMARCHING_RADIUS_MIN = 0.001;
const RAYMARCHING_RADIUS_MAX = 20.0;
const SHADOW_BIAS = 0.001;

fn scene(pos: vec3<f32>) -> f32 {
    let sphere_collision = length(pos) - 0.25;
    let floor_collision = pos.y + 0.25;
    return min(sphere_collision, floor_collision);
}

fn collision_normal(pos: vec3<f32>) -> vec3<f32> {
    let e = vec2(0.0001, 0.0);
    return normalize(vec3(scene(pos + e.xyy) - scene(pos - e.xyy), scene(pos + e.yxy) - scene(pos - e.yxy), scene(pos + e.yyx) - scene(pos - e.yyx)));
}

fn cast_ray(origin: vec3<f32>, direction: vec3<f32>) -> f32 {
    var raymarch_dist = 0.0;
    for (var i = 0u; i < RAYMARCHING_MAX_STEPS; i++) {
        let scan_pos = origin + raymarch_dist * direction;
        let collision_distance = scene(scan_pos);
        if collision_distance < RAYMARCHING_RADIUS_MIN {
            break;
        }
        raymarch_dist += collision_distance;
        if raymarch_dist > RAYMARCHING_RADIUS_MAX {
            break;
        }
    }
    if raymarch_dist > RAYMARCHING_RADIUS_MAX {
        return -1.0;
    }
    return raymarch_dist;
}

fn gamma_correction(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3(0.4545));
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let size = in.position.xy / in.uv;
    let ratio = size.x / size.y;
    // Shift coords to be in centered right-hand coordinates
    let centered_uv = in.uv - 0.5;
    let canvas_pos = centered_uv * vec2(ratio, -1.0);
    // Camera parameters
    let cam_rotation_speed: f32 = 0.1 * globals.time;
    let camera_eye = vec3(sin(cam_rotation_speed), 0.0, cos(cam_rotation_speed));
    let camera_target = vec3(0.0, 0.0, 0.0);
    let canvas_distance = 1.5;
    // Transform canvas coordinates
    let camera_axis_z = normalize(camera_target - camera_eye);
    let camera_axis_x = normalize(cross(camera_axis_z, vec3(0.0, 1.0, 0.0)));
    let camera_axis_y = normalize(cross(camera_axis_x, camera_axis_z));
    let camera_transform = mat3x3(camera_axis_x, camera_axis_y, camera_axis_z);
    let direction = normalize(camera_transform * vec3(canvas_pos, canvas_distance));
    // lights
    let sun_dir = normalize(vec3(0.8, 0.4, 0.2));
    let sun_color = vec3(7.0, 4.5, 3.0);
    let sky_dir = normalize(vec3(0.0, 1.0, 0.0));
    let sky_color = vec3(0.0, 0.05, 0.2);
    let bounce_dir = vec3(0.0, -1.0, 0.0);
    let bounce_color = vec3(0.7, 0.3, 0.2);
    let gray_material = vec3(0.18);
    // Raymarching
    let horizon_gray_color = vec3(0.7, 0.75, 0.8);
    var output_col = vec3<f32>(0.4, 0.75, 1.0) - 0.7*direction.y;
    output_col = mix(output_col, horizon_gray_color, exp(-10.0*direction.y));
    let raymarch_dist = cast_ray(camera_eye, direction);
    if raymarch_dist > 0.0 {
        let collision_pos = camera_eye + raymarch_dist * direction;
        let collision_normal = collision_normal(collision_pos);
        let sun_diffusion = clamp(dot(collision_normal, sun_dir), 0.0, 1.0);
        let sun_shadow = step(cast_ray(collision_pos + collision_normal * SHADOW_BIAS, sun_dir), 0.0);
        let sky_diffusion = clamp(0.5 + 0.5 * dot(collision_normal, sky_dir), 0.0, 1.0);
        let ground_bounce_diffusion = clamp(0.5 + 0.5 * dot(collision_normal, bounce_dir), 0.0, 1.0);
        output_col = gray_material * sun_diffusion * sun_color * sun_shadow;
        output_col += gray_material * sky_diffusion * sky_color;
        output_col += gray_material * ground_bounce_diffusion * bounce_color;
    }
    output_col = gamma_correction(output_col);
    return vec4<f32>(output_col, 1.0);
}