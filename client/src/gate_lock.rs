use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{gate::GateSide, map::MapOptions, player::PlayerType};

#[derive(Component)]
pub struct LockPlugin;

impl Plugin for LockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GateLockUnlockEvent>()
            .add_startup_system(setup)
            .add_system(animate)
            .add_system(emit_gatelock_unlocked_event)
            .add_system_to_stage(CoreStage::PostUpdate, handle_collisions);
    }
}

#[derive(Component, Default)]
pub struct GateLock {
    state: GateLockState,
    time_since_pressed: f32,
}

#[derive(Default, PartialEq)]
enum GateLockState {
    Unlocked,
    Unlocking,
    #[default]
    Locked,
}

fn setup(mut commands: Commands, map_options: Res<MapOptions>) {
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            visibility: Visibility {
                is_visible: false,
                ..default()
            },
            transform: Transform::from_xyz(0.5 * map_options.wall_width - 10.0, 2.0, -15.0),
            ..default()
        })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock {
            state: GateLockState::Locked,
            time_since_pressed: 0.0,
        })
        .insert(GateSide::East)
        .insert(Name::new("Gate Lock NE"));

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            visibility: Visibility {
                is_visible: false,
                ..default()
            },
            transform: Transform::from_xyz(0.5 * map_options.wall_width - 10.0, 2.0, 15.0),
            ..default()
        })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock {
            state: GateLockState::Locked,
            time_since_pressed: 0.0,
        })
        .insert(GateSide::East)
        .insert(Name::new("Gate Lock SE"));

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            visibility: Visibility {
                is_visible: false,
                ..default()
            },
            transform: Transform::from_xyz(0.5 * -map_options.wall_width + 10.0, 2.0, -15.0),
            ..default()
        })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock {
            state: GateLockState::Locked,
            time_since_pressed: 0.0,
        })
        .insert(GateSide::West)
        .insert(Name::new("Gate Lock NW"));

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            visibility: Visibility {
                is_visible: false,
                ..default()
            },
            transform: Transform::from_xyz(0.5 * -map_options.wall_width + 10.0, 2.0, 15.0),
            ..default()
        })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock {
            state: GateLockState::Locked,
            time_since_pressed: 0.0,
        })
        .insert(GateSide::West)
        .insert(Name::new("Gate Lock SW"));
}

fn animate(time: Res<Time>, mut lights: Query<(&mut PointLight, &mut GateLock)>) {
    for (mut light, mut lock) in lights.iter_mut() {
        match lock.state {
            GateLockState::Unlocked => {
                light.color = Color::rgb(1.0, 1.0, 1.0);
            }
            GateLockState::Unlocking => {
                lock.time_since_pressed += time.delta_seconds();

                // slowly turn green as the gate is unlocked
                let percent_unlocked_radians =
                    (lock.time_since_pressed / 3.0) * (std::f32::consts::PI / 2.0);
                light.color = Color::rgb(0.0, 0.0, 1.0)
                    + Color::rgb(0.0, 1.0, 0.0) * percent_unlocked_radians.sin()
                    + Color::rgb(0.0, 0.0, 1.0) * -percent_unlocked_radians.sin();
            }
            GateLockState::Locked => {}
        }
    }
}

fn handle_collisions(
    player: Query<&Transform, With<PlayerType>>,
    mut gates: Query<(&mut GateLock, &mut Visibility)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                if !player.get(*entity_a).is_ok() && !player.get(*entity_b).is_ok() {
                    continue;
                }

                println!(
                    "Player collision started between {:?} and {:?}",
                    entity_a, entity_b
                );

                if let Ok((mut gate_lock, mut visibility)) = gates.get_mut(*entity_a) {
                    if gate_lock.state == GateLockState::Locked {
                        println!("Gate lock {:?} pressed", entity_a);
                        gate_lock.state = GateLockState::Unlocking;
                        visibility.is_visible = true;
                    }
                }

                if let Ok((mut gate_lock, mut visibility)) = gates.get_mut(*entity_b) {
                    if gate_lock.state == GateLockState::Locked {
                        println!("Gate lock {:?} pressed", entity_b);
                        gate_lock.state = GateLockState::Unlocking;
                        visibility.is_visible = true;
                    }
                }
            }
            CollisionEvent::Stopped(entity_a, entity_b, _) => {
                if !player.get(*entity_a).is_ok() && !player.get(*entity_b).is_ok() {
                    continue;
                }

                println!(
                    "Player collision started between {:?} and {:?}",
                    entity_a, entity_b
                );

                if let Ok((mut gate_lock, mut visibility)) = gates.get_mut(*entity_a) {
                    if gate_lock.state == GateLockState::Unlocking {
                        println!("Gate lock {:?} stopped", entity_a);
                        gate_lock.state = GateLockState::Locked;
                        gate_lock.time_since_pressed = 0.0;
                        visibility.is_visible = false;
                    }
                }

                if let Ok((mut gate_lock, mut visibility)) = gates.get_mut(*entity_b) {
                    if gate_lock.state == GateLockState::Unlocking {
                        println!("Gate lock {:?} stopped", entity_b);
                        gate_lock.state = GateLockState::Locked;
                        gate_lock.time_since_pressed = 0.0;
                        visibility.is_visible = false;
                    }
                }
            }
        }
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct GateLockUnlockEvent {
    pub side: GateSide,
}

fn emit_gatelock_unlocked_event(
    mut locks: Query<(&mut GateLock, &GateSide)>,
    mut event_writer: EventWriter<GateLockUnlockEvent>,
) {
    for (mut lock, direction) in locks.iter_mut() {
        if lock.state != GateLockState::Unlocking {
            continue;
        }

        if lock.time_since_pressed > 3.0 {
            lock.state = GateLockState::Unlocked;
            event_writer.send(GateLockUnlockEvent { side: *direction });
        }
    }
}
