use std::collections::BTreeMap;

use bevy::{prelude::*, sprite::Material2d};

pub trait CSpaceProvider: Provider + Resource + FromImage {
    type FilterMaterial: Material2d + FromImage;
    type Viz2dMaterial: Material2d + Default;
    type Viz3dMaterial: Material + Default;

    fn get_filter(&self) -> Self::FilterMaterial;
    fn get_viz2d_material(&self) -> Self::Viz2dMaterial;
    fn get_viz3d_material(&self) -> Self::Viz3dMaterial;
}

pub trait FromImage {
    fn from_image(img: Handle<Image>) -> Self;
}

pub trait Provider {
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
    /// set new param (unadjusted), [0,1]
    fn set(&mut self, new: f32);
    /// give current value
    fn current(&self) -> f32;

    /// draw 3d viz mesh
    fn create_mesh(&mut self, img: &Image) -> Mesh;

    /// returns the value for histogram given a pixel
    fn convert(&self, pixel: Color) -> i64;
    /// returns 2d histogram data for given image
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
