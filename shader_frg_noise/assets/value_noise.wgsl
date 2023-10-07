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

fn average_noise_mix(uv: vec2<f32>) -> f32 {
    let scaled_uv = material.frequence * uv;
    let rand_tl = random(floor(scaled_uv));
    let rand_tr = random(floor(scaled_uv + vec2(1.0, 0.0)));
    let rand_bl = random(floor(scaled_uv + vec2(0.0, 1.0)));
    let rand_br = random(ceil(scaled_uv));
    let top_avg = mix(rand_tl, rand_tr, fract(scaled_uv.x));
    let bottom_avg = mix(rand_bl, rand_br, fract(scaled_uv.x));
    return mix(top_avg, bottom_avg, fract(scaled_uv.y));
}

fn average_noise_smoothstep(uv: vec2<f32>) -> f32 {
    let scaled_uv = material.frequence * uv;
    let f = smoothstep(vec2(0.), vec2(1.), fract(scaled_uv));
    let rand_tl = random(floor(scaled_uv));
    let rand_tr = random(floor(scaled_uv + vec2(1.0, 0.0)));
    let rand_bl = random(floor(scaled_uv + vec2(0.0, 1.0)));
    let rand_br = random(ceil(scaled_uv));
    let top_avg = mix(rand_tl, rand_tr, f.x);
    let bottom_avg = mix(rand_bl, rand_br, f.x);
    return mix(top_avg, bottom_avg, f.y);
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3(average_noise_smoothstep(in.uv)), 1.0);
}