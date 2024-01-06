use bevy::prelude::*;

// spawn camera
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        // ThirdPersonCamera {
        //     zoom: Zoom::new(1.5, 60.0), //min and max distance
        //     ..default()
        // },
        Camera3dBundle {
            transform: Transform { translation: (Vec3::new(0.0, 3.3, 3.9)), rotation: (Quat::from_rotation_x(-0.5)), scale: (Vec3::new(1.0, 1.0, 1.0)) },
            ..default()
        },
        
    ))
    .insert(Name::new("Camera"));
}

pub fn update_camera(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {

    for mut transform in query.iter_mut() {
        let move_speed = 8.0;

        if keys.pressed(KeyCode::W) {
            transform.translation.z -= move_speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::S) {
            transform.translation.z += move_speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::A) {
            transform.translation.x -= move_speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::D) {
            transform.translation.x += move_speed * time.delta_seconds();
        }

    }
}

pub fn zoom_perspective(
    mut query_camera: Query<&mut Projection, With<Camera>>,
    keys: Res<Input<KeyCode>>
) {
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = query_camera.single_mut().into_inner() else {
        return;
    };

    if keys.pressed(KeyCode::Q) {
        persp.fov += 0.01;
    }
    if keys.pressed(KeyCode::E) {
        persp.fov -= 0.01;
    }
    
}