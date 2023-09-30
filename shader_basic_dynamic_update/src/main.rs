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
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    time::common_conditions::on_timer,
    window::WindowResized,
};

const DYNAMIC_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845128402094821);

const SHADER_RED: &str = r#"
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

const SHADER_BLUE: &str = r#"
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
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
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup_system)
        .add_systems(Update, on_resize_system)
        .add_systems(
            Update,
            switch_shader.run_if(on_timer(Duration::from_secs(2))),
        )
        .run();
}

#[derive(Component)]
struct Canvas;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    shaders.set_untracked(
        DYNAMIC_SHADER_HANDLE,
        Shader::from_wgsl(SHADER_BLUE, file!()),
    );
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(CustomMaterial {}),
            ..default()
        })
        .insert(Canvas);
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

fn on_resize_system(
    mut q_transform: Query<&mut Transform, With<Canvas>>,
    mut e_resize: EventReader<WindowResized>,
) {
    if e_resize.is_empty() {
        return;
    }
    let size = e_resize.iter().next().unwrap();
    let mut transform = q_transform.single_mut();
    *transform = Transform::default().with_scale(Vec3::new(size.width, size.height, 1.0));
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "3bb72d82-d404-42e1-b225-2b1debd79518"]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        DYNAMIC_SHADER_HANDLE.typed().into()
    }
}
