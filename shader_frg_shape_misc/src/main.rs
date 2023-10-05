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

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::fs;

const SHADER_CHOICE: &[ShaderInput] = &[
    ShaderInput {
        name: "Triangle",
        file: "shader_shape_triangle.wgsl",
    },
    ShaderInput {
        name: "Square",
        file: "shader_shape_square.wgsl",
    },
    ShaderInput {
        name: "Animated cosinus",
        file: "shader_shape_cos_anim.wgsl",
    },
];

fn main() {
    App::new()
        .insert_resource(ShaderChoice(&SHADER_CHOICE[0]))
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, system_gui)
        .run();
}

const DYNAMIC_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13828845128402094821);

#[derive(PartialEq)]
struct ShaderInput {
    name: &'static str,
    file: &'static str,
}

#[derive(Resource)]
struct ShaderChoice(&'static ShaderInput);

#[derive(Component)]
struct Canvas;

fn setup_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    shaders: ResMut<Assets<Shader>>,
    shader_choice: ResMut<ShaderChoice>,
) {
    contexts
        .ctx_mut()
        .set_visuals(egui::style::Visuals::light());
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(CustomMaterial {}),
            ..default()
        })
        .insert(Canvas);
    load_shader(&shader_choice, shaders)
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

fn system_gui(
    mut contexts: EguiContexts,
    mut shader_choice: ResMut<ShaderChoice>,
    shaders: ResMut<Assets<Shader>>,
) {
    let ctx = contexts.ctx_mut();
    let shader_choice_init = shader_choice.0;
    egui::Window::new("Shader shapes")
        .resizable(false)
        .show(ctx, |ui| {
            egui::ComboBox::from_label("Select shape")
                .selected_text(shader_choice.0.name)
                .show_ui(ui, |ui| {
                    for choice in SHADER_CHOICE {
                        ui.selectable_value(&mut shader_choice.0, choice, choice.name);
                    }
                });
        });
    if shader_choice_init != shader_choice.0 {
        load_shader(&shader_choice, shaders);
    }
}

fn load_shader(shader_choice: &ShaderChoice, mut shaders: ResMut<Assets<Shader>>) {
    let shader_path = std::path::Path::new(file!())
        .ancestors()
        .nth(2)
        .unwrap()
        .join("assets")
        .join(shader_choice.0.file);
    info!("Loading {shader_path:?}");
    shaders.set_untracked(
        DYNAMIC_SHADER_HANDLE,
        Shader::from_wgsl(
            fs::read_to_string(shader_path).unwrap(),
            shader_choice.0.file,
        ),
    );
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "3bb72d82-d404-42e1-b225-2b1debd79518"]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        DYNAMIC_SHADER_HANDLE.typed().into()
    }
}
