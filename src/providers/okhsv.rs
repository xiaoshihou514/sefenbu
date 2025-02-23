use bevy::color::Srgba;

use super::histgen::HistogramGen;

struct OkHSV;
impl HistogramGen for OkHSV {
    fn within_mask(param: f32, pixels: Vec<Srgba>) -> super::histgen::ColorMask {
        todo!()
    }
}
