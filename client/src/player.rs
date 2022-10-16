use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(input);
    }
}

#[derive(Component)]
pub enum PlayerType {
    Local,
    Remote,
    NPC,
}

struct RobotAnimations {
    idle: Handle<AnimationClip>,
    walk: Handle<AnimationClip>,
}

// This function returns true when quat1 and quat2 are within n radians of each other
fn quat_within(quat1: Quat, quat2: Quat, n: f32) -> bool {
    let dot = quat1.dot(quat2);
    let angle = dot.acos() * 2.0;
    angle <= n
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = SceneBundle {
        scene: asset_server.load("Robot.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..Default::default()
    };

    commands
        .spawn_bundle(scene)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.5, 1.5, 1.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(PlayerType::Local);

    commands.insert_resource(RobotAnimations {
        idle: asset_server.load("Robot.glb#Animation2"),
        walk: asset_server.load("Robot.glb#Animation10"),
    });
}

fn input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &PlayerType)>,
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

    for (mut player_trsfrm, player_type) in player_query.iter_mut() {
        match player_type {
            PlayerType::Local => {
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

                if direction == Vec3::ZERO && (any_released || any_pressed) {
                    for mut player in player_animations.iter_mut() {
                        player.play(animations.idle.clone()).repeat();
                    }
                    return;
                }

                if any_pressed {
                    for mut player in player_animations.iter_mut() {
                        player.play(animations.walk.clone()).repeat();
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
                        player_trsfrm.rotation =
                            player_trsfrm.rotation.lerp(target_rotation, lerp_t);
                    }
                }
            }
            _ => {}
        }
    }
}
