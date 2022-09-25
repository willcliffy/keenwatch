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
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let ground_size = 75.0;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
        ..default()
    }).insert(Collider::cuboid(ground_size / 2.0, 0.0, ground_size / 2.0));

    let mr_robot = asset_server.load("Robot.glb#Scene0");
    commands.spawn_bundle(SceneBundle {
        scene: mr_robot,
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.5, 1.5, 1.5))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(Player);

    commands.insert_resource(RobotAnimations {
        idle: asset_server.load("Robot.glb#Animation2"),
        walk: asset_server.load("Robot.glb#Animation10"),
    });
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 500.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 48.0, 36.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // if players collide with this cube, they will be teleported to the other side of the map
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.5, 0.3, 0.5).into()),
        transform: Transform::from_xyz(35.0, 0.0, 0.0),
        ..default()
    })
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(Sensor);
}

fn input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet<(
        Query<(&mut Transform, &Player)>,
        Query<(&mut Transform, &Camera)>,
        Query<(&mut Transform, &DirectionalLight)>
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

    let mut light_direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Q) {
        light_direction -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::E) {
        light_direction += Vec3::new(1.0, 0.0, 0.0);
    }

    let mut camera = set.p1();
    let (mut camera_t, _) = camera.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        camera_t.translation = Vec3::new(camera_t.translation.x, 0.0, camera_t.translation.z);
        *camera_t = camera_t.looking_at(Vec3::ZERO, Vec3::Y);
    } else {
        camera_t.translation += direction * time.delta_seconds() * 10.0;
        camera_t.translation += camera_direction * time.delta_seconds() * 10.0;
    }


    let mut light = set.p2();
    for (mut light_t, _) in light.iter_mut() {
        light_t.translation += light_direction * time.delta_seconds() * 10.0;
    }

    let mut player = set.p0();
    let (mut player_t, _) = player.single_mut();
    player_t.translation += direction * time.delta_seconds() * 10.0;

    if direction == Vec3::ZERO && keyboard_input.any_just_released([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]) {
        for mut player in player_animations.iter_mut() {
            player.play(animations.idle.clone()).repeat();
        }
        return
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

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}