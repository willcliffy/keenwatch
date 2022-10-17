use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod camera;
mod gate;
mod gate_lock;
mod map;
mod npc;
mod player;

pub struct KeenwatchPluginGroup;

impl PluginGroup for KeenwatchPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(player::PlayerPlugin);
        group.add(map::MapPlugin);
        group.add(camera::KeenwatchCameraPlugin);
        group.add(gate::GatePlugin);
        group.add(npc::NpcPlugin);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(KeenwatchPluginGroup)
        .add_system(link_animations) // see below
        .run();
}

// Going to choose to keep hacks in main so that I can easily remove them later

// This solves a problem with multiple AnimationPlayers. You can't query the AnimationPlayer component
// along with Player components so you need to tie the AnimationPlayer to the Player entity using
// an additional component.
// See: https://github.com/bevyengine/bevy/discussions/5564
#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    mut commands: Commands,
) {
    for entity in player_query.iter() {
        commands
            .entity(get_top_parent(entity, &parent_query))
            .insert(AnimationEntityLink(entity.clone()));
    }
}
