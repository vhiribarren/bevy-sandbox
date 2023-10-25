#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

@group(1) @binding(0)
var<uniform> octaves: u32;
@group(1) @binding(1)
var<uniform> base_frequence: f32;
@group(1) @binding(2)
var<uniform> lacunarity: f32;
@group(1) @binding(3)
var<uniform> gain: f32;

fn random (uv: vec2<f32>) -> f32 {
    return fract(sin(dot(uv.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

fn average_noise_mix(scaled_uv: vec2<f32>) -> f32 {
    let rand_tl = random(floor(scaled_uv));
    let rand_tr = random(floor(scaled_uv + vec2(1.0, 0.0)));
    let rand_bl = random(floor(scaled_uv + vec2(0.0, 1.0)));
    let rand_br = random(ceil(scaled_uv));
    let top_avg = mix(rand_tl, rand_tr, fract(scaled_uv.x));
    let bottom_avg = mix(rand_bl, rand_br, fract(scaled_uv.x));
    return mix(top_avg, bottom_avg, fract(scaled_uv.y));
}

fn average_noise_smoothstep(scaled_uv: vec2<f32>) -> f32 {
    let percent = smoothstep(vec2(0.), vec2(1.), fract(scaled_uv));
    let rand_tl = random(floor(scaled_uv));
    let rand_tr = random(floor(scaled_uv + vec2(1.0, 0.0)));
    let rand_bl = random(floor(scaled_uv + vec2(0.0, 1.0)));
    let rand_br = random(ceil(scaled_uv));
    let top_avg = mix(rand_tl, rand_tr, percent.x);
    let bottom_avg = mix(rand_bl, rand_br, percent.x);
    return mix(top_avg, bottom_avg, percent.y);
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let size = in.position.xy/in.uv;
    let ratio = size.x/size.y;
    var scaled_u = in.uv.x * base_frequence * ratio;
    var scaled_v = in.uv.y * base_frequence;
    var noise_val = 0.0;
    var freq = base_frequence;
    var amp = 1.0;
    var total_amplitude = 0.0;
    for (var i = u32(0); i < octaves; i++) {
        total_amplitude += amp;
        noise_val += amp * average_noise_smoothstep(vec2(scaled_u, scaled_v));
        amp *= gain;
        scaled_u *= lacunarity;
        scaled_v *= lacunarity;
    }
    noise_val /= total_amplitude;
    return vec4<f32>(vec3(noise_val), 1.0);
}