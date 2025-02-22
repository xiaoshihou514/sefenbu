use bevy::prelude::Resource;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    /// Input image
    #[arg(short, long, num_args = 1, value_name = "FILE")]
    pub file: String,

    /// Color space to use
    #[arg(short, long, value_name = "COLOR SPACE")]
    pub using: Option<String>,
}

pub enum ColorSpace {
    OKHSV,
    OKHSL,
    HSLuv,
    HSL,
    HSV,
}

impl From<String> for ColorSpace {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "okhsv" => ColorSpace::OKHSV,
            "okhsl" => ColorSpace::OKHSL,
            "hsluv" => ColorSpace::HSLuv,
            "hsl" => ColorSpace::HSL,
            "hsv" => ColorSpace::HSV,
            _ => panic!("Invalid color space"),
        }
    }
}

#[derive(Resource)]
pub struct ProgOpt {
    pub file: String,
    pub space: ColorSpace,
}

impl From<Cli> for ProgOpt {
    fn from(args: Cli) -> Self {
        ProgOpt {
            file: args.file,
            space: args.using.unwrap_or(String::from("okhsv")).into(),
        }
    }
}
