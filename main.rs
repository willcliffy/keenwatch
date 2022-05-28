use bevy::{
    prelude::*,
    core::FixedTimestep,
    math::Vec3Swizzles,
};
use bevy_ecs_tilemap::prelude::*;

use crate::src::{
    utils::camera,
    utils::tiled::*,
    utils::texture::set_texture_filters_to_nearest,
    player,
    player::Player,
};

mod src;

const INTERNAL_TIMESTEP: f64 = 60.0 / 60.0;
const CAMERA_SCALE: f32 = 0.2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Keenwatch"),
            ..Default::default()
        })
        .add_startup_system(startup)
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(set_texture_filters_to_nearest)
        .add_system(camera::movement)
        .add_system(player::update)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(INTERNAL_TIMESTEP))
            .with_system(gameloop))
        .run()
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.scale = CAMERA_SCALE;

    commands.spawn_bundle(camera);

    let handle: Handle<TiledMap> = asset_server.load("map3.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(10.0, 10.0, 0.0),
            scale: Vec3::new(8.0, 8.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 2.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player);
}

fn gameloop(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut map_query: MapQuery,
) {
    dbg!("tick");
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        transform.translation += time.delta_seconds() * direction * 50.;

        let mut position = transform.translation.xy().extend(1.0);
        position.y += 5.25; // Have calculation closer to player feet.
        let sprite_pos_z = map_query.get_zindex_for_pixel_pos(position, 0u16, 0u16);
        transform.translation.z = sprite_pos_z;
    }
}
