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

const SCAN_MAX_RADIUS: f32 = 20.0;
const SCAN_MIN_RADIUS: f32 = 0.001;
const SCAN_MAX_STEPS: u32 = 200u;
const EPSILON: f32 = 0.001;

const MATERIAL_SKY: u32 = 0u;
const MATERIAL_RED: u32 = 1u;
const MATERIAL_BLUE: u32 = 2u;

fn materials(idx: u32) -> vec3<f32> {
    switch idx {
        case 0u: {
            return vec3(0.2);
        }
        case 1u: {
            return vec3(1., 0., 0.);
        }
        case 2u: {
            return vec3(0., 0., 1.);
        }
        default: {
            return vec3(1.);
        }
    }
}

struct CollisionInfo {
    distance: f32,
    material_idx: u32,
}

fn min_collision(col_left: CollisionInfo, col_right: CollisionInfo) -> CollisionInfo {
    if col_left.distance < col_right.distance {
        return col_left;
    }
    return col_right;
}

fn scene_plane(pos: vec3<f32>) -> CollisionInfo {
    return CollisionInfo(sdf_plane(translate_coords(pos, vec3(0., -0.5, 0.0))), MATERIAL_RED);
}

fn scene_sphere(pos: vec3<f32>) -> CollisionInfo {
    return CollisionInfo(sdf_sphere(translate_coords(pos, vec3(0., 0., -5.0)), 0.5), MATERIAL_BLUE);
}

fn scene(pos: vec3<f32>) -> CollisionInfo {
    let col_plane = scene_plane(pos);
    let col_sphere = scene_sphere(pos);
    return min_collision(col_plane, col_sphere);
}

fn collision_normal(pos: vec3<f32>) -> vec3<f32> {
    let d = scene(pos).distance;
    let normal = vec3(
        scene(pos + vec3(EPSILON, 0., 0.)).distance - d,
        scene(pos + vec3(0., EPSILON, 0.)).distance - d,
        scene(pos + vec3(0., 0., EPSILON)).distance - d,
    );
    return normalize(normal);
}

fn raymarch(origin: vec3<f32>, norm_dir: vec3<f32>) -> CollisionInfo {
    var total_dist = 0.0;
    var current_pos = origin;
    var last_material_idx = 0u;
    for (var i = 0u; i < SCAN_MAX_STEPS; i++) {
        let collision_info = scene(current_pos);
        let current_sdf = collision_info.distance;
        last_material_idx = collision_info.material_idx;
        if current_sdf > SCAN_MAX_RADIUS {
            return CollisionInfo(SCAN_MAX_RADIUS, MATERIAL_SKY);
        } else if current_sdf > SCAN_MIN_RADIUS {
            current_pos += current_sdf * norm_dir;
            total_dist += current_sdf;
        } else {
            break;
        }
    }
    return CollisionInfo(total_dist, last_material_idx);
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
    let canvas_point = vec3(pos, 0.0);
    let direction = normalize(canvas_point - camera);
    // Ray marching
    let collision_info = raymarch(camera, direction);
    var val = vec3(0.0);
    if collision_info.distance < SCAN_MAX_RADIUS {
        let collision_point = camera + collision_info.distance * direction;
        let collision_normal = collision_normal(collision_point);
        val = collision_normal;
    }
    //let val = materials(collision_info.material_idx);
    return vec4<f32>(vec3(val), 1.0);
}