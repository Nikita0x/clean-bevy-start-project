use bevy::prelude::*;
use bevy_third_person_camera::*;

// spawn camera
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        ThirdPersonCamera {
            zoom: Zoom::new(1.5, 60.0), //min and max distance
            ..default()
        },
        Camera3dBundle::default()
    ))
    .insert(Name::new("Camera"));
}