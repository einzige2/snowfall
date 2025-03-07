use crate::mesh_gen::{TerrainResolution, generate_terrain};
use bevy::prelude::*;

pub mod mesh_gen;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let terrain_size = 32.0;

    let custom_texture_handle: Handle<Image> = asset_server.load("textures/uv_checked.png");
    let terrain_mesh_handle: Handle<Mesh> =
        meshes.add(generate_terrain(terrain_size, TerrainResolution::MEDIUM));

    commands.spawn((
        Mesh3d(terrain_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle),
            ..default()
        })),
        Transform::from_xyz(-(terrain_size / 2.0), 0.0, -(terrain_size / 2.0)),
    ));

    let camera_light_transform =
        Transform::from_xyz(48.0, 48.0, 48.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((Camera3d::default(), camera_light_transform));
    commands.spawn((
        PointLight {
            range: 100.0,
            ..default()
        },
        camera_light_transform,
    ));
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera_transform = query.single_mut();
    let forward = camera_transform.forward();
    let left = camera_transform.left();

    let movement_speed = 7.5;

    // MOVE FORWARD
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        camera_transform.translation += forward * time.delta().as_secs_f32() * movement_speed;
    }

    // MOVE LEFT
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        camera_transform.translation += left * time.delta().as_secs_f32() * movement_speed;
    }

    // MOVE DOWN
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        camera_transform.translation += -forward * time.delta().as_secs_f32() * movement_speed;
    }

    // MOVE RIGHT
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        camera_transform.translation += -left * time.delta().as_secs_f32() * movement_speed;
    }
}
