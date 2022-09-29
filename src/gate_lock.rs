use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameComponents;

#[derive(Component)]
pub struct GateLock {
    pub unlocked: bool,
    pub pressed: bool,
    pub time_since_pressed: f32,
}

pub fn setup(
    mut commands: Commands,
    mut game_components: ResMut<GameComponents>,
) {
    // Gate 1
    let entity = commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        visibility: Visibility {
            is_visible: false,
            ..default()
        },
        transform: Transform::from_xyz(30.0, 0.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock { unlocked: false, pressed: false, time_since_pressed: 0.0 })
        .id();

    // Gate 2
    let entity = commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        visibility: Visibility {
            is_visible: false,
            ..default()
        },
        transform: Transform::from_xyz(30.0, 0.5, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock { unlocked: false, pressed: false, time_since_pressed: 0.0 })
        .id();

    // Gate 3
    let entity = commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        visibility: Visibility {
            is_visible: false,
            ..default()
        },
        transform: Transform::from_xyz(-30.0, 0.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock { unlocked: false, pressed: false, time_since_pressed: 0.0 })
        .id();

    // Gate 4
    let entity = commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        visibility: Visibility {
            is_visible: false,
            ..default()
        },
        transform: Transform::from_xyz(-30.0, 0.5, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(Collider::cylinder(1.0, 3.0))
        .insert(Sensor)
        .insert(GateLock { unlocked: false, pressed: false, time_since_pressed: 0.0 })
        .id();

    game_components.cube_entity_id = entity.id();
}

pub fn animate_lights(
    time: Res<Time>,
    mut lights: Query<(&mut PointLight, &mut GateLock)>,
) {
    for (mut light, mut lock) in lights.iter_mut() {
        if lock.unlocked {
            println!("unlocked");
            light.color = Color::rgb(0.0, 1.0, 0.0);
            light.range = 10.0;
            continue;
        }

        if lock.pressed {
            println!("Pressed");
            lock.time_since_pressed += time.delta_seconds();
            if lock.time_since_pressed > 3.0 {
                lock.pressed = false;
                lock.time_since_pressed = 0.0;
                lock.unlocked = true;
                continue
            }
            light.color = Color::rgb(0.0, 0.0, 1.0) + Color::rgb(0.5, 0.5, 0.0) * lock.time_since_pressed.sin();
            continue;
        }

        println!("nothing");
    }
}