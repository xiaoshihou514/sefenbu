use std::collections::BTreeMap;

use bevy::prelude::*;
pub trait Provider {
    /// returns the value for histogram given a pixel
    fn convert(&self, pixel: Color) -> i64;
    /// the max value for the key field
    fn max(&self) -> f32;
    /// the min value for the key field
    fn min(&self) -> f32;
    /// the delta value for the provider
    fn delta(&self) -> f32;
    /// perform changes onto the provider
    fn incr(&mut self, change: f32);
    /// perform changes onto the provider
    fn decr(&mut self, change: f32);
    /// give current value
    fn current(&self) -> f32;
    /// draw 3d viz mesh
    fn create_mesh(&self, img: &Image) -> Mesh;

    fn histogram_data(&self, img: &Image) -> Vec<(f32, f32)> {
        let mut result: BTreeMap<i64, i64> = BTreeMap::new();
        let w = img.width();
        let h = img.height();
        for i in 0..w {
            for j in 0..h {
                let c = img.get_color_at(i, j).unwrap();
                let h = self.convert(c);
                result.insert(h, result.get(&h).map(|i| i.to_owned() + 1).unwrap_or(1));
            }
        }
        // FIXME: might have accuracy issue
        result
            .iter()
            .map(|(x, y)| (*x as f32, *y as f32 / (w * h) as f32))
            .collect()
    }
}
