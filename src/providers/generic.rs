use std::collections::BTreeMap;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::Material2d,
};

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
    // for params
    const MAX: f32;
    const MIN: f32;
    const DELTA: f32;

    // for other 2 params
    const X_MAX: f32;
    const X_DELTA: f32;
    const Z_MAX: f32;
    const Z_DELTA: f32;

    // length of rect
    const DX: f32 = Self::X_DELTA * Self::X_DELTA / Self::X_MAX;
    const DZ: f32 = Self::Z_DELTA * Self::Z_DELTA / Self::Z_MAX;

    // normalized values
    const X_DELTA_N: f32 = 1. / Self::X_MAX;
    const Z_DELTA_N: f32 = 1. / Self::Z_MAX;

    /// perform changes onto the provider
    fn incr(&mut self, change: f32);
    /// perform changes onto the provider
    fn decr(&mut self, change: f32);
    /// set new param (unadjusted), [0,1]
    fn set(&mut self, new: f32);
    /// give current value
    fn current(&self) -> f32;

    /// return data point and if it's relevant wrt current param
    fn inspect(&mut self, img: &Image, i: u32, j: u32) -> (bool, (i64, i64));

    /// draw 3d viz mesh
    fn create_mesh(&mut self, img: &Image) -> Mesh {
        // collect distribution
        let mut stats: BTreeMap<(i64, i64), i64> = BTreeMap::new();
        let w = img.width();
        let h = img.height();
        // this look takes the most time
        for i in 0..w {
            for j in 0..h {
                let (relevant, data) = self.inspect(img, i, j);
                if relevant {
                    stats.insert(
                        data,
                        stats.get(&data).map(|i| i.to_owned() + 1).unwrap_or(1),
                    );
                }
            }
        }

        let max_ = *stats.iter().max_by_key(|x| x.1).unwrap_or((&(0, 0), &0)).1;
        let max = if max_ == 0 { 1 } else { max_ } as f32;
        let mut vtxs: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let (mut i, mut j, mut k) = (0, 0, 0);

        // add mesh vertexes and indices
        while i <= Self::X_MAX as i64 {
            while j <= Self::Z_MAX as i64 {
                // draw cube
                let base_x = i as f32 * Self::X_DELTA_N;
                let base_z = j as f32 * Self::Z_DELTA_N;
                let y = *stats.get(&(i, j)).unwrap_or(&0) as f32 / max;
                // top 4
                let mut top_vtxs = vec![
                    [base_x - 0.5, y, base_z - 0.5],
                    [base_x + Self::DX - 0.5, y, base_z - 0.5],
                    [base_x - 0.5, y, base_z + Self::DZ - 0.5],
                    [base_x + Self::DX - 0.5, y, base_z + Self::DZ - 0.5],
                ];
                vtxs.append(&mut top_vtxs);
                // bottom 4
                let mut bot_vtxs = vec![
                    [base_x - 0.5, 0., base_z - 0.5],
                    [base_x + Self::DX - 0.5, 0., base_z - 0.5],
                    [base_x - 0.5, 0., base_z + Self::DZ - 0.5],
                    [base_x + Self::DX - 0.5, 0., base_z + Self::DZ - 0.5],
                ];
                vtxs.append(&mut bot_vtxs);

                // top
                indices.append(&mut vec![k + 1, k, k + 3]);
                indices.append(&mut vec![k + 3, k, k + 2]);
                // right
                indices.append(&mut vec![k + 7, k + 3, k + 6]);
                indices.append(&mut vec![k + 6, k + 3, k + 2]);
                // left
                indices.append(&mut vec![k + 1, k + 5, k + 4]);
                indices.append(&mut vec![k + 4, k + 0, k + 1]);
                // back
                indices.append(&mut vec![k + 3, k + 7, k + 5]);
                indices.append(&mut vec![k + 5, k + 1, k + 3]);
                // front
                indices.append(&mut vec![k + 0, k + 4, k + 6]);
                indices.append(&mut vec![k + 6, k + 2, k + 0]);

                k += 8;
                j += Self::Z_DELTA as i64;
            }
            // HACK: assuming Y_MIN is 0 here
            j = 0;
            i += Self::X_DELTA as i64;
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vtxs)
        .with_inserted_indices(Indices::U32(indices))
    }

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
