use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod camera;
mod gate;
mod gate_lock;
mod map;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(camera::KeenwatchCameraPlugin)
        .add_plugin(gate::GatePlugin)
        .run();
}
