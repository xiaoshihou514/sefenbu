mod cli;
mod controls;
mod providers;
mod scene;

use bevy::{prelude::*, sprite::Material2dPlugin};
use clap::Parser;
use controls::*;
use providers::okhsv::{Okhsv2DVizMaterial, Okhsv3DVizMaterial, OkhsvMaterial};
use scene::*;

fn main() {
    let args = cli::Cli::parse();

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
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        })
        .set(bevy::log::LogPlugin::default());

    App::new()
        .insert_resource(args)
        .add_plugins((
            default_plugin,
            Material2dPlugin::<OkhsvMaterial>::default(),
            Material2dPlugin::<Okhsv2DVizMaterial>::default(),
            MaterialPlugin::<Okhsv3DVizMaterial>::default(),
        ))
        .add_systems(Startup, setup_scene_pre)
        .add_systems(Update, draw_scene)
        .add_systems(Update, control_blob)
        .add_systems(Update, change_param)
        .add_systems(Update, set_viewports)
        .run();
}
