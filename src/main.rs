mod cli;
mod controls;
mod providers;
mod scene;

use bevy::{prelude::*, sprite::Material2dPlugin};
use clap::Parser;
use cli::Cli;
use controls::*;
use providers::{
    generic::CSpaceProvider, hsl::HslProvider, hsv::HsvProvider, okhsl::OkhslProvider,
    okhsv::OkhsvProvider,
};
use scene::*;

fn main() {
    let args = Cli::parse();
    match args.using.clone().unwrap_or("okhsv".to_string()).as_str() {
        "okhsv" => app_run::<OkhsvProvider>(args),
        "okhsl" => app_run::<OkhslProvider>(args),
        "hsv" => app_run::<HsvProvider>(args),
        "hsl" => app_run::<HslProvider>(args),
        s => {
            println!("Did not recognize color space '{}'", s);
            std::process::exit(-1);
        }
    }
}

fn app_run<A: CSpaceProvider>(args: Cli)
where
    Material2dPlugin<A::FilterMaterial>: Plugin,
    Material2dPlugin<A::Viz2dMaterial>: Plugin,
    MaterialPlugin<A::Viz3dMaterial>: Plugin,
{
    let mut exe_path = std::env::current_exe().unwrap();
    if !(exe_path.pop()) {
        println!("Cannot pop exe path: {}", exe_path.to_string_lossy());
        std::process::exit(-1);
    }

    let default_plugin = DefaultPlugins
        .set(AssetPlugin {
            file_path: exe_path.to_string_lossy().into_owned(),
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
            Material2dPlugin::<A::FilterMaterial>::default(),
            Material2dPlugin::<A::Viz2dMaterial>::default(),
            MaterialPlugin::<A::Viz3dMaterial>::default(),
        ))
        .add_systems(Startup, setup_scene_pre::<A>)
        .add_systems(Update, draw_scene::<A>)
        .add_systems(Update, control_blob)
        .add_systems(Update, change_param::<A>)
        .add_systems(Update, set_viewports)
        .run();
}
