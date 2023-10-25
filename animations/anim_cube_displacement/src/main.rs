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

use std::{array, ops::Range};

use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use rand::Rng;
use std::f32::consts::PI;

const PERIOD_RANGE: Range<f32> = 1.0..30.0;
const PHASE_RANGE: Range<f32> = 0.0..PI / 2.0;
const AMPLITUDE_RANGE: Range<f32> = 0.8..2.0;
const X_MAX: u32 = 20;
const Y_MAX: u32 = 20;
const Z_MAX: u32 = 20;
const CAM_Z: f32 = 30.0;

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
        .add_systems(Update, move_cubes)
        .add_systems(Update, input_keys)
        .run();
}

#[derive(Component)]
struct Cube;

#[derive(Component)]
struct RootCube;

#[derive(Component)]
struct Displacement {
    orig: Vec3,
    period: Vec3,
    phase: Vec3,
    amplitude: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gen_period = || rand::thread_rng().gen_range(PERIOD_RANGE);
    let gen_phase = || rand::thread_rng().gen_range(PHASE_RANGE);
    let gen_amplitude = || rand::thread_rng().gen_range(AMPLITUDE_RANGE);
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 0.9 }));
    let material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    commands
        .spawn((SpatialBundle::default(), RootCube))
        .with_children(|c| {
            for x in 0..X_MAX {
                for y in 0..Y_MAX {
                    for z in -(Z_MAX as i32)..0 {
                        let (x, y, z) = (x as f32, y as f32, z as f32);
                        let displacement = Displacement {
                            orig: Vec3 { x, y, z },
                            period: Vec3::from_array(array::from_fn(|_| gen_period())),
                            phase: Vec3::from_array(array::from_fn(|_| gen_phase())),
                            amplitude: Vec3::from_array(array::from_fn(|_| gen_amplitude())),
                        };
                        c.spawn((
                            PbrBundle {
                                mesh: mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_xyz(x, y, z),
                                ..default()
                            },
                            Cube,
                            displacement,
                        ));
                    }
                }
            }
        });

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
    let (x_cam, y_cam, z_cam) = ((X_MAX / 2) as f32, (Y_MAX / 2) as f32, (Z_MAX / 2) as f32);
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(x_cam, y_cam, CAM_Z)
            .looking_at(Vec3::new(x_cam, y_cam, z_cam), Vec3::Y),
        ..default()
    });
}

fn move_cubes(
    mut q_transforms: Query<(&mut Transform, &Displacement), With<Cube>>,
    time: Res<Time>,
) {
    for (mut transform, d) in &mut q_transforms {
        let offset = Vec3::from_array(array::from_fn(|idx| {
            d.amplitude[idx]
                * (time.elapsed_seconds() * 2.0 * PI / d.period[idx] + d.phase[idx]).sin()
        }));
        transform.translation = d.orig + offset;
    }
}

fn input_keys(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RootCube>>,
) {
    let cube_center = Vec3::new(
        (X_MAX / 2) as f32,
        (Y_MAX / 2) as f32,
        -(Z_MAX as i32 / 2) as f32,
    );
    for mut transform in &mut query {
        let angle = time.delta_seconds() * 2.0;
        let mut rotation = Quat::default();
        if input.pressed(KeyCode::Up) {
            rotation = rotation.mul_quat(Quat::from_rotation_x(angle));
        }
        if input.pressed(KeyCode::Down) {
            rotation = rotation.mul_quat(Quat::from_rotation_x(-angle));
        }
        if input.pressed(KeyCode::Left) {
            rotation = rotation.mul_quat(Quat::from_rotation_y(angle));
        }
        if input.pressed(KeyCode::Right) {
            rotation = rotation.mul_quat(Quat::from_rotation_y(-angle));
        }
        transform.rotate_around(cube_center, rotation);
    }
}
