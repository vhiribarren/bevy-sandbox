#import shader_view::fragment_input FragmentInput

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var x = in.uv.x;
    var y =  in.uv.y;
    return vec4<f32>(x, y, 1.0, 1.0);
}