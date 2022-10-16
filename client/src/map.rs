use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapOptions {
            ground_color: Color::rgb(112.0 / 256.0, 128.0 / 256.0, 144.0 / 256.0),
            ground_length: 100.0,
            ground_width: 125.0,

            wall_color: Color::rgb(0.3, 0.3, 0.3),
            wall_length: 100.0,
            wall_width: 125.0,
            wall_height: 15.0,
            wall_thickness: 1.0,

            goalroom_length: 30.0,
            goalroom_width: 30.0,
            goalroom_height: 15.0,
            goalroom_thickness: 1.0,
        })
        .add_startup_system(setup);
    }
}

pub struct MapOptions {
    ground_color: Color,
    ground_length: f32,
    ground_width: f32,

    wall_color: Color,
    pub wall_length: f32,
    pub wall_width: f32,
    pub wall_height: f32,
    pub wall_thickness: f32,

    pub goalroom_length: f32,
    pub goalroom_width: f32,
    pub goalroom_height: f32,
    pub goalroom_thickness: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_options: Res<MapOptions>,
) {
    // Lights
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(
            -std::f32::consts::FRAC_PI_2 + 0.2,
        )),
        ..default()
    });

    // Ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                // size should be 5 times the max of ground_length and map_options.ground_width
                size: 5.0 * map_options.ground_length.max(map_options.ground_width),
            })),
            material: materials.add(map_options.ground_color.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Collider::cuboid(
            (5.0 / 2.0) * map_options.ground_width,
            0.0,
            (5.0 / 2.0) * map_options.ground_length,
        ));

    // Walls

    // Front
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.wall_width,
                map_options.wall_height,
                map_options.wall_thickness,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                0.5 * map_options.wall_height,
                0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.wall_width,
            0.5 * map_options.wall_height,
            0.5 * map_options.wall_thickness,
        ));

    // Back
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.wall_width,
                map_options.wall_height,
                map_options.wall_thickness,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                0.5 * map_options.wall_height,
                0.5 * -map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.wall_width,
            0.5 * map_options.wall_height,
            0.5 * map_options.wall_thickness,
        ));

    // Left top
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                1.0 * map_options.wall_thickness,
                1.0 * map_options.wall_height,
                0.4 * map_options.wall_length,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * -map_options.wall_width,
                0.5 * map_options.wall_height,
                0.3 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.wall_thickness,
            0.5 * map_options.wall_height,
            0.2 * map_options.wall_length,
        ));

    // Left bottom
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                1.0 * map_options.wall_thickness,
                1.0 * map_options.wall_height,
                0.4 * map_options.wall_length,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * -map_options.wall_width,
                0.5 * map_options.wall_height,
                0.3 * -map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.wall_thickness,
            0.5 * map_options.wall_height,
            0.2 * map_options.wall_length,
        ));

    // Right top
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                1.0 * map_options.wall_thickness,
                1.0 * map_options.wall_height,
                0.4 * map_options.wall_length,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * map_options.wall_width,
                0.5 * map_options.wall_height,
                0.3 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.wall_thickness,
            0.5 * map_options.wall_height,
            0.2 * map_options.wall_length,
        ));

    // Right bottom
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                1.0 * map_options.wall_thickness,
                1.0 * map_options.wall_height,
                0.4 * map_options.wall_length,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * map_options.wall_width,
                0.5 * map_options.wall_height,
                0.3 * -map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.wall_thickness,
            0.5 * map_options.wall_height,
            0.2 * map_options.wall_length,
        ));

    // Right goal room
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: map_options.goalroom_length,
            })),
            material: materials.add(map_options.ground_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * map_options.wall_width + 0.5 * map_options.goalroom_width,
                0.0,
                0.5 * map_options.wall_length - 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_length,
            0.0,
            0.5 * map_options.goalroom_width,
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_length,
                map_options.goalroom_height,
                map_options.goalroom_thickness,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * map_options.wall_width + 0.5 * map_options.goalroom_width,
                0.5 * map_options.goalroom_height,
                0.5 * -map_options.wall_length
                    + 0.5 * map_options.goalroom_length
                    + 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_length,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_thickness,
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_thickness,
                map_options.goalroom_height,
                map_options.goalroom_length,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * map_options.wall_width + map_options.goalroom_width,
                0.5 * map_options.goalroom_height,
                0.5 * -map_options.wall_length + 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_thickness,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_length,
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_length,
                map_options.goalroom_height,
                map_options.goalroom_thickness,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * map_options.wall_width + 0.5 * map_options.goalroom_width,
                0.5 * map_options.goalroom_height,
                0.5 * -map_options.wall_length - 0.5 * map_options.goalroom_length
                    + 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_length,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_thickness,
        ));

    // Left goal room - a plane for the ground plus three walls
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: map_options.goalroom_length,
            })),
            material: materials.add(map_options.ground_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * -map_options.wall_width - 0.5 * map_options.goalroom_width,
                0.0,
                0.5 * -map_options.wall_length + 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_length,
            0.0,
            0.5 * map_options.goalroom_length,
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_length,
                map_options.goalroom_height,
                map_options.goalroom_thickness,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * -map_options.wall_width - 0.5 * map_options.goalroom_width,
                0.5 * map_options.goalroom_height,
                0.5 * -map_options.wall_length
                    + 0.5 * map_options.goalroom_length
                    + 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_length,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_thickness,
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_thickness,
                map_options.goalroom_height,
                map_options.goalroom_length,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * -map_options.wall_width - map_options.goalroom_width,
                0.5 * map_options.goalroom_height,
                0.5 * -map_options.wall_length + 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_thickness,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_length,
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                map_options.goalroom_length,
                map_options.goalroom_height,
                map_options.goalroom_thickness,
            ))),
            material: materials.add(map_options.wall_color.into()),
            transform: Transform::from_translation(Vec3::new(
                0.5 * -map_options.wall_width - 0.5 * map_options.goalroom_width,
                0.5 * map_options.goalroom_height,
                0.5 * -map_options.wall_length - 0.5 * map_options.goalroom_length
                    + 0.5 * map_options.wall_length,
            )),
            ..default()
        })
        .insert(Collider::cuboid(
            0.5 * map_options.goalroom_length,
            0.5 * map_options.goalroom_height,
            0.5 * map_options.goalroom_thickness,
        ));
}
