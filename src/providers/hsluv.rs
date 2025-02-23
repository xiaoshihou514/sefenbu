use bevy::color::Srgba;

use super::histgen::HistogramGen;

struct HsLuv;
impl HistogramGen for HsLuv {
    fn within_mask(param: f32, pixels: Vec<Srgba>) -> super::histgen::ColorMask {
        todo!()
    }
}
