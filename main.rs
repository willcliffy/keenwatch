use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use gate_lock::GateLock;

#[path = "src/camera.rs"] mod camera;
#[path = "src/player.rs"] mod player;
#[path = "src/gate_lock.rs"] mod gate_lock;

#[derive(Component)]
pub struct GameComponents {
    pub player_entity_id: u32,
    pub cube_entity_id: u32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(GameComponents {
            player_entity_id: 0,
            cube_entity_id: 0
        })
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup)
                .with_system(camera::setup)
                .with_system(player::setup)
                .with_system(gate_lock::setup)
        )
        .add_system_set(
            SystemSet::new()
                .with_system(camera::handle_input)
                .with_system(player::handle_input)
                .with_system(gate_lock::animate_lights)
        )
        .add_system_to_stage(CoreStage::PostUpdate, handle_collisions)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_size = 75.0;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    }).insert(Collider::cuboid(ground_size / 2.0, 0.0, ground_size / 2.0));

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 30000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..default()
    });
}

fn handle_collisions(
    game_components: Res<GameComponents>,
    mut gates: Query<(&mut GateLock, &mut Visibility)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                if entity_a.id() != game_components.player_entity_id && entity_b.id() != game_components.player_entity_id {
                    continue;
                }

                // TODO - this will break when I want the gates to act indivudually
                if entity_a.id() != game_components.cube_entity_id && entity_b.id() != game_components.cube_entity_id {
                    continue;
                }

                println!("Collision started between {:?} and {:?}", entity_a, entity_b);
                for (mut gate, mut gate_vis) in gates.iter_mut() { // started colliding with one of the gates - for now, this activates all gates
                    println!("Gate collision started");
                    gate.pressed = true;
                    gate_vis.is_visible = true;
                }
            }
            CollisionEvent::Stopped(entity_a, entity_b, _) => {
                if entity_a.id() != game_components.player_entity_id && entity_b.id() != game_components.player_entity_id {
                    continue;
                }

                // TODO - this will break when I want the gates to act indivudually
                if entity_a.id() != game_components.cube_entity_id && entity_b.id() != game_components.cube_entity_id {
                    continue;
                }

                for (mut gate, mut gate_vis) in gates.iter_mut() { // started colliding with one of the gates - for now, this activates all gates
                    println!("Gate collision stopped");
                    gate.pressed = false;
                    if !gate.unlocked {
                        gate_vis.is_visible = false;
                    }
                }
            }
        }
    }
}
