use bevy::color::Srgba;

use super::histgen::HistogramGen;

struct OkHSL;
impl HistogramGen for OkHSL {
    fn within_mask(param: f32, pixels: Vec<Srgba>) -> super::histgen::ColorMask {
        todo!()
    }
}
