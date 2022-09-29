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
    mut lights: Query<(&mut PointLight, &mut Visibility, &mut GateLock)>,
) {
    for (mut light, mut visibility, mut lock) in lights.iter_mut() {
        if lock.unlocked {
            if lock.time_since_pressed < 3.25 {
                light.color = Color::rgb(1.0, 1.0, 1.0)
            } else {
                light.color = Color::rgb(0.0, 1.0, 0.0);
            }
            continue;
        }

        if lock.pressed {
            if lock.time_since_pressed == 0.0 {
                visibility.is_visible = true;
            }

            lock.time_since_pressed += time.delta_seconds();
            if lock.time_since_pressed > 3.0 {
                lock.pressed = false;
                lock.unlocked = true;
                continue
            }

            // slowly turn green as the gate is unlocked
            let percent_unlocked_radians = (lock.time_since_pressed / 3.0) * (std::f32::consts::PI / 2.0);
            light.color = Color::rgb(0.0, 0.0, 1.0)
                + Color::rgb(0.0, 1.0, 0.0) * percent_unlocked_radians.sin() 
                + Color::rgb(0.0, 0.0, 1.0) * -percent_unlocked_radians.sin();
            continue;
        } else {
            if lock.time_since_pressed > 0.0 {
                visibility.is_visible = false;
                lock.time_since_pressed = 0.0;
            }
        }

    }
}