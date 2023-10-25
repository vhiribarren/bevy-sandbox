#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals
#import alea_sandbox_lib::noise average_noise_smoothstep


@group(1) @binding(0)
var<uniform> octaves: u32;
@group(1) @binding(1)
var<uniform> base_frequence: f32;
@group(1) @binding(2)
var<uniform> lacunarity: f32;
@group(1) @binding(3)
var<uniform> gain: f32;


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
        var iter1 = vec2(0.0);
        iter1.x = amp * average_noise_smoothstep(vec2(scaled_u, scaled_v));
        iter1.y = amp * average_noise_smoothstep(vec2(scaled_u, scaled_v));

        noise_val += amp * average_noise_smoothstep(vec2(scaled_u, scaled_v) + 4.0*iter1);
        amp *= gain;
        scaled_u *= lacunarity;
        scaled_v *= lacunarity;
    }
    noise_val /= total_amplitude;
    return vec4<f32>(vec3(noise_val), 1.0);
}