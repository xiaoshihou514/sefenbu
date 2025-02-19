mod cli;

use bevy::prelude::*;
use clap::Parser;

fn main() {
    let _ = cli::Cli::parse();
    App::new().add_plugins(DefaultPlugins).run();
}
