use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::Collider;

use crate::{
    gate_lock::{GateLockUnlockEvent, LockPlugin},
    map::MapOptions,
};

#[derive(Component)]
pub struct GatePlugin;

impl Plugin for GatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LockPlugin)
            .add_startup_system(setup)
            .add_system(animate)
            .add_system(receive_gatelock_unlocked_event);
    }
}

#[derive(Inspectable, Default)]
enum GateState {
    Open,
    Opening,
    #[default]
    Closed,
}

#[derive(Component, Inspectable, Default, PartialEq, Clone, Copy)]
pub enum GateSide {
    #[default]
    East,
    West,
}

#[derive(Component, Inspectable, Default)]
struct Gate {
    state: GateState,
    side: GateSide,
    time_since_unlocked: f32,
    num_gates_unlocked: u32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_options: Res<MapOptions>,
) {
    let gate_east = Gate {
        state: GateState::Closed,
        side: GateSide::East,
        time_since_unlocked: 0.0,
        num_gates_unlocked: 0,
    };

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_thickness - 0.1,
                map_options.goalroom_height,
                map_options.goalroom_width,
            ))),
            material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                0.5 * map_options.wall_width,
                0.5 * map_options.wall_height - 0.1,
                0.0,
            ),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_thickness - 0.1,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_width,
        ))
        .insert(gate_east)
        .insert(Name::new("Gate East"));

    let gate_west = Gate {
        state: GateState::Closed,
        side: GateSide::West,
        time_since_unlocked: 0.0,
        num_gates_unlocked: 0,
    };

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_thickness - 0.1,
                map_options.goalroom_height,
                map_options.goalroom_width,
            ))),
            material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                0.5 * -map_options.wall_width,
                0.5 * map_options.wall_height - 0.1,
                0.0,
            ),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_thickness - 0.1,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_width,
        ))
        .insert(gate_west)
        .insert(Name::new("Gate West"));
}

fn animate(
    time: Res<Time>,
    map_options: Res<MapOptions>,
    mut gates: Query<(&mut Gate, &mut Transform)>,
) {
    for (mut gate, mut transform) in gates.iter_mut() {
        match gate.state {
            GateState::Open => {}
            GateState::Opening => {
                gate.time_since_unlocked += time.delta_seconds();

                if gate.time_since_unlocked > 4.0 {
                    gate.state = GateState::Open;
                    gate.time_since_unlocked = 0.0;
                    continue;
                }

                // lower the gate over 4 seconds
                transform.translation.y -= map_options.wall_height * time.delta_seconds() / 4.0;
            }
            GateState::Closed => {}
        }
    }
}

fn receive_gatelock_unlocked_event(
    mut gates: Query<&mut Gate>,
    mut gate_lock_events: EventReader<GateLockUnlockEvent>,
) {
    for event in gate_lock_events.iter() {
        match event.side {
            GateSide::East => {
                for mut gate in gates.iter_mut() {
                    if gate.side != GateSide::East {
                        continue;
                    }

                    gate.num_gates_unlocked += 1;
                    if gate.num_gates_unlocked == 2 {
                        gate.state = GateState::Opening;
                    }
                }
            }
            GateSide::West => {
                for mut gate in gates.iter_mut() {
                    if gate.side != GateSide::West {
                        continue;
                    }

                    gate.num_gates_unlocked += 1;
                    if gate.num_gates_unlocked == 2 {
                        gate.state = GateState::Opening;
                    }
                }
            }
        }
    }
}
