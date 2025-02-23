use std::collections::HashMap;

use bevy::color::Srgba;

pub struct ColorMask {
    /// (offset, (count, rgb))
    pub histogram: HashMap<(i32, i32), (u32, Srgba)>,
    /// pixels that are in the color space slice are preserved and the rest are mapped to
    /// transparent
    pub masked: Vec<Srgba>,
}

pub trait HistogramGen {
    /// Given `param`, return the masked pixels and statistics
    fn within_mask(param: f32, pixels: Vec<Srgba>) -> ColorMask;
}
