use bevy::color::Srgba;

use super::histgen::HistogramGen;

struct Hsv;
impl HistogramGen for Hsv {
    fn within_mask(param: f32, pixels: Vec<Srgba>) -> super::histgen::ColorMask {
        todo!()
    }
}
