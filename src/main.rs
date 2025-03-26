mod cli;
mod controls;
mod providers;
mod scene;

use bevy::prelude::*;
use clap::Parser;
use controls::{change_param, control_blob};
use providers::okhsv::OkhsvMaterial;
use scene::{draw_image_await_load, setup_scene};

fn main() {
    let args = cli::Cli::parse();
    let progopt: cli::ProgOpt = args.into();

    let default_plugin = DefaultPlugins
        .set(AssetPlugin {
            file_path: std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            ..Default::default()
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "sefenbu".to_string(),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        })
        .set(bevy::log::LogPlugin::default());

    App::new()
        .insert_resource(progopt)
        .add_plugins((default_plugin, MaterialPlugin::<OkhsvMaterial>::default()))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, draw_image_await_load)
        .add_systems(Update, control_blob)
        .add_systems(Update, change_param)
        .run();
}
