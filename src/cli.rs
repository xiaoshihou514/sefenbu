use bevy::prelude::Resource;
use clap::Parser;

#[derive(Parser, Resource)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    /// Input image
    #[arg(value_name = "FILE")]
    pub file: String,
}
