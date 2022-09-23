use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player;

struct RobotAnimations {
    idle: Handle<AnimationClip>,
    walk: Handle<AnimationClip>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(input_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }).insert(Collider::cuboid(10.0, 0.1, 10.0));

    let mr_robot = asset_server.load("Robot.glb#Scene0");
    commands.spawn_bundle(SceneBundle {
        scene: mr_robot,
        transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(0.5 * Vec3::ONE),
        ..Default::default()
    }).insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(Restitution::coefficient(0.7))
    .insert(Player);

    commands.insert_resource(RobotAnimations {
        idle: asset_server.load("Robot.glb#Animation2"),
        walk: asset_server.load("Robot.glb#Animation10"),
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 5.0),
        ..default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 10.0, -5.0),
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 12.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet<(
        Query<(&mut Transform, &Player)>,
        Query<(&mut Transform, &Camera)>,
        Query<(&mut Transform, &PointLight)>
    )>,
    animations: Res<RobotAnimations>,
    mut player_animations: Query<&mut AnimationPlayer, &AnimationPlayer>,
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

    let mut camera = set.p1();
    let (mut camera_t, _) = camera.single_mut();
    camera_t.translation += direction * time.delta_seconds() * 5.0;

    let mut light = set.p2();
    for (mut light_t, _) in light.iter_mut() {
        light_t.translation += direction * time.delta_seconds() * 2.0;
    }

    let mut player = set.p0();
    let (mut player_t, _) = player.single_mut();
    player_t.translation += direction * time.delta_seconds() * 5.0;

    if direction == Vec3::ZERO && keyboard_input.any_just_released([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]) {
        for mut player in player_animations.iter_mut() {
            player.play(animations.idle.clone()).repeat();
        }
        return
    }

    if player_t.rotation != Quat::from_rotation_y(direction.x.atan2(direction.z)) && direction != Vec3::ZERO {
        let max_turn_angle = 0.25;
        let lerp_t = 0.25;

        if quat_within(player_t.rotation, Quat::from_rotation_y(direction.x.atan2(direction.z)), max_turn_angle) {
            player_t.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
        } else {
            player_t.rotation = player_t.rotation.lerp(Quat::from_rotation_y(direction.x.atan2(direction.z)), lerp_t);
        }
    }
    
    if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]) {
        for mut player in player_animations.iter_mut() {
            player.play(animations.walk.clone()).repeat();
        }
    }
}

// This function returns true when quat1 and quat2 are within n radians of each other
fn quat_within(quat1: Quat, quat2: Quat, n: f32) -> bool {
    let dot = quat1.dot(quat2);
    let angle = dot.acos() * 2.0;
    angle <= n
}
