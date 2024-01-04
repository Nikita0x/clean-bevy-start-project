use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_basic_scene)
        .add_systems(Update, keyboard_input)
        .run(); 
}

// marker components
#[derive(Component)]
struct Player;



// spawn camera
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        ThirdPersonCamera {
            zoom: Zoom::new(1.5, 60.0), //min and max distance
            ..default()
        },
        Camera3dBundle::default()
    ))
    .insert(Name::new("Camera"));
}

// adding basic scene to look at
fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    // adding Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0, subdivisions: 0}
        )),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));
    // adding Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert((Player, ThirdPersonCameraTarget));
    // adding light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Light"));
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        let move_speed = 5.0;
        let delta_time = 0.016;

        if keys.pressed(KeyCode::W) {
            transform.translation.z -= move_speed * delta_time;
        }

        if keys.pressed(KeyCode::S) {
            transform.translation.z += move_speed * delta_time;
        }

        if keys.pressed(KeyCode::A) {
            transform.translation.x -= move_speed * delta_time;
        }

        if keys.pressed(KeyCode::D) {
            transform.translation.x += move_speed * delta_time;
        }

        // You can add more controls for other directions if needed
    }
}