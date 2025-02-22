mod cli;
mod controls;
mod scene;

use bevy::prelude::*;
use clap::Parser;
use controls::rotate_blob;
use scene::setup_scene;

fn main() {
    let _ = cli::Cli::parse();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, rotate_blob)
        .run();
}
