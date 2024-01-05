use bevy::prelude::*;
use crate::player::Player;

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
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

        // You can add more controls for other directions if needed
    }
}