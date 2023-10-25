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
use shader_frg_plugin_lib::ShaderLibPlugin;
use shader_frg_plugin_viewer::ShaderViewerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShaderViewerPlugin::<CustomMaterial>::default())
        .add_plugins(ShaderLibPlugin)
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
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.label("Octaves");
                ui.add(egui::DragValue::new(&mut material.octaves));
                ui.end_row();
                ui.label("Initial frequence");
                ui.add(
                    egui::Slider::new(&mut material.base_frequence, 0.001..=1000.0)
                        .logarithmic(true),
                );
                ui.end_row();
                ui.label("Frequence increase factor");
                ui.add(egui::Slider::new(&mut material.lacunarity, 1.0..=1000.0).logarithmic(true));
                ui.end_row();
                ui.label("Amplitude decrease factor");
                ui.add(egui::Slider::new(&mut material.gain, 0.001..=1.0).logarithmic(true));
                ui.end_row();
            });
        });
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "3bb72d82-d404-42e1-b225-2b1debd79518"]
struct CustomMaterial {
    #[uniform(0)]
    octaves: u32,
    #[uniform(1)]
    base_frequence: f32,
    #[uniform(2)]
    lacunarity: f32,
    #[uniform(3)]
    gain: f32,
}

impl Default for CustomMaterial {
    fn default() -> Self {
        Self {
            octaves: 1,
            base_frequence: 1.0,
            lacunarity: 2.0,
            gain: 0.5,
        }
    }
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "value_noise_warp.wgsl".into()
    }
}
