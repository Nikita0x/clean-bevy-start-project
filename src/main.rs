use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;

mod player;
mod camera;
mod movement;
mod scene;

use player::PlayerPlugin;

use camera::spawn_camera;
use camera::update_camera;
use camera::zoom_perspective;

use movement::keyboard_input;

use scene::spawn_basic_scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_basic_scene)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, update_camera)
        .add_systems(Update, zoom_perspective)
        .run(); 
}


