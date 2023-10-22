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

use std::{hash::Hash, marker::PhantomData};

use bevy::{
    prelude::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

#[derive(Component)]
pub struct Canvas;

#[derive(Default)]
pub struct ShaderViewerPlugin<M: Material2d>(PhantomData<M>);

impl<M: Material2d + Default> Plugin for ShaderViewerPlugin<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(Material2dPlugin::<M>::default())
            .add_systems(Startup, setup_system::<M>)
            .add_systems(Update, on_resize_system);
    }
}

fn setup_system<M: Material2d + Default>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<M>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(M::default()),
            ..default()
        })
        .insert(Canvas);
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
