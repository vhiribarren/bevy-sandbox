/*
MIT License

Copyright (c) 2023 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::time::Duration;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
    time::common_conditions::on_timer,
};
use shader_frg_plugin_viewer::ShaderViewerPlugin;

const DYNAMIC_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845128402094821);

const SHADER_RED: &str = r#"
#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

const SHADER_BLUE: &str = r#"
#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 1.0, 1.0);
}
"#;

#[derive(Resource)]
struct ShaderState(bool);

fn main() {
    App::new()
        .insert_resource(ShaderState(true))
        .add_plugins(DefaultPlugins)
        .add_plugins(ShaderViewerPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup_dynamic_handler)
        .add_systems(
            Update,
            switch_shader.run_if(on_timer(Duration::from_secs(2))),
        )
        .run();
}

fn setup_dynamic_handler(mut shaders: ResMut<Assets<Shader>>) {
    shaders.set_untracked(
        DYNAMIC_SHADER_HANDLE,
        Shader::from_wgsl(SHADER_BLUE, file!()),
    );
}

fn switch_shader(mut shaders: ResMut<Assets<Shader>>, mut shader_state: ResMut<ShaderState>) {
    let new_shader = match shader_state.0 {
        false => SHADER_BLUE,
        true => SHADER_RED,
    };
    shaders.set_untracked(
        DYNAMIC_SHADER_HANDLE,
        Shader::from_wgsl(new_shader, file!()),
    );
    shader_state.0 = !shader_state.0;
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "3bb72d82-d404-42e1-b225-2b1debd79518"]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        DYNAMIC_SHADER_HANDLE.typed().into()
    }
}
