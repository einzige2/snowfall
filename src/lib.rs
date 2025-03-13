use crate::mesh_gen::{TerrainResolution, generate_terrain};
use bevy::{
    app::AppExit,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

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
    let terrain_size = 256.0;

    let custom_texture_handle: Handle<Image> = asset_server.load("textures/uv_checked.png");
    let terrain_mesh_handle: Handle<Mesh> =
        meshes.add(generate_terrain(terrain_size, TerrainResolution::LOW));

    commands.spawn((
        Mesh3d(terrain_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle),
            ..default()
        })),
        Transform::from_xyz(-(terrain_size / 2.0), 0.0, -(terrain_size / 2.0)),
    ));

    let camera_light_transform =
        Transform::from_xyz(32.0, 32.0, 32.0).looking_at(Vec3::ZERO, Vec3::Y);

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
    // input
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    // queries
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    // events
    mut evr_mouse_motion: EventReader<MouseMotion>,
    mut exit: EventWriter<AppExit>,
    // resources
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut primary_window = window_query.single_mut();

    let forward = camera_transform.forward();
    let left = camera_transform.left();

    let movement_speed = 7.5;

    // EXIT (we use just_pressed because we only neeed to send this once)
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }

    // HIDE MOUSE
    if mouse_input.just_pressed(MouseButton::Left) {
        if primary_window.cursor_options.grab_mode != CursorGrabMode::Locked {
            primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor_options.visible = false;
        }
    }

    // SHOW MOUSE
    if mouse_input.just_pressed(MouseButton::Right) {
        if primary_window.cursor_options.grab_mode == CursorGrabMode::Locked {
            primary_window.cursor_options.grab_mode = CursorGrabMode::None;
            primary_window.cursor_options.visible = true;
        }
    }

    let dt = time.delta().as_secs_f32();

    // ROTATE CAMERA
    let mut mouse_move_x: f32 = 0.0;
    let mut mouse_move_y: f32 = 0.0;
    for ev in evr_mouse_motion.read() {
        mouse_move_x += ev.delta.x;
        mouse_move_y += ev.delta.y;
    }

    let (yaw, pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + mouse_move_x * dt;
    let pitch = pitch + mouse_move_y * dt;

    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // MOVE FORWARD
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        camera_transform.translation += forward * dt * movement_speed;
    }

    // MOVE LEFT
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        camera_transform.translation += left * dt * movement_speed;
    }

    // MOVE DOWN
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        camera_transform.translation += -forward * dt * movement_speed;
    }

    // MOVE RIGHT
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        camera_transform.translation += -left * dt * movement_speed;
    }
}
