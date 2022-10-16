use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameComponents;

#[derive(Component)]
pub struct Player;

pub struct RobotAnimations {
    idle: Handle<AnimationClip>,
    walk: Handle<AnimationClip>,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_components: ResMut<GameComponents>,
) {
    let scene = SceneBundle {
        scene: asset_server.load("Robot.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..Default::default()
    };

    let entity = commands.spawn_bundle(scene)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.5, 1.5, 1.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Player)
        .id();

    commands.insert_resource(RobotAnimations {
        idle: asset_server.load("Robot.glb#Animation2"),
        walk: asset_server.load("Robot.glb#Animation10"),
    });

    game_components.player_entity_id = entity.id();
}

pub fn handle_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    mut player_animations: Query<&mut AnimationPlayer, &AnimationPlayer>,
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

    let (_, mut player_t) = player_query.single_mut();
    player_t.translation += direction * time.delta_seconds() * 15.0;

    let any_pressed = keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]);
    let any_released = keyboard_input.any_just_released([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]);

    if direction == Vec3::ZERO && (any_released || any_pressed) {
        for mut player in player_animations.iter_mut() {
            player.play(animations.idle.clone()).repeat();
        }
        return
    }

    if any_pressed {
        for mut player in player_animations.iter_mut() {
            player.play(animations.walk.clone()).repeat();
        }
    }

    if player_t.rotation != Quat::from_rotation_y(direction.x.atan2(direction.z)) && direction != Vec3::ZERO {
        let max_rotations = 0.25; // in radians
        let lerp_t = 0.25; // f32 between 0 and 1
        let target_rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));

        if quat_within(player_t.rotation, target_rotation, max_rotations) {
            player_t.rotation = target_rotation;
        } else {
            player_t.rotation = player_t.rotation.lerp(target_rotation, lerp_t);
        }
    }
}

// This function returns true when quat1 and quat2 are within n radians of each other
fn quat_within(quat1: Quat, quat2: Quat, n: f32) -> bool {
    let dot = quat1.dot(quat2);
    let angle = dot.acos() * 2.0;
    angle <= n
}
