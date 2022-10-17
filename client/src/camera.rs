use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::player::LocalPlayer;

pub struct KeenwatchCameraPlugin;

impl Plugin for KeenwatchCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(handle_input);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(-0.5)),
        ..default()
    });
}

fn handle_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>,
) {
    // always follow the player
    let player_transform = player.single();
    let mut transform = camera_query.single_mut();
    transform.translation = player_transform.translation + Vec3::new(0.0, 50.0, 50.0);
    transform.look_at(player_transform.translation, Vec3::Y);

    // there are two additional possible inputs to control the camera
    // 1. scroll wheel to zoom in and out
    // 2. wasd to rotate the camera around the player
}
