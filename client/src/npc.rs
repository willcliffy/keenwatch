use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct NpcPlayer;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("Robot.glb#Scene0").clone(),
            transform: Transform::from_xyz(10.0, 1.5, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.5, 1.5, 1.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(NpcPlayer)
        .insert(Name::new("NPC"));
}
