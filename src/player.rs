use bevy::prelude::*;
use bevy_third_person_camera::*;


#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    model: SceneBundle,
    id: Player,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        model: SceneBundle {
            scene: asset_server.load("Player.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 1.1, 0.0),
            ..default()
        },
        id: Player,
    })
    .insert((Player, ThirdPersonCameraTarget));
}