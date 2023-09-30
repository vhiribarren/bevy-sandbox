use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup_system)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, update_shader)
        .run();
}

#[derive(Component)]
struct Canvas;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(CustomMaterial::default()),
            ..default()
        })
        .insert(Canvas);
}

fn update_shader(time: Res<Time>, mut materials: ResMut<Assets<CustomMaterial>>) {
    materials.iter_mut().for_each(|material| {
        material.1.time = time.elapsed_seconds();
    });
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

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "3bb72d82-d404-42e1-b225-2b1debd79518"]
struct CustomMaterial {
    #[uniform(0)]
    time: f32,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader_basic_time.wgsl".into()
    }
}
