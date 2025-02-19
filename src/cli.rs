use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    /// Input image
    #[arg(short, long, num_args = 1, value_name = "FILE")]
    file: PathBuf,

    /// Color space to use
    #[arg(short, long, value_name = "COLOR SPACE")]
    using: Option<String>,
}
