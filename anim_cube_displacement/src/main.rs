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

use std::ops::Range;

use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

const DISPLACEMENT_RANGE: Range<f32> = -0.2..0.2;
const X_MAX: u32 = 50;
const Y_MAX: u32 = 50;
const Z_MAX: u32 = 2;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut displacement = || rng.gen_range(DISPLACEMENT_RANGE);
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 0.9 }));
    let material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    for x in 0..X_MAX {
        for y in 0..Y_MAX {
            for z in -(Z_MAX as i32)..0 {
                commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_xyz(
                        displacement() + x as f32,
                        displacement() + y as f32,
                        displacement() + z as f32,
                    ),
                    ..default()
                });
            }
        }
    }
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.08,
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            radius: 1000.0,
            range: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.75 * X_MAX as f32, 0.75 * Y_MAX as f32, 10.),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 50.0).looking_at(
            Vec3::new((X_MAX / 2) as f32, (Y_MAX / 2) as f32, 0.0),
            Vec3::Y,
        ),
        ..default()
    });
}
