use bevy::prelude::*;
use bevy::input::mouse::*;

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
    keys: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = query_camera.single_mut().into_inner() else {
        return;
    };

    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 && persp.fov < 2.0 {
                    persp.fov += 0.08;
                } else if ev.y < 0.0 && persp.fov > 0.5 {
                    persp.fov -= 0.08;
                }
                println!("Scroll (line units): vertical: {}", ev.y);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}", ev.y);
            }
        }
    };
}