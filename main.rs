use bevy::{
    prelude::*,
    core::FixedTimestep,
};

pub const INTERNAL_TIMESTEP: f64 = 30.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(INTERNAL_TIMESTEP))
            .with_system(gameloop))
        .run()
}

fn gameloop() {
    println!("Hello, world!");
}
