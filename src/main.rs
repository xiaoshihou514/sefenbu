mod cli;
mod controls;
mod scene;

use bevy::prelude::*;
use clap::Parser;
use controls::control_blob;
use scene::{draw_image, setup_scene};

fn main() {
    let args = cli::Cli::parse();
    let progopt: cli::ProgOpt = args.into();

    App::new()
        .insert_resource(progopt)
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: std::env::current_dir()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                ..Default::default()
            }),
        )
        .add_systems(Startup, setup_scene)
        .add_systems(Update, draw_image)
        .add_systems(Update, control_blob)
        .run();
}
