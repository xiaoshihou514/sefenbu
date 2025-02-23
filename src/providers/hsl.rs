use bevy::color::Srgba;

use super::histgen::{ColorMask, HistogramGen};

struct Hsl;
impl HistogramGen for Hsl {
    fn within_mask(saturation: f32, pixels: Vec<Srgba>) -> ColorMask {
        todo!();
    }
}
