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
    asset::ChangeWatcher,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use shader_frg_plugin_lib::ShaderLibPlugin;
use shader_frg_plugin_viewer::ShaderViewerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            ..Default::default()
        }))
        .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        .add_plugins(ShaderLibPlugin)
        .add_plugins(ShaderViewerPlugin::<CustomMaterial>::default())
        .add_systems(Update, input_keys)
        .run();
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "3bb72d82-d404-42e1-b225-2b1debd79518"]
struct CustomMaterial {
    #[uniform(0)]
    offset_horizontal: f32,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "inigo_quilez_happy_jumping.wgsl".into()
    }
}

fn input_keys(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for (_, material) in &mut materials.iter_mut() {
        let mut offset_horizontal = 0.0;
        if input.pressed(KeyCode::Left) {
            offset_horizontal += -time.delta().as_secs_f32();
        }
        if input.pressed(KeyCode::Right) {
            offset_horizontal += time.delta().as_secs_f32();
        }
        material.offset_horizontal += offset_horizontal;
    }
}
