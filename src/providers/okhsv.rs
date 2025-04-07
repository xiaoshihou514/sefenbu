use std::collections::{BTreeMap, HashMap};

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

use super::oklab_common::{Hsv, Lab};

// global state
#[derive(Resource)]
pub struct OkhsvProvider {
    pub filter: OkhsvMaterial,
    pub viz2d_material: Okhsv2DVizMaterial,
}

const OKHSV_DELTA: f32 = 2.;
impl OkhsvProvider {
    #[rustfmt::skip]
    pub fn max(&self) -> f32 { 360. }
    #[rustfmt::skip]
    pub fn min(&self) -> f32 { 0. }
    #[rustfmt::skip]
    pub fn delta(&self) -> f32 { OKHSV_DELTA }

    pub fn decr(&mut self, change: f32) {
        self.filter.h -= change;
        // overflow protection
        self.filter.h = self.filter.h.max(0.);
        self.viz2d_material.h -= change;
        // overflow protection
        self.viz2d_material.h = self.filter.h.max(0.);
    }

    pub fn incr(&mut self, change: f32) {
        self.filter.h += change;
        // overflow protection
        self.filter.h = self.filter.h.min(360.);
        self.viz2d_material.h += change;
        // overflow protection
        self.viz2d_material.h = self.filter.h.min(360.);
    }

    // returns a vector of (value, proportion) pair
    // TODO: abstract
    pub fn histogram_data(&self, img: &Image) -> Vec<(f32, f32)> {
        let mut result: BTreeMap<i64, i64> = BTreeMap::new();
        let w = img.width();
        let h = img.height();
        for i in 0..w {
            for j in 0..h {
                let c: Oklaba = img.get_color_at(i, j).unwrap().into();
                let okhsv: Hsv = Hsv::from(&Lab {
                    L: c.lightness,
                    a: c.a,
                    b: c.b,
                });
                // FIXME: might have accuracy issue
                let h = (okhsv.h * 360. / self.delta()) as i64 * (self.delta() as i64);
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
        return OkhsvMaterial {
            h,
            delta: OKHSV_DELTA,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        };
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
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl Okhsv2DVizMaterial {
    pub fn new(h: f32, image: Handle<Image>) -> Okhsv2DVizMaterial {
        return Okhsv2DVizMaterial {
            h,
            delta: OKHSV_DELTA,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        };
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
