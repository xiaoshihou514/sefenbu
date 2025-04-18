use std::collections::BTreeMap;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{encase::rts_array::Length, AsBindGroup, ShaderRef},
    },
    sprite::{AlphaMode2d, Material2d},
};

use crate::COLOR_3D_VIZ_COORD;

use super::{
    generic::Provider,
    oklab_common::{Hsv, Lab},
};

// global state
#[derive(Resource)]
pub struct OkhsvProvider {
    pub filter: OkhsvMaterial,
    pub viz2d_material: Okhsv2DVizMaterial,
    pub viz3d_material: Okhsv3DVizMaterial,
}

fn to_okhsv(c: Color) -> Hsv {
    let lab: Oklaba = c.into();
    Hsv::from(&Lab {
        L: lab.lightness,
        a: lab.a,
        b: lab.b,
    })
}

const OKHSV_H_DELTA: f32 = 2.;
const OKHSV_H_MAX: f32 = 360.;
const OKHSV_H_MIN: f32 = 0.;
const OKHSV_SV_MAX: f32 = 100.;
const OKHSV_SV_DELTA: f32 = 4.;
const OKHSV_SV_DELTA_I: i64 = OKHSV_SV_DELTA as i64;
const OKHSV_SV_DELTA_N: f32 = OKHSV_SV_DELTA / OKHSV_SV_MAX;
const OKHSV_SV_SQ_SZ: f32 = OKHSV_SV_DELTA / OKHSV_SV_MAX * OKHSV_SV_DELTA;
const OKHSV_SV_STEPS: i64 = OKHSV_SV_MAX as i64 / OKHSV_SV_DELTA as i64;
impl Provider for OkhsvProvider {
    #[rustfmt::skip]
    fn max(&self) -> f32 { OKHSV_H_MAX }
    #[rustfmt::skip]
    fn min(&self) -> f32 { OKHSV_H_MIN }
    #[rustfmt::skip]
    fn delta(&self) -> f32 { OKHSV_H_DELTA }
    #[rustfmt::skip]
    fn current(&self) -> f32 { self.filter.h }

    fn decr(&mut self, change: f32) {
        // overflow protection
        self.filter.h = OKHSV_H_MIN.max(self.filter.h - change);
        self.viz2d_material.h = OKHSV_H_MIN.max(self.viz2d_material.h - change);
        self.viz3d_material.h = OKHSV_H_MIN.max(self.viz3d_material.h - change);
    }

    fn incr(&mut self, change: f32) {
        // overflow protection
        self.filter.h = OKHSV_H_MAX.min(self.filter.h + change);
        self.viz2d_material.h = OKHSV_H_MAX.min(self.viz2d_material.h + change);
        self.viz3d_material.h = OKHSV_H_MAX.min(self.viz3d_material.h + change);
    }

    fn convert(&self, c: Color) -> i64 {
        let okhsv: Hsv = to_okhsv(c);
        (okhsv.h * OKHSV_H_MAX / OKHSV_H_DELTA) as i64 * (OKHSV_H_DELTA as i64)
    }

    fn create_mesh(&self, img: &Image) -> Mesh {
        // collect distribution
        let mut stats: BTreeMap<(i64, i64), i64> = BTreeMap::new();
        let w = img.width();
        let h = img.height();
        let current = self.current();
        // this look takes the most time
        for i in 0..w {
            for j in 0..h {
                let okhsv = to_okhsv(img.get_color_at(i, j).unwrap());
                if (okhsv.h * OKHSV_H_MAX - current).abs() > OKHSV_H_DELTA {
                    // within range, increment count
                    let key = (
                        (okhsv.s * OKHSV_SV_MAX / OKHSV_SV_DELTA) as i64 * (OKHSV_SV_DELTA as i64),
                        (okhsv.v * OKHSV_SV_MAX / OKHSV_SV_DELTA) as i64 * (OKHSV_SV_DELTA as i64),
                    );

                    stats.insert(key, stats.get(&key).map(|i| i.to_owned() + 1).unwrap_or(1));
                }
            }
        }

        let max_ = *stats.iter().max_by_key(|x| x.1).unwrap_or((&(0, 0), &0)).1;
        let max = if max_ == 0 { 1 } else { max_ } as f32;
        let mut vtxs: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let (mut i, mut j, mut k) = (0, 0, 0);

        // add mesh vertexes and indices
        while i < OKHSV_SV_STEPS {
            while j < OKHSV_SV_STEPS {
                // draw cube
                let base_x = i as f32 * OKHSV_SV_DELTA_N;
                let base_z = j as f32 * OKHSV_SV_DELTA_N;
                let y = *stats.get(&(i, j)).unwrap_or(&0) as f32 / max;
                // top 4
                let mut top_vtxs = vec![
                    [base_x - 0.5, y, base_z - 0.5],
                    [base_x + OKHSV_SV_SQ_SZ - 0.5, y, base_z - 0.5],
                    [base_x - 0.5, y, base_z + OKHSV_SV_SQ_SZ - 0.5],
                    [
                        base_x + OKHSV_SV_SQ_SZ - 0.5,
                        y,
                        base_z + OKHSV_SV_SQ_SZ - 0.5,
                    ],
                ];
                vtxs.append(&mut top_vtxs);
                // bottom 4
                let mut bot_vtxs = vec![
                    [base_x - 0.5, 0., base_z - 0.5],
                    [base_x + OKHSV_SV_SQ_SZ - 0.5, 0., base_z - 0.5],
                    [base_x - 0.5, 0., base_z + OKHSV_SV_SQ_SZ - 0.5],
                    [
                        base_x + OKHSV_SV_SQ_SZ - 0.5,
                        0.,
                        base_z + OKHSV_SV_SQ_SZ - 0.5,
                    ],
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
                j += OKHSV_SV_DELTA_I;
            }
            j = 0;
            i += OKHSV_SV_DELTA_I;
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vtxs)
        .with_inserted_indices(Indices::U32(indices))
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OkhsvMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl OkhsvMaterial {
    pub fn new(h: f32, image: Handle<Image>) -> OkhsvMaterial {
        OkhsvMaterial {
            h,
            delta: OKHSV_H_DELTA,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for OkhsvMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsv.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Okhsv2DVizMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    _alpha_mode: AlphaMode2d,
}

impl Okhsv2DVizMaterial {
    pub fn new(h: f32) -> Self {
        Okhsv2DVizMaterial {
            h,
            delta: OKHSV_H_DELTA,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for Okhsv2DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsv_2dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Okhsv3DVizMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    #[uniform(2)]
    pub bottom: Vec3,
    _alpha_mode: AlphaMode,
}

impl Okhsv3DVizMaterial {
    pub fn new(h: f32) -> Self {
        Okhsv3DVizMaterial {
            h,
            delta: OKHSV_H_DELTA,
            bottom: COLOR_3D_VIZ_COORD - Vec3::new(0.5, 0.5, 0.5),
            _alpha_mode: AlphaMode::AlphaToCoverage,
        }
    }
}
impl Material for Okhsv3DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsv_3dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self._alpha_mode
    }
}
