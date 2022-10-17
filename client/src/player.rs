use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::AnimationEntityLink;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(input)
            .add_system(setup_once_loaded);
    }
}

#[derive(Component, Default)]
pub struct LocalPlayer;

#[derive(Component, Default, Clone)]
pub struct RobotAnimations {
    pub idle: Handle<AnimationClip>,
    pub walk: Handle<AnimationClip>,
}

// This function returns true when quat1 and quat2 are within n radians of each other
fn quat_within(quat1: Quat, quat2: Quat, n: f32) -> bool {
    n >= 2.0 * quat1.dot(quat2).acos()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("Robot.glb#Scene0").clone(),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.5, 1.5, 1.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LocalPlayer)
        .insert(Name::new("Player"));

    commands.insert_resource(RobotAnimations {
        idle: asset_server.load("Robot.glb#Animation2"),
        walk: asset_server.load("Robot.glb#Animation10"),
    });
}

fn setup_once_loaded(
    animations: Res<RobotAnimations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in players.iter_mut() {
        player.play(animations.idle.clone()).repeat();
    }
}

// I should change this to use mouse clicks for navigation instead of WASD
fn input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &AnimationEntityLink), &LocalPlayer>,
    mut player_animations: Query<&mut AnimationPlayer>,
    animations: Res<RobotAnimations>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction -= Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction += Vec3::new(0.0, 0.0, 1.0);
    }

    for (mut player_trsfrm, animation_entity) in player_query.iter_mut() {
        player_trsfrm.translation += direction * time.delta_seconds() * 15.0;

        let any_pressed = keyboard_input.any_just_pressed([
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::Down,
        ]);
        let any_released = keyboard_input.any_just_released([
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::Down,
        ]);

        if any_released || any_pressed {
            if direction == Vec3::ZERO {
                if let Ok(mut player_animation) = player_animations.get_mut(animation_entity.0) {
                    player_animation.play(animations.idle.clone()).repeat();
                }
                return;
            }

            if let Ok(mut player_animation) = player_animations.get_mut(animation_entity.0) {
                player_animation.play(animations.walk.clone()).repeat();
            }
        }

        if player_trsfrm.rotation != Quat::from_rotation_y(direction.x.atan2(direction.z))
            && direction != Vec3::ZERO
        {
            let max_rotations = 0.25; // in radians
            let lerp_t = 0.25; // f32 between 0 and 1
            let target_rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));

            if quat_within(player_trsfrm.rotation, target_rotation, max_rotations) {
                player_trsfrm.rotation = target_rotation;
            } else {
                player_trsfrm.rotation = player_trsfrm.rotation.lerp(target_rotation, lerp_t);
            }
        }
    }
}
