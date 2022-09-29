use bevy::prelude::*;

use crate::player::Player;

pub(crate) fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 50.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub(crate) fn handle_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    let player_transform = player.single();
    let mut camera_t = camera_query.single_mut();
    camera_t.translation = player_transform.translation.clone() + Vec3::new(0.0, 50.0, 40.0);

    let mut camera_direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        camera_direction -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        camera_direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::W) {
        camera_direction -= Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera_direction += Vec3::new(0.0, 0.0, 1.0);
    }

    camera_t.translation += camera_direction * time.delta_seconds() * 10.0;
}
