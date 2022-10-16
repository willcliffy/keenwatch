use bevy::{input::mouse::MouseWheel, prelude::*};

use super::player::PlayerType;

#[derive(Component)]
pub struct KeenwatchCameraPlugin;

impl Plugin for KeenwatchCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(handle_input);
    }
}

#[derive(Component)]
struct KeenwatchCamera {
    zoom: f32,
    zoom_speed: f32,
    zoom_dx_dz: f32,
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 100.0, 80.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(KeenwatchCamera {
            zoom: 80.0,
            zoom_speed: 500.0,
            zoom_dx_dz: 0.8,
        });
}

fn handle_input(
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut KeenwatchCamera, &mut Transform), With<Camera>>,
    player: Query<&Transform, (With<PlayerType>, Without<Camera>)>,
) {
    let player_transform = player.single();
    let (mut cam, mut camera_t) = camera_query.single_mut();
    camera_t.translation =
        player_transform.translation.clone() + Vec3::new(0.0, cam.zoom, cam.zoom * cam.zoom_dx_dz);

    for ev in scroll_evr.iter() {
        cam.zoom -= ev.y * cam.zoom_speed * time.delta_seconds();
    }
}
