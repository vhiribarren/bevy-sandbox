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
    sprite::Material2d,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use shader_frg_viewer_plugin::ShaderViewerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShaderViewerPlugin::<CustomMaterial>::default())
        .add_plugins(EguiPlugin)
        .add_systems(Update, system_gui)
        .run();
}

fn system_gui(mut contexts: EguiContexts, mut materials: ResMut<Assets<CustomMaterial>>) {
    let ctx = contexts.ctx_mut();
    let (_, material) = materials.iter_mut().next().unwrap();
    egui::Window::new("Noise configuration")
        .resizable(false)
        .show(ctx, |ui| {
            ui.add(egui::DragValue::new(&mut material.frequence));
        });
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "3bb72d82-d404-42e1-b225-2b1debd79518"]
struct CustomMaterial {
    #[uniform(0)]
    frequence: f32,
}

impl Default for CustomMaterial {
    fn default() -> Self {
        Self { frequence: 10.0 }
    }
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "white_noise.wgsl".into()
    }
}
